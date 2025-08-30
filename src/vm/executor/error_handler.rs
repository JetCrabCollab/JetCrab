use crate::vm::value::Value;

#[derive(Debug, Clone)]
pub enum ExecutionError {
    StackUnderflow,
    InvalidType { expected: String, got: String },
    PropertyNotFound { object: String, property: String },
    FunctionNotFound { name: String },
    DivisionByZero,
    InvalidArrayIndex { index: usize, length: usize },
}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionError::StackUnderflow => write!(f, "Stack underflow"),
            ExecutionError::InvalidType { expected, got } => {
                write!(f, "Invalid type: expected {}, got {}", expected, got)
            }
            ExecutionError::PropertyNotFound { object, property } => {
                write!(f, "Property '{}' not found on object '{}'", property, object)
            }
            ExecutionError::FunctionNotFound { name } => {
                write!(f, "Function '{}' not found", name)
            }
            ExecutionError::DivisionByZero => write!(f, "Division by zero"),
            ExecutionError::InvalidArrayIndex { index, length } => {
                write!(f, "Invalid array index: {} (length: {})", index, length)
            }
        }
    }
}

impl std::error::Error for ExecutionError {}

pub trait ErrorHandler {
    fn handle_error(&mut self, error: ExecutionError) -> Value;
    fn log_error(&mut self, error: &ExecutionError);
}

pub struct DefaultErrorHandler;

impl DefaultErrorHandler {
    pub fn new() -> Self {
        Self
    }
}

impl ErrorHandler for DefaultErrorHandler {
    fn handle_error(&mut self, error: ExecutionError) -> Value {
        eprintln!("Execution error: {}", error);
        Value::Undefined
    }

    fn log_error(&mut self, error: &ExecutionError) {
        eprintln!("Error logged: {}", error);
    }
}
