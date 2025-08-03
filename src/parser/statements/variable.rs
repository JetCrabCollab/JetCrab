use crate::ast::{Node, VariableDeclaration, VariableDeclarator};
use crate::lexer::TokenKind;
use crate::parser::error::ParseResult;
use crate::parser::Parser;

impl Parser {
    pub fn parse_variable_declaration(&mut self) -> ParseResult<Node> {
        let kind = if let Some(token) = &self.current {
            if let TokenKind::Keyword(kw) = &token.kind {
                match kw.as_str() {
                    "let" => "let",
                    "const" => "const",
                    "var" => "var",
                    _ => unreachable!(),
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        };

        self.advance();

        let mut declarations = Vec::new();

        loop {
            let id = if self.check(TokenKind::LeftBrace) || self.check(TokenKind::LeftBracket) {
                self.parse_destructuring_pattern()?
            } else {
                self.parse_identifier()?
            };
            let init = if self.check(TokenKind::Assign) {
                self.advance();
                Some(Box::new(self.parse_expression()?))
            } else {
                None
            };

            let span = self.create_span_from_tokens();
            declarations.push(VariableDeclarator {
                id: Box::new(id),
                init,
                span: Some(span),
            });

            if !self.check(TokenKind::Comma) {
                break;
            }
            self.advance();
        }

        if self.check(TokenKind::Semicolon) {
            self.advance();
        }

        let span = self.create_span_from_tokens();
        Ok(Node::VariableDeclaration(VariableDeclaration {
            kind: kind.to_string(),
            declarations,
            span: Some(span),
        }))
    }
}
