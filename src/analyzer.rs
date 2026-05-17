use crate::config::CommentPatterns;
use aho_corasick::AhoCorasick;

/// Default config file (contents are included from the sample JSON config file)
pub const DEFAULT_CONFIG: &str = include_str!("../sample.json");

/// Pre-compiled matchers for comment patterns.
pub struct CommentMatchers {
    pub single_line: AhoCorasick,
    pub multi_line_start: AhoCorasick,
    pub multi_line_end: AhoCorasick,
    pub patterns: CommentPatterns,
}

impl CommentMatchers {
    pub fn new(patterns: CommentPatterns) -> Self {
        Self {
            single_line: AhoCorasick::new(&patterns.single_line)
                .expect("valid single-line patterns"),
            multi_line_start: AhoCorasick::new(&patterns.multi_line_start)
                .expect("valid multi-line start patterns"),
            multi_line_end: AhoCorasick::new(&patterns.multi_line_end)
                .expect("valid multi-line end patterns"),
            patterns,
        }
    }
}

/// Determine whether `line` is empty or consists solely of comments.
///
/// Returns `(is_comment_or_empty, next_in_multiline_comment)`.
///
/// `in_multiline_comment` tracks whether the previous line opened a
/// multi-line block comment that has not yet been closed.
#[must_use]
pub fn is_comment_or_empty(
    line: &str,
    matchers: &CommentMatchers,
    mut in_multiline_comment: bool,
) -> (bool, bool) {
    let trimmed = line.trim();

    if trimmed.is_empty() {
        return (true, in_multiline_comment);
    }

    if in_multiline_comment {
        if let Some((_, after)) = find_earliest_pattern(
            trimmed,
            &matchers.multi_line_end,
            &matchers.patterns.multi_line_end,
        ) {
            in_multiline_comment = false;
            if !after.trim().is_empty() {
                return (false, in_multiline_comment);
            }
        }
        return (true, in_multiline_comment);
    }

    if matchers
        .single_line
        .find(trimmed)
        .is_some_and(|m| m.start() == 0)
    {
        return (true, in_multiline_comment);
    }

    if let Some((start_pos, after_start)) = find_earliest_pattern(
        trimmed,
        &matchers.multi_line_start,
        &matchers.patterns.multi_line_start,
    ) {
        let has_code_before_start = start_pos > 0 && !trimmed[..start_pos].trim().is_empty();

        if let Some((_, after_end)) = find_earliest_pattern(
            after_start,
            &matchers.multi_line_end,
            &matchers.patterns.multi_line_end,
        ) {
            if !after_end.trim().is_empty() {
                return (false, in_multiline_comment);
            }
            if has_code_before_start {
                return (false, in_multiline_comment);
            }
            return (true, in_multiline_comment);
        }

        in_multiline_comment = true;
        if has_code_before_start {
            return (false, in_multiline_comment);
        }
        return (true, in_multiline_comment);
    }

    (false, in_multiline_comment)
}

