use crate::ast::{AssignmentExpression, Node};
use crate::parser::error::ParseResult;
use crate::parser::Parser;

impl Parser {
    pub fn parse_assignment_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_logical_or_expression()?;

        if self.is_assignment_operator() {
            let operator = self.current_token_string();
            self.advance();
            let right = Box::new(self.parse_assignment_expression()?);

            let span = self.create_span_from_tokens();
            left = Node::AssignmentExpression(AssignmentExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }

        Ok(left)
    }
}
