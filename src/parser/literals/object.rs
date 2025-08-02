use crate::ast::{Node, ObjectLiteral, Property};
use crate::lexer::TokenKind;
use crate::parser::error::{ParseResult, ParserError};
use crate::parser::Parser;

impl Parser {
    pub fn parse_object_literal(&mut self) -> ParseResult<Node> {
        self.advance();

        let mut properties = Vec::new();

        while !self.check(TokenKind::RightBrace) && !self.is_eof() {
            properties.push(self.parse_property()?);

            if self.check(TokenKind::Comma) {
                self.advance();
            }
        }

        self.expect(TokenKind::RightBrace)?;

        let span = self.create_span_from_tokens();
        Ok(Node::ObjectLiteral(ObjectLiteral {
            properties,
            span: Some(span),
        }))
    }

    pub fn parse_property(&mut self) -> ParseResult<Node> {
        let key = if self.check_identifier() {
            Box::new(self.parse_identifier()?)
        } else if let Some(token) = &self.current {
            if let TokenKind::String(_) = &token.kind {
                Box::new(self.parse_primary_expression()?)
            } else {
                return Err(ParserError::invalid_syntax(
                    "Expected identifier or string literal",
                    self.current_position().unwrap_or_default(),
                ));
            }
        } else {
            return Err(ParserError::unexpected_end_of_input(None));
        };

        self.expect(TokenKind::Colon)?;
        let value = Box::new(self.parse_expression()?);

        let span = self.create_span_from_tokens();
        Ok(Node::Property(Property {
            key,
            value,
            kind: "init".to_string(),
            computed: false,
            method: false,
            shorthand: false,
            span: Some(span),
        }))
    }
}
