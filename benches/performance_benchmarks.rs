use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jetcrab::api::{Compiler, Engine};
use jetcrab::ast::visitor::{AstPrinter, NodeCounter, Visitor};
use jetcrab::semantic::analyzer::SemanticAnalyzer;

fn benchmark_template_literals(c: &mut Criterion) {
    let source = r#"
        const name = "World";
        const greeting = `Hello ${name}!`;
        const multi = `Line 1
        Line 2 ${name}
        Line 3 ${name.toUpperCase()}`;
    "#;

    c.bench_function("template_literals", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            engine.evaluate(black_box(source))
        })
    });
}

fn benchmark_meta_properties(c: &mut Criterion) {
    let source = r#"
        function MyClass() {
            if (new.target !== MyClass) {
                throw new Error("Must be called with new");
            }
        }
        new MyClass();
    "#;

    c.bench_function("meta_properties", |b| {
        b.iter(|| {
            let mut engine = Engine::new();
            engine.evaluate(black_box(source))
        })
    });
}

fn benchmark_visitor_pattern(c: &mut Criterion) {
    let source = "let x = 5; let y = 10; let z = x + y;";
    let mut parser = jetcrab::parser::Parser::new(source);
    let ast = parser.parse().unwrap();

    c.bench_function("node_counter", |b| {
        b.iter(|| {
            let mut counter = NodeCounter::new();
            counter.visit_node(black_box(&ast))
        })
    });

    c.bench_function("ast_printer", |b| {
        b.iter(|| {
            let mut printer = AstPrinter::new();
            printer.visit_node(black_box(&ast))
        })
    });
}

fn benchmark_semantic_analysis(c: &mut Criterion) {
    let source = r#"
        let x = 5;
        let y = 10;
        function test(a, b) {
            let result = a + b;
            return result;
        }
        test(x, y);
    "#;

    let mut parser = jetcrab::parser::Parser::new(source);
    let ast = parser.parse().unwrap();

    c.bench_function("semantic_analysis", |b| {
        b.iter(|| {
            let mut analyzer = SemanticAnalyzer::new();
            analyzer.analyze(black_box(&ast))
        })
    });
}

fn benchmark_bytecode_generation(c: &mut Criterion) {
    let source = r#"
        let x = 5;
        let y = 10;
        let result = x + y * 2;
        if (result > 20) {
            return "high";
        } else {
            return "low";
        }
    "#;

    let mut compiler = Compiler::new();

    c.bench_function("bytecode_generation", |b| {
        b.iter(|| compiler.compile(black_box(source)))
    });
}

fn benchmark_large_program(c: &mut Criterion) {
    let source = r#"

        let data = [];
        for (let i = 0; i < 100; i++) {
            data.push({
                id: i,
                name: `Item ${i}`,
                value: i * 2,
                active: i % 2 === 0
            });
        }
        
        const filtered = data.filter(item => item.active);
        const mapped = filtered.map(item => ({
            ...item,
            computed: item.value * 2
        }));
        
        let sum = 0;
        for (const item of mapped) {
            sum += item.computed;
        }
        
        return sum;
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
    benchmark_meta_properties,
    benchmark_visitor_pattern,
    benchmark_semantic_analysis,
    benchmark_bytecode_generation,
    benchmark_large_program
);
criterion_main!(benches);
