use criterion::{criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci_20", |b| b.iter(|| fibonacci(20)));
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);
