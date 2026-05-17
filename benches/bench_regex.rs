use criterion::{Criterion, criterion_group};
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;

pub fn extension_matching_benchmark(c: &mut Criterion) {
    let extensions = vec![
        "rs", "js", "ts", "py", "java", "go", "cpp", "c", "h", "hpp", "jsx", "tsx", "rb", "php",
        "swift", "kt", "md", "html", "css",
    ];
    let regex_string = format!(".*\\.({})$", extensions.join("|"));
    let regex = Regex::new(&regex_string).unwrap();

    let extension_set: HashSet<String> = extensions.iter().cloned().map(String::from).collect();

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

    let mut group = c.benchmark_group("extension_matching");

    group.bench_function("regex", |b| {
        b.iter(|| {
            for path in &test_paths {
                std::hint::black_box(regex.is_match(path));
            }
        });
    });

    group.bench_function("hashset", |b| {
        b.iter(|| {
            for path in &test_paths {
                let ext = Path::new(path)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");
                std::hint::black_box(extension_set.contains(ext));
            }
        });
    });

    group.finish();
}

criterion_group!(benches, extension_matching_benchmark);
