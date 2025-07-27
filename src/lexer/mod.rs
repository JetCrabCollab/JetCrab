pub mod error;
pub mod lexer;
pub mod token;

pub use error::LexerError;
pub use lexer::Lexer;
pub use token::{Position, Span, Token, TokenKind};

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
