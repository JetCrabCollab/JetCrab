//! Unit tests for lexer tokens

use jetcrab::lexer::tokens::keywords::Keyword;
use jetcrab::lexer::tokens::literals::Literal;
use jetcrab::lexer::tokens::operators::Operator;
use jetcrab::lexer::tokens::punctuation::Punctuation;

#[test]
fn test_keyword_enum() {
    let keyword = Keyword::Function;
    assert_eq!(keyword.as_str(), "function");
}

#[test]
fn test_operator_enum() {
    let operator = Operator::Plus;
    assert_eq!(operator.as_str(), "+");
    assert!(operator.is_arithmetic());
    assert_eq!(operator.precedence(), 9);
}

#[test]
fn test_punctuation_enum() {
    let punctuation = Punctuation::LeftParen;
    assert_eq!(punctuation.as_str(), "(");
    assert!(punctuation.is_opening());
}

#[test]
fn test_literal_enum() {
    let literal = Literal::Number(42.0);
    assert_eq!(literal.as_number(), Some(42.0));
    assert!(!literal.is_falsy());
    assert!(literal.is_truthy());
}
