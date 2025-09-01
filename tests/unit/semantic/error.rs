use jetcrab::semantic::error::SemanticError;

#[test]
fn test_semantic_error_creation() {
    let error = SemanticError::TypeMismatch {
        expected: "number".to_string(),
        found: "string".to_string(),
        position: (10, 5),
    };
    
    match error {
        SemanticError::TypeMismatch { expected, found, position } => {
            assert_eq!(expected, "number");
            assert_eq!(found, "string");
            assert_eq!(position, (10, 5));
        }
        _ => panic!("Expected TypeMismatch error"),
    }
}

#[test]
fn test_semantic_error_variants() {
    let type_error = SemanticError::TypeMismatch {
        expected: "number".to_string(),
        found: "string".to_string(),
        position: (1, 1),
    };
    
    let undefined_error = SemanticError::UndefinedVariable {
        name: "x".to_string(),
        position: (2, 1),
    };
    
    let duplicate_error = SemanticError::DuplicateVariable {
        name: "y".to_string(),
        position: (3, 1),
    };
    
    match type_error {
        SemanticError::TypeMismatch { expected, found, .. } => {
            assert_eq!(expected, "number");
            assert_eq!(found, "string");
        }
        _ => panic!("Expected TypeMismatch error"),
    }
    
    match undefined_error {
        SemanticError::UndefinedVariable { name, .. } => {
            assert_eq!(name, "x");
        }
        _ => panic!("Expected UndefinedVariable error"),
    }
    
    match duplicate_error {
        SemanticError::DuplicateVariable { name, .. } => {
            assert_eq!(name, "y");
        }
        _ => panic!("Expected DuplicateVariable error"),
    }
}

#[test]
fn test_semantic_error_display() {
    let error = SemanticError::TypeMismatch {
        expected: "number".to_string(),
        found: "string".to_string(),
        position: (5, 10),
    };
    
    let display_str = format!("{}", error);
    assert!(display_str.contains("TypeMismatch"));
    assert!(display_str.contains("number"));
    assert!(display_str.contains("string"));
    assert!(display_str.contains("5:10"));
}

#[test]
fn test_semantic_error_debug() {
    let error = SemanticError::UndefinedVariable {
        name: "test_var".to_string(),
        position: (8, 12),
    };
    
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("UndefinedVariable"));
    assert!(debug_str.contains("test_var"));
    assert!(debug_str.contains("8:12"));
}
