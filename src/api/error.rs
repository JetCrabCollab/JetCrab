use crate::ast::node::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiError {
    CompilationError {
        message: String,
        position: Option<Position>,
    },

    ExecutionError {
        message: String,
        position: Option<Position>,
    },

    InvalidInput {
        message: String,
        input: String,
        position: Option<Position>,
    },

    EngineError {
        message: String,
        position: Option<Position>,
    },

    InterpreterError {
        message: String,
        position: Option<Position>,
    },

    ConfigurationError {
        message: String,
        position: Option<Position>,
    },

    ResourceError {
        resource: String,
        message: String,
        position: Option<Position>,
    },

    TimeoutError {
        operation: String,
        timeout_ms: u64,
        position: Option<Position>,
    },
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::CompilationError { message, position } => {
                write!(f, "Compilation error: {}", message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            ApiError::ExecutionError { message, position } => {
                write!(f, "Execution error: {}", message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            ApiError::InvalidInput {
                message,
                input,
                position,
            } => {
                write!(f, "Invalid input '{}': {}", input, message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            ApiError::EngineError { message, position } => {
                write!(f, "Engine error: {}", message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            ApiError::InterpreterError { message, position } => {
                write!(f, "Interpreter error: {}", message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            ApiError::ConfigurationError { message, position } => {
                write!(f, "Configuration error: {}", message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            ApiError::ResourceError {
                resource,
                message,
                position,
            } => {
                write!(f, "Resource '{}' error: {}", resource, message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            ApiError::TimeoutError {
                operation,
                timeout_ms,
                position,
            } => {
                write!(
                    f,
                    "Operation '{}' timed out after {}ms",
                    operation, timeout_ms
                )?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for ApiError {}
