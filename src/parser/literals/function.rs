use crate::ast::{FunctionExpression, Node};
use crate::lexer::TokenKind;
use crate::parser::error::ParseResult;
use crate::parser::Parser;

impl Parser {
    pub fn parse_function_expression(&mut self) -> ParseResult<Node> {
        self.advance();

        let id = if self.check_identifier() {
            Some(Box::new(self.parse_identifier()?))
        } else {
            None
        };

        self.expect(TokenKind::LeftParen)?;
        let params = self.parse_parameters()?;
        self.expect(TokenKind::RightParen)?;

        let body = Box::new(self.parse_function_body()?);

        let span = self.create_span_from_tokens();
        Ok(Node::FunctionExpression(FunctionExpression {
            id,
            params,
            body,
            generator: false,
            r#async: false,
            span: Some(span),
        }))
    }
}
