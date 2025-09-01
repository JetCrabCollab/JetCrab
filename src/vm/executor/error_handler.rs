//! # Execution Error Handler
//!
//! Defines all error types that can occur during VM execution and provides
//! comprehensive error handling capabilities.
//!
//! ## Error Categories
//!
//! - **Stack Errors**: Underflow, overflow, invalid operations
//! - **Type Errors**: Invalid type conversions, unsupported operations
//! - **Runtime Errors**: General execution failures, exceptions
//! - **Memory Errors**: Heap allocation failures, invalid references
//! - **Control Flow Errors**: Invalid jumps, function call failures
//!
//! ## Error Handling Strategy
//!
//! The VM uses a comprehensive error handling strategy:
//!
//! 1. **Immediate Detection**: Errors are detected as soon as they occur
//! 2. **Detailed Information**: Each error contains context about what went wrong
//! 3. **Recovery Options**: Some errors can be recovered from automatically
//! 4. **User Feedback**: Clear error messages for debugging
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::vm::executor::error_handler::ExecutionError;
//!
//! // Example of handling different error types
//! let error = ExecutionError::TypeError("Cannot add string and number".to_string());
//! match error {
//!     ExecutionError::StackUnderflow => {
//!         eprintln!("Stack underflow occurred");
//!     }
//!     ExecutionError::TypeError(msg) => {
//!         eprintln!("Type error: {}", msg);
//!     }
//!     e => eprintln!("Other error: {:?}", e),
//! }
//! ```

use std::error::Error;
use std::fmt;

/// Represents all possible errors that can occur during VM execution
///
/// This enum provides a comprehensive set of error types covering
/// all aspects of virtual machine operation.
#[derive(Debug, Clone)]
pub enum ExecutionError {
    /// Stack is empty when trying to pop a value
    StackUnderflow,

    /// Stack is full when trying to push a value
    StackOverflow,

    /// Invalid type conversion or operation
    TypeError(String),

    /// General runtime execution error
    RuntimeError(String),

    /// Invalid memory access or allocation failure
    MemoryError(String),

    /// Invalid instruction or bytecode
    InvalidInstruction(String),

    /// Function call or return error
    FunctionError(String),

    /// Variable access or assignment error
    VariableError(String),

    /// Heap allocation or garbage collection error
    HeapError(String),

    /// Control flow error (invalid jumps, etc.)
    ControlFlowError(String),

    /// Built-in function error
    BuiltinError(String),

    /// Division by zero
    DivisionByZero,

    /// Invalid array index
    InvalidIndex(usize),

    /// Property not found on object
    PropertyNotFound(String),

    /// Method not found on object
    MethodNotFound(String),

    /// Invalid argument count for function call
    InvalidArgumentCount { expected: usize, received: usize },

    /// Recursion limit exceeded
    RecursionLimitExceeded(usize),

    /// Timeout during execution
    ExecutionTimeout,

    /// Unsupported operation
    UnsupportedOperation(String),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::StackUnderflow => write!(f, "Stack underflow"),
            ExecutionError::StackOverflow => write!(f, "Stack overflow"),
            ExecutionError::TypeError(msg) => write!(f, "Type error: {msg}"),
            ExecutionError::RuntimeError(msg) => write!(f, "Runtime error: {msg}"),
            ExecutionError::MemoryError(msg) => write!(f, "Memory error: {msg}"),
            ExecutionError::InvalidInstruction(msg) => write!(f, "Invalid instruction: {msg}"),
            ExecutionError::FunctionError(msg) => write!(f, "Function error: {msg}"),
            ExecutionError::VariableError(msg) => write!(f, "Variable error: {msg}"),
            ExecutionError::HeapError(msg) => write!(f, "Heap error: {msg}"),
            ExecutionError::ControlFlowError(msg) => write!(f, "Control flow error: {msg}"),
            ExecutionError::BuiltinError(msg) => write!(f, "Built-in error: {msg}"),
            ExecutionError::DivisionByZero => write!(f, "Division by zero"),
            ExecutionError::InvalidIndex(idx) => write!(f, "Invalid index: {idx}"),
            ExecutionError::PropertyNotFound(prop) => write!(f, "Property not found: {prop}"),
            ExecutionError::MethodNotFound(method) => write!(f, "Method not found: {method}"),
            ExecutionError::InvalidArgumentCount { expected, received } => {
                write!(
                    f,
                    "Invalid argument count: expected {expected}, received {received}"
                )
            }
            ExecutionError::RecursionLimitExceeded(limit) => {
                write!(f, "Recursion limit exceeded: {limit}")
            }
            ExecutionError::ExecutionTimeout => write!(f, "Execution timeout"),
            ExecutionError::UnsupportedOperation(op) => write!(f, "Unsupported operation: {op}"),
        }
    }
}

