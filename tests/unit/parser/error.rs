use jetcrab::parser::error::ParserError;

#[test]
fn test_parser_error_creation() {
    let error = ParserError::UnexpectedToken {
        expected: "number".to_string(),
        found: "string".to_string(),
        position: (10, 5),
    };
    
    match error {
        ParserError::UnexpectedToken { expected, found, position } => {
            assert_eq!(expected, "number");
            assert_eq!(found, "string");
            assert_eq!(position, (10, 5));
        }
        _ => panic!("Expected UnexpectedToken error"),
    }
}

#[test]
fn test_parser_error_variants() {
    let unexpected_token = ParserError::UnexpectedToken {
        expected: "number".to_string(),
        found: "string".to_string(),
        position: (1, 1),
    };
    
    let unexpected_eof = ParserError::UnexpectedEof {
        expected: "expression".to_string(),
        position: (2, 1),
    };
    
    let invalid_syntax = ParserError::InvalidSyntax {
        message: "Invalid syntax".to_string(),
        position: (3, 1),
    };
    
    match unexpected_token {
        ParserError::UnexpectedToken { expected, found, .. } => {
            assert_eq!(expected, "number");
            assert_eq!(found, "string");
        }
        _ => panic!("Expected UnexpectedToken error"),
    }
    
    match unexpected_eof {
        ParserError::UnexpectedEof { expected, .. } => {
            assert_eq!(expected, "expression");
        }
        _ => panic!("Expected UnexpectedEof error"),
    }
    
    match invalid_syntax {
        ParserError::InvalidSyntax { message, .. } => {
            assert_eq!(message, "Invalid syntax");
        }
        _ => panic!("Expected InvalidSyntax error"),
    }
}

#[test]
fn test_parser_error_display() {
    let error = ParserError::UnexpectedToken {
        expected: "number".to_string(),
        found: "string".to_string(),
        position: (5, 10),
    };
    
    let display_str = format!("{}", error);
    assert!(display_str.contains("UnexpectedToken"));
    assert!(display_str.contains("number"));
    assert!(display_str.contains("string"));
    assert!(display_str.contains("5:10"));
}

#[test]
fn test_parser_error_debug() {
    let error = ParserError::UnexpectedEof {
        expected: "expression".to_string(),
        position: (8, 12),
    };
    
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("UnexpectedEof"));
    assert!(debug_str.contains("expression"));
    assert!(debug_str.contains("8:12"));
}
