use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jetcrab::lexer::tokenize;

fn lexer_benchmark(c: &mut Criterion) {
    let source = "let x = 42; function add(a, b) { return a + b; }";

    c.bench_function("tokenize", |b| b.iter(|| tokenize(black_box(source))));
}

criterion_group!(benches, lexer_benchmark);
criterion_main!(benches);
