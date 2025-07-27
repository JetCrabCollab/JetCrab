use jetcrab::vm::{Bytecode, Executor, Instruction, Value};

#[test]
fn test_execute_basic_arithmetic() {
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::Add,
    ]);
    let constants = vec![Value::Number(3.0), Value::Number(2.0)];
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![Value::Number(5.0)]);
}

#[test]
fn test_execute_sub_mul_div() {
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::Sub,
        Instruction::PushConst(2.into()),
        Instruction::Mul,
        Instruction::PushConst(3.into()),
        Instruction::Div,
    ]);
    let constants = vec![
        Value::Number(10.0),
        Value::Number(3.0),
        Value::Number(2.0),
        Value::Number(7.0),
    ];
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![Value::Number(2.0)]);
}

#[test]
fn test_execute_pop_dup() {
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::Dup,
        Instruction::Pop,
    ]);
    let constants = vec![Value::Number(42.0), Value::Number(100.0)];
    exec.execute(&bytecode, &constants);
    assert_eq!(
        exec.stack.values,
        vec![Value::Number(42.0), Value::Number(100.0)]
    );
}

#[test]
fn test_execute_load_store_local() {
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0.into()),
        Instruction::StoreLocal(0.into()),
        Instruction::LoadLocal(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::Add,
    ]);
    let constants = vec![Value::Number(42.0), Value::Number(10.0)];
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![Value::Number(52.0)]);
}

#[test]
fn test_stack_push_pop_frame() {
    let mut exec = Executor::new();
    exec.stack.push_frame(exec.frame.clone());
    assert_eq!(exec.stack.frames.len(), 1);

    let popped_frame = exec.stack.pop_frame();
    assert!(popped_frame.is_some());
    assert_eq!(exec.stack.frames.len(), 0);
}

#[test]
fn test_execute_jump() {
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0.into()),
        Instruction::Jump(3.into()),
        Instruction::PushConst(1.into()),
        Instruction::PushConst(2.into()),
    ]);
    let constants = vec![
        Value::Number(42.0),
        Value::Number(999.0),
        Value::Number(100.0),
    ];
    exec.execute(&bytecode, &constants);
    assert_eq!(
        exec.stack.values,
        vec![Value::Number(42.0), Value::Number(100.0)]
    );
}

#[test]
fn test_execute_jump_if_true() {
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0.into()),
        Instruction::JumpIfTrue(3.into()),
        Instruction::PushConst(1.into()),
        Instruction::PushConst(2.into()),
    ]);
    let constants = vec![
        Value::Boolean(true),
        Value::Number(999.0),
        Value::Number(100.0),
    ];
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![Value::Number(100.0)]);
}

#[test]
fn test_execute_jump_if_false() {
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0.into()),
        Instruction::JumpIfFalse(3.into()),
        Instruction::PushConst(1.into()),
        Instruction::PushConst(2.into()),
    ]);
    let constants = vec![
        Value::Boolean(false),
        Value::Number(999.0),
        Value::Number(100.0),
    ];
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![Value::Number(100.0)]);
}

#[test]
fn test_execute_load_store_global() {
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0.into()),
        Instruction::StoreGlobal(0.into()),
        Instruction::LoadGlobal(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::Add,
    ]);
    let constants = vec![Value::Number(42.0), Value::Number(10.0)];
    exec.execute(&bytecode, &constants);
    assert_eq!(exec.stack.values, vec![Value::Number(52.0)]);
    assert_eq!(exec.globals[0], Value::Number(42.0));
}

#[test]
fn test_execute_comparison_operations() {
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::Lt,
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::Gt,
        Instruction::PushConst(2.into()),
        Instruction::PushConst(2.into()),
        Instruction::Eq,
    ]);
    let constants = vec![Value::Number(5.0), Value::Number(3.0), Value::Number(5.0)];
    exec.execute(&bytecode, &constants);
    assert_eq!(
        exec.stack.values,
        vec![
            Value::Boolean(false),
            Value::Boolean(true),
            Value::Boolean(true)
        ]
    );
}

#[test]
fn test_execute_new_object() {
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![Instruction::NewObject]);
    exec.execute(&bytecode, &[]);
    assert_eq!(exec.stack.values.len(), 1);
    assert!(matches!(exec.stack.values[0], Value::Object(_)));
}

#[test]
fn test_execute_new_array() {
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![Instruction::NewArray(0.into())]);
    exec.execute(&bytecode, &[]);
    assert_eq!(exec.stack.values.len(), 1);
    assert!(matches!(exec.stack.values[0], Value::Array(_)));
}

#[test]
fn test_execute_object_properties() {
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::NewObject,
        Instruction::Dup,
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::SetProperty,
        Instruction::Dup,
        Instruction::PushConst(0.into()),
        Instruction::GetProperty,
    ]);
    let constants = vec![
        Value::String("name".to_string()),
        Value::String("John".to_string()),
    ];
    exec.execute(&bytecode, &constants);

    assert_eq!(exec.stack.values.len(), 2);
    assert!(matches!(exec.stack.values[0], Value::Object(_)));
    assert_eq!(exec.stack.values[1], Value::String("John".to_string()));
}

#[test]
fn test_execute_load_arg() {
    let mut exec = Executor::new();
    exec.frame.arguments = vec![Value::Number(42.0), Value::String("hello".to_string())];

    let bytecode = Bytecode::new(vec![
        Instruction::LoadArg(0.into()),
        Instruction::LoadArg(1.into()),
    ]);
    exec.execute(&bytecode, &[]);

    assert_eq!(
        exec.stack.values,
        vec![Value::Number(42.0), Value::String("hello".to_string())]
    );
}

#[test]
fn test_execute_load_this() {
    let mut exec = Executor::new();
    exec.frame.this_value = Some(Value::String("this_value".to_string()));

    let bytecode = Bytecode::new(vec![Instruction::LoadThis]);
    exec.execute(&bytecode, &[]);

    assert_eq!(
        exec.stack.values,
        vec![Value::String("this_value".to_string())]
    );
}

#[test]
fn test_execute_load_closure_var() {
    let mut exec = Executor::new();
    exec.frame
        .closure_vars
        .insert("x".to_string(), Value::Number(42.0));
    exec.frame
        .closure_vars
        .insert("y".to_string(), Value::String("hello".to_string()));

    let bytecode = Bytecode::new(vec![
        Instruction::LoadClosureVar("x".to_string()),
        Instruction::LoadClosureVar("y".to_string()),
        Instruction::LoadClosureVar("z".to_string()),
    ]);
    exec.execute(&bytecode, &[]);

    assert_eq!(
        exec.stack.values,
        vec![
            Value::Number(42.0),
            Value::String("hello".to_string()),
            Value::Undefined
        ]
    );
}

#[test]
fn test_execute_string_concatenation() {
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::Add,
    ]);
    let constants = vec![
        Value::String("Hello".to_string()),
        Value::String("World".to_string()),
    ];
    exec.execute(&bytecode, &constants);

    assert_eq!(
        exec.stack.values,
        vec![Value::String("HelloWorld".to_string())]
    );
}

#[test]
fn test_execute_number_string_concatenation() {
    let mut exec = Executor::new();
    let bytecode = Bytecode::new(vec![
        Instruction::PushConst(0.into()),
        Instruction::PushConst(1.into()),
        Instruction::Add,
    ]);
    let constants = vec![
        Value::Number(42.0),
        Value::String(" is the answer".to_string()),
    ];
    exec.execute(&bytecode, &constants);

    assert_eq!(
        exec.stack.values,
        vec![Value::String("42 is the answer".to_string())]
    );
}
