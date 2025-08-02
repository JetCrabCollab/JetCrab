use crate::ast::{NewExpression, Node};
use crate::lexer::TokenKind;
use crate::parser::error::ParseResult;
use crate::parser::Parser;

impl Parser {
    pub fn parse_new_expression(&mut self) -> ParseResult<Node> {
        self.advance();

        let callee = Box::new(self.parse_primary_expression()?);

        let arguments = if self.check(TokenKind::LeftParen) {
            self.advance();
            let args = self.parse_arguments()?;
            self.expect(TokenKind::RightParen)?;
            args
        } else {
            Vec::new()
        };

        let span = self.create_span_from_tokens();
        Ok(Node::NewExpression(NewExpression {
            callee,
            arguments,
            span: Some(span),
        }))
    }
}
