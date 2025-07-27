use crate::ast::node::Position;
use crate::vm::types::{ColumnNumber, LineNumber};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SemanticError {
    UndeclaredVariable {
        name: String,
        position: Option<Position>,
    },

    UninitializedVariable {
        name: String,
        position: Option<Position>,
    },

    ConstReassignment {
        name: String,
        position: Option<Position>,
    },

    TypeMismatch {
        expected: String,
        found: String,
        position: Option<Position>,
    },

    UndeclaredFunction {
        name: String,
        position: Option<Position>,
    },

    WrongArgumentCount {
        function_name: String,
        expected: usize,
        found: usize,
        position: Option<Position>,
    },

    InvalidThisUsage {
        position: Option<Position>,
    },

    DuplicateDeclaration {
        name: String,
        position: Option<Position>,
    },

    InvalidOperation {
        operation: String,
        type_name: String,
        position: Option<Position>,
    },

    UndefinedVariable(String, LineNumber, ColumnNumber),
    DuplicateDeclarationLegacy(String, LineNumber, ColumnNumber),
    TypeMismatchLegacy(String, String, LineNumber, ColumnNumber),
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SemanticError::UndeclaredVariable { name, position } => {
                write!(f, "Undeclared variable '{name}'")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            SemanticError::UninitializedVariable { name, position } => {
                write!(f, "Variable '{name}' is used before being initialized")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            SemanticError::ConstReassignment { name, position } => {
                write!(f, "Cannot reassign const variable '{name}'")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            SemanticError::TypeMismatch {
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
            SemanticError::UndeclaredFunction { name, position } => {
                write!(f, "Undeclared function '{name}'")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            SemanticError::WrongArgumentCount {
                function_name,
                expected,
                found,
                position,
            } => {
                write!(
                    f,
                    "Function '{function_name}' expects {} arguments, but {} were provided",
                    expected, found
                )?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            SemanticError::InvalidThisUsage { position } => {
                write!(f, "Invalid use of 'this' outside of method or constructor")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            SemanticError::DuplicateDeclaration { name, position } => {
                write!(f, "Duplicate declaration of '{name}'")?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            SemanticError::InvalidOperation {
                operation,
                type_name,
                position,
            } => {
                write!(
                    f,
                    "Invalid operation '{operation}' on type '{type_name}'",
                )?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }

            SemanticError::UndefinedVariable(name, line, col) => {
                write!(
                    f,
                    "Undefined variable '{name}' at line {line}, column {col}"
                )
            }
            SemanticError::DuplicateDeclarationLegacy(name, line, col) => {
                write!(
                    f,
                    "Duplicate declaration of '{name}' at line {line}, column {col}"
                )
            }
            SemanticError::TypeMismatchLegacy(expected, found, line, col) => {
                write!(
                    f,
                    "Type mismatch: expected {expected}, found {found} at line {line}, column {col}"
                )
            }
        }
    }
}

impl std::error::Error for SemanticError {}
