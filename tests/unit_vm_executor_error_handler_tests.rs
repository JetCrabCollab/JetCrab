//! Unit tests for VM executor error handler module
//!
//! This module tests the ExecutionError enum and all its associated methods,
//! including error creation, display formatting, and utility functions.

use jetcrab::vm::executor::error_handler::ExecutionError;
use std::error::Error;

#[cfg(test)]
mod execution_error_tests {
    use super::*;

    // Test basic error creation and variants
    #[test]
    fn test_stack_underflow_error() {
        let error = ExecutionError::StackUnderflow;
        assert!(matches!(error, ExecutionError::StackUnderflow));
    }

    #[test]
    fn test_stack_overflow_error() {
        let error = ExecutionError::StackOverflow;
        assert!(matches!(error, ExecutionError::StackOverflow));
    }

    #[test]
    fn test_type_error_creation() {
        let error = ExecutionError::TypeError("Cannot add string and number".to_string());
        assert!(matches!(error, ExecutionError::TypeError(_)));
        if let ExecutionError::TypeError(msg) = error {
            assert_eq!(msg, "Cannot add string and number");
        }
    }

    #[test]
    fn test_runtime_error_creation() {
        let error = ExecutionError::RuntimeError("Function execution failed".to_string());
        assert!(matches!(error, ExecutionError::RuntimeError(_)));
        if let ExecutionError::RuntimeError(msg) = error {
            assert_eq!(msg, "Function execution failed");
        }
    }

    #[test]
    fn test_memory_error_creation() {
        let error = ExecutionError::MemoryError("Heap allocation failed".to_string());
        assert!(matches!(error, ExecutionError::MemoryError(_)));
        if let ExecutionError::MemoryError(msg) = error {
            assert_eq!(msg, "Heap allocation failed");
        }
    }

    #[test]
    fn test_invalid_instruction_error() {
        let error = ExecutionError::InvalidInstruction("Unknown opcode".to_string());
        assert!(matches!(error, ExecutionError::InvalidInstruction(_)));
        if let ExecutionError::InvalidInstruction(msg) = error {
            assert_eq!(msg, "Unknown opcode");
        }
    }

    #[test]
    fn test_function_error_creation() {
        let error = ExecutionError::FunctionError("Invalid function call".to_string());
        assert!(matches!(error, ExecutionError::FunctionError(_)));
        if let ExecutionError::FunctionError(msg) = error {
            assert_eq!(msg, "Invalid function call");
        }
    }

    #[test]
    fn test_variable_error_creation() {
        let error = ExecutionError::VariableError("Variable not defined".to_string());
        assert!(matches!(error, ExecutionError::VariableError(_)));
        if let ExecutionError::VariableError(msg) = error {
            assert_eq!(msg, "Variable not defined");
        }
    }

    #[test]
    fn test_heap_error_creation() {
        let error = ExecutionError::HeapError("Garbage collection failed".to_string());
        assert!(matches!(error, ExecutionError::HeapError(_)));
        if let ExecutionError::HeapError(msg) = error {
            assert_eq!(msg, "Garbage collection failed");
        }
    }

    #[test]
    fn test_control_flow_error_creation() {
        let error = ExecutionError::ControlFlowError("Invalid jump target".to_string());
        assert!(matches!(error, ExecutionError::ControlFlowError(_)));
        if let ExecutionError::ControlFlowError(msg) = error {
            assert_eq!(msg, "Invalid jump target");
        }
    }

    #[test]
    fn test_builtin_error_creation() {
        let error = ExecutionError::BuiltinError("Console not available".to_string());
        assert!(matches!(error, ExecutionError::BuiltinError(_)));
        if let ExecutionError::BuiltinError(msg) = error {
            assert_eq!(msg, "Console not available");
        }
    }

    #[test]
    fn test_division_by_zero_error() {
        let error = ExecutionError::DivisionByZero;
        assert!(matches!(error, ExecutionError::DivisionByZero));
    }

