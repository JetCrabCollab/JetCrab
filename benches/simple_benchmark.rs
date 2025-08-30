use criterion::{criterion_group, criterion_main, Criterion};

fn simple_benchmark(c: &mut Criterion) {
    c.bench_function("simple_math", |b| {
        b.iter(|| {
            let mut sum = 0.0;
            for i in 0..1000 {
                sum += i as f64;
            }
            sum
        })
    });
}

criterion_group!(benches, simple_benchmark);
criterion_main!(benches);
