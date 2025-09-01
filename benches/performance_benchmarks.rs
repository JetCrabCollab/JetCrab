use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jetcrab::api::Engine;

fn benchmark_template_literals(c: &mut Criterion) {
    let source = r#"
        const name = "World";
        const greeting = `Hello ${name}!`;
        const multi = `Line 1
        Line 2 ${name}
        Line 3 ${name}`;
    "#;

    c.bench_function("template_literals", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            engine.evaluate(black_box(source))
        })
    });
}

fn benchmark_function_execution(c: &mut Criterion) {
    let source = r#"
        function add(a, b) {
            return a + b;
        }
        function multiply(a, b) {
            return a * b;
        }
        add(5, 3) + multiply(2, 4);
    "#;

    c.bench_function("function_execution", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            engine.evaluate(black_box(source))
        })
    });
}

fn benchmark_object_operations(c: &mut Criterion) {
    let source = r#"
        let obj = {
            name: "test",
            value: 42,
            nested: {
                prop: "nested_value"
            }
        };
        obj.name + obj.nested.prop;
    "#;

    c.bench_function("object_operations", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            engine.evaluate(black_box(source))
        })
    });
}

fn benchmark_builtin_functions(c: &mut Criterion) {
    let source = r#"
        console.log("Hello", "World", 42);
        JSON.stringify("Hello World");
        Math.sqrt(16);
        "Hello World".length;
        [1, 2, 3, 4, 5].length;
    "#;

    c.bench_function("builtin_functions", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            engine.evaluate(black_box(source))
        })
    });
}

fn benchmark_complex_expression(c: &mut Criterion) {
    let source = r#"
        let x = 5;
        let y = 10;
        let result = x + y * 2;
        if (result > 20) {
            "high"
        } else {
            "low"
        }
    "#;

    c.bench_function("complex_expression", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            engine.evaluate(black_box(source))
        })
    });
}

fn benchmark_large_program(c: &mut Criterion) {
    let source = r#"
        let data = [];
        for (let i = 0; i < 10; i++) {
            data.push({
                id: i,
                name: `Item ${i}`,
                value: i * 2,
                active: i % 2 === 0
            });
        }
        
        let sum = 0;
        for (let i = 0; i < data.length; i++) {
            if (data[i].active) {
                sum += data[i].value;
            }
        }
        
        sum;
    "#;

    c.bench_function("large_program", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            engine.evaluate(black_box(source))
        })
    });
}

criterion_group!(
    benches,
    benchmark_template_literals,
    benchmark_function_execution,
    benchmark_object_operations,
    benchmark_builtin_functions,
    benchmark_complex_expression,
    benchmark_large_program
);
criterion_main!(benches);
