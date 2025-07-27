use crate::ast::node::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AstError {
    SerializationError {
        message: String,
        position: Option<Position>,
    },

    DeserializationError {
        message: String,
        position: Option<Position>,
    },

    InvalidNode {
        node_type: String,
        message: String,
        position: Option<Position>,
    },

    VisitorError {
        message: String,
        position: Option<Position>,
    },
}

impl std::fmt::Display for AstError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstError::SerializationError { message, position } => {
                write!(f, "AST serialization error: {}", message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            AstError::DeserializationError { message, position } => {
                write!(f, "AST deserialization error: {}", message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            AstError::InvalidNode {
                node_type,
                message,
                position,
            } => {
                write!(f, "Invalid AST node '{}': {}", node_type, message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            AstError::VisitorError { message, position } => {
                write!(f, "AST visitor error: {}", message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for AstError {}
