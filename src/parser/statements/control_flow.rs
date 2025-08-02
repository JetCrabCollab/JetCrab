use crate::ast::{
    BreakStatement, CatchClause, ContinueStatement, Node, ThrowStatement, TryStatement,
};
use crate::lexer::TokenKind;
use crate::parser::error::ParseResult;
use crate::parser::Parser;

impl Parser {
    pub fn parse_break_statement(&mut self) -> ParseResult<Node> {
        self.advance();

        let label = if self.check_identifier() {
            Some(Box::new(self.parse_identifier()?))
        } else {
            None
        };

        let span = self.create_span_from_tokens();
        Ok(Node::BreakStatement(BreakStatement {
            label,
            span: Some(span),
        }))
    }

    pub fn parse_continue_statement(&mut self) -> ParseResult<Node> {
        self.advance();

        let label = if self.check_identifier() {
            Some(Box::new(self.parse_identifier()?))
        } else {
            None
        };

        let span = self.create_span_from_tokens();
        Ok(Node::ContinueStatement(ContinueStatement {
            label,
            span: Some(span),
        }))
    }

    pub fn parse_throw_statement(&mut self) -> ParseResult<Node> {
        self.advance();

        let argument = Box::new(self.parse_expression()?);

        let span = self.create_span_from_tokens();
        Ok(Node::ThrowStatement(ThrowStatement {
            argument,
            span: Some(span),
        }))
    }

    pub fn parse_try_statement(&mut self) -> ParseResult<Node> {
        self.advance();

        let block = Box::new(self.parse_block_statement()?);

        let handler = if let Some(token) = &self.current {
            if let TokenKind::Keyword(kw) = &token.kind {
                if kw == "catch" {
                    Some(Box::new(self.parse_catch_clause()?))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let finalizer = if let Some(token) = &self.current {
            if let TokenKind::Keyword(kw) = &token.kind {
                if kw == "finally" {
                    self.advance();
                    Some(Box::new(self.parse_block_statement()?))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let span = self.create_span_from_tokens();
        Ok(Node::TryStatement(TryStatement {
            block,
            handler,
            finalizer,
            span: Some(span),
        }))
    }

    pub fn parse_catch_clause(&mut self) -> ParseResult<Node> {
        self.advance();

        self.expect(TokenKind::LeftParen)?;
        let param = Box::new(self.parse_identifier()?);
        self.expect(TokenKind::RightParen)?;

        let body = Box::new(self.parse_block_statement()?);

        let span = self.create_span_from_tokens();
        Ok(Node::CatchClause(CatchClause {
            param,
            body,
            span: Some(span),
        }))
    }
}
