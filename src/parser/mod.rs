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
