use minijinja::{Environment, context, Value};
use crate::ir::BehaviorIR;

const DEFAULT_CPP_HEADER: &str = include_str!("../templates/cpp/file_unit.h.j2");
const DEFAULT_CPP_IMPL: &str = include_str!("../templates/cpp/file_unit.cpp.j2");
const DEFAULT_CPP_STMTS: &str = include_str!("../templates/cpp/statements.j2");

pub struct GeneratedFile {
    pub path: String,
    pub content: String,
}

pub struct GenerationReport {
    pub files: Vec<GeneratedFile>,
    pub warnings: Vec<String>,
}

pub fn generate(ir: &BehaviorIR, language: &str) -> Result<GenerationReport, String> {
    match language {
        "cpp" => generate_cpp(ir),
        _ => Err(format!("Unsupported language: {}", language)),
    }
}

fn generate_cpp(ir: &BehaviorIR) -> Result<GenerationReport, String> {
    let mut env = Environment::new();
    env.set_recursion_limit(128);

    // Load templates
    env.add_template("statements.j2", DEFAULT_CPP_STMTS)
        .map_err(|e| format!("Failed to load statements template: {}", e))?;
    env.add_template("file_unit.h.j2", DEFAULT_CPP_HEADER)
        .map_err(|e| format!("Failed to load header template: {}", e))?;
    env.add_template("file_unit.cpp.j2", DEFAULT_CPP_IMPL)
        .map_err(|e| format!("Failed to load impl template: {}", e))?;

    // We need to make the render_stmt macro available in the cpp templates.
    // MiniJinja supports {% from "..." import ... %} in templates.
    // But since our file_unit.cpp.j2 calls render_stmt directly,
    // we'll prepend the import to the impl template.
    let impl_with_import = format!(
        "{{% from \"statements.j2\" import render_stmt %}}{}",
        DEFAULT_CPP_IMPL
    );
    env.add_template("file_unit_impl.j2", &impl_with_import)
        .map_err(|e| format!("Failed to load impl template with import: {}", e))?;

    let mut report = GenerationReport {
        files: vec![],
        warnings: vec![],
    };

    let diagram_value = Value::from_serialize(ir);

    for class in &ir.classes {
        // Skip actor lifelines
        if class.lifeline_type == "actor" {
            report.warnings.push(format!(
                "Skipped actor lifeline '{}' (actors don't generate code)",
                class.name
            ));
            continue;
        }

        if class.methods.is_empty() {
            report.warnings.push(format!(
                "Class '{}' has no methods (no incoming messages)",
                class.name
            ));
        }

        let class_value = Value::from_serialize(class);
        let namespace = &class.namespace;

        // Render header
        let header_tmpl = env
            .get_template("file_unit.h.j2")
            .map_err(|e| format!("Template error: {}", e))?;
        let header = header_tmpl
            .render(context! {
                class => class_value,
                diagram => diagram_value,
                namespace => namespace,
            })
            .map_err(|e| format!("Render header error for '{}': {}", class.name, e))?;

        report.files.push(GeneratedFile {
            path: format!("{}.h", class.name),
            content: header,
        });

        // Render implementation
        let impl_tmpl = env
            .get_template("file_unit_impl.j2")
            .map_err(|e| format!("Template error: {}", e))?;
        let impl_content = impl_tmpl
            .render(context! {
                class => class_value,
                diagram => diagram_value,
                namespace => namespace,
            })
            .map_err(|e| format!("Render impl error for '{}': {}", class.name, e))?;

        report.files.push(GeneratedFile {
            path: format!("{}.cpp", class.name),
            content: impl_content,
        });
    }

    Ok(report)
}
