use jetcrab::lexer::token::{Token, TokenKind, TokenPosition};

#[test]
fn test_token_creation() {
    let token = Token::new(
        TokenKind::Number(42.0),
        "42".to_string(),
        TokenPosition::new(10, 5),
    );
    
    assert!(matches!(token.kind, TokenKind::Number(42.0)));
    assert_eq!(token.lexeme, "42");
    assert_eq!(token.position.line, 10);
    assert_eq!(token.position.column, 5);
}

#[test]
fn test_token_kinds() {
    let number_token = Token::new(TokenKind::Number(3.14), "3.14".to_string(), TokenPosition::new(1, 1));
    let string_token = Token::new(TokenKind::String("hello".to_string()), "\"hello\"".to_string(), TokenPosition::new(1, 1));
    let identifier_token = Token::new(TokenKind::Identifier("x".to_string()), "x".to_string(), TokenPosition::new(1, 1));
    let keyword_token = Token::new(TokenKind::Keyword("let".to_string()), "let".to_string(), TokenPosition::new(1, 1));
    
    assert!(matches!(number_token.kind, TokenKind::Number(3.14)));
    assert!(matches!(string_token.kind, TokenKind::String(ref s) if s == "hello"));
    assert!(matches!(identifier_token.kind, TokenKind::Identifier(ref s) if s == "x"));
    assert!(matches!(keyword_token.kind, TokenKind::Keyword(ref s) if s == "let"));
}

#[test]
fn test_token_position() {
    let position = TokenPosition::new(15, 20);
    
    assert_eq!(position.line, 15);
    assert_eq!(position.column, 20);
}

#[test]
fn test_token_clone() {
    let original = Token::new(
        TokenKind::Boolean(true),
        "true".to_string(),
        TokenPosition::new(5, 10),
    );
    
    let cloned = original.clone();
    
    assert_eq!(original.kind, cloned.kind);
    assert_eq!(original.lexeme, cloned.lexeme);
    assert_eq!(original.position.line, cloned.position.line);
    assert_eq!(original.position.column, cloned.position.column);
}

#[test]
fn test_token_debug() {
    let token = Token::new(
        TokenKind::Plus,
        "+".to_string(),
        TokenPosition::new(8, 12),
    );
    
    let debug_str = format!("{:?}", token);
    assert!(debug_str.contains("Plus"));
    assert!(debug_str.contains("+"));
    assert!(debug_str.contains("8:12"));
}

#[test]
fn test_token_equality() {
    let token1 = Token::new(TokenKind::Number(42.0), "42".to_string(), TokenPosition::new(1, 1));
    let token2 = Token::new(TokenKind::Number(42.0), "42".to_string(), TokenPosition::new(1, 1));
    let token3 = Token::new(TokenKind::Number(42.0), "42".to_string(), TokenPosition::new(2, 1));
    
    assert_eq!(token1, token2);
    assert_ne!(token1, token3);
}