    #[test]
    fn test_invalid_index_error() {
        let error = ExecutionError::InvalidIndex(42);
        assert!(matches!(error, ExecutionError::InvalidIndex(_)));
        if let ExecutionError::InvalidIndex(idx) = error {
            assert_eq!(idx, 42);
        }
    }

    #[test]
    fn test_property_not_found_error() {
        let error = ExecutionError::PropertyNotFound("length".to_string());
        assert!(matches!(error, ExecutionError::PropertyNotFound(_)));
        if let ExecutionError::PropertyNotFound(prop) = error {
            assert_eq!(prop, "length");
        }
    }

    #[test]
    fn test_method_not_found_error() {
        let error = ExecutionError::MethodNotFound("toString".to_string());
        assert!(matches!(error, ExecutionError::MethodNotFound(_)));
        if let ExecutionError::MethodNotFound(method) = error {
            assert_eq!(method, "toString");
        }
    }

    #[test]
    fn test_invalid_argument_count_error() {
        let error = ExecutionError::InvalidArgumentCount {
            expected: 2,
            received: 3,
        };
        assert!(matches!(error, ExecutionError::InvalidArgumentCount { .. }));
        if let ExecutionError::InvalidArgumentCount { expected, received } = error {
            assert_eq!(expected, 2);
            assert_eq!(received, 3);
        }
    }

    #[test]
    fn test_recursion_limit_exceeded_error() {
        let error = ExecutionError::RecursionLimitExceeded(1000);
        assert!(matches!(error, ExecutionError::RecursionLimitExceeded(_)));
        if let ExecutionError::RecursionLimitExceeded(limit) = error {
            assert_eq!(limit, 1000);
        }
    }

    #[test]
    fn test_execution_timeout_error() {
        let error = ExecutionError::ExecutionTimeout;
        assert!(matches!(error, ExecutionError::ExecutionTimeout));
    }

    #[test]
    fn test_unsupported_operation_error() {
        let error = ExecutionError::UnsupportedOperation("BigInt operations".to_string());
        assert!(matches!(error, ExecutionError::UnsupportedOperation(_)));
        if let ExecutionError::UnsupportedOperation(op) = error {
            assert_eq!(op, "BigInt operations");
        }
    }

    // Test constructor methods
    #[test]
    fn test_new_type_error() {
        let error = ExecutionError::new_type_error("Cannot add string and number");
        assert!(matches!(error, ExecutionError::TypeError(_)));
        if let ExecutionError::TypeError(msg) = error {
            assert_eq!(msg, "Cannot add string and number");
        }
    }

    #[test]
    fn test_new_type_error_with_string() {
        let error = ExecutionError::new_type_error("Cannot add string and number".to_string());
        assert!(matches!(error, ExecutionError::TypeError(_)));
        if let ExecutionError::TypeError(msg) = error {
            assert_eq!(msg, "Cannot add string and number");
        }
    }

    #[test]
    fn test_new_runtime_error() {
        let error = ExecutionError::new_runtime_error("Function execution failed");
        assert!(matches!(error, ExecutionError::RuntimeError(_)));
        if let ExecutionError::RuntimeError(msg) = error {
            assert_eq!(msg, "Function execution failed");
        }
    }

    #[test]
    fn test_new_memory_error() {
        let error = ExecutionError::new_memory_error("Heap allocation failed");
        assert!(matches!(error, ExecutionError::MemoryError(_)));
        if let ExecutionError::MemoryError(msg) = error {
            assert_eq!(msg, "Heap allocation failed");
        }
    }

    #[test]
    fn test_new_function_error() {
        let error = ExecutionError::new_function_error("Invalid function call");
        assert!(matches!(error, ExecutionError::FunctionError(_)));
        if let ExecutionError::FunctionError(msg) = error {
            assert_eq!(msg, "Invalid function call");
        }
    }

