use criterion::{criterion_group, Criterion};

use clocscan::config::CommentPatterns;
use clocscan::file_reading::is_comment_or_empty;

pub fn is_comment_or_empty_benchmark(c: &mut Criterion) {
    let comment_patterns = CommentPatterns {
        single_line: vec![
            "//".to_string(),
            "#".to_string(),
            ";".to_string(),
            "--".to_string(),
        ],
        multi_line_start: vec!["/*".to_string()],
        multi_line_end: vec!["*/".to_string()],
    };

    let test_cases = vec![
        "fn main() {",
        "   // This is a comment",
        "let x = 5;",
        "/* multi line",
        "   comment */",
        "",
        "   ",
        "# Python comment",
    ];

    c.bench_function("is_comment_or_empty", |b| {
        let mut in_multiline = false;
        b.iter(|| {
            for line in &test_cases {
                std::hint::black_box(is_comment_or_empty(
                    line,
                    &comment_patterns,
                    &mut in_multiline,
                ));
            }
        });
    });
}

criterion_group!(benches, is_comment_or_empty_benchmark);
