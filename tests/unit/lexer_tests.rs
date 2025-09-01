use jetcrab::lexer::core::Lexer;

#[test]
fn test_lexer_new() {
    let _lexer = Lexer::new("42");
}

#[test]
fn test_lexer_tokenize() {
    let mut lexer = Lexer::new("42");
    let tokens = lexer.tokenize();
    assert!(tokens.is_ok());
}

#[test]
fn test_lexer_tokenize_number() {
    let mut lexer = Lexer::new("42");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());
}

#[test]
fn test_lexer_tokenize_string() {
    let mut lexer = Lexer::new("\"test\"");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());
}

#[test]
fn test_lexer_tokenize_identifier() {
    let mut lexer = Lexer::new("x");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());
}

#[test]
fn test_lexer_tokenize_keyword() {
    let mut lexer = Lexer::new("let");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());
}

#[test]
fn test_lexer_tokenize_operator() {
    let mut lexer = Lexer::new("+");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());
}

#[test]
fn test_lexer_tokenize_punctuation() {
    let mut lexer = Lexer::new(";");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());
}

#[test]
fn test_lexer_tokenize_multiple_tokens() {
    let mut lexer = Lexer::new("let x = 42;");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(tokens.len() >= 5);
}

#[test]
fn test_lexer_tokenize_whitespace() {
    let mut lexer = Lexer::new(" 42 ");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());
}

#[test]
fn test_lexer_tokenize_comments() {
    let mut lexer = Lexer::new("// comment\n42");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());
}

#[test]
fn test_lexer_tokenize_multiline() {
    let mut lexer = Lexer::new("let\nx = 42");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(tokens.len() >= 4);
}

#[test]
fn test_lexer_tokenize_complex_expression() {
    let mut lexer = Lexer::new("x + y * 2");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(tokens.len() >= 5);
}

#[test]
fn test_lexer_tokenize_function_call() {
    let mut lexer = Lexer::new("func(x, y)");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(tokens.len() >= 6);
}

#[test]
fn test_lexer_tokenize_object_literal() {
    let mut lexer = Lexer::new("{key: value}");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(tokens.len() >= 5);
}

#[test]
fn test_lexer_tokenize_array_literal() {
    let mut lexer = Lexer::new("[1, 2, 3]");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(tokens.len() >= 7);
}

#[test]
fn test_lexer_tokenize_template_literal() {
    let mut lexer = Lexer::new("`test ${x}`");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());
}