impl Error for ExecutionError {}

impl ExecutionError {
    /// Creates a new type error with the given message
    ///
    /// # Arguments
    /// * `message` - The error message
    ///
    /// # Returns
    /// * A new `ExecutionError::TypeError`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::ExecutionError;
    /// let error = ExecutionError::new_type_error("Cannot add string and number");
    /// assert!(matches!(error, ExecutionError::TypeError(_)));
    /// ```
    pub fn new_type_error(message: impl Into<String>) -> Self {
        ExecutionError::TypeError(message.into())
    }

    /// Creates a new runtime error with the given message
    ///
    /// # Arguments
    /// * `message` - The error message
    ///
    /// # Returns
    /// * A new `ExecutionError::RuntimeError`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::ExecutionError;
    /// let error = ExecutionError::new_runtime_error("Function execution failed");
    /// assert!(matches!(error, ExecutionError::RuntimeError(_)));
    /// ```
    pub fn new_runtime_error(message: impl Into<String>) -> Self {
        ExecutionError::RuntimeError(message.into())
    }

    /// Creates a new memory error with the given message
    ///
    /// # Arguments
    /// * `message` - The error message
    ///
    /// # Returns
    /// * A new `ExecutionError::MemoryError`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::ExecutionError;
    /// let error = ExecutionError::new_memory_error("Heap allocation failed");
    /// assert!(matches!(error, ExecutionError::MemoryError(_)));
    /// ```
    pub fn new_memory_error(message: impl Into<String>) -> Self {
        ExecutionError::MemoryError(message.into())
    }

    /// Creates a new function error with the given message
    ///
    /// # Arguments
    /// * `message` - The error message
    ///
    /// # Returns
    /// * A new `ExecutionError::FunctionError`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::ExecutionError;
    /// let error = ExecutionError::new_function_error("Invalid function call");
    /// assert!(matches!(error, ExecutionError::FunctionError(_)));
    /// ```
    pub fn new_function_error(message: impl Into<String>) -> Self {
        ExecutionError::FunctionError(message.into())
    }

    /// Creates a new variable error with the given message
    ///
    /// # Arguments
    /// * `message` - The error message
    ///
    /// # Returns
    /// * A new `ExecutionError::VariableError`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::ExecutionError;
    /// let error = ExecutionError::new_variable_error("Variable not defined");
    /// assert!(matches!(error, ExecutionError::VariableError(_)));
    /// ```
    pub fn new_variable_error(message: impl Into<String>) -> Self {
        ExecutionError::VariableError(message.into())
    }

    /// Creates a new heap error with the given message
    ///
    /// # Arguments
    /// * `message` - The error message
    ///
    /// # Returns
    /// * A new `ExecutionError::HeapError`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::ExecutionError;
    /// let error = ExecutionError::new_heap_error("Garbage collection failed");
    /// assert!(matches!(error, ExecutionError::HeapError(_)));
    /// ```
    pub fn new_heap_error(message: impl Into<String>) -> Self {
        ExecutionError::HeapError(message.into())
    }

    /// Creates a new control flow error with the given message
    ///
    /// # Arguments
    /// * `message` - The error message
    ///
    /// # Returns
    /// * A new `ExecutionError::ControlFlowError`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::ExecutionError;
    /// let error = ExecutionError::new_control_flow_error("Invalid jump target");
    /// assert!(matches!(error, ExecutionError::ControlFlowError(_)));
    /// ```
    pub fn new_control_flow_error(message: impl Into<String>) -> Self {
        ExecutionError::ControlFlowError(message.into())
    }

    /// Creates a new built-in error with the given message
    ///
    /// # Arguments
    /// * `message` - The error message
    ///
    /// # Returns
    /// * A new `ExecutionError::BuiltinError`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::ExecutionError;
    /// let error = ExecutionError::new_builtin_error("Console not available");
    /// assert!(matches!(error, ExecutionError::BuiltinError(_)));
    /// ```
    pub fn new_builtin_error(message: impl Into<String>) -> Self {
        ExecutionError::BuiltinError(message.into())
    }

