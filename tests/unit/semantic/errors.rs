use jetcrab::semantic::errors::{SemanticError, ErrorType, ErrorLocation};

#[test]
fn test_error_creation() {
    let error = SemanticError::new(
        ErrorType::TypeMismatch,
        "Cannot assign string to number".to_string(),
        Some(ErrorLocation::new(10, 5)),
    );
    
    assert!(matches!(error.error_type, ErrorType::TypeMismatch));
    assert_eq!(error.message, "Cannot assign string to number");
    assert!(error.location.is_some());
}

#[test]
fn test_error_without_location() {
    let error = SemanticError::new(
        ErrorType::UndefinedVariable,
        "Variable 'x' is not defined".to_string(),
        None,
    );
    
    assert!(matches!(error.error_type, ErrorType::UndefinedVariable));
    assert_eq!(error.message, "Variable 'x' is not defined");
    assert!(error.location.is_none());
}

#[test]
fn test_error_types() {
    let type_mismatch = SemanticError::new(ErrorType::TypeMismatch, "Type error".to_string(), None);
    let undefined_var = SemanticError::new(ErrorType::UndefinedVariable, "Undefined var".to_string(), None);
    let duplicate_var = SemanticError::new(ErrorType::DuplicateVariable, "Duplicate var".to_string(), None);
    let invalid_operation = SemanticError::new(ErrorType::InvalidOperation, "Invalid op".to_string(), None);
    
    assert!(matches!(type_mismatch.error_type, ErrorType::TypeMismatch));
    assert!(matches!(undefined_var.error_type, ErrorType::UndefinedVariable));
    assert!(matches!(duplicate_var.error_type, ErrorType::DuplicateVariable));
    assert!(matches!(invalid_operation.error_type, ErrorType::InvalidOperation));
}

#[test]
fn test_error_location() {
    let location = ErrorLocation::new(15, 8);
    
    assert_eq!(location.line, 15);
    assert_eq!(location.column, 8);
}

#[test]
fn test_error_clone() {
    let original = SemanticError::new(
        ErrorType::TypeMismatch,
        "Original error".to_string(),
        Some(ErrorLocation::new(5, 10)),
    );
    
    let cloned = original.clone();
    
    assert_eq!(original.error_type, cloned.error_type);
    assert_eq!(original.message, cloned.message);
    assert_eq!(original.location, cloned.location);
}

#[test]
fn test_error_display() {
    let error = SemanticError::new(
        ErrorType::UndefinedVariable,
        "Variable not found".to_string(),
        Some(ErrorLocation::new(8, 12)),
    );
    
    let display_str = format!("{}", error);
    assert!(display_str.contains("UndefinedVariable"));
    assert!(display_str.contains("Variable not found"));
    assert!(display_str.contains("8:12"));
}

#[test]
fn test_error_debug() {
    let error = SemanticError::new(
        ErrorType::InvalidOperation,
        "Debug error".to_string(),
        None,
    );
    
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("InvalidOperation"));
    assert!(debug_str.contains("Debug error"));
}
