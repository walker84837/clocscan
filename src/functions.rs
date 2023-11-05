use anyhow::{Context, Result};
use std::fs;

pub fn read_file_contents(file_path: &str) -> Result<String> {
    fs::read_to_string(file_path).with_context(|| format!("Failed to read file: {}", file_path))
}
