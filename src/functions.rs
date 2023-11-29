use anyhow::{Context, Result};
use std::fs;

/// Reads the contents of a file and returns them as a `String`.
pub fn read_file_contents(file_path: &str) -> Result<String> {
    fs::read_to_string(file_path).with_context(|| format!("Failed to read file: {}", file_path))
}

/// Checks if a given line is a comment in various programming languages.
pub fn is_comment_line(line: &str) -> bool {
    let trimmed_line = line.trim();

    trimmed_line.starts_with("//")
        || trimmed_line.starts_with("//")
        || trimmed_line.starts_with('#')
        || trimmed_line.starts_with(';')
        || trimmed_line.starts_with("/*") && trimmed_line.ends_with("*/")
}
