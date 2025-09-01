use criterion::{criterion_group, criterion_main, Criterion};

fn simple_bench(c: &mut Criterion) {
    c.bench_function("simple", |b| {
        b.iter(|| {
            let mut sum = 0;
            for i in 0..100 {
                sum += i;
            }
            sum
        })
    });
}

criterion_group!(benches, simple_bench);
criterion_main!(benches);
