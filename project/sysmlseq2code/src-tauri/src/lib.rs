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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            save_diagram,
            load_diagram
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
