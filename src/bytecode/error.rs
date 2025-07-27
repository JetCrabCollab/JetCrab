use crate::ast::node::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BytecodeError {
    GenerationError {
        message: String,
        position: Option<Position>,
    },

    OptimizationError {
        message: String,
        position: Option<Position>,
    },

    InvalidInstruction {
        instruction: String,
        message: String,
        position: Option<Position>,
    },

    ConstantPoolFull {
        message: String,
        position: Option<Position>,
    },

    UnsupportedNode {
        node_type: String,
        message: String,
        position: Option<Position>,
    },

    StackOverflow {
        message: String,
        position: Option<Position>,
    },
}

impl std::fmt::Display for BytecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BytecodeError::GenerationError { message, position } => {
                write!(f, "Bytecode generation error: {}", message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            BytecodeError::OptimizationError { message, position } => {
                write!(f, "Bytecode optimization error: {}", message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            BytecodeError::InvalidInstruction {
                instruction,
                message,
                position,
            } => {
                write!(f, "Invalid instruction '{}': {}", instruction, message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            BytecodeError::ConstantPoolFull { message, position } => {
                write!(f, "Constant pool full: {}", message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            BytecodeError::UnsupportedNode {
                node_type,
                message,
                position,
            } => {
                write!(f, "Unsupported node type '{}': {}", node_type, message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            BytecodeError::StackOverflow { message, position } => {
                write!(f, "Stack overflow: {}", message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for BytecodeError {}
