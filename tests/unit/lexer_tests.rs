use jetcrab::lexer::*;
use jetcrab::vm::types::*;

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

    if let Some(first_token) = tokens.first() {
        assert!(matches!(first_token.kind, TokenKind::Number(42.0)));
    }
}

#[test]
fn test_lexer_tokenize_string() {
    let mut lexer = Lexer::new("\"test\"");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());

    if let Some(first_token) = tokens.first() {
        assert!(matches!(first_token.kind, TokenKind::String(_)));
    }
}

#[test]
fn test_lexer_tokenize_identifier() {
    let mut lexer = Lexer::new("x");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());

    if let Some(first_token) = tokens.first() {
        assert!(matches!(first_token.kind, TokenKind::Identifier(_)));
    }
}

#[test]
fn test_lexer_tokenize_keyword() {
    let mut lexer = Lexer::new("let");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());

    if let Some(first_token) = tokens.first() {
        assert!(matches!(first_token.kind, TokenKind::Keyword(_)));
    }
}

#[test]
fn test_lexer_tokenize_operator() {
    let mut lexer = Lexer::new("+");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());

    if let Some(first_token) = tokens.first() {
        assert!(matches!(first_token.kind, TokenKind::Plus));
    }
}

#[test]
fn test_lexer_tokenize_punctuation() {
    let mut lexer = Lexer::new(";");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());

    if let Some(first_token) = tokens.first() {
        assert!(matches!(first_token.kind, TokenKind::Semicolon));
    }
}

#[test]
fn test_lexer_tokenize_multiple_tokens() {
    let mut lexer = Lexer::new("let x = 42;");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(tokens.len() >= 5);
}

#[test]
fn test_lexer_tokenize_with_positions() {
    let mut lexer = Lexer::new("let x");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert_eq!(tokens.len(), 3);

    if tokens.len() >= 2 {
        let let_token = &tokens[0];
        let x_token = &tokens[1];
        assert_eq!(let_token.span.start.line, LineNumber::new(1));
        assert_eq!(let_token.span.start.column, ColumnNumber::new(1));
        assert_eq!(let_token.span.end.line, LineNumber::new(1));
        assert_eq!(let_token.span.end.column, ColumnNumber::new(4));
        assert_eq!(x_token.span.start.line, LineNumber::new(1));
        assert_eq!(x_token.span.start.column, ColumnNumber::new(6));
        assert_eq!(x_token.span.end.line, LineNumber::new(1));
        assert_eq!(x_token.span.end.column, ColumnNumber::new(7));
    }
}

#[test]
fn test_lexer_tokenize_whitespace() {
    let mut lexer = Lexer::new(" 42 ");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());

    if let Some(first_token) = tokens.first() {
        assert!(matches!(first_token.kind, TokenKind::Number(42.0)));
    }
}

#[test]
fn test_lexer_tokenize_comments() {
    let mut lexer = Lexer::new("// comment\n42");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());

    if let Some(first_token) = tokens.first() {
        assert!(matches!(first_token.kind, TokenKind::Number(42.0)));
    }
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

#[test]
fn test_lexer_tokenize_regex() {
    let mut lexer = Lexer::new("/test/g");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());
}

#[test]
fn test_lexer_tokenize_arrow_function() {
    let mut lexer = Lexer::new("(x) => x * 2");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(tokens.len() >= 7);
}

#[test]
fn test_lexer_tokenize_class() {
    let mut lexer = Lexer::new("class Test {}");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(tokens.len() >= 4);
}

#[test]
fn test_lexer_tokenize_import() {
    let mut lexer = Lexer::new("import {x} from 'module'");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(tokens.len() >= 6);
}

#[test]
fn test_lexer_tokenize_export() {
    let mut lexer = Lexer::new("export default x");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(tokens.len() >= 3);
}

#[test]
fn test_lexer_tokenize_async_await() {
    let mut lexer = Lexer::new("async function test() { await x; }");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(tokens.len() >= 10);
}

#[test]
fn test_lexer_tokenize_destructuring() {
    let mut lexer = Lexer::new("let {x, y} = obj");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(tokens.len() >= 8);
}

#[test]
fn test_lexer_tokenize_spread() {
    let mut lexer = Lexer::new("...array");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(tokens.len() >= 2);
}

#[test]
fn test_lexer_tokenize_optional_chaining() {
    let mut lexer = Lexer::new("obj?.prop");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(tokens.len() >= 3);
}

#[test]
fn test_lexer_tokenize_nullish_coalescing() {
    let mut lexer = Lexer::new("x ?? y");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(tokens.len() >= 3);
}

#[test]
fn test_lexer_tokenize_bigint() {
    let mut lexer = Lexer::new("42n");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());

    if let Some(first_token) = tokens.first() {
        assert!(matches!(first_token.kind, TokenKind::BigInt(_)));
    }
}

#[test]
fn test_lexer_tokenize_private_field() {
    let mut lexer = Lexer::new("#private");
    let tokens = lexer.tokenize().expect("Lexer should succeed");
    assert!(!tokens.is_empty());

    if let Some(first_token) = tokens.first() {
        assert!(matches!(first_token.kind, TokenKind::PrivateField));
    }
}
