use criterion::{black_box, criterion_group, criterion_main, Criterion};

use jetcrab::parser::Parser;

fn parser_benchmark(c: &mut Criterion) {
    let source = "let x = 42; function add(a, b) { return a + b; }";

    c.bench_function("parse", |b| {
        b.iter(|| {
            let mut parser = Parser::new(black_box(source));
            parser.parse()
        })
    });
}

criterion_group!(benches, parser_benchmark);
criterion_main!(benches);
