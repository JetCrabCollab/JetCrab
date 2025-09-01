//! VM Error Unit Tests
//!
//! Tests for VM error handling

use jetcrab::vm::error::{VmError, ErrorKind, ErrorPosition};

#[test]
fn test_error_creation() {
    let error = VmError::new(
        ErrorKind::TypeError,
        "Invalid type for operation".to_string(),
        Some(ErrorPosition::new(10, 5)),
    );
    
    assert!(matches!(error.kind, ErrorKind::TypeError));
    assert_eq!(error.message, "Invalid type for operation");
    assert!(error.position.is_some());
}

#[test]
fn test_error_without_position() {
    let error = VmError::new(
        ErrorKind::RuntimeError,
        "Runtime error occurred".to_string(),
        None,
    );
    
    assert!(matches!(error.kind, ErrorKind::RuntimeError));
    assert_eq!(error.message, "Runtime error occurred");
    assert!(error.position.is_none());
}

#[test]
fn test_error_kinds() {
    let type_error = VmError::new(ErrorKind::TypeError, "Type error".to_string(), None);
    let runtime_error = VmError::new(ErrorKind::RuntimeError, "Runtime error".to_string(), None);
    let syntax_error = VmError::new(ErrorKind::SyntaxError, "Syntax error".to_string(), None);
    let memory_error = VmError::new(ErrorKind::MemoryError, "Memory error".to_string(), None);
    
    assert!(matches!(type_error.kind, ErrorKind::TypeError));
    assert!(matches!(runtime_error.kind, ErrorKind::RuntimeError));
    assert!(matches!(syntax_error.kind, ErrorKind::SyntaxError));
    assert!(matches!(memory_error.kind, ErrorKind::MemoryError));
}

#[test]
fn test_error_position() {
    let position = ErrorPosition::new(15, 8);
    
    assert_eq!(position.line, 15);
    assert_eq!(position.column, 8);
}

#[test]
fn test_error_display() {
    let error = VmError::new(
        ErrorKind::TypeError,
        "Test error".to_string(),
        Some(ErrorPosition::new(5, 10)),
    );
    
    let display_str = format!("{}", error);
    assert!(display_str.contains("TypeError"));
    assert!(display_str.contains("Test error"));
    assert!(display_str.contains("5:10"));
}

#[test]
fn test_error_debug() {
    let error = VmError::new(
        ErrorKind::RuntimeError,
        "Debug error".to_string(),
        None,
    );
    
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("RuntimeError"));
    assert!(debug_str.contains("Debug error"));
}
