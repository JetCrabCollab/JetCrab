use crate::ast::{BinaryExpression, Node};
use crate::parser::error::ParseResult;
use crate::parser::Parser;

impl Parser {
    pub fn parse_relational_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_shift_expression()?;

        while self.is_relational_operator() {
            let operator = self.current_token_string();
            self.advance();
            let right = Box::new(self.parse_shift_expression()?);

            let span = self.create_span_from_tokens();
            left = Node::BinaryExpression(BinaryExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }

        Ok(left)
    }

    pub fn parse_shift_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_additive_expression()?;

        while self.is_shift_operator() {
            let operator = self.current_token_string();
            self.advance();
            let right = Box::new(self.parse_additive_expression()?);

            let span = self.create_span_from_tokens();
            left = Node::BinaryExpression(BinaryExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }

        Ok(left)
    }
}
