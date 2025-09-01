use jetcrab::lexer::error::LexerError;

#[test]
fn test_lexer_error_creation() {
    let error = LexerError::UnterminatedString {
        position: (10, 5),
    };
    
    match error {
        LexerError::UnterminatedString { position } => {
            assert_eq!(position, (10, 5));
        }
        _ => panic!("Expected UnterminatedString error"),
    }
}

#[test]
fn test_lexer_error_variants() {
    let unterminated_string = LexerError::UnterminatedString {
        position: (1, 1),
    };
    
    let invalid_number = LexerError::InvalidNumber {
        value: "123.456.789".to_string(),
        position: (2, 1),
    };
    
    let unexpected_character = LexerError::UnexpectedCharacter {
        character: '@',
        position: (3, 1),
    };
    
    match unterminated_string {
        LexerError::UnterminatedString { position } => {
            assert_eq!(position, (1, 1));
        }
        _ => panic!("Expected UnterminatedString error"),
    }
    
    match invalid_number {
        LexerError::InvalidNumber { value, .. } => {
            assert_eq!(value, "123.456.789");
        }
        _ => panic!("Expected InvalidNumber error"),
    }
    
    match unexpected_character {
        LexerError::UnexpectedCharacter { character, .. } => {
            assert_eq!(character, '@');
        }
        _ => panic!("Expected UnexpectedCharacter error"),
    }
}

#[test]
fn test_lexer_error_display() {
    let error = LexerError::InvalidNumber {
        value: "invalid".to_string(),
        position: (5, 10),
    };
    
    let display_str = format!("{}", error);
    assert!(display_str.contains("InvalidNumber"));
    assert!(display_str.contains("invalid"));
    assert!(display_str.contains("5:10"));
}

#[test]
fn test_lexer_error_debug() {
    let error = LexerError::UnexpectedCharacter {
        character: '!',
        position: (8, 12),
    };
    
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("UnexpectedCharacter"));
    assert!(debug_str.contains("!"));
    assert!(debug_str.contains("8:12"));
}
