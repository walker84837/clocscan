/// Default config file (contents are included from the sample JSON config file)
pub const DEFAULT_CONFIG: &str = include_str!("../docs/sample.json");

/// Checks if a given line is a comment in various programming languages.
// TODO: configure this via JSON config file
pub fn is_comment_line(line: &str) -> bool {
    let comment_patterns = ["//", "#", ";", "/*", "*/", "--"];

    let trimmed_line = line.trim();
    comment_patterns
        .iter()
        .any(|&pattern| trimmed_line.starts_with(pattern))
}
