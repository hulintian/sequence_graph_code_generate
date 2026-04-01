use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorIR {
    pub diagram_id: String,
    pub diagram_name: String,
    pub classes: Vec<ClassModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassModel {
    pub name: String,
    pub lifeline_id: String,
    pub namespace: String,
    pub stereotype: String,
    pub lifeline_type: String,
    pub methods: Vec<MethodModel>,
    pub dependencies: Vec<String>,
    /// Standard library includes needed (e.g. "<string>", "<vector>")
    pub std_includes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodModel {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: String,
    pub body: Vec<Statement>,
    pub is_self_call: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfBranch {
    pub condition: String,
    pub stmts: Vec<Statement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParBranch {
    pub label: String,
    pub stmts: Vec<Statement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Statement {
    Call {
        source_class: String,
        target_class: String,
        method_name: String,
        arguments: Vec<String>,
        return_type: String,
        message_type: String,
        is_self_call: bool,
    },
    If {
        branches: Vec<IfBranch>,
    },
    Loop {
        condition: String,
        body_stmts: Vec<Statement>,
    },
    Opt {
        condition: String,
        body_stmts: Vec<Statement>,
    },
    Break {
        condition: String,
        body_stmts: Vec<Statement>,
    },
    Par {
        branches: Vec<ParBranch>,
    },
    Create {
        target_class: String,
        arguments: Vec<String>,
    },
    Destroy {
        target_class: String,
    },
    Return {
        value: String,
        return_type: String,
    },
}
