use jetcrab::vm::{Bytecode, Executor, Instruction, Value};

fn main() {
    println!("=== JetCrab VM Demo ===\n");

    println!("1. Operações Aritméticas:");
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::Add,
        Instruction::PushConst(2.into()),
        Instruction::Mul,
        Instruction::PushConst(3.into()),
        Instruction::Div,
    ]);
    let constants = vec![
        Value::Number(10.0),
        Value::Number(5.0),
        Value::Number(3.0),
        Value::Number(9.0),
    ];
    exec.execute(&bytecode, &constants);
    println!("   Resultado: {:?}", exec.stack.values);
    println!();

    println!("2. Controle de Fluxo:");
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::JumpIfTrue(5.into()),
        Instruction::PushConst(2.into()),
        Instruction::Jump(6.into()),
        Instruction::PushConst(3.into()),
        Instruction::PushConst(4.into()),
    ]);
    let constants = vec![
        Value::Number(42.0),
        Value::Boolean(true),
        Value::String("skipped".to_string()),
        Value::String("executed".to_string()),
        Value::Number(100.0),
    ];
    exec.execute(&bytecode, &constants);
    println!("   Resultado: {:?}", exec.stack.values);
    println!();

    println!("3. Variáveis Locais e Globais:");
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0.into()),
        Instruction::StoreLocal(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::StoreGlobal(0.into()),
        Instruction::LoadLocal(0.into()),
        Instruction::LoadGlobal(0.into()),
        Instruction::Add,
    ]);
    let constants = vec![Value::Number(42.0), Value::Number(100.0)];
    exec.execute(&bytecode, &constants);
    println!("   Resultado: {:?}", exec.stack.values);
    println!("   Global[0]: {:?}", exec.globals[0]);
    println!();

    println!("4. Objetos e Propriedades:");
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::NewObject,
        Instruction::Dup,
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::SetProperty,
        Instruction::Dup,
        Instruction::PushConst(2.into()),
        Instruction::PushConst(3.into()),
        Instruction::SetProperty,
        Instruction::Dup,
        Instruction::PushConst(0.into()),
        Instruction::GetProperty,
        Instruction::Dup,
        Instruction::PushConst(2.into()),
        Instruction::GetProperty,
    ]);
    let constants = vec![
        Value::String("name".to_string()),
        Value::String("John".to_string()),
        Value::String("age".to_string()),
        Value::Number(30.0),
    ];
    exec.execute(&bytecode, &constants);
    println!("   Objeto criado com propriedades");
    println!("   name: {:?}", exec.stack.values[1]);
    println!("   age: {:?}", exec.stack.values[2]);
    println!();

    println!("5. Arrays:");
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![Instruction::NewArray(0.into())]);
    exec.execute(&bytecode, &[]);
    println!("   Array criado: {:?}", exec.stack.values[0]);
    println!();

    println!("6. Concatenação de Strings:");
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::Add,
        Instruction::PushConst(2.into()),
        Instruction::Add,
        Instruction::PushConst(3.into()),
        Instruction::Add,
    ]);
    let constants = vec![
        Value::String("Hello".to_string()),
        Value::String(" ".to_string()),
        Value::String("World".to_string()),
        Value::String("!".to_string()),
    ];
    exec.execute(&bytecode, &constants);
    println!("   Resultado: {:?}", exec.stack.values[0]);
    println!();

    println!("7. Operações de Comparação:");
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::Gt,
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::Lt,
        Instruction::PushConst(2.into()),
        Instruction::PushConst(2.into()),
        Instruction::Eq,
    ]);
    let constants = vec![Value::Number(5.0), Value::Number(3.0), Value::Number(5.0)];
    exec.execute(&bytecode, &constants);
    println!("   5 > 3: {:?}", exec.stack.values[0]);
    println!("   5 < 3: {:?}", exec.stack.values[1]);
    println!("   5 == 5: {:?}", exec.stack.values[2]);
    println!();

    println!("8. Contexto de Função:");
    let mut exec = Executor::new();
    exec.frame.arguments = vec![Value::Number(42.0), Value::String("hello".to_string())];
    exec.frame.this_value = Some(Value::String("this_value".to_string()));
    exec.frame
        .closure_vars
        .insert("x".to_string(), Value::Number(100.0));

    let bytecode = Bytecode::new(vec![
        Instruction::LoadArg(0.into()),
        Instruction::LoadArg(1.into()),
        Instruction::LoadThis,
        Instruction::LoadClosureVar("x".to_string()),
    ]);
    exec.execute(&bytecode, &[]);
    println!("   Argumento 0: {:?}", exec.stack.values[0]);
    println!("   Argumento 1: {:?}", exec.stack.values[1]);
    println!("   This: {:?}", exec.stack.values[2]);
    println!("   Closure var x: {:?}", exec.stack.values[3]);
    println!();

    println!("=== Demo Concluído! ===");
    println!("O JetCrab VM agora suporta:");
    println!("✅ Operações aritméticas completas");
    println!("✅ Controle de fluxo (jumps condicionais)");
    println!("✅ Variáveis locais e globais");
    println!("✅ Sistema de objetos com propriedades");
    println!("✅ Arrays");
    println!("✅ Concatenação de strings");
    println!("✅ Operações de comparação");
    println!("✅ Contexto de função (arguments, this, closure vars)");
    println!("✅ Type-safe handles para heap objects");
    println!("✅ Frame management para chamadas de função");
}