    #[test]
    fn test_new_variable_error() {
        let error = ExecutionError::new_variable_error("Variable not defined");
        assert!(matches!(error, ExecutionError::VariableError(_)));
        if let ExecutionError::VariableError(msg) = error {
            assert_eq!(msg, "Variable not defined");
        }
    }

    #[test]
    fn test_new_heap_error() {
        let error = ExecutionError::new_heap_error("Garbage collection failed");
        assert!(matches!(error, ExecutionError::HeapError(_)));
        if let ExecutionError::HeapError(msg) = error {
            assert_eq!(msg, "Garbage collection failed");
        }
    }

    #[test]
    fn test_new_control_flow_error() {
        let error = ExecutionError::new_control_flow_error("Invalid jump target");
        assert!(matches!(error, ExecutionError::ControlFlowError(_)));
        if let ExecutionError::ControlFlowError(msg) = error {
            assert_eq!(msg, "Invalid jump target");
        }
    }

    #[test]
    fn test_new_builtin_error() {
        let error = ExecutionError::new_builtin_error("Console not available");
        assert!(matches!(error, ExecutionError::BuiltinError(_)));
        if let ExecutionError::BuiltinError(msg) = error {
            assert_eq!(msg, "Console not available");
        }
    }

    #[test]
    fn test_new_unsupported_operation() {
        let error = ExecutionError::new_unsupported_operation("BigInt operations");
        assert!(matches!(error, ExecutionError::UnsupportedOperation(_)));
        if let ExecutionError::UnsupportedOperation(op) = error {
            assert_eq!(op, "BigInt operations");
        }
    }

    // Test Display implementation
    #[test]
    fn test_display_stack_underflow() {
        let error = ExecutionError::StackUnderflow;
        assert_eq!(format!("{}", error), "Stack underflow");
    }

    #[test]
    fn test_display_stack_overflow() {
        let error = ExecutionError::StackOverflow;
        assert_eq!(format!("{}", error), "Stack overflow");
    }

    #[test]
    fn test_display_type_error() {
        let error = ExecutionError::TypeError("Cannot add string and number".to_string());
        assert_eq!(
            format!("{}", error),
            "Type error: Cannot add string and number"
        );
    }

    #[test]
    fn test_display_runtime_error() {
        let error = ExecutionError::RuntimeError("Function execution failed".to_string());
        assert_eq!(
            format!("{}", error),
            "Runtime error: Function execution failed"
        );
    }

    #[test]
    fn test_display_memory_error() {
        let error = ExecutionError::MemoryError("Heap allocation failed".to_string());
        assert_eq!(format!("{}", error), "Memory error: Heap allocation failed");
    }

    #[test]
    fn test_display_invalid_instruction() {
        let error = ExecutionError::InvalidInstruction("Unknown opcode".to_string());
        assert_eq!(format!("{}", error), "Invalid instruction: Unknown opcode");
    }

    #[test]
    fn test_display_function_error() {
        let error = ExecutionError::FunctionError("Invalid function call".to_string());
        assert_eq!(
            format!("{}", error),
            "Function error: Invalid function call"
        );
    }

    #[test]
    fn test_display_variable_error() {
        let error = ExecutionError::VariableError("Variable not defined".to_string());
        assert_eq!(format!("{}", error), "Variable error: Variable not defined");
    }

    #[test]
    fn test_display_heap_error() {
        let error = ExecutionError::HeapError("Garbage collection failed".to_string());
        assert_eq!(
            format!("{}", error),
            "Heap error: Garbage collection failed"
        );
    }

    #[test]
    fn test_display_control_flow_error() {
        let error = ExecutionError::ControlFlowError("Invalid jump target".to_string());
        assert_eq!(
            format!("{}", error),
            "Control flow error: Invalid jump target"
        );
    }

    #[test]
    fn test_display_builtin_error() {
        let error = ExecutionError::BuiltinError("Console not available".to_string());
        assert_eq!(
            format!("{}", error),
            "Built-in error: Console not available"
        );
    }

