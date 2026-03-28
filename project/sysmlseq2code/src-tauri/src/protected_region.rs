use std::collections::HashMap;

const BEGIN_PREFIX: &str = "// <user-code-begin ";
const END_PREFIX: &str = "// <user-code-end ";

/// Extract all user-code protected regions from file content.
/// Returns a map of region_id -> user code content.
pub fn extract_regions(content: &str) -> HashMap<String, String> {
    let mut regions = HashMap::new();
    let mut current_id: Option<String> = None;
    let mut current_lines: Vec<&str> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(id) = parse_begin_marker(trimmed) {
            current_id = Some(id);
            current_lines.clear();
        } else if parse_end_marker(trimmed).is_some() {
            if let Some(id) = current_id.take() {
                let code = if current_lines.is_empty() {
                    String::new()
                } else {
                    let mut s = current_lines.join("\n");
                    s.push('\n');
                    s
                };
                regions.insert(id, code);
            }
        } else if current_id.is_some() {
            current_lines.push(line);
        }
    }

    regions
}

/// Merge saved user-code regions back into newly generated content.
/// For each protected region in new_content, if saved_regions has content
/// for that region ID, replace the empty region with the saved content.
pub fn merge_regions(new_content: &str, saved_regions: &HashMap<String, String>) -> String {
    if saved_regions.is_empty() {
        return new_content.to_string();
    }

    let mut result = Vec::new();
    let mut skip_until_end = false;

    for line in new_content.lines() {
        let trimmed = line.trim();

        if let Some(id) = parse_begin_marker(trimmed) {
            result.push(line.to_string());
            if let Some(saved) = saved_regions.get(&id) {
                if !saved.is_empty() {
                    result.push(saved.trim_end_matches('\n').to_string());
                }
                skip_until_end = true;
            }
        } else if parse_end_marker(trimmed).is_some() {
            skip_until_end = false;
            result.push(line.to_string());
        } else if !skip_until_end {
            result.push(line.to_string());
        }
    }

    let mut output = result.join("\n");
    // Preserve trailing newline if original had one
    if new_content.ends_with('\n') {
        output.push('\n');
    }
    output
}

fn parse_begin_marker(trimmed: &str) -> Option<String> {
    if trimmed.starts_with(BEGIN_PREFIX) && trimmed.ends_with('>') {
        let id = &trimmed[BEGIN_PREFIX.len()..trimmed.len() - 1];
        Some(id.to_string())
    } else {
        None
    }
}

fn parse_end_marker(trimmed: &str) -> Option<String> {
    if trimmed.starts_with(END_PREFIX) && trimmed.ends_with('>') {
        let id = &trimmed[END_PREFIX.len()..trimmed.len() - 1];
        Some(id.to_string())
    } else {
        None
    }
}
