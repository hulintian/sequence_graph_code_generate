use std::collections::{HashMap, HashSet};
use serde::Deserialize;
use crate::ir::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagramInput {
    pub metadata: MetadataInput,
    pub lifelines: Vec<LifelineInput>,
    pub messages: Vec<MessageInput>,
    pub combined_fragments: Vec<FragmentInput>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataInput {
    pub id: String,
    pub name: String,
    pub code_gen_config: CodeGenConfigInput,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeGenConfigInput {
    pub output_dir: String,
    pub language: String,
    pub template_set: String,
    pub old_version_dir: Option<String>,
    pub namespace: String,
    pub lifeline_overrides: HashMap<String, LifelineOverride>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifelineOverride {
    pub output_dir: Option<String>,
    pub namespace: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifelineInput {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub lifeline_type: String,
    pub properties: LifelineProperties,
}

#[derive(Debug, Deserialize)]
pub struct LifelineProperties {
    pub stereotype: String,
    pub namespace: String,
    #[serde(default)]
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageInput {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub source_lifeline_id: String,
    pub target_lifeline_id: String,
    pub order_index: i32,
    #[serde(default)]
    pub arguments: Vec<ArgInput>,
    #[serde(default)]
    pub return_type: String,
    #[serde(default)]
    pub guard: String,
    pub parent_fragment_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArgInput {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FragmentInput {
    pub id: String,
    #[serde(rename = "type")]
    pub fragment_type: String,
    pub parent_fragment_id: Option<String>,
    pub operands: Vec<OperandInput>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperandInput {
    pub id: String,
    pub guard: String,
    pub message_ids: Vec<String>,
}

pub fn parse_diagram(input: &DiagramInput) -> Result<BehaviorIR, String> {
    let lifeline_map: HashMap<&str, &LifelineInput> = input
        .lifelines
        .iter()
        .map(|l| (l.id.as_str(), l))
        .collect();

    let fragment_map: HashMap<&str, &FragmentInput> = input
        .combined_fragments
        .iter()
        .map(|f| (f.id.as_str(), f))
        .collect();

    let msg_map: HashMap<&str, &MessageInput> = input
        .messages
        .iter()
        .map(|m| (m.id.as_str(), m))
        .collect();

    let mut sorted_messages: Vec<&MessageInput> = input.messages.iter().collect();
    sorted_messages.sort_by_key(|m| m.order_index);

    let global_ns = &input.metadata.code_gen_config.namespace;

    let mut classes = Vec::new();

    for lifeline in &input.lifelines {
        // Skip actor lifelines - they initiate calls but don't generate code
        if lifeline.lifeline_type == "actor" {
            continue;
        }

        let namespace = if lifeline.properties.namespace.is_empty() {
            global_ns.clone()
        } else {
            lifeline.properties.namespace.clone()
        };

        // Find all incoming messages to this lifeline (these define methods)
        let incoming: Vec<&MessageInput> = sorted_messages
            .iter()
            .filter(|m| m.target_lifeline_id == lifeline.id && m.message_type != "return")
            .copied()
            .collect();

        // Group by method name to deduplicate
        let mut method_names_seen: HashSet<String> = HashSet::new();
        let mut methods: Vec<MethodModel> = Vec::new();

        for msg in &incoming {
            if method_names_seen.contains(&msg.name) {
                continue;
            }
            method_names_seen.insert(msg.name.clone());

            let is_self_call = msg.source_lifeline_id == msg.target_lifeline_id;

            let params: Vec<Parameter> = msg
                .arguments
                .iter()
                .map(|a| Parameter {
                    name: a.name.clone(),
                    param_type: a.arg_type.clone(),
                })
                .collect();

            let return_type = if msg.return_type.is_empty() {
                "void".to_string()
            } else {
                msg.return_type.clone()
            };

            // Build method body: outgoing messages from this lifeline during this activation
            let body = build_method_body(
                lifeline,
                msg,
                &sorted_messages,
                &fragment_map,
                &msg_map,
                &lifeline_map,
            );

            methods.push(MethodModel {
                name: msg.name.clone(),
                params,
                return_type,
                body,
                is_self_call,
            });
        }

        // If this lifeline has outgoing messages but no incoming messages,
        // it's an initiator — create an entry point method containing all its calls
        if incoming.is_empty() {
            let outgoing: Vec<&MessageInput> = sorted_messages
                .iter()
                .filter(|m| {
                    m.source_lifeline_id == lifeline.id && m.message_type != "return"
                })
                .copied()
                .collect();

            if !outgoing.is_empty() {
                let body = build_statements(
                    &outgoing,
                    &fragment_map,
                    &msg_map,
                    &lifeline_map,
                    None,
                );
                methods.push(MethodModel {
                    name: "execute".to_string(),
                    params: vec![],
                    return_type: "void".to_string(),
                    body,
                    is_self_call: false,
                });
            }
        }

        // Compute dependencies
        let mut deps: Vec<String> = Vec::new();
        collect_dependencies(&methods, &lifeline.name, &mut deps);
        deps.sort();
        deps.dedup();

        // Collect all types used → determine standard library includes
        let std_includes = collect_std_includes(&methods);

        classes.push(ClassModel {
            name: lifeline.name.clone(),
            lifeline_id: lifeline.id.clone(),
            namespace,
            stereotype: lifeline.properties.stereotype.clone(),
            lifeline_type: lifeline.lifeline_type.clone(),
            methods,
            dependencies: deps,
            std_includes,
        });
    }

    Ok(BehaviorIR {
        diagram_id: input.metadata.id.clone(),
        diagram_name: input.metadata.name.clone(),
        classes,
    })
}

/// Build the body of a method from the activation window of a lifeline
fn build_method_body(
    lifeline: &LifelineInput,
    entry_msg: &MessageInput,
    sorted_messages: &[&MessageInput],
    fragment_map: &HashMap<&str, &FragmentInput>,
    msg_map: &HashMap<&str, &MessageInput>,
    lifeline_map: &HashMap<&str, &LifelineInput>,
) -> Vec<Statement> {
    // Find the activation window: messages sourced from this lifeline
    // after the entry message, until the return to the caller or next incoming
    let start_idx = entry_msg.order_index;

    // Find end of activation: return message from this lifeline back to caller, or next incoming
    let return_idx = sorted_messages
        .iter()
        .filter(|m| {
            m.source_lifeline_id == lifeline.id
                && m.target_lifeline_id == entry_msg.source_lifeline_id
                && m.message_type == "return"
                && m.order_index > start_idx
        })
        .map(|m| m.order_index)
        .min();

    let next_incoming_idx = sorted_messages
        .iter()
        .filter(|m| {
            m.target_lifeline_id == lifeline.id
                && m.message_type != "return"
                && m.order_index > start_idx
        })
        .map(|m| m.order_index)
        .min();

    let end_idx = match (return_idx, next_incoming_idx) {
        (Some(r), Some(n)) => r.min(n),
        (Some(r), None) => r,
        (None, Some(n)) => n,
        (None, None) => i32::MAX,
    };

    // Collect outgoing non-return messages from this lifeline within the activation window
    let activation_msgs: Vec<&MessageInput> = sorted_messages
        .iter()
        .filter(|m| {
            m.source_lifeline_id == lifeline.id
                && m.order_index > start_idx
                && m.order_index < end_idx
                && m.message_type != "return"
        })
        .copied()
        .collect();

    build_statements(
        &activation_msgs,
        fragment_map,
        msg_map,
        lifeline_map,
        None,
    )
}

/// Recursively build Statement tree from a set of messages
fn build_statements(
    messages: &[&MessageInput],
    fragment_map: &HashMap<&str, &FragmentInput>,
    msg_map: &HashMap<&str, &MessageInput>,
    lifeline_map: &HashMap<&str, &LifelineInput>,
    scope_fragment_id: Option<&str>,
) -> Vec<Statement> {
    let mut stmts: Vec<Statement> = Vec::new();
    let mut processed_fragments: HashSet<String> = HashSet::new();

    for msg in messages {
        // Skip return messages — they don't generate Call statements
        if msg.message_type == "return" {
            continue;
        }

        let msg_fragment = msg.parent_fragment_id.as_deref();

        // Check if this message belongs to the current scope
        if msg_fragment != scope_fragment_id {
            // Message belongs to a different fragment
            if let Some(frag_id) = msg_fragment {
                if processed_fragments.contains(frag_id) {
                    continue; // Already handled as part of a fragment
                }
                if let Some(fragment) = fragment_map.get(frag_id) {
                    // Only process if the fragment's parent matches our scope
                    if fragment.parent_fragment_id.as_deref() == scope_fragment_id {
                        processed_fragments.insert(frag_id.to_string());
                        let stmt = build_fragment_statement(
                            fragment,
                            messages,
                            fragment_map,
                            msg_map,
                            lifeline_map,
                        );
                        stmts.push(stmt);
                    }
                }
            }
            continue;
        }

        // Top-level message in current scope
        let source_name = lifeline_map
            .get(msg.source_lifeline_id.as_str())
            .map(|l| l.name.clone())
            .unwrap_or_default();
        let target_name = lifeline_map
            .get(msg.target_lifeline_id.as_str())
            .map(|l| l.name.clone())
            .unwrap_or_default();

        match msg.message_type.as_str() {
            "create" => {
                stmts.push(Statement::Create {
                    target_class: target_name,
                    arguments: msg.arguments.iter().map(|a| a.name.clone()).collect(),
                });
            }
            "destroy" => {
                stmts.push(Statement::Destroy {
                    target_class: target_name,
                });
            }
            _ => {
                let is_self_call = msg.source_lifeline_id == msg.target_lifeline_id;
                stmts.push(Statement::Call {
                    source_class: source_name,
                    target_class: target_name,
                    method_name: msg.name.clone(),
                    arguments: msg.arguments.iter().map(|a| a.name.clone()).collect(),
                    return_type: if msg.return_type.is_empty() {
                        "void".to_string()
                    } else {
                        msg.return_type.clone()
                    },
                    message_type: msg.message_type.clone(),
                    is_self_call,
                });
            }
        }
    }

    stmts
}

/// Build a compound statement from a combined fragment
fn build_fragment_statement(
    fragment: &FragmentInput,
    _all_messages: &[&MessageInput],
    fragment_map: &HashMap<&str, &FragmentInput>,
    msg_map: &HashMap<&str, &MessageInput>,
    lifeline_map: &HashMap<&str, &LifelineInput>,
) -> Statement {
    // Helper: build statements for a single operand
    let build_operand_stmts = |operand: &OperandInput| -> Vec<Statement> {
        let operand_msgs: Vec<&MessageInput> = operand
            .message_ids
            .iter()
            .filter_map(|id| msg_map.get(id.as_str()).copied())
            .collect();
        build_statements(
            &operand_msgs,
            fragment_map,
            msg_map,
            lifeline_map,
            Some(&fragment.id),
        )
    };

    match fragment.fragment_type.as_str() {
        "alt" => {
            // N-way if/else-if/else: one IfBranch per operand
            let branches: Vec<IfBranch> = fragment
                .operands
                .iter()
                .map(|op| IfBranch {
                    condition: op.guard.clone(),
                    stmts: build_operand_stmts(op),
                })
                .collect();

            Statement::If { branches }
        }
        "loop" => {
            let body_stmts = if !fragment.operands.is_empty() {
                build_operand_stmts(&fragment.operands[0])
            } else {
                vec![]
            };

            let condition = fragment
                .operands
                .first()
                .map(|o| o.guard.clone())
                .unwrap_or_default();

            Statement::Loop {
                condition,
                body_stmts,
            }
        }
        "opt" => {
            let body_stmts = if !fragment.operands.is_empty() {
                build_operand_stmts(&fragment.operands[0])
            } else {
                vec![]
            };

            let condition = fragment
                .operands
                .first()
                .map(|o| o.guard.clone())
                .unwrap_or_default();

            Statement::Opt {
                condition,
                body_stmts,
            }
        }
        "break" => {
            let body_stmts = if !fragment.operands.is_empty() {
                build_operand_stmts(&fragment.operands[0])
            } else {
                vec![]
            };

            let condition = fragment
                .operands
                .first()
                .map(|o| o.guard.clone())
                .unwrap_or_default();

            Statement::Break {
                condition,
                body_stmts,
            }
        }
        "par" => {
            // Each operand becomes a parallel branch
            let branches: Vec<ParBranch> = fragment
                .operands
                .iter()
                .enumerate()
                .map(|(i, op)| ParBranch {
                    label: if op.guard.is_empty() {
                        format!("branch_{}", i)
                    } else {
                        op.guard.clone()
                    },
                    stmts: build_operand_stmts(op),
                })
                .collect();

            Statement::Par { branches }
        }
        // Unknown fragment types: fall back to opt
        _ => {
            let body_stmts = if !fragment.operands.is_empty() {
                build_operand_stmts(&fragment.operands[0])
            } else {
                vec![]
            };

            let condition = fragment
                .operands
                .first()
                .map(|o| o.guard.clone())
                .unwrap_or_default();

            Statement::Opt {
                condition,
                body_stmts,
            }
        }
    }
}

/// Collect class names that appear as Call targets in method bodies
fn collect_dependencies(methods: &[MethodModel], self_name: &str, deps: &mut Vec<String>) {
    for method in methods {
        collect_stmt_deps(&method.body, self_name, deps);
    }
}

fn collect_stmt_deps(stmts: &[Statement], self_name: &str, deps: &mut Vec<String>) {
    for stmt in stmts {
        match stmt {
            Statement::Call {
                target_class,
                is_self_call,
                ..
            } => {
                if !is_self_call && target_class != self_name && !deps.contains(target_class) {
                    deps.push(target_class.clone());
                }
            }
            Statement::If { branches } => {
                for branch in branches {
                    collect_stmt_deps(&branch.stmts, self_name, deps);
                }
            }
            Statement::Loop { body_stmts, .. }
            | Statement::Opt { body_stmts, .. }
            | Statement::Break { body_stmts, .. } => {
                collect_stmt_deps(body_stmts, self_name, deps);
            }
            Statement::Par { branches } => {
                for branch in branches {
                    collect_stmt_deps(&branch.stmts, self_name, deps);
                }
            }
            Statement::Create { target_class, .. } | Statement::Destroy { target_class } => {
                if target_class != self_name && !deps.contains(target_class) {
                    deps.push(target_class.clone());
                }
            }
            Statement::Return { .. } => {}
        }
    }
}

/// Map C++ type names to standard library headers
fn type_to_std_include(type_name: &str) -> Option<&'static str> {
    // Normalize: strip std:: prefix, strip const/&/* qualifiers, trim
    let t = type_name
        .replace("std::", "")
        .replace("const ", "")
        .replace('&', "")
        .replace('*', "")
        .trim()
        .to_lowercase();
    // Also handle template types like vector<int>
    let base = t.split('<').next().unwrap_or(&t).trim();
    match base {
        "string" => Some("<string>"),
        "vector" => Some("<vector>"),
        "map" => Some("<map>"),
        "unordered_map" => Some("<unordered_map>"),
        "set" => Some("<set>"),
        "unordered_set" => Some("<unordered_set>"),
        "list" => Some("<list>"),
        "deque" => Some("<deque>"),
        "queue" => Some("<queue>"),
        "stack" => Some("<stack>"),
        "array" => Some("<array>"),
        "optional" => Some("<optional>"),
        "variant" => Some("<variant>"),
        "any" => Some("<any>"),
        "tuple" => Some("<tuple>"),
        "pair" => Some("<utility>"),
        "shared_ptr" | "unique_ptr" | "weak_ptr" => Some("<memory>"),
        "function" => Some("<functional>"),
        "thread" => Some("<thread>"),
        "mutex" => Some("<mutex>"),
        "future" | "promise" | "async" => Some("<future>"),
        "iostream" | "cout" | "cin" | "cerr" => Some("<iostream>"),
        "fstream" | "ifstream" | "ofstream" => Some("<fstream>"),
        "sstream" | "stringstream" | "istringstream" | "ostringstream" => Some("<sstream>"),
        _ => None,
    }
}

/// Collect all types used in methods, return needed standard includes
fn collect_std_includes(methods: &[MethodModel]) -> Vec<String> {
    let mut types: HashSet<String> = HashSet::new();

    for method in methods {
        // Return type
        if method.return_type != "void" {
            types.insert(method.return_type.clone());
        }
        // Parameter types
        for p in &method.params {
            types.insert(p.param_type.clone());
        }
        // Types from statements
        collect_stmt_types(&method.body, &mut types);
    }

    let mut includes: Vec<String> = types
        .iter()
        .filter_map(|t| type_to_std_include(t).map(|s| s.to_string()))
        .collect();
    includes.sort();
    includes.dedup();
    includes
}

fn collect_stmt_types(stmts: &[Statement], types: &mut HashSet<String>) {
    for stmt in stmts {
        match stmt {
            Statement::Call {
                return_type,
                message_type,
                ..
            } => {
                if return_type != "void" {
                    types.insert(return_type.clone());
                }
                // Async calls need <future>
                if message_type == "async" {
                    types.insert("future".to_string());
                }
            }
            Statement::If { branches } => {
                for branch in branches {
                    collect_stmt_types(&branch.stmts, types);
                }
            }
            Statement::Loop { body_stmts, .. }
            | Statement::Opt { body_stmts, .. }
            | Statement::Break { body_stmts, .. } => {
                collect_stmt_types(body_stmts, types);
            }
            Statement::Par { branches } => {
                // Par needs <thread> and <future>
                types.insert("thread".to_string());
                types.insert("future".to_string());
                for branch in branches {
                    collect_stmt_types(&branch.stmts, types);
                }
            }
            Statement::Create { .. } | Statement::Destroy { .. } => {
                // Create/destroy may need <memory> for unique_ptr
                types.insert("unique_ptr".to_string());
            }
            Statement::Return { return_type, .. } => {
                if return_type != "void" {
                    types.insert(return_type.clone());
                }
            }
        }
    }
}