    #[test]
    fn test_display_division_by_zero() {
        let error = ExecutionError::DivisionByZero;
        assert_eq!(format!("{}", error), "Division by zero");
    }

    #[test]
    fn test_display_invalid_index() {
        let error = ExecutionError::InvalidIndex(42);
        assert_eq!(format!("{}", error), "Invalid index: 42");
    }

    #[test]
    fn test_display_property_not_found() {
        let error = ExecutionError::PropertyNotFound("length".to_string());
        assert_eq!(format!("{}", error), "Property not found: length");
    }

    #[test]
    fn test_display_method_not_found() {
        let error = ExecutionError::MethodNotFound("toString".to_string());
        assert_eq!(format!("{}", error), "Method not found: toString");
    }

    #[test]
    fn test_display_invalid_argument_count() {
        let error = ExecutionError::InvalidArgumentCount {
            expected: 2,
            received: 3,
        };
        assert_eq!(
            format!("{}", error),
            "Invalid argument count: expected 2, received 3"
        );
    }

    #[test]
    fn test_display_recursion_limit_exceeded() {
        let error = ExecutionError::RecursionLimitExceeded(1000);
        assert_eq!(format!("{}", error), "Recursion limit exceeded: 1000");
    }

    #[test]
    fn test_display_execution_timeout() {
        let error = ExecutionError::ExecutionTimeout;
        assert_eq!(format!("{}", error), "Execution timeout");
    }

    #[test]
    fn test_display_unsupported_operation() {
        let error = ExecutionError::UnsupportedOperation("BigInt operations".to_string());
        assert_eq!(
            format!("{}", error),
            "Unsupported operation: BigInt operations"
        );
    }

    // Test Error trait implementation
    #[test]
    fn test_error_trait_implementation() {
        let error = ExecutionError::TypeError("Test error".to_string());
        assert!(error.source().is_none());
    }

    // Test is_recoverable method
    #[test]
    fn test_is_recoverable_stack_underflow() {
        let error = ExecutionError::StackUnderflow;
        assert!(error.is_recoverable());
    }

    #[test]
    fn test_is_recoverable_stack_overflow() {
        let error = ExecutionError::StackOverflow;
        assert!(error.is_recoverable());
    }

    #[test]
    fn test_is_recoverable_invalid_index() {
        let error = ExecutionError::InvalidIndex(42);
        assert!(error.is_recoverable());
    }

    #[test]
    fn test_is_recoverable_property_not_found() {
        let error = ExecutionError::PropertyNotFound("length".to_string());
        assert!(error.is_recoverable());
    }

    #[test]
    fn test_is_recoverable_method_not_found() {
        let error = ExecutionError::MethodNotFound("toString".to_string());
        assert!(error.is_recoverable());
    }

    #[test]
    fn test_is_not_recoverable_type_error() {
        let error = ExecutionError::TypeError("Invalid operation".to_string());
        assert!(!error.is_recoverable());
    }

    #[test]
    fn test_is_not_recoverable_memory_error() {
        let error = ExecutionError::MemoryError("Heap corruption".to_string());
        assert!(!error.is_recoverable());
    }

    #[test]
    fn test_is_not_recoverable_division_by_zero() {
        let error = ExecutionError::DivisionByZero;
        assert!(!error.is_recoverable());
    }

    #[test]
    fn test_is_not_recoverable_invalid_instruction() {
        let error = ExecutionError::InvalidInstruction("Unknown opcode".to_string());
        assert!(!error.is_recoverable());
    }

    // Test is_fatal method
    #[test]
    fn test_is_fatal_memory_error() {
        let error = ExecutionError::MemoryError("Heap corruption".to_string());
        assert!(error.is_fatal());
    }

    #[test]
    fn test_is_fatal_invalid_instruction() {
        let error = ExecutionError::InvalidInstruction("Unknown opcode".to_string());
        assert!(error.is_fatal());
    }

