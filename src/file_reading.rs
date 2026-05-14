use crate::config::CommentPatterns;

/// Default config file (contents are included from the sample JSON config file)
pub const DEFAULT_CONFIG: &str = include_str!("../sample.json");

/// Determine whether `line` is empty or consists solely of comments.
///
/// Returns `(is_comment_or_empty, next_in_multiline_comment)`.
///
/// `in_multiline_comment` tracks whether the previous line opened a
/// multi-line block comment that has not yet been closed.
#[must_use]
pub fn is_comment_or_empty(
    line: &str,
    comment_patterns: &CommentPatterns,
    mut in_multiline_comment: bool,
) -> (bool, bool) {
    let trimmed = line.trim();

    if trimmed.is_empty() {
        return (true, in_multiline_comment);
    }

    if in_multiline_comment {
        if let Some((_, after)) = find_earliest_pattern(trimmed, &comment_patterns.multi_line_end) {
            in_multiline_comment = false;
            if !after.trim().is_empty() {
                return (false, in_multiline_comment);
            }
        }
        return (true, in_multiline_comment);
    }

    if comment_patterns
        .single_line
        .iter()
        .any(|pat| trimmed.starts_with(pat.as_str()))
    {
        return (true, in_multiline_comment);
    }

    if let Some((start_pos, after_start)) =
        find_earliest_pattern(trimmed, &comment_patterns.multi_line_start)
    {
        let has_code_before_start = start_pos > 0 && !trimmed[..start_pos].trim().is_empty();

        if let Some((_, after_end)) =
            find_earliest_pattern(after_start, &comment_patterns.multi_line_end)
        {
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
fn find_earliest_pattern<'a>(s: &'a str, patterns: &[String]) -> Option<(usize, &'a str)> {
    let mut earliest: Option<(usize, usize)> = None; // (pos, pattern_idx)
    for (pi, pat) in patterns.iter().enumerate() {
        if let Some(pos) = s.find(pat.as_str()) {
            match earliest {
                Some((earliest_pos, _)) if pos < earliest_pos => {
                    earliest = Some((pos, pi));
                }
                None => {
                    earliest = Some((pos, pi));
                }
                _ => {}
            }
        }
    }
    earliest.map(|(pos, pi)| (pos, &s[pos + patterns[pi].len()..]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CommentPatterns;

    /// Minimal comment patterns covering C-style, Python-style, and shell-style.
    fn c_py_sh_patterns() -> CommentPatterns {
        CommentPatterns {
            single_line: vec!["//".into(), "#".into(), ";".into()],
            multi_line_start: vec!["/*".into()],
            multi_line_end: vec!["*/".into()],
        }
    }

    /// Patterns that include `"""` (Python docstrings).
    fn py_docstring_patterns() -> CommentPatterns {
        CommentPatterns {
            single_line: vec!["#".into()],
            multi_line_start: vec!["/*".into(), "\"\"\"".into()],
            multi_line_end: vec!["*/".into(), "\"\"\"".into()],
        }
    }

    // Basic single-line

    #[test]
    fn empty_line() {
        let p = c_py_sh_patterns();
        let m = false;
        assert!(is_comment_or_empty("", &p, m).0);
        assert!(is_comment_or_empty("   ", &p, m).0);
    }

    #[test]
    fn single_line_comment() {
        let p = c_py_sh_patterns();
        let m = false;
        assert!(is_comment_or_empty("// comment", &p, m).0);
        assert!(is_comment_or_empty("  // indented", &p, m).0);
        assert!(is_comment_or_empty("# python", &p, m).0);
        assert!(is_comment_or_empty("; assembly", &p, m).0);
    }

    #[test]
    fn code_line() {
        let p = c_py_sh_patterns();
        let m = false;
        assert!(!is_comment_or_empty("fn main() {}", &p, m).0);
        assert!(!is_comment_or_empty("let x = 5;", &p, m).0);
        assert!(!is_comment_or_empty("print('hello')", &p, m).0);
    }

    // Inline trailing comment (code before comment marker)

    #[test]
    fn code_with_trailing_comment() {
        let p = c_py_sh_patterns();
        let m = false;
        assert!(!is_comment_or_empty("let x = 5; // explain", &p, m).0);
        assert!(!is_comment_or_empty("x = 1 + 2  # inline", &p, m).0);
    }

    // Multi-line block comments

    #[test]
    fn multiline_block_opens_and_closes() {
        let p = c_py_sh_patterns();
        let m = false;
        let (r, m) = is_comment_or_empty("/* comment */", &p, m);
        assert!(r);
        assert!(!m);
    }

    #[test]
    fn multiline_spanning_several_lines() {
        let p = c_py_sh_patterns();
        let m = false;

        let (r, m) = is_comment_or_empty("/* open", &p, m);
        assert!(r);
        assert!(m);
        let (r, m) = is_comment_or_empty("  still a comment", &p, m);
        assert!(r);
        assert!(m);
        let (r, m) = is_comment_or_empty("close */", &p, m);
        assert!(r);
        assert!(!m);
    }

    #[test]
    fn code_before_multiline_start() {
        let p = c_py_sh_patterns();
        let m = false;
        let (r, m) = is_comment_or_empty("let x = 1; /* comment starts", &p, m);
        assert!(!r);
        assert!(m);
        let (r, m) = is_comment_or_empty("still comment", &p, m);
        assert!(r);
        let (r, m) = is_comment_or_empty("close */", &p, m);
        assert!(r);
        assert!(!m);
    }

    #[test]
    fn code_after_multiline_on_same_line() {
        let p = c_py_sh_patterns();
        let m = false;
        let (r, m) = is_comment_or_empty("/* comment */ let x = 1;", &p, m);
        assert!(!r);
        assert!(!m);
    }

    #[test]
    fn mid_line_end_marker() {
        let p = c_py_sh_patterns();
        let m = false;

        let (r, m) = is_comment_or_empty("/* block", &p, m);
        assert!(r);
        assert!(m);
        let (r, m) = is_comment_or_empty("  */   ", &p, m);
        assert!(r);
        assert!(!m);
    }

    #[test]
    fn mid_line_end_marker_with_code_after() {
        let p = c_py_sh_patterns();
        let m = false;

        let (r, m) = is_comment_or_empty("/* block", &p, m);
        assert!(r);
        assert!(m);
        let (r, m) = is_comment_or_empty("  */  let x = 1;", &p, m);
        assert!(!r);
        assert!(!m);
    }

    // Python docstrings (""" ... """)

    #[test]
    fn python_docstring_single_line() {
        let p = py_docstring_patterns();
        let m = false;
        let (r, m) = is_comment_or_empty("\"\"\"docstring\"\"\"", &p, m);
        assert!(r);
        assert!(!m);
    }

    #[test]
    fn python_docstring_multi_line() {
        let p = py_docstring_patterns();
        let m = false;

        let (r, m) = is_comment_or_empty("\"\"\"", &p, m);
        assert!(r);
        assert!(m);
        let (r, m) = is_comment_or_empty("hello world", &p, m);
        assert!(r);
        assert!(m);
        let (r, m) = is_comment_or_empty("\"\"\"", &p, m);
        assert!(r);
        assert!(!m);
    }

    // ── find_earliest_pattern helper ───────────────────────────────

    #[test]
    fn earliest_pattern_none() {
        assert_eq!(find_earliest_pattern("hello", &[]), None);
        assert_eq!(
            find_earliest_pattern("hello", &["x".into(), "y".into()]),
            None
        );
    }

    #[test]
    fn earliest_pattern_returns_first() {
        let patterns = vec!["/*".into(), "//".into()];
        let (pos, after) = find_earliest_pattern("  // not /* here", &patterns).unwrap();
        // `//` occurs before `/*` in the string
        assert_eq!(pos, 2);
        assert_eq!(after, " not /* here");
    }
}
