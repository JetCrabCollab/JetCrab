use criterion::{criterion_group, criterion_main, Criterion};
use jetcrab::vm::executor::Executor;
use jetcrab::vm::{instructions::Instruction, Bytecode, Value};

fn vm_benchmark(c: &mut Criterion) {
    let instructions = vec![
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::Add,
    ];
    let constants = vec![Value::Number(42.0), Value::Number(10.0)];
    let bytecode = Bytecode::new(instructions);

    c.bench_function("vm_execute", |b| {
        b.iter(|| {
            let mut executor = Executor::new();
            let _ = executor.execute(&bytecode, &constants);
        })
    });
}

fn vm_arithmetic_benchmark(c: &mut Criterion) {
    let instructions = vec![
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::Add,
        Instruction::PushConst(2.into()),
        Instruction::Add,
    ];
    let constants = vec![Value::Number(5.0), Value::Number(3.0), Value::Number(2.0)];
    let bytecode = Bytecode::new(instructions);

    c.bench_function("vm_arithmetic", |b| {
        b.iter(|| {
            let mut executor = Executor::new();
            let _ = executor.execute(&bytecode, &constants);
        })
    });
}

criterion_group!(benches, vm_benchmark, vm_arithmetic_benchmark);
criterion_main!(benches);
