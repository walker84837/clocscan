use serde_derive::Deserialize;

/// Configuration for extensions to look for
#[derive(Deserialize)]
pub struct CodeFileConfig {
    pub code_file_extensions: Vec<CodeFileExtension>,
    pub ignore: IgnoreConfig,
    pub comment_patterns: CommentPatterns,
}

/// Individual extension with file type
#[derive(Deserialize)]
pub struct CodeFileExtension {
    pub extension: String,
    pub file_type: String,
}

/// Comment patterns for different languages
#[derive(Deserialize)]
pub struct CommentPatterns {
    pub single_line: Vec<String>,
    pub multi_line_start: Vec<String>,
    pub multi_line_end: Vec<String>,
}

/// Files and folders to ignore
#[derive(Deserialize)]
pub struct IgnoreConfig {
    pub folders: Vec<String>,
    pub files: Vec<String>,
}
