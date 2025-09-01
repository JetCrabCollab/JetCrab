//! Unit tests for VM Error types

use jetcrab::ast::Position;
use jetcrab::vm::error::VmError;

#[test]
fn test_vm_error_execution_error() {
    let error = VmError::ExecutionError {
        message: "Test execution error".to_string(),
        instruction: Some("ADD".to_string()),
        position: Some(Position::new(10, 5)),
    };

    assert!(matches!(error, VmError::ExecutionError { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("VM execution error: Test execution error"));
    assert!(error_string.contains("instruction: ADD"));
    assert!(error_string.contains("at line 10, column 5"));
}

#[test]
fn test_vm_error_execution_error_no_instruction() {
    let error = VmError::ExecutionError {
        message: "Test execution error".to_string(),
        instruction: None,
        position: Some(Position::new(10, 5)),
    };

    let error_string = format!("{}", error);
    assert!(error_string.contains("VM execution error: Test execution error"));
    assert!(!error_string.contains("instruction:"));
    assert!(error_string.contains("at line 10, column 5"));
}

#[test]
fn test_vm_error_execution_error_no_position() {
    let error = VmError::ExecutionError {
        message: "Test execution error".to_string(),
        instruction: Some("ADD".to_string()),
        position: None,
    };

    let error_string = format!("{}", error);
    assert!(error_string.contains("VM execution error: Test execution error"));
    assert!(error_string.contains("instruction: ADD"));
    assert!(!error_string.contains("at line"));
}

#[test]
fn test_vm_error_stack_underflow() {
    let error = VmError::StackUnderflow {
        message: "Stack is empty".to_string(),
        position: Some(Position::new(5, 3)),
    };

    assert!(matches!(error, VmError::StackUnderflow { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("Stack underflow: Stack is empty"));
    assert!(error_string.contains("at line 5, column 3"));
}

#[test]
fn test_vm_error_stack_overflow() {
    let error = VmError::StackOverflow {
        message: "Stack is full".to_string(),
        position: Some(Position::new(15, 8)),
    };

    assert!(matches!(error, VmError::StackOverflow { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("Stack overflow: Stack is full"));
    assert!(error_string.contains("at line 15, column 8"));
}

#[test]
fn test_vm_error_invalid_instruction() {
    let error = VmError::InvalidInstruction {
        instruction: "INVALID".to_string(),
        message: "Unknown instruction".to_string(),
        position: Some(Position::new(20, 12)),
    };

    assert!(matches!(error, VmError::InvalidInstruction { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("Invalid instruction 'INVALID': Unknown instruction"));
    assert!(error_string.contains("at line 20, column 12"));
}

#[test]
fn test_vm_error_type_mismatch() {
    let error = VmError::TypeMismatch {
        expected: "Number".to_string(),
        found: "String".to_string(),
        position: Some(Position::new(25, 7)),
    };

    assert!(matches!(error, VmError::TypeMismatch { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("Type mismatch: expected Number, found String"));
    assert!(error_string.contains("at line 25, column 7"));
}

#[test]
fn test_vm_error_undefined_variable() {
    let error = VmError::UndefinedVariable {
        name: "myVar".to_string(),
        position: Some(Position::new(30, 4)),
    };

    assert!(matches!(error, VmError::UndefinedVariable { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("Undefined variable 'myVar'"));
    assert!(error_string.contains("at line 30, column 4"));
}

#[test]
fn test_vm_error_undefined_function() {
    let error = VmError::UndefinedFunction {
        name: "myFunction".to_string(),
        position: Some(Position::new(35, 9)),
    };

    assert!(matches!(error, VmError::UndefinedFunction { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("Undefined function 'myFunction'"));
    assert!(error_string.contains("at line 35, column 9"));
}

#[test]
fn test_vm_error_division_by_zero() {
    let error = VmError::DivisionByZero {
        position: Some(Position::new(40, 6)),
    };

    assert!(matches!(error, VmError::DivisionByZero { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("Division by zero"));
    assert!(error_string.contains("at line 40, column 6"));
}

#[test]
fn test_vm_error_division_by_zero_no_position() {
    let error = VmError::DivisionByZero { position: None };

    let error_string = format!("{}", error);
    assert!(error_string.contains("Division by zero"));
    assert!(!error_string.contains("at line"));
}

#[test]
fn test_vm_error_out_of_memory() {
    let error = VmError::OutOfMemory {
        message: "Heap exhausted".to_string(),
        position: Some(Position::new(45, 2)),
    };

    assert!(matches!(error, VmError::OutOfMemory { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("Out of memory: Heap exhausted"));
    assert!(error_string.contains("at line 45, column 2"));
}

#[test]
fn test_vm_error_runtime_error() {
    let error = VmError::RuntimeError {
        message: "Unexpected runtime condition".to_string(),
        position: Some(Position::new(50, 11)),
    };

    assert!(matches!(error, VmError::RuntimeError { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("Runtime error: Unexpected runtime condition"));
    assert!(error_string.contains("at line 50, column 11"));
}

#[test]
fn test_vm_error_type_error() {
    let error = VmError::TypeError {
        message: "Cannot read property of undefined".to_string(),
        position: Some(Position::new(55, 8)),
    };

    assert!(matches!(error, VmError::TypeError { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("TypeError: Cannot read property of undefined"));
    assert!(error_string.contains("at line 55, column 8"));
}

#[test]
fn test_vm_error_reference_error() {
    let error = VmError::ReferenceError {
        message: "Variable is not defined".to_string(),
        position: Some(Position::new(60, 3)),
    };

    assert!(matches!(error, VmError::ReferenceError { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("ReferenceError: Variable is not defined"));
    assert!(error_string.contains("at line 60, column 3"));
}

#[test]
fn test_vm_error_range_error() {
    let error = VmError::RangeError {
        message: "Index out of bounds".to_string(),
        position: Some(Position::new(65, 14)),
    };

    assert!(matches!(error, VmError::RangeError { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("RangeError: Index out of bounds"));
    assert!(error_string.contains("at line 65, column 14"));
}

#[test]
fn test_vm_error_syntax_error() {
    let error = VmError::SyntaxError {
        message: "Unexpected token".to_string(),
        position: Some(Position::new(70, 1)),
    };

    assert!(matches!(error, VmError::SyntaxError { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("SyntaxError: Unexpected token"));
    assert!(error_string.contains("at line 70, column 1"));
}

#[test]
fn test_vm_error_undefined_property() {
    let error = VmError::UndefinedProperty {
        object: "obj".to_string(),
        property: "prop".to_string(),
        position: Some(Position::new(75, 5)),
    };

    assert!(matches!(error, VmError::UndefinedProperty { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("Cannot read property 'prop' of obj"));
    assert!(error_string.contains("at line 75, column 5"));
}

#[test]
fn test_vm_error_invalid_function_call() {
    let error = VmError::InvalidFunctionCall {
        function: "myFunc".to_string(),
        message: "Wrong number of arguments".to_string(),
        position: Some(Position::new(80, 7)),
    };

    assert!(matches!(error, VmError::InvalidFunctionCall { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("Invalid function call 'myFunc': Wrong number of arguments"));
    assert!(error_string.contains("at line 80, column 7"));
}

#[test]
fn test_vm_error_context_error() {
    let error = VmError::ContextError {
        message: "Invalid execution context".to_string(),
        position: Some(Position::new(85, 9)),
    };

    assert!(matches!(error, VmError::ContextError { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("Context error: Invalid execution context"));
    assert!(error_string.contains("at line 85, column 9"));
}

#[test]
fn test_vm_error_builtin_error() {
    let error = VmError::BuiltinError {
        builtin: "Math.sqrt".to_string(),
        message: "Invalid argument".to_string(),
        position: Some(Position::new(90, 13)),
    };

    assert!(matches!(error, VmError::BuiltinError { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("Builtin error 'Math.sqrt': Invalid argument"));
    assert!(error_string.contains("at line 90, column 13"));
}

#[test]
fn test_vm_error_object_error() {
    let error = VmError::ObjectError {
        message: "Object creation failed".to_string(),
        position: Some(Position::new(95, 6)),
    };

    assert!(matches!(error, VmError::ObjectError { .. }));
    let error_string = format!("{}", error);
    assert!(error_string.contains("Object error: Object creation failed"));
    assert!(error_string.contains("at line 95, column 6"));
}

#[test]
fn test_vm_error_implements_error_trait() {
    let error = VmError::RuntimeError {
        message: "Test error".to_string(),
        position: None,
    };

    let error_ref: &dyn std::error::Error = &error;
    assert!(error_ref.source().is_none());
}

#[test]
fn test_vm_error_clone() {
    let error = VmError::TypeMismatch {
        expected: "Number".to_string(),
        found: "String".to_string(),
        position: Some(Position::new(1, 1)),
    };

    let cloned_error = error.clone();
    assert!(matches!(cloned_error, VmError::TypeMismatch { .. }));
}

#[test]
fn test_vm_error_debug() {
    let error = VmError::StackUnderflow {
        message: "Test".to_string(),
        position: None,
    };

    let debug_string = format!("{:?}", error);
    assert!(debug_string.contains("StackUnderflow"));
    assert!(debug_string.contains("Test"));
}

#[test]
fn test_vm_error_serialization() {
    let error = VmError::ExecutionError {
        message: "Test error".to_string(),
        instruction: Some("ADD".to_string()),
        position: Some(Position::new(1, 1)),
    };

    let serialized = serde_json::to_string(&error).unwrap();
    assert!(serialized.contains("ExecutionError"));
    assert!(serialized.contains("Test error"));
    assert!(serialized.contains("ADD"));

    let deserialized: VmError = serde_json::from_str(&serialized).unwrap();
    assert!(matches!(deserialized, VmError::ExecutionError { .. }));
}

#[test]
fn test_vm_error_all_variants() {
    let errors = vec![
        VmError::ExecutionError {
            message: "exec".to_string(),
            instruction: None,
            position: None,
        },
        VmError::StackUnderflow {
            message: "underflow".to_string(),
            position: None,
        },
        VmError::StackOverflow {
            message: "overflow".to_string(),
            position: None,
        },
        VmError::InvalidInstruction {
            instruction: "INVALID".to_string(),
            message: "invalid".to_string(),
            position: None,
        },
        VmError::TypeMismatch {
            expected: "Number".to_string(),
            found: "String".to_string(),
            position: None,
        },
        VmError::UndefinedVariable {
            name: "var".to_string(),
            position: None,
        },
        VmError::UndefinedFunction {
            name: "func".to_string(),
            position: None,
        },
        VmError::DivisionByZero { position: None },
        VmError::OutOfMemory {
            message: "memory".to_string(),
            position: None,
        },
        VmError::RuntimeError {
            message: "runtime".to_string(),
            position: None,
        },
        VmError::TypeError {
            message: "type".to_string(),
            position: None,
        },
        VmError::ReferenceError {
            message: "reference".to_string(),
            position: None,
        },
        VmError::RangeError {
            message: "range".to_string(),
            position: None,
        },
        VmError::SyntaxError {
            message: "syntax".to_string(),
            position: None,
        },
        VmError::UndefinedProperty {
            object: "obj".to_string(),
            property: "prop".to_string(),
            position: None,
        },
        VmError::InvalidFunctionCall {
            function: "func".to_string(),
            message: "call".to_string(),
            position: None,
        },
        VmError::ContextError {
            message: "context".to_string(),
            position: None,
        },
        VmError::BuiltinError {
            builtin: "builtin".to_string(),
            message: "builtin".to_string(),
            position: None,
        },
        VmError::ObjectError {
            message: "object".to_string(),
            position: None,
        },
    ];

    for error in errors {
        let error_string = format!("{}", error);
        assert!(!error_string.is_empty());
    }
}