    #[test]
    fn test_is_fatal_recursion_limit_exceeded() {
        let error = ExecutionError::RecursionLimitExceeded(1000);
        assert!(error.is_fatal());
    }

    #[test]
    fn test_is_fatal_execution_timeout() {
        let error = ExecutionError::ExecutionTimeout;
        assert!(error.is_fatal());
    }

    #[test]
    fn test_is_not_fatal_stack_underflow() {
        let error = ExecutionError::StackUnderflow;
        assert!(!error.is_fatal());
    }

    #[test]
    fn test_is_not_fatal_type_error() {
        let error = ExecutionError::TypeError("Invalid operation".to_string());
        assert!(!error.is_fatal());
    }

    #[test]
    fn test_is_not_fatal_division_by_zero() {
        let error = ExecutionError::DivisionByZero;
        assert!(!error.is_fatal());
    }

    #[test]
    fn test_is_not_fatal_property_not_found() {
        let error = ExecutionError::PropertyNotFound("length".to_string());
        assert!(!error.is_fatal());
    }

    // Test get_user_message method
    #[test]
    fn test_get_user_message_stack_underflow() {
        let error = ExecutionError::StackUnderflow;
        assert_eq!(error.get_user_message(), "Stack is empty");
    }

    #[test]
    fn test_get_user_message_stack_overflow() {
        let error = ExecutionError::StackOverflow;
        assert_eq!(error.get_user_message(), "Stack is full");
    }

    #[test]
    fn test_get_user_message_type_error() {
        let error = ExecutionError::TypeError("Cannot add string and number".to_string());
        assert_eq!(
            error.get_user_message(),
            "Type error: Cannot add string and number"
        );
    }

    #[test]
    fn test_get_user_message_runtime_error() {
        let error = ExecutionError::RuntimeError("Function execution failed".to_string());
        assert_eq!(
            error.get_user_message(),
            "Runtime error: Function execution failed"
        );
    }

    #[test]
    fn test_get_user_message_memory_error() {
        let error = ExecutionError::MemoryError("Heap allocation failed".to_string());
        assert_eq!(
            error.get_user_message(),
            "Memory error: Heap allocation failed"
        );
    }

    #[test]
    fn test_get_user_message_invalid_instruction() {
        let error = ExecutionError::InvalidInstruction("Unknown opcode".to_string());
        assert_eq!(
            error.get_user_message(),
            "Invalid instruction: Unknown opcode"
        );
    }

    #[test]
    fn test_get_user_message_function_error() {
        let error = ExecutionError::FunctionError("Invalid function call".to_string());
        assert_eq!(
            error.get_user_message(),
            "Function error: Invalid function call"
        );
    }

    #[test]
    fn test_get_user_message_variable_error() {
        let error = ExecutionError::VariableError("Variable not defined".to_string());
        assert_eq!(
            error.get_user_message(),
            "Variable error: Variable not defined"
        );
    }

    #[test]
    fn test_get_user_message_heap_error() {
        let error = ExecutionError::HeapError("Garbage collection failed".to_string());
        assert_eq!(
            error.get_user_message(),
            "Heap error: Garbage collection failed"
        );
    }

    #[test]
    fn test_get_user_message_control_flow_error() {
        let error = ExecutionError::ControlFlowError("Invalid jump target".to_string());
        assert_eq!(
            error.get_user_message(),
            "Control flow error: Invalid jump target"
        );
    }

    #[test]
    fn test_get_user_message_builtin_error() {
        let error = ExecutionError::BuiltinError("Console not available".to_string());
        assert_eq!(
            error.get_user_message(),
            "Built-in error: Console not available"
        );
    }

    #[test]
    fn test_get_user_message_division_by_zero() {
        let error = ExecutionError::DivisionByZero;
        assert_eq!(error.get_user_message(), "Division by zero");
    }

    #[test]
    fn test_get_user_message_invalid_index() {
        let error = ExecutionError::InvalidIndex(42);
        assert_eq!(error.get_user_message(), "Invalid index: 42");
    }

