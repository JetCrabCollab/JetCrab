//! Runtime Errors - JavaScript-specific runtime errors
//! 
//! This module defines JavaScript runtime errors that can occur during execution:
//! - TypeError: Type-related errors
//! - ReferenceError: Reference-related errors
//! - RangeError: Range and bounds errors
//! - SyntaxError: Syntax-related errors
//! - Error: Generic runtime errors

use std::fmt;
use crate::vm::error::VmError;

/// JavaScript runtime error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    /// Type error - invalid type for operation
    TypeError {
        message: String,
        operation: String,
        expected_type: String,
        actual_type: String,
    },
    
    /// Reference error - undefined variable or property
    ReferenceError {
        message: String,
        identifier: String,
        scope: String,
    },
    
    /// Range error - value out of valid range
    RangeError {
        message: String,
        value: String,
        min: Option<String>,
        max: Option<String>,
    },
    
    /// Syntax error - invalid syntax
    SyntaxError {
        message: String,
        line: Option<u32>,
        column: Option<u32>,
        token: Option<String>,
    },
    
    /// Generic runtime error
    Error {
        message: String,
        cause: Option<String>,
    },
}

impl RuntimeError {
    /// Create a new TypeError
    pub fn type_error(operation: &str, expected_type: &str, actual_type: &str) -> Self {
        let message = format!(
            "TypeError: Cannot perform '{}' on type '{}', expected '{}'",
            operation, actual_type, expected_type
        );
        
        Self::TypeError {
            message,
            operation: operation.to_string(),
            expected_type: expected_type.to_string(),
            actual_type: actual_type.to_string(),
        }
    }

    /// Create a new ReferenceError
    pub fn reference_error(identifier: &str, scope: &str) -> Self {
        let message = format!(
            "ReferenceError: '{}' is not defined in scope '{}'",
            identifier, scope
        );
        
        Self::ReferenceError {
            message,
            identifier: identifier.to_string(),
            scope: scope.to_string(),
        }
    }

    /// Create a new RangeError
    pub fn range_error(value: &str, min: Option<&str>, max: Option<&str>) -> Self {
        let message = match (min, max) {
            (Some(min), Some(max)) => {
                format!("RangeError: Value '{}' is out of range [{}, {}]", value, min, max)
            }
            (Some(min), None) => {
                format!("RangeError: Value '{}' is below minimum '{}'", value, min)
            }
            (None, Some(max)) => {
                format!("RangeError: Value '{}' is above maximum '{}'", value, max)
            }
            (None, None) => {
                format!("RangeError: Value '{}' is out of valid range", value)
            }
        };
        
        Self::RangeError {
            message,
            value: value.to_string(),
            min: min.map(|s| s.to_string()),
            max: max.map(|s| s.to_string()),
        }
    }

    /// Create a new SyntaxError
    pub fn syntax_error(message: &str, line: Option<u32>, column: Option<u32>, token: Option<&str>) -> Self {
        let full_message = match (line, column, token) {
            (Some(line), Some(column), Some(token)) => {
                format!("SyntaxError: {} at line {}, column {} (token: '{}')", message, line, column, token)
            }
            (Some(line), Some(column), None) => {
                format!("SyntaxError: {} at line {}, column {}", message, line, column)
            }
            (Some(line), None, None) => {
                format!("SyntaxError: {} at line {}", message, line)
            }
            _ => format!("SyntaxError: {}", message),
        };
        
        Self::SyntaxError {
            message: full_message,
            line,
            column,
            token: token.map(|s| s.to_string()),
        }
    }

    /// Create a new generic Error
    pub fn error(message: &str, cause: Option<&str>) -> Self {
        let full_message = if let Some(cause) = cause {
            format!("Error: {} (caused by: {})", message, cause)
        } else {
            format!("Error: {}", message)
        };
        
        Self::Error {
            message: full_message,
            cause: cause.map(|s| s.to_string()),
        }
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        match self {
            Self::TypeError { message, .. } => message,
            Self::ReferenceError { message, .. } => message,
            Self::RangeError { message, .. } => message,
            Self::SyntaxError { message, .. } => message,
            Self::Error { message, .. } => message,
        }
    }

    /// Get the error type name
    pub fn error_type(&self) -> &'static str {
        match self {
            Self::TypeError { .. } => "TypeError",
            Self::ReferenceError { .. } => "ReferenceError",
            Self::RangeError { .. } => "RangeError",
            Self::SyntaxError { .. } => "SyntaxError",
            Self::Error { .. } => "Error",
        }
    }

    /// Convert to VmError
    pub fn into_vm_error(self) -> VmError {
        VmError::RuntimeError(self)
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for RuntimeError {}

impl From<RuntimeError> for VmError {
    fn from(error: RuntimeError) -> Self {
        error.into_vm_error()
    }
}

/// Helper functions for common runtime errors
pub mod helpers {
    use super::RuntimeError;

    /// Create a TypeError for invalid operation on undefined
    pub fn undefined_operation(operation: &str) -> RuntimeError {
        RuntimeError::type_error(operation, "defined value", "undefined")
    }

    /// Create a TypeError for invalid operation on null
    pub fn null_operation(operation: &str) -> RuntimeError {
        RuntimeError::type_error(operation, "defined value", "null")
    }

    /// Create a TypeError for invalid operation on primitive
    pub fn primitive_operation(operation: &str, primitive_type: &str) -> RuntimeError {
        RuntimeError::type_error(operation, "object", primitive_type)
    }

    /// Create a ReferenceError for undefined variable
    pub fn undefined_variable(name: &str) -> RuntimeError {
        RuntimeError::reference_error(name, "global")
    }

    /// Create a ReferenceError for undefined property
    pub fn undefined_property(object: &str, property: &str) -> RuntimeError {
        RuntimeError::reference_error(property, &format!("object '{}'", object))
    }

    /// Create a RangeError for array index out of bounds
    pub fn array_index_out_of_bounds(index: usize, length: usize) -> RuntimeError {
        RuntimeError::range_error(
            &index.to_string(),
            Some("0"),
            Some(&(length - 1).to_string())
        )
    }

    /// Create a RangeError for invalid array length
    pub fn invalid_array_length(length: usize) -> RuntimeError {
        RuntimeError::range_error(
            &length.to_string(),
            Some("0"),
            Some("2^32-1")
        )
    }
}
