pub mod ir;
pub mod parser;
pub mod generator;
pub mod protected_region;
pub mod incremental;

use std::fs;
use std::path::Path;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save_diagram(path: String, content: String) -> Result<(), String> {
    let file_path = Path::new(&path);

    // Create parent directories if needed
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Write backup first
    if file_path.exists() {
        let backup_path = format!("{}.bak", path);
        fs::copy(&path, &backup_path).map_err(|e| format!("Failed to create backup: {}", e))?;
    }

    // Atomic write: write to .tmp then rename
    let tmp_path = format!("{}.tmp", path);
    fs::write(&tmp_path, &content).map_err(|e| format!("Failed to write temp file: {}", e))?;
    fs::rename(&tmp_path, &path).map_err(|e| format!("Failed to rename temp file: {}", e))?;

    Ok(())
}

#[tauri::command]
fn load_diagram(path: String) -> Result<String, String> {
    if path.is_empty() {
        return Err("No file path provided".to_string());
    }
    let content =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;

    // Basic JSON validation
    serde_json::from_str::<serde_json::Value>(&content)
        .map_err(|e| format!("Invalid JSON: {}", e))?;

    Ok(content)
}

#[derive(serde::Serialize)]
struct GenerationResult {
    success: bool,
    files: Vec<incremental::WriteResult>,
    warnings: Vec<String>,
}

#[derive(serde::Serialize)]
struct PreviewFile {
    path: String,
    content: String,
}

#[tauri::command]
fn generate_code(diagram_json: String, output_dir: String, old_version_dir: Option<String>) -> Result<GenerationResult, String> {
    let input: parser::DiagramInput = serde_json::from_str(&diagram_json)
        .map_err(|e| format!("Invalid diagram JSON: {}", e))?;

    let ir = parser::parse_diagram(&input)?;
    let language = &input.metadata.code_gen_config.language;
    let report = generator::generate(&ir, language)?;

    let write_results = incremental::write_files(
        &report.files,
        &output_dir,
        old_version_dir.as_deref(),
    )?;

    Ok(GenerationResult {
        success: true,
        files: write_results,
        warnings: report.warnings,
    })
}

#[tauri::command]
fn preview_code(diagram_json: String) -> Result<Vec<PreviewFile>, String> {
    let input: parser::DiagramInput = serde_json::from_str(&diagram_json)
        .map_err(|e| format!("Invalid diagram JSON: {}", e))?;

    let ir = parser::parse_diagram(&input)?;
    let language = &input.metadata.code_gen_config.language;
    let report = generator::generate(&ir, language)?;

    Ok(report
        .files
        .into_iter()
        .map(|f| PreviewFile {
            path: f.path,
            content: f.content,
        })
        .collect())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            save_diagram,
            load_diagram,
            generate_code,
            preview_code,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