    #[test]
    fn test_get_user_message_property_not_found() {
        let error = ExecutionError::PropertyNotFound("length".to_string());
        assert_eq!(error.get_user_message(), "Property not found: 'length'");
    }

    #[test]
    fn test_get_user_message_method_not_found() {
        let error = ExecutionError::MethodNotFound("toString".to_string());
        assert_eq!(error.get_user_message(), "Method not found: 'toString'");
    }

    #[test]
    fn test_get_user_message_invalid_argument_count() {
        let error = ExecutionError::InvalidArgumentCount {
            expected: 2,
            received: 3,
        };
        assert_eq!(
            error.get_user_message(),
            "Invalid argument count: expected 2, received 3"
        );
    }

    #[test]
    fn test_get_user_message_recursion_limit_exceeded() {
        let error = ExecutionError::RecursionLimitExceeded(1000);
        assert_eq!(error.get_user_message(), "Recursion limit exceeded: 1000");
    }

    #[test]
    fn test_get_user_message_execution_timeout() {
        let error = ExecutionError::ExecutionTimeout;
        assert_eq!(error.get_user_message(), "Execution timeout");
    }

    #[test]
    fn test_get_user_message_unsupported_operation() {
        let error = ExecutionError::UnsupportedOperation("BigInt operations".to_string());
        assert_eq!(
            error.get_user_message(),
            "Unsupported operation: BigInt operations"
        );
    }

    // Test get_debug_message method
    #[test]
    fn test_get_debug_message() {
        let error = ExecutionError::TypeError("Cannot add string and number".to_string());
        let debug_msg = error.get_debug_message();
        assert!(debug_msg.contains("TypeError"));
        assert!(debug_msg.contains("Cannot add string and number"));
    }

    #[test]
    fn test_get_debug_message_stack_underflow() {
        let error = ExecutionError::StackUnderflow;
        let debug_msg = error.get_debug_message();
        assert!(debug_msg.contains("StackUnderflow"));
    }

    // Test Clone trait
    #[test]
    fn test_clone_error() {
        let error = ExecutionError::TypeError("Test error".to_string());
        let cloned_error = error.clone();
        assert!(matches!(cloned_error, ExecutionError::TypeError(_)));
        if let ExecutionError::TypeError(msg) = cloned_error {
            assert_eq!(msg, "Test error");
        }
    }

    #[test]
    fn test_clone_complex_error() {
        let error = ExecutionError::InvalidArgumentCount {
            expected: 2,
            received: 3,
        };
        let cloned_error = error.clone();
        assert!(matches!(
            cloned_error,
            ExecutionError::InvalidArgumentCount { .. }
        ));
        if let ExecutionError::InvalidArgumentCount { expected, received } = cloned_error {
            assert_eq!(expected, 2);
            assert_eq!(received, 3);
        }
    }

    // Test Debug trait
    #[test]
    fn test_debug_formatting() {
        let error = ExecutionError::TypeError("Test error".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("TypeError"));
        assert!(debug_str.contains("Test error"));
    }

