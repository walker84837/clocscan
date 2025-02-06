use serde_derive::Deserialize;
///
#[derive(Deserialize)]
pub struct CodeFileConfig {
    pub code_file_extensions: Vec<CodeFileExtension>,
    pub ignore: IgnoreConfig,
}

///
#[derive(Deserialize)]
pub struct CodeFileExtension {
    pub extension: String,
    pub file_type: String,
}

///
#[derive(Deserialize)]
pub struct IgnoreConfig {
    pub folders: Vec<String>,
    pub files: Vec<String>,
}
