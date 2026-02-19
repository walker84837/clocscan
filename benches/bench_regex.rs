use criterion::{criterion_group, Criterion};
use regex::Regex;

pub fn regex_matching_benchmark(c: &mut Criterion) {
    let extensions = vec![
        "rs", "js", "ts", "py", "java", "go", "cpp", "c", "h", "hpp", "jsx", "tsx", "rb", "php",
        "swift", "kt", "md", "html", "css",
    ];
    let regex_string = format!(".*\\.({})$", extensions.join("|"));
    let regex = Regex::new(&regex_string).unwrap();

    let test_paths = vec![
        "src/main.rs",
        "src/lib.rs",
        "src/utils.js",
        "src/app.ts",
        "src/index.tsx",
        "tests/test.py",
        "src/App.java",
        "src/main.go",
        "src/main.cpp",
        "include/header.h",
        "src/styles.css",
        "README.md",
    ];

    c.bench_function("regex_file_matching", |b| {
        b.iter(|| {
            for path in &test_paths {
                std::hint::black_box(regex.is_match(path));
            }
        });
    });
}

criterion_group!(benches, regex_matching_benchmark);
