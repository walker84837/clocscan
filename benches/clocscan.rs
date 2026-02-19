mod bench_is_comment;
mod bench_regex;

use criterion::criterion_main;

criterion_main!(bench_is_comment::benches, bench_regex::benches);
