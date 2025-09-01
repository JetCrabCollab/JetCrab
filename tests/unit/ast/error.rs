use jetcrab::ast::error::AstError;

#[test]
fn test_ast_error_creation() {
    let error = AstError::InvalidNode {
        message: "Invalid node type".to_string(),
        position: (10, 5),
    };
    
    match error {
        AstError::InvalidNode { message, position } => {
            assert_eq!(message, "Invalid node type");
            assert_eq!(position, (10, 5));
        }
        _ => panic!("Expected InvalidNode error"),
    }
}

#[test]
fn test_ast_error_variants() {
    let invalid_node = AstError::InvalidNode {
        message: "Invalid node".to_string(),
        position: (1, 1),
    };
    
    let missing_child = AstError::MissingChild {
        expected: "expression".to_string(),
        position: (2, 1),
    };
    
    let invalid_structure = AstError::InvalidStructure {
        message: "Invalid structure".to_string(),
        position: (3, 1),
    };
    
    match invalid_node {
        AstError::InvalidNode { message, .. } => {
            assert_eq!(message, "Invalid node");
        }
        _ => panic!("Expected InvalidNode error"),
    }
    
    match missing_child {
        AstError::MissingChild { expected, .. } => {
            assert_eq!(expected, "expression");
        }
        _ => panic!("Expected MissingChild error"),
    }
    
    match invalid_structure {
        AstError::InvalidStructure { message, .. } => {
            assert_eq!(message, "Invalid structure");
        }
        _ => panic!("Expected InvalidStructure error"),
    }
}

#[test]
fn test_ast_error_display() {
    let error = AstError::MissingChild {
        expected: "expression".to_string(),
        position: (5, 10),
    };
    
    let display_str = format!("{}", error);
    assert!(display_str.contains("MissingChild"));
    assert!(display_str.contains("expression"));
    assert!(display_str.contains("5:10"));
}

#[test]
fn test_ast_error_debug() {
    let error = AstError::InvalidStructure {
        message: "Invalid structure".to_string(),
        position: (8, 12),
    };
    
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("InvalidStructure"));
    assert!(debug_str.contains("Invalid structure"));
    assert!(debug_str.contains("8:12"));
}
