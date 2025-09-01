use criterion::{criterion_group, criterion_main, Criterion};

fn basic_math(c: &mut Criterion) {
    c.bench_function("addition", |b| {
        b.iter(|| {
            let mut result = 0.0;
            for i in 0..100 {
                result += i as f64;
            }
            result
        })
    });
}

fn string_operations(c: &mut Criterion) {
    c.bench_function("string_concat", |b| {
        b.iter(|| {
            let mut result = String::new();
            for i in 0..100 {
                result.push_str(&i.to_string());
            }
            result
        })
    });
}

criterion_group!(benches, basic_math, string_operations);
criterion_main!(benches);
