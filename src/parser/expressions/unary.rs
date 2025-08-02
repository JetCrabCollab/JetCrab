use crate::ast::{CallExpression, MemberExpression, Node, UnaryExpression, UpdateExpression};
use crate::lexer::TokenKind;
use crate::parser::error::ParseResult;
use crate::parser::Parser;

impl Parser {
    pub fn parse_unary_expression(&mut self) -> ParseResult<Node> {
        if self.is_unary_operator() {
            let operator = self.current_token_string();
            let prefix = true;
            self.advance();
            let argument = Box::new(self.parse_unary_expression()?);

            let span = self.create_span_from_tokens();
            return Ok(Node::UnaryExpression(UnaryExpression {
                operator,
                argument,
                prefix,
                span: Some(span),
            }));
        }

        self.parse_postfix_expression()
    }

    pub fn parse_postfix_expression(&mut self) -> ParseResult<Node> {
        let mut expr = self.parse_primary_expression()?;

        while let Some(token) = &self.current {
            match &token.kind {
                TokenKind::LeftBracket => {
                    self.advance();
                    let property = Box::new(self.parse_expression()?);
                    self.expect(TokenKind::RightBracket)?;

                    let span = self.create_span_from_tokens();
                    expr = Node::MemberExpression(MemberExpression {
                        object: Box::new(expr),
                        property,
                        computed: true,
                        optional: false,
                        span: Some(span),
                    });
                }

                TokenKind::Dot => {
                    self.advance();
                    let property = Box::new(self.parse_identifier()?);

                    let span = self.create_span_from_tokens();
                    expr = Node::MemberExpression(MemberExpression {
                        object: Box::new(expr),
                        property,
                        computed: false,
                        optional: false,
                        span: Some(span),
                    });
                }

                TokenKind::LeftParen => {
                    self.advance();
                    let arguments = self.parse_arguments()?;
                    self.expect(TokenKind::RightParen)?;

                    let span = self.create_span_from_tokens();
                    expr = Node::CallExpression(CallExpression {
                        callee: Box::new(expr),
                        arguments,
                        span: Some(span),
                    });
                }

                TokenKind::Increment | TokenKind::Decrement => {
                    let operator = self.current_token_string();
                    let prefix = false;
                    self.advance();

                    let span = self.create_span_from_tokens();
                    expr = Node::UpdateExpression(UpdateExpression {
                        operator,
                        argument: Box::new(expr),
                        prefix,
                        span: Some(span),
                    });
                }

                _ => break,
            }
        }

        Ok(expr)
    }
}
