pub mod core;
pub mod error;
pub mod scanners;
pub mod token;
pub mod tokens;
pub mod utils;

pub use core::Lexer;
pub use error::LexerError;
pub use token::{Token, TokenKind};
pub use tokens::{Keyword, Literal, Operator, Punctuation};

pub fn tokenize(source: &str) -> Result<Vec<Token>, LexerError> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}

pub fn tokenize_fallback(source: &str) -> Vec<Token> {
    match tokenize(source) {
        Ok(tokens) => tokens,
        Err(_) => vec![Token::with_positions(TokenKind::Eof, 1, 1, 1, 1)],
    }
}
