use jetcrab::vm::executor::instruction_handlers::ControlFlowHandler;
use jetcrab::vm::executor::stack_manager::StackManager;
use jetcrab::vm::executor::traits::StackOperations;
use jetcrab::vm::executor::variable_manager::VariableManagerImpl;
use jetcrab::vm::registers::Registers;
use jetcrab::vm::types::CodeAddress;
use jetcrab::vm::value::Value;

#[test]
fn test_control_flow_handler_jump() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(100);

    let result = ControlFlowHandler::jump::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 100);
}

#[test]
fn test_control_flow_handler_jump_if_true_boolean_true() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(50);

    stack.push(Value::Boolean(true));
    let result = ControlFlowHandler::jump_if_true::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 50);
}

#[test]
fn test_control_flow_handler_jump_if_true_boolean_false() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(50);

    stack.push(Value::Boolean(false));
    let result = ControlFlowHandler::jump_if_true::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_control_flow_handler_jump_if_true_number_non_zero() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(75);

    stack.push(Value::Number(42.0));
    let result = ControlFlowHandler::jump_if_true::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 75);
}

#[test]
fn test_control_flow_handler_jump_if_true_number_zero() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(75);

    stack.push(Value::Number(0.0));
    let result = ControlFlowHandler::jump_if_true::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_control_flow_handler_jump_if_true_number_nan() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(75);

    stack.push(Value::Number(f64::NAN));
    let result = ControlFlowHandler::jump_if_true::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_control_flow_handler_jump_if_true_string_non_empty() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(25);

    stack.push(Value::String("hello".to_string()));
    let result = ControlFlowHandler::jump_if_true::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 25);
}

#[test]
fn test_control_flow_handler_jump_if_true_string_empty() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(25);

    stack.push(Value::String("".to_string()));
    let result = ControlFlowHandler::jump_if_true::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_control_flow_handler_jump_if_true_null() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(30);

    stack.push(Value::Null);
    let result = ControlFlowHandler::jump_if_true::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_control_flow_handler_jump_if_true_undefined() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(30);

    stack.push(Value::Undefined);
    let result = ControlFlowHandler::jump_if_true::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_control_flow_handler_jump_if_true_other_value() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(40);

    stack.push(Value::Object(jetcrab::vm::handle::ObjectHandle::new(
        0.into(),
    )));
    let result = ControlFlowHandler::jump_if_true::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 40);
}

#[test]
fn test_control_flow_handler_jump_if_true_stack_underflow() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(50);

    let result = ControlFlowHandler::jump_if_true::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_err());
}

#[test]
fn test_control_flow_handler_jump_if_false_boolean_true() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(60);

    stack.push(Value::Boolean(true));
    let result = ControlFlowHandler::jump_if_false::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_control_flow_handler_jump_if_false_boolean_false() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(60);

    stack.push(Value::Boolean(false));
    let result = ControlFlowHandler::jump_if_false::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 60);
}

#[test]
fn test_control_flow_handler_jump_if_false_number_non_zero() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(80);

    stack.push(Value::Number(42.0));
    let result = ControlFlowHandler::jump_if_false::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_control_flow_handler_jump_if_false_number_zero() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(80);

    stack.push(Value::Number(0.0));
    let result = ControlFlowHandler::jump_if_false::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 80);
}

#[test]
fn test_control_flow_handler_jump_if_false_number_nan() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(80);

    stack.push(Value::Number(f64::NAN));
    let result = ControlFlowHandler::jump_if_false::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 80);
}

#[test]
fn test_control_flow_handler_jump_if_false_string_non_empty() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(90);

    stack.push(Value::String("hello".to_string()));
    let result = ControlFlowHandler::jump_if_false::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_control_flow_handler_jump_if_false_string_empty() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(90);

    stack.push(Value::String("".to_string()));
    let result = ControlFlowHandler::jump_if_false::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 90);
}

#[test]
fn test_control_flow_handler_jump_if_false_null() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(100);

    stack.push(Value::Null);
    let result = ControlFlowHandler::jump_if_false::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 100);
}

#[test]
fn test_control_flow_handler_jump_if_false_undefined() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(100);

    stack.push(Value::Undefined);
    let result = ControlFlowHandler::jump_if_false::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 100);
}

#[test]
fn test_control_flow_handler_jump_if_false_other_value() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(110);

    stack.push(Value::Object(jetcrab::vm::handle::ObjectHandle::new(
        0.into(),
    )));
    let result = ControlFlowHandler::jump_if_false::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_control_flow_handler_jump_if_false_stack_underflow() {
    let mut stack = StackManager::new();
    let mut registers = Registers::new();
    let target_ip = CodeAddress::new(120);

    let result = ControlFlowHandler::jump_if_false::<StackManager, VariableManagerImpl>(
        &mut stack,
        &mut registers,
        target_ip,
    );

    assert!(result.is_err());
}