    /// Creates a new unsupported operation error with the given message
    ///
    /// # Arguments
    /// * `message` - The error message
    ///
    /// # Returns
    /// * A new `ExecutionError::UnsupportedOperation`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::ExecutionError;
    /// let error = ExecutionError::new_unsupported_operation("BigInt operations");
    /// assert!(matches!(error, ExecutionError::UnsupportedOperation(_)));
    /// ```
    pub fn new_unsupported_operation(message: impl Into<String>) -> Self {
        ExecutionError::UnsupportedOperation(message.into())
    }

    /// Checks if this error is recoverable
    ///
    /// Some errors can be recovered from automatically, while others
    /// require manual intervention or indicate a serious problem.
    ///
    /// # Returns
    /// * `true` if the error is recoverable
    /// * `false` if the error requires manual intervention
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::ExecutionError;
    /// let stack_error = ExecutionError::StackUnderflow;
    /// assert!(stack_error.is_recoverable());
    ///
    /// let type_error = ExecutionError::TypeError("Invalid operation".to_string());
    /// assert!(!type_error.is_recoverable());
    /// ```
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            ExecutionError::StackUnderflow
                | ExecutionError::StackOverflow
                | ExecutionError::InvalidIndex(_)
                | ExecutionError::PropertyNotFound(_)
                | ExecutionError::MethodNotFound(_)
        )
    }

    /// Checks if this error is fatal
    ///
    /// Fatal errors cannot be recovered from and typically indicate
    /// a serious problem with the program or VM state.
    ///
    /// # Returns
    /// * `true` if the error is fatal
    /// * `false` if the error is not fatal
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::ExecutionError;
    /// let memory_error = ExecutionError::MemoryError("Heap corruption".to_string());
    /// assert!(memory_error.is_fatal());
    ///
    /// let stack_error = ExecutionError::StackUnderflow;
    /// assert!(!stack_error.is_fatal());
    /// ```
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            ExecutionError::MemoryError(_)
                | ExecutionError::InvalidInstruction(_)
                | ExecutionError::RecursionLimitExceeded(_)
                | ExecutionError::ExecutionTimeout
        )
    }

    /// Gets a user-friendly error message
    ///
    /// Returns a formatted error message suitable for display to users.
    ///
    /// # Returns
    /// * A formatted error message string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::ExecutionError;
    /// let error = ExecutionError::TypeError("Cannot add string and number".to_string());
    /// let message = error.get_user_message();
    /// assert!(message.contains("Type error"));
    /// ```
    pub fn get_user_message(&self) -> String {
        match self {
            ExecutionError::StackUnderflow => "Stack is empty".to_string(),
            ExecutionError::StackOverflow => "Stack is full".to_string(),
            ExecutionError::TypeError(msg) => format!("Type error: {msg}"),
            ExecutionError::RuntimeError(msg) => format!("Runtime error: {msg}"),
            ExecutionError::MemoryError(msg) => format!("Memory error: {msg}"),
            ExecutionError::InvalidInstruction(msg) => format!("Invalid instruction: {msg}"),
            ExecutionError::FunctionError(msg) => format!("Function error: {msg}"),
            ExecutionError::VariableError(msg) => format!("Variable error: {msg}"),
            ExecutionError::HeapError(msg) => format!("Heap error: {msg}"),
            ExecutionError::ControlFlowError(msg) => format!("Control flow error: {msg}"),
            ExecutionError::BuiltinError(msg) => format!("Built-in error: {msg}"),
            ExecutionError::DivisionByZero => "Division by zero".to_string(),
            ExecutionError::InvalidIndex(idx) => format!("Invalid index: {idx}"),
            ExecutionError::PropertyNotFound(prop) => format!("Property not found: '{prop}'"),
            ExecutionError::MethodNotFound(method) => format!("Method not found: '{method}'"),
            ExecutionError::InvalidArgumentCount { expected, received } => {
                format!("Invalid argument count: expected {expected}, received {received}")
            }
            ExecutionError::RecursionLimitExceeded(limit) => {
                format!("Recursion limit exceeded: {limit}")
            }
            ExecutionError::ExecutionTimeout => "Execution timeout".to_string(),
            ExecutionError::UnsupportedOperation(op) => format!("Unsupported operation: {op}"),
        }
    }

    /// Gets a debug error message
    ///
    /// Returns a detailed error message suitable for debugging and development.
    ///
    /// # Returns
    /// * A detailed debug message string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::ExecutionError;
    /// let error = ExecutionError::TypeError("Cannot add string and number".to_string());
    /// let debug_msg = error.get_debug_message();
    /// assert!(debug_msg.contains("TypeError"));
    /// ```
    pub fn get_debug_message(&self) -> String {
        format!("{self:?}")
    }
}
