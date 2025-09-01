use crate::ast::{ArrayLiteral, Node, SpreadElement};
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
            } else if self.check(TokenKind::Spread) {
                self.advance();
                let argument = Box::new(self.parse_expression()?);
                let span = self.create_span_from_tokens();
                elements.push(Some(Node::SpreadElement(SpreadElement {
                    argument,
                    span: Some(span),
                })));
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
