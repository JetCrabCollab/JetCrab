use jetcrab::ast::Position;
use jetcrab::vm::error::VmError;

#[test]
fn test_vm_error_execution_error() {
    let error = VmError::ExecutionError {
        message: "Test error".to_string(),
        instruction: Some("ADD".to_string()),
        position: None,
    };
    let error_str = format!("{error:?}");
    assert!(error_str.contains("Test error"));
}

#[test]
fn test_vm_error_stack_underflow() {
    let error = VmError::StackUnderflow {
        message: "Stack underflow".to_string(),
        position: None,
    };
    let error_str = format!("{error:?}");
    assert!(error_str.contains("StackUnderflow"));
}

#[test]
fn test_vm_error_stack_overflow() {
    let error = VmError::StackOverflow {
        message: "Stack overflow".to_string(),
        position: None,
    };
    let error_str = format!("{error:?}");
    assert!(error_str.contains("StackOverflow"));
}

#[test]
fn test_vm_error_invalid_instruction() {
    let error = VmError::InvalidInstruction {
        instruction: "INVALID".to_string(),
        message: "Invalid instruction".to_string(),
        position: None,
    };
    let error_str = format!("{error:?}");
    assert!(error_str.contains("InvalidInstruction"));
}

#[test]
fn test_vm_error_type_mismatch() {
    let error = VmError::TypeMismatch {
        expected: "Number".to_string(),
        found: "String".to_string(),
        position: None,
    };
    let error_str = format!("{error:?}");
    assert!(error_str.contains("TypeMismatch"));
}

#[test]
fn test_vm_error_undefined_variable() {
    let error = VmError::UndefinedVariable {
        name: "x".to_string(),
        position: None,
    };
    let error_str = format!("{error:?}");
    assert!(error_str.contains("UndefinedVariable"));
}

#[test]
fn test_vm_error_undefined_function() {
    let error = VmError::UndefinedFunction {
        name: "func".to_string(),
        position: None,
    };
    let error_str = format!("{error:?}");
    assert!(error_str.contains("UndefinedFunction"));
}

#[test]
fn test_vm_error_division_by_zero() {
    let error = VmError::DivisionByZero { position: None };
    let error_str = format!("{error:?}");
    assert!(error_str.contains("DivisionByZero"));
}

#[test]
fn test_vm_error_out_of_memory() {
    let error = VmError::OutOfMemory {
        message: "Out of memory".to_string(),
        position: None,
    };
    let error_str = format!("{error:?}");
    assert!(error_str.contains("OutOfMemory"));
}

#[test]
fn test_vm_error_runtime_error() {
    let error = VmError::RuntimeError {
        message: "Runtime error".to_string(),
        position: None,
    };
    let error_str = format!("{error:?}");
    assert!(error_str.contains("RuntimeError"));
}

#[test]
fn test_vm_error_with_position() {
    let pos = Position::new(10, 5);
    let error = VmError::ExecutionError {
        message: "Test error".to_string(),
        instruction: None,
        position: Some(pos),
    };
    let error_str = format!("{error:?}");
    assert!(error_str.contains("Test error"));
}

#[test]
fn test_vm_error_clone() {
    let error = VmError::ExecutionError {
        message: "Test error".to_string(),
        instruction: None,
        position: None,
    };
    let cloned = error.clone();
    assert!(format!("{error:?}") == format!("{cloned:?}"));
}

#[test]
fn test_vm_error_display() {
    let error = VmError::ExecutionError {
        message: "Test error".to_string(),
        instruction: None,
        position: None,
    };
    let display_str = error.to_string();
    assert!(display_str.contains("VM execution error"));
    assert!(display_str.contains("Test error"));
}
