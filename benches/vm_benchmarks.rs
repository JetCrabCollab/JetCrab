use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jetcrab::vm::{instructions::Instruction, Bytecode, Executor, Value};

fn vm_benchmark(c: &mut Criterion) {
    let instructions = vec![
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::Add,
    ];
    let constants = vec![Value::Number(42.0), Value::Number(10.0)];
    let bytecode = Bytecode::new(instructions);

    c.bench_function("execute", |b| {
        b.iter(|| {
            let mut executor = Executor::new();
            executor.execute(&bytecode, &constants)
        })
    });
}

criterion_group!(benches, vm_benchmark);
criterion_main!(benches);
