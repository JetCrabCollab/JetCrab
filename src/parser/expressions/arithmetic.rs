use crate::ast::{BinaryExpression, Node};
use crate::parser::error::ParseResult;
use crate::parser::Parser;

impl Parser {
    pub fn parse_additive_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_multiplicative_expression()?;

        while self.is_additive_operator() {
            let operator = self.current_token_string();
            self.advance();
            let right = Box::new(self.parse_multiplicative_expression()?);

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

    pub fn parse_multiplicative_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_exponentiation_expression()?;

        while self.is_multiplicative_operator() {
            let operator = self.current_token_string();
            self.advance();
            let right = Box::new(self.parse_exponentiation_expression()?);

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

    pub fn parse_exponentiation_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_unary_expression()?;

        while self.is_exponentiation_operator() {
            let operator = self.current_token_string();
            self.advance();
            let right = Box::new(self.parse_exponentiation_expression()?);

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
