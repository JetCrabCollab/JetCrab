use crate::ast::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeError {
    TypeError {
        message: String,
        position: Option<Position>,
    },

    ReferenceError {
        message: String,
        position: Option<Position>,
    },

    RangeError {
        message: String,
        position: Option<Position>,
    },

    SyntaxError {
        message: String,
        position: Option<Position>,
    },

    UndefinedProperty {
        object: String,
        property: String,
        position: Option<Position>,
    },

    InvalidFunctionCall {
        function: String,
        message: String,
        position: Option<Position>,
    },

    ContextError {
        message: String,
        position: Option<Position>,
    },

    BuiltinError {
        builtin: String,
        message: String,
        position: Option<Position>,
    },

    ObjectError {
        message: String,
        position: Option<Position>,
    },
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::TypeError { message, position } => {
                write!(f, "TypeError: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            RuntimeError::ReferenceError { message, position } => {
                write!(f, "ReferenceError: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            RuntimeError::RangeError { message, position } => {
                write!(f, "RangeError: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            RuntimeError::SyntaxError { message, position } => {
                write!(f, "SyntaxError: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            RuntimeError::UndefinedProperty {
                object,
                property,
                position,
            } => {
                write!(f, "Cannot read property '{property}' of {object}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            RuntimeError::InvalidFunctionCall {
                function,
                message,
                position,
            } => {
                write!(f, "Invalid call to '{function}': {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            RuntimeError::ContextError { message, position } => {
                write!(f, "Context error: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            RuntimeError::BuiltinError {
                builtin,
                message,
                position,
            } => {
                write!(f, "Builtin '{builtin}' error: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            RuntimeError::ObjectError { message, position } => {
                write!(f, "Object error: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for RuntimeError {}
