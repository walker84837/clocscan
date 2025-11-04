use crate::config::CommentPatterns;

/// Default config file (contents are included from the sample JSON config file)
pub const DEFAULT_CONFIG: &str = include_str!("../sample.json");

/// Checks if a given line is a comment in various programming languages.
pub fn is_comment_or_empty(
    line: &str,
    comment_patterns: &CommentPatterns,
    in_multiline_comment: &mut bool,
) -> bool {
    let trimmed_line = line.trim();

    if trimmed_line.is_empty() {
        return true;
    }

    if *in_multiline_comment {
        if comment_patterns
            .multi_line_end
            .iter()
            .any(|pattern| trimmed_line.ends_with(pattern))
        {
            *in_multiline_comment = false;
        }
        return true;
    }

    if comment_patterns
        .multi_line_start
        .iter()
        .any(|pattern| trimmed_line.starts_with(pattern))
    {
        *in_multiline_comment = true;
        return true;
    }

    comment_patterns
        .single_line
        .iter()
        .any(|pattern| trimmed_line.starts_with(pattern))
}
