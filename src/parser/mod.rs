//! # JavaScript Parser Module
//!
//! Provides JavaScript source code parsing capabilities, converting text input
//! into an Abstract Syntax Tree (AST) representation for further processing.
//!
//! ## Overview
//!
//! The parser module implements a recursive descent parser that handles:
//!
//! - **Expressions**: Arithmetic, logical, assignment, and function calls
//! - **Statements**: Control flow, declarations, and blocks
//! - **Literals**: Objects, arrays, functions, and primitive values
//! - **Recovery**: Error recovery and partial parsing
//!
//! ## Features
//!
//! - **ECMAScript 2020+ Support**: Modern JavaScript syntax
//! - **Error Recovery**: Continues parsing after syntax errors
//! - **Position Tracking**: Accurate source location information
//! - **Modular Design**: Separate parsers for different constructs
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::parser::{parse, parse_with_recovery};
//!
//! // Parse with error handling
//! let ast = parse("let x = 42;").unwrap();
//!
//! // Parse with recovery for invalid code
//! let (ast, errors) = parse_with_recovery("let x = ;");
//! ```

pub mod core;
pub mod error;
pub mod expressions;
pub mod literals;
pub mod recovery;
pub mod statements;
pub mod utils;

pub use core::Parser;
pub use error::ParserError;

pub fn parse(source: &str) -> Result<crate::ast::Node, ParserError> {
    let mut parser = Parser::new(source);
    parser.parse()
}

pub fn parse_with_recovery(source: &str) -> (Option<crate::ast::Node>, Vec<ParserError>) {
    let mut parser = Parser::new(source);
    parser.parse_with_recovery()
}
