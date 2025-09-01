use jetcrab::lexer::{Lexer, TokenKind, LexerError};

#[test]
fn test_tokenize_numbers() {
    let mut lexer = Lexer::new("123");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    assert!(matches!(tokens[0].kind, TokenKind::Number(123.0)));
    assert!(matches!(tokens[1].kind, TokenKind::Eof));
}

#[test]
fn test_tokenize_strings() {
    let mut lexer = Lexer::new("\"hello\"");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    assert!(matches!(tokens[0].kind, TokenKind::String(ref s) if s == "hello"));
    assert!(matches!(tokens[1].kind, TokenKind::Eof));
}

#[test]
fn test_tokenize_identifiers() {
    let mut lexer = Lexer::new("x");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    assert!(matches!(tokens[0].kind, TokenKind::Identifier(ref s) if s == "x"));
    assert!(matches!(tokens[1].kind, TokenKind::Eof));
}

#[test]
fn test_tokenize_keywords() {
    let mut lexer = Lexer::new("true false null undefined");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 5);
    assert!(matches!(tokens[0].kind, TokenKind::Boolean(true)));
    assert!(matches!(tokens[1].kind, TokenKind::Boolean(false)));
    assert!(matches!(tokens[2].kind, TokenKind::Null));
    assert!(matches!(tokens[3].kind, TokenKind::Undefined));
    assert!(matches!(tokens[4].kind, TokenKind::Eof));
}

#[test]
fn test_tokenize_operators() {
    let mut lexer = Lexer::new("+-*/%");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 6);
    assert!(matches!(tokens[0].kind, TokenKind::Plus));
    assert!(matches!(tokens[1].kind, TokenKind::Minus));
    assert!(matches!(tokens[2].kind, TokenKind::Star));
    assert!(matches!(tokens[3].kind, TokenKind::Slash));
    assert!(matches!(tokens[4].kind, TokenKind::Percent));
    assert!(matches!(tokens[5].kind, TokenKind::Eof));
}

#[test]
fn test_tokenize_whitespace() {
    let mut lexer = Lexer::new("  \n  \t  123  ");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 2);
    assert!(matches!(tokens[0].kind, TokenKind::Number(123.0)));
    assert!(matches!(tokens[1].kind, TokenKind::Eof));
}

#[test]
fn test_tokenize_only_whitespace() {
    let mut lexer = Lexer::new("  \n  \t  ");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0].kind, TokenKind::Eof));
}

#[test]
fn test_tokenize_multiple_tokens() {
    let mut lexer = Lexer::new("let x = 42");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens.len(), 5);
    assert!(matches!(tokens[0].kind, TokenKind::Keyword(ref s) if s == "let"));
    assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "x"));
    assert!(matches!(tokens[2].kind, TokenKind::Assign));
    assert!(matches!(tokens[3].kind, TokenKind::Number(42.0)));
    assert!(matches!(tokens[4].kind, TokenKind::Eof));
}

#[test]
fn test_unterminated_string() {
    let mut lexer = Lexer::new("\"hello");
    let result = lexer.tokenize();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        LexerError::UnterminatedString
    ));
}

#[test]
fn test_invalid_number() {
    let mut lexer = Lexer::new("123.456.789");
    let result = lexer.tokenize();
    assert!(result.is_err());
    assert!(
        matches!(result.unwrap_err(), LexerError::InvalidNumber(ref s) if s == "123.456.789")
    );
}
