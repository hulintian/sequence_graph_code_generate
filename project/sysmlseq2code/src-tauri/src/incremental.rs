use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

use crate::generator::GeneratedFile;
use crate::protected_region;

#[derive(Debug, Clone, serde::Serialize)]
pub struct WriteResult {
    pub path: String,
    pub action: String,
}

pub fn write_files(
    files: &[GeneratedFile],
    output_dir: &str,
    old_version_dir: Option<&str>,
) -> Result<Vec<WriteResult>, String> {
    let out = Path::new(output_dir);
    fs::create_dir_all(out).map_err(|e| format!("Cannot create output dir: {}", e))?;

    // Create old version backup dir if specified
    if let Some(old_dir) = old_version_dir {
        fs::create_dir_all(old_dir).ok();
    }

    let mut results = Vec::new();

    for file in files {
        let target_path = out.join(&file.path);

        if target_path.exists() {
            let existing = fs::read_to_string(&target_path)
                .map_err(|e| format!("Cannot read {}: {}", file.path, e))?;

            // Extract user code regions from existing file
            let saved_regions = protected_region::extract_regions(&existing);

            // Merge user code into new content
            let merged = protected_region::merge_regions(&file.content, &saved_regions);

            // Check if content actually changed
            if hash_content(&existing) == hash_content(&merged) {
                results.push(WriteResult {
                    path: file.path.clone(),
                    action: "unchanged".to_string(),
                });
                continue;
            }

            // Backup old version
            if let Some(old_dir) = old_version_dir {
                let backup_path = Path::new(old_dir).join(&file.path);
                if let Some(parent) = backup_path.parent() {
                    fs::create_dir_all(parent).ok();
                }
                fs::copy(&target_path, &backup_path).ok();
            }

            fs::write(&target_path, &merged)
                .map_err(|e| format!("Cannot write {}: {}", file.path, e))?;

            let action = if saved_regions.is_empty() {
                "updated"
            } else {
                "merged_user_code"
            };
            results.push(WriteResult {
                path: file.path.clone(),
                action: action.to_string(),
            });
        } else {
            // New file
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent).ok();
            }
            fs::write(&target_path, &file.content)
                .map_err(|e| format!("Cannot write {}: {}", file.path, e))?;
            results.push(WriteResult {
                path: file.path.clone(),
                action: "created".to_string(),
            });
        }
    }

    Ok(results)
}

fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}
