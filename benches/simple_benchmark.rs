use criterion::{criterion_group, criterion_main, Criterion};
use jetcrab::api::Engine;

fn basic_arithmetic_benchmark(c: &mut Criterion) {
    c.bench_function("basic_arithmetic", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            engine.evaluate("2 + 2 * 3")
        })
    });
}

fn variable_declaration_benchmark(c: &mut Criterion) {
    c.bench_function("variable_declaration", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            engine.evaluate("let x = 10; let y = 5; x + y")
        })
    });
}

fn function_call_benchmark(c: &mut Criterion) {
    c.bench_function("function_call", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            engine.evaluate(
                r#"
                function add(a, b) {
                    return a + b;
                }
                add(5, 3)
            "#,
            )
        })
    });
}

fn template_literal_benchmark(c: &mut Criterion) {
    c.bench_function("template_literal", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            engine.evaluate(
                r#"
                let name = "World";
                `Hello ${name}!`
            "#,
            )
        })
    });
}

fn object_literal_benchmark(c: &mut Criterion) {
    c.bench_function("object_literal", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            engine.evaluate(
                r#"
                let obj = { name: "test", value: 42 };
                obj.name
            "#,
            )
        })
    });
}

criterion_group!(
    benches,
    basic_arithmetic_benchmark,
    variable_declaration_benchmark,
    function_call_benchmark,
    template_literal_benchmark,
    object_literal_benchmark
);
criterion_main!(benches);
