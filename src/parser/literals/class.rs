use crate::ast::{ClassExpression, Node};
use crate::lexer::TokenKind;
use crate::parser::error::ParseResult;
use crate::parser::Parser;

impl Parser {
    pub fn parse_class_expression(&mut self) -> ParseResult<Node> {
        self.advance();

        let id = if self.check_identifier() {
            Some(Box::new(self.parse_identifier()?))
        } else {
            None
        };

        let super_class = if let Some(token) = &self.current {
            if let TokenKind::Keyword(kw) = &token.kind {
                if kw == "extends" {
                    self.advance();
                    Some(Box::new(self.parse_expression()?))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let body = Box::new(self.parse_class_body()?);

        let span = self.create_span_from_tokens();
        Ok(Node::ClassExpression(ClassExpression {
            id,
            super_class,
            body,
            span: Some(span),
        }))
    }
}