/// Find the earliest occurrence of any pattern in `patterns` within `s`.
///
/// Returns `Some((byte_index_of_pattern, remainder_after_pattern))` or
/// `None` if no pattern matches.
fn find_earliest_pattern<'a>(
    s: &'a str,
    matcher: &AhoCorasick,
    patterns: &[String],
) -> Option<(usize, &'a str)> {
    matcher.find(s).map(|m| {
        let pos = m.start();
        let pat_index = m.pattern().as_usize();
        (pos, &s[pos + patterns[pat_index].len()..])
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CommentPatterns;

    /// Minimal comment patterns covering C-style, Python-style, and shell-style.
    fn common_comment_matchers() -> CommentMatchers {
        CommentMatchers::new(CommentPatterns {
            single_line: vec!["//".into(), "#".into(), ";".into()],
            multi_line_start: vec!["/*".into()],
            multi_line_end: vec!["*/".into()],
        })
    }

    /// Patterns that include `"""` (Python docstrings).
    fn py_docstring_matchers() -> CommentMatchers {
        CommentMatchers::new(CommentPatterns {
            single_line: vec!["#".into()],
            multi_line_start: vec!["/*".into(), "\"\"\"".into()],
            multi_line_end: vec!["*/".into(), "\"\"\"".into()],
        })
    }

    // Basic single-line

    #[test]
    fn empty_line() {
        let m = common_comment_matchers();
        let in_multi = false;
        assert!(is_comment_or_empty("", &m, in_multi).0);
        assert!(is_comment_or_empty("   ", &m, in_multi).0);
    }

    #[test]
    fn single_line_comment() {
        let m = common_comment_matchers();
        let in_multi = false;
        assert!(is_comment_or_empty("// comment", &m, in_multi).0);
        assert!(is_comment_or_empty("  // indented", &m, in_multi).0);
        assert!(is_comment_or_empty("# python", &m, in_multi).0);
        assert!(is_comment_or_empty("; assembly", &m, in_multi).0);
    }

    #[test]
    fn code_line() {
        let m = common_comment_matchers();
        let in_multi = false;
        assert!(!is_comment_or_empty("fn main() {}", &m, in_multi).0);
        assert!(!is_comment_or_empty("let x = 5;", &m, in_multi).0);
        assert!(!is_comment_or_empty("print('hello')", &m, in_multi).0);
    }

    // Inline trailing comment (code before comment marker)

    #[test]
    fn code_with_trailing_comment() {
        let m = common_comment_matchers();
        let in_multi = false;
        assert!(!is_comment_or_empty("let x = 5; // explain", &m, in_multi).0);
        assert!(!is_comment_or_empty("x = 1 + 2  # inline", &m, in_multi).0);
    }

    // Multi-line block comments

    #[test]
    fn multiline_block_opens_and_closes() {
        let m = common_comment_matchers();
        let in_multi = false;
        let (r, res_multi) = is_comment_or_empty("/* comment */", &m, in_multi);
        assert!(r);
        assert!(!res_multi);
    }

    #[test]
    fn multiline_spanning_several_lines() {
        let m = common_comment_matchers();
        let in_multi = false;

        let (r, res_multi) = is_comment_or_empty("/* open", &m, in_multi);
        assert!(r);
        assert!(res_multi);
        let (r, res_multi) = is_comment_or_empty("  still a comment", &m, res_multi);
        assert!(r);
        assert!(res_multi);
        let (r, res_multi) = is_comment_or_empty("close */", &m, res_multi);
        assert!(r);
        assert!(!res_multi);
    }

    #[test]
    fn code_before_multiline_start() {
        let m = common_comment_matchers();
        let in_multi = false;
        let (r, res_multi) = is_comment_or_empty("let x = 1; /* comment starts", &m, in_multi);
        assert!(!r);
        assert!(res_multi);
        let (r, res_multi) = is_comment_or_empty("still comment", &m, res_multi);
        assert!(r);
        let (r, res_multi) = is_comment_or_empty("close */", &m, res_multi);
        assert!(r);
        assert!(!res_multi);
    }

    #[test]
    fn code_after_multiline_on_same_line() {
        let m = common_comment_matchers();
        let in_multi = false;
        let (r, res_multi) = is_comment_or_empty("/* comment */ let x = 1;", &m, in_multi);
        assert!(!r);
        assert!(!res_multi);
    }

    #[test]
    fn mid_line_end_marker() {
        let m = common_comment_matchers();
        let in_multi = false;

        let (r, res_multi) = is_comment_or_empty("/* block", &m, in_multi);
        assert!(r);
        assert!(res_multi);
        let (r, res_multi) = is_comment_or_empty("  */   ", &m, res_multi);
        assert!(r);
        assert!(!res_multi);
    }

    #[test]
    fn mid_line_end_marker_with_code_after() {
        let m = common_comment_matchers();
        let in_multi = false;

        let (r, res_multi) = is_comment_or_empty("/* block", &m, in_multi);
        assert!(r);
        assert!(res_multi);
        let (r, res_multi) = is_comment_or_empty("  */  let x = 1;", &m, res_multi);
        assert!(!r);
        assert!(!res_multi);
    }

    // Python docstrings (""" ... """)

    #[test]
    fn python_docstring_single_line() {
        let m = py_docstring_matchers();
        let in_multi = false;
        let (r, res_multi) = is_comment_or_empty("\"\"\"docstring\"\"\"", &m, in_multi);
        assert!(r);
        assert!(!res_multi);
    }

    #[test]
    fn python_docstring_multi_line() {
        let m = py_docstring_matchers();
        let in_multi = false;

        let (r, res_multi) = is_comment_or_empty("\"\"\"", &m, in_multi);
        assert!(r);
        assert!(res_multi);
        let (r, res_multi) = is_comment_or_empty("hello world", &m, res_multi);
        assert!(r);
        assert!(res_multi);
        let (r, res_multi) = is_comment_or_empty("\"\"\"", &m, res_multi);
        assert!(r);
        assert!(!res_multi);
    }

    // find_earliest_pattern helper

    #[test]
    fn earliest_pattern_none() {
        let patterns = vec!["x".into(), "y".into()];
        let matcher = AhoCorasick::new(&patterns).unwrap();
        assert_eq!(find_earliest_pattern("hello", &matcher, &patterns), None);
    }

    #[test]
    fn earliest_pattern_returns_first() {
        let patterns = vec!["/*".into(), "//".into()];
        let matcher = AhoCorasick::new(&patterns).unwrap();
        let (pos, after) = find_earliest_pattern("  // not /* here", &matcher, &patterns).unwrap();
        // `//` occurs before `/*` in the string
        assert_eq!(pos, 2);
        assert_eq!(after, " not /* here");
    }
}
