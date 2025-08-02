use crate::ast::{Position, Span};
use crate::lexer::Token;
use thiserror::Error;

pub type ParseResult<T> = Result<T, ParserError>;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParserError {
    #[error("Unexpected token '{token}' at {position}")]
    UnexpectedToken {
        token: String,
        position: Position,
        expected: Option<String>,
    },

    #[error("Unexpected end of input at {position}")]
    UnexpectedEndOfInput {
        position: Position,
        expected: Option<String>,
    },

    #[error("Invalid syntax: {message} at {position}")]
    InvalidSyntax { message: String, position: Position },

    #[error("Invalid expression: {message} at {position}")]
    InvalidExpression { message: String, position: Position },

    #[error("Invalid statement: {message} at {position}")]
    InvalidStatement { message: String, position: Position },

    #[error("Invalid declaration: {message} at {position}")]
    InvalidDeclaration { message: String, position: Position },

    #[error("Invalid function: {message} at {position}")]
    InvalidFunction { message: String, position: Position },

    #[error("Invalid class: {message} at {position}")]
    InvalidClass { message: String, position: Position },

    #[error("Invalid module: {message} at {position}")]
    InvalidModule { message: String, position: Position },

    #[error("Lexer error: {message} at {position}")]
    LexerError { message: String, position: Position },

    #[error("Internal parser error: {message}")]
    InternalError { message: String },
}

impl ParserError {
    pub fn unexpected_token(token: &Token, expected: Option<&str>) -> Self {
        let position = Position {
            line: token.start().line,
            column: token.start().column,
        };
        ParserError::UnexpectedToken {
            token: format!("{:?}", token.kind),
            position,
            expected: expected.map(|s| s.to_string()),
        }
    }

    pub fn unexpected_end_of_input(expected: Option<&str>) -> Self {
        ParserError::UnexpectedEndOfInput {
            position: Position::new(1, 1),
            expected: expected.map(|s| s.to_string()),
        }
    }

    pub fn invalid_syntax(message: &str, position: Position) -> Self {
        ParserError::InvalidSyntax {
            message: message.to_string(),
            position,
        }
    }

    pub fn invalid_expression(message: &str, position: Position) -> Self {
        ParserError::InvalidExpression {
            message: message.to_string(),
            position,
        }
    }

    pub fn invalid_statement(message: &str, position: Position) -> Self {
        ParserError::InvalidStatement {
            message: message.to_string(),
            position,
        }
    }

    pub fn invalid_declaration(message: &str, position: Position) -> Self {
        ParserError::InvalidDeclaration {
            message: message.to_string(),
            position,
        }
    }

    pub fn invalid_function(message: &str, position: Position) -> Self {
        ParserError::InvalidFunction {
            message: message.to_string(),
            position,
        }
    }

    pub fn invalid_class(message: &str, position: Position) -> Self {
        ParserError::InvalidClass {
            message: message.to_string(),
            position,
        }
    }

    pub fn invalid_module(message: &str, position: Position) -> Self {
        ParserError::InvalidModule {
            message: message.to_string(),
            position,
        }
    }

    pub fn lexer_error(message: &str, position: Position) -> Self {
        ParserError::LexerError {
            message: message.to_string(),
            position,
        }
    }

    pub fn internal_error(message: &str) -> Self {
        ParserError::InternalError {
            message: message.to_string(),
        }
    }

    pub fn position(&self) -> Option<Position> {
        match self {
            ParserError::UnexpectedToken { position, .. } => Some(*position),
            ParserError::UnexpectedEndOfInput { position, .. } => Some(*position),
            ParserError::InvalidSyntax { position, .. } => Some(*position),
            ParserError::InvalidExpression { position, .. } => Some(*position),
            ParserError::InvalidStatement { position, .. } => Some(*position),
            ParserError::InvalidDeclaration { position, .. } => Some(*position),
            ParserError::InvalidFunction { position, .. } => Some(*position),
            ParserError::InvalidClass { position, .. } => Some(*position),
            ParserError::InvalidModule { position, .. } => Some(*position),
            ParserError::LexerError { position, .. } => Some(*position),
            ParserError::InternalError { .. } => None,
        }
    }

    pub fn span(&self) -> Option<Span> {
        self.position().map(|pos| Span::new(pos, pos))
    }
}