    #[test]
    fn test_debug_formatting_complex() {
        let error = ExecutionError::InvalidArgumentCount {
            expected: 2,
            received: 3,
        };
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("InvalidArgumentCount"));
        assert!(debug_str.contains("expected: 2"));
        assert!(debug_str.contains("received: 3"));
    }

    // Test error matching patterns
    #[test]
    fn test_error_matching_patterns() {
        let errors = vec![
            ExecutionError::StackUnderflow,
            ExecutionError::StackOverflow,
            ExecutionError::TypeError("test".to_string()),
            ExecutionError::RuntimeError("test".to_string()),
            ExecutionError::MemoryError("test".to_string()),
            ExecutionError::InvalidInstruction("test".to_string()),
            ExecutionError::FunctionError("test".to_string()),
            ExecutionError::VariableError("test".to_string()),
            ExecutionError::HeapError("test".to_string()),
            ExecutionError::ControlFlowError("test".to_string()),
            ExecutionError::BuiltinError("test".to_string()),
            ExecutionError::DivisionByZero,
            ExecutionError::InvalidIndex(42),
            ExecutionError::PropertyNotFound("test".to_string()),
            ExecutionError::MethodNotFound("test".to_string()),
            ExecutionError::InvalidArgumentCount {
                expected: 1,
                received: 2,
            },
            ExecutionError::RecursionLimitExceeded(100),
            ExecutionError::ExecutionTimeout,
            ExecutionError::UnsupportedOperation("test".to_string()),
        ];

        for error in errors {
            match error {
                ExecutionError::StackUnderflow => assert!(true),
                ExecutionError::StackOverflow => assert!(true),
                ExecutionError::TypeError(_) => assert!(true),
                ExecutionError::RuntimeError(_) => assert!(true),
                ExecutionError::MemoryError(_) => assert!(true),
                ExecutionError::InvalidInstruction(_) => assert!(true),
                ExecutionError::FunctionError(_) => assert!(true),
                ExecutionError::VariableError(_) => assert!(true),
                ExecutionError::HeapError(_) => assert!(true),
                ExecutionError::ControlFlowError(_) => assert!(true),
                ExecutionError::BuiltinError(_) => assert!(true),
                ExecutionError::DivisionByZero => assert!(true),
                ExecutionError::InvalidIndex(_) => assert!(true),
                ExecutionError::PropertyNotFound(_) => assert!(true),
                ExecutionError::MethodNotFound(_) => assert!(true),
                ExecutionError::InvalidArgumentCount { .. } => assert!(true),
                ExecutionError::RecursionLimitExceeded(_) => assert!(true),
                ExecutionError::ExecutionTimeout => assert!(true),
                ExecutionError::UnsupportedOperation(_) => assert!(true),
            }
        }
    }

    // Test error categorization
    #[test]
    fn test_error_categorization() {
        // Recoverable errors
        let recoverable_errors = vec![
            ExecutionError::StackUnderflow,
            ExecutionError::StackOverflow,
            ExecutionError::InvalidIndex(42),
            ExecutionError::PropertyNotFound("test".to_string()),
            ExecutionError::MethodNotFound("test".to_string()),
        ];

        for error in recoverable_errors {
            assert!(
                error.is_recoverable(),
                "Error should be recoverable: {:?}",
                error
            );
            assert!(!error.is_fatal(), "Error should not be fatal: {:?}", error);
        }

        // Fatal errors
        let fatal_errors = vec![
            ExecutionError::MemoryError("test".to_string()),
            ExecutionError::InvalidInstruction("test".to_string()),
            ExecutionError::RecursionLimitExceeded(100),
            ExecutionError::ExecutionTimeout,
        ];

        for error in fatal_errors {
            assert!(error.is_fatal(), "Error should be fatal: {:?}", error);
            assert!(
                !error.is_recoverable(),
                "Error should not be recoverable: {:?}",
                error
            );
        }

        // Neither recoverable nor fatal
        let neutral_errors = vec![
            ExecutionError::TypeError("test".to_string()),
            ExecutionError::RuntimeError("test".to_string()),
            ExecutionError::FunctionError("test".to_string()),
            ExecutionError::VariableError("test".to_string()),
            ExecutionError::HeapError("test".to_string()),
            ExecutionError::ControlFlowError("test".to_string()),
            ExecutionError::BuiltinError("test".to_string()),
            ExecutionError::DivisionByZero,
            ExecutionError::InvalidArgumentCount {
                expected: 1,
                received: 2,
            },
            ExecutionError::UnsupportedOperation("test".to_string()),
        ];

        for error in neutral_errors {
            assert!(
                !error.is_recoverable(),
                "Error should not be recoverable: {:?}",
                error
            );
            assert!(!error.is_fatal(), "Error should not be fatal: {:?}", error);
        }
    }
}
