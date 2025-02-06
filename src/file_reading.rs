use std::{
    fs::File,
    io::{self, BufReader, Read},
};
use thiserror::Error;

/// Default config file
pub const DEFAULT_CONFIG: &str = include_str!("../docs/sample.json");

/// Error enum for generic file reading
#[derive(Debug, Error)]
pub enum FileError {
    #[error("Failed to read file: {0}")]
    FileReadError(#[from] io::Error),
}

/// Reads the contents of a file and returns them as a `String`.
///
/// Fails when UTF-8 contents are not found or the file can't be read due to permission
/// errors, etc...
pub fn read_file_contents(file_path: impl AsRef<str>) -> Result<String, FileError> {
    let path = file_path.as_ref();
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    Ok(contents)
}

/// Checks if a given line is a comment in various programming languages.
// TODO: configure this via JSON config file
pub fn is_comment_line(line: &str) -> bool {
    let comment_patterns = ["//", "#", ";", "/*", "*/", "--"];

    let trimmed_line = line.trim();
    comment_patterns
        .iter()
        .any(|&pattern| trimmed_line.starts_with(pattern))
}
