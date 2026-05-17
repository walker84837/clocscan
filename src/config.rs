use serde::Deserialize;

/// Top-level configuration parsed from the JSON config file.
///
/// If a field is omitted from the JSON, its `Default` value is used
/// (empty vecs / empty strings).
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct CodeFileConfig {
    /// Recognised file extensions and their human-readable labels.
    pub code_file_extensions: Vec<CodeFileExtension>,
    /// Folders and file names to skip during traversal.
    pub ignore: IgnoreConfig,
    /// Patterns used to detect single-line and multi-line comments.
    pub comment_patterns: CommentPatterns,
}

impl Default for CodeFileConfig {
    fn default() -> Self {
        // SAFETY: DEFAULT_CONFIG is embedded at compile time via include_str!
        // and verified to be valid JSON; a panic here is a programming error.
        serde_json::from_str(crate::analyzer::DEFAULT_CONFIG)
            .expect("embedded DEFAULT_CONFIG (sample.json) is not valid JSON")
    }
}

/// A single file-extension entry in the config.
#[derive(Debug, Clone, Deserialize)]
pub struct CodeFileExtension {
    /// Extension without the leading dot, e.g. `"rs"`.
    pub extension: String,
    /// Human-readable label, e.g. `"Rust"`.
    pub file_type: String,
}

/// Patterns for recognising comments in source files.
#[derive(Debug, Clone, Deserialize)]
pub struct CommentPatterns {
    /// Markers that start a single-line comment (e.g. `"//"`, `"#"`).
    pub single_line: Vec<String>,
    /// Markers that start a multi-line block comment (e.g. `"/*"`).
    pub multi_line_start: Vec<String>,
    /// Markers that end a multi-line block comment (e.g. `"*/"`).
    pub multi_line_end: Vec<String>,
}

/// Directories and files to skip during scanning.
#[derive(Debug, Clone, Deserialize)]
pub struct IgnoreConfig {
    /// Directory names (or path fragments) to ignore.
    pub folders: Vec<String>,
    /// File names (or path fragments) to ignore.
    pub files: Vec<String>,
}
