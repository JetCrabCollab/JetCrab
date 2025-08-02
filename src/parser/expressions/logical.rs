use crate::ast::{LogicalExpression, Node};
use crate::lexer::TokenKind;
use crate::parser::error::ParseResult;
use crate::parser::Parser;

impl Parser {
    pub fn parse_logical_or_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_logical_and_expression()?;

        while self.check(TokenKind::LogicalOr) || self.check(TokenKind::NullishCoalescing) {
            let operator = self.current_token_string();
            self.advance();
            let right = Box::new(self.parse_logical_and_expression()?);

            let span = self.create_span_from_tokens();
            left = Node::LogicalExpression(LogicalExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }

        Ok(left)
    }

    pub fn parse_logical_and_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_equality_expression()?;

        while self.check(TokenKind::LogicalAnd) {
            let operator = self.current_token_string();
            self.advance();
            let right = Box::new(self.parse_equality_expression()?);

            let span = self.create_span_from_tokens();
            left = Node::LogicalExpression(LogicalExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }

        Ok(left)
    }
}
