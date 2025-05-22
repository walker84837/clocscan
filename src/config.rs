use serde_derive::Deserialize;

/// Configuration for extensions to look for
#[derive(Deserialize)]
pub struct CodeFileConfig {
    pub code_file_extensions: Vec<CodeFileExtension>,
    pub ignore: IgnoreConfig,
}

/// Individual extension with file type
#[derive(Deserialize)]
pub struct CodeFileExtension {
    pub extension: String,
    pub file_type: String,
}

/// Files and folders to ignore
#[derive(Deserialize)]
pub struct IgnoreConfig {
    pub folders: Vec<String>,
    pub files: Vec<String>,
}
