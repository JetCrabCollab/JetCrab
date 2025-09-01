use crate::ast::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VmError {
    ExecutionError {
        message: String,
        instruction: Option<String>,
        position: Option<Position>,
    },

    StackUnderflow {
        message: String,
        position: Option<Position>,
    },

    StackOverflow {
        message: String,
        position: Option<Position>,
    },

    InvalidInstruction {
        instruction: String,
        message: String,
        position: Option<Position>,
    },

    TypeMismatch {
        expected: String,
        found: String,
        position: Option<Position>,
    },

    UndefinedVariable {
        name: String,
        position: Option<Position>,
    },

    UndefinedFunction {
        name: String,
        position: Option<Position>,
    },

    DivisionByZero {
        position: Option<Position>,
    },

    OutOfMemory {
        message: String,
        position: Option<Position>,
    },

    RuntimeError {
        message: String,
        position: Option<Position>,
    },

    // JavaScript-specific runtime errors
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

impl std::fmt::Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmError::ExecutionError {
                message,
                instruction,
                position,
            } => {
                write!(f, "VM execution error: {message}")?;
                if let Some(instr) = instruction {
                    write!(f, " (instruction: {instr})")?;
                }
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            VmError::StackUnderflow { message, position } => {
                write!(f, "Stack underflow: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            VmError::StackOverflow { message, position } => {
                write!(f, "Stack overflow: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            VmError::InvalidInstruction {
                instruction,
                message,
                position,
            } => {
                write!(f, "Invalid instruction '{instruction}': {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            VmError::TypeMismatch {
                expected,
                found,
                position,
            } => {
                write!(f, "Type mismatch: expected {expected}, found {found}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            VmError::UndefinedVariable { name, position } => {
                write!(f, "Undefined variable '{name}'")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            VmError::UndefinedFunction { name, position } => {
                write!(f, "Undefined function '{name}'")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            VmError::DivisionByZero { position } => {
                write!(f, "Division by zero")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            VmError::OutOfMemory { message, position } => {
                write!(f, "Out of memory: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            VmError::RuntimeError { message, position } => {
                write!(f, "Runtime error: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }

            // JavaScript-specific runtime errors
            VmError::TypeError { message, position } => {
                write!(f, "TypeError: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }

            VmError::ReferenceError { message, position } => {
                write!(f, "ReferenceError: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }

            VmError::RangeError { message, position } => {
                write!(f, "RangeError: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }

            VmError::SyntaxError { message, position } => {
                write!(f, "SyntaxError: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }

            VmError::UndefinedProperty {
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

            VmError::InvalidFunctionCall {
                function,
                message,
                position,
            } => {
                write!(f, "Invalid function call '{function}': {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }

            VmError::ContextError { message, position } => {
                write!(f, "Context error: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }

            VmError::BuiltinError {
                builtin,
                message,
                position,
            } => {
                write!(f, "Builtin error '{builtin}': {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }

            VmError::ObjectError { message, position } => {
                write!(f, "Object error: {message}")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for VmError {}
