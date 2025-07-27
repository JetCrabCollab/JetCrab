pub mod error;
pub mod parser;
pub mod recovery;

pub use error::ParserError;
pub use parser::Parser;

pub fn parse(source: &str) -> Result<crate::ast::node::Node, ParserError> {
    let mut parser = Parser::new(source);
    parser.parse()
}

pub fn parse_with_recovery(source: &str) -> (Option<crate::ast::node::Node>, Vec<ParserError>) {
    let mut parser = Parser::new(source);
    parser.parse_with_recovery()
}
