use crate::ast::{ArrayLiteral, Node};
use crate::lexer::TokenKind;
use crate::parser::error::ParseResult;
use crate::parser::Parser;

impl Parser {
    pub fn parse_array_literal(&mut self) -> ParseResult<Node> {
        self.advance();

        let mut elements = Vec::new();

        while !self.check(TokenKind::RightBracket) && !self.is_eof() {
            if self.check(TokenKind::Comma) {
                elements.push(None);
                self.advance();
            } else {
                elements.push(Some(self.parse_expression()?));

                if self.check(TokenKind::Comma) {
                    self.advance();
                }
            }
        }

        self.expect(TokenKind::RightBracket)?;

        let span = self.create_span_from_tokens();
        Ok(Node::ArrayLiteral(ArrayLiteral {
            elements,
            span: Some(span),
        }))
    }
}
