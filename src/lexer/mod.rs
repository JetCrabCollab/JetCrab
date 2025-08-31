//! # JavaScript Lexer Module
//!
//! Provides lexical analysis (tokenization) of JavaScript source code,
//! converting raw text into a stream of tokens for the parser.
//!
//! ## Overview
//!
//! The lexer module implements a scanner that produces:
//!
//! - **Tokens**: Keywords, identifiers, literals, and operators
//! - **Position Information**: Accurate source location tracking
//! - **Error Handling**: Graceful handling of invalid input
//! - **Fallback Mode**: Basic tokenization for error recovery
//!
//! ## Token Types
//!
//! - **Keywords**: Language keywords like `let`, `function`, `if`
//! - **Identifiers**: Variable and function names
//! - **Literals**: Numbers, strings, booleans, and null
//! - **Operators**: Arithmetic, logical, and assignment operators
//! - **Punctuation**: Brackets, semicolons, and other symbols
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::lexer::{tokenize, tokenize_fallback};
//!
//! // Tokenize with error handling
//! let tokens = tokenize("let x = 42;")?;
//!
//! // Fallback tokenization for invalid code
//! let tokens = tokenize_fallback("let x = ;");
//! ```

pub mod core;
pub mod error;
pub mod scanners;
pub mod token;
pub mod tokens;
pub mod utils;

pub use core::{Lexer, LineNumber, ColumnNumber};
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
