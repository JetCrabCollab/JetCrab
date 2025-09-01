use jetcrab::vm::executor::instruction_dispatcher::InstructionDispatcher;
use jetcrab::vm::executor::stack_manager::StackManager;
use jetcrab::vm::executor::traits::StackOperations;
use jetcrab::vm::executor::variable_manager::VariableManagerImpl;
use jetcrab::vm::frame::Frame;
use jetcrab::vm::instructions::Instruction;
use jetcrab::vm::memory::heap::Heap;
use jetcrab::vm::registers::Registers;
use jetcrab::vm::runtime::Builtins;
use jetcrab::vm::value::Value;

#[test]
fn test_instruction_dispatcher_push_const() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    frame.set_constants(vec![Value::Number(42.0)]);
    let instruction = Instruction::PushConst(0.into());

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Number(42.0)));
}

#[test]
fn test_instruction_dispatcher_pop() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(42.0));
    let instruction = Instruction::Pop;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert!(stack.is_empty());
}

#[test]
fn test_instruction_dispatcher_dup() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(42.0));
    let instruction = Instruction::Dup;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.pop(), Some(Value::Number(42.0)));
    assert_eq!(stack.pop(), Some(Value::Number(42.0)));
}

#[test]
fn test_instruction_dispatcher_add() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(10.0));
    stack.push(Value::Number(5.0));
    let instruction = Instruction::Add;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Number(15.0)));
}

#[test]
fn test_instruction_dispatcher_subtract() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(10.0));
    stack.push(Value::Number(3.0));
    let instruction = Instruction::Sub;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Number(7.0)));
}

#[test]
fn test_instruction_dispatcher_multiply() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(4.0));
    stack.push(Value::Number(3.0));
    let instruction = Instruction::Mul;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Number(12.0)));
}

#[test]
fn test_instruction_dispatcher_divide() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(15.0));
    stack.push(Value::Number(3.0));
    let instruction = Instruction::Div;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Number(5.0)));
}

#[test]
fn test_instruction_dispatcher_modulo() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(10.0));
    stack.push(Value::Number(3.0));
    let instruction = Instruction::Mod;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Number(1.0)));
}

#[test]
fn test_instruction_dispatcher_power() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(2.0));
    stack.push(Value::Number(3.0));
    let instruction = Instruction::Exp;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Number(8.0)));
}

#[test]
fn test_instruction_dispatcher_increment() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(5.0));
    let instruction = Instruction::Inc;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Number(6.0)));
}

#[test]
fn test_instruction_dispatcher_decrement() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(5.0));
    let instruction = Instruction::Dec;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Number(4.0)));
}

#[test]
fn test_instruction_dispatcher_logical_and() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Boolean(true));
    stack.push(Value::Boolean(false));
    let instruction = Instruction::And;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Boolean(false)));
}

#[test]
fn test_instruction_dispatcher_logical_or() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Boolean(true));
    stack.push(Value::Boolean(false));
    let instruction = Instruction::Or;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Boolean(true)));
}

#[test]
fn test_instruction_dispatcher_logical_not() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Boolean(true));
    let instruction = Instruction::Not;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Boolean(false)));
}

#[test]
fn test_instruction_dispatcher_equal() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(5.0));
    stack.push(Value::Number(5.0));
    let instruction = Instruction::Eq;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Boolean(true)));
}

#[test]
fn test_instruction_dispatcher_not_equal() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(5.0));
    stack.push(Value::Number(3.0));
    let instruction = Instruction::Ne;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Boolean(true)));
}

#[test]
fn test_instruction_dispatcher_less_than() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(3.0));
    stack.push(Value::Number(5.0));
    let instruction = Instruction::Lt;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Boolean(true)));
}

#[test]
fn test_instruction_dispatcher_less_equal() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(5.0));
    stack.push(Value::Number(5.0));
    let instruction = Instruction::Le;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Boolean(true)));
}

#[test]
fn test_instruction_dispatcher_greater_than() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(5.0));
    stack.push(Value::Number(3.0));
    let instruction = Instruction::Gt;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Boolean(true)));
}

#[test]
fn test_instruction_dispatcher_greater_equal() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(5.0));
    stack.push(Value::Number(5.0));
    let instruction = Instruction::Ge;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Boolean(true)));
}

#[test]
fn test_instruction_dispatcher_strict_equal() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(5.0));
    stack.push(Value::Number(5.0));
    let instruction = Instruction::StrictEq;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Boolean(true)));
}

#[test]
fn test_instruction_dispatcher_strict_not_equal() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    stack.push(Value::Number(5.0));
    stack.push(Value::String("5".to_string()));
    let instruction = Instruction::StrictNe;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Boolean(true)));
}

#[test]
fn test_instruction_dispatcher_load_this() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    frame.this_value = Some(Value::String("test".to_string()));
    let instruction = Instruction::LoadThis;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::String("test".to_string())));
}

#[test]
fn test_instruction_dispatcher_load_this_undefined() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    let instruction = Instruction::LoadThis;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Undefined));
}

#[test]
fn test_instruction_dispatcher_load_closure_var() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    frame
        .closure_vars
        .insert("test_var".to_string(), Value::Number(42.0));
    let instruction = Instruction::LoadClosureVar("test_var".to_string());

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Number(42.0)));
}

#[test]
fn test_instruction_dispatcher_load_closure_var_undefined() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    let instruction = Instruction::LoadClosureVar("nonexistent".to_string());

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
    assert_eq!(stack.pop(), Some(Value::Undefined));
}

#[test]
fn test_instruction_dispatcher_halt() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    let instruction = Instruction::Halt;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_instruction_dispatcher_call_by_name() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    let instruction = Instruction::CallByName(0.into());

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_instruction_dispatcher_to_string() {
    let mut stack = StackManager::new();
    let mut heap = Heap::new();
    let mut variables = VariableManagerImpl::new();
    let mut frame = Frame::new();
    let mut registers = Registers::new();
    let mut builtins = Builtins::new();

    let instruction = Instruction::ToString;

    let result = InstructionDispatcher::execute_instruction(
        &instruction,
        &mut stack,
        &mut heap,
        &mut variables,
        &mut frame,
        &mut registers,
        &mut builtins,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}
