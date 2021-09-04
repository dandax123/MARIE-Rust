use std::fs;
pub(crate) fn extract_lines_from_file(filename: &str) -> std::io::Result<Vec<String>> {
    let contents = fs::read_to_string(filename)?;
    let mut file_contents = Vec::new();
    for line in contents.lines() {
        if !line.is_empty() {
            file_contents.push(line.trim().to_string())
        }
    }
    Ok(file_contents)
}
