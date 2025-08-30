//! # Arithmetic Handler
//!
//! Handles all arithmetic operations in the VM including basic math operations,
//! increment/decrement operations, and power operations.
//!
//! ## Operations Supported
//!
//! - **Basic Arithmetic**: add, subtract, multiply, divide, modulo
//! - **Power Operations**: power (exponentiation)
//! - **Increment/Decrement**: increment, decrement
//! - **Unary Operations**: negate
//!
//! ## Error Handling
//!
//! All operations return `Result<(), ExecutionError>` and handle:
//! - Stack underflow (insufficient operands)
//! - Division by zero
//! - Invalid numeric operations
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::vm::executor::instruction_handlers::ArithmeticHandler;
//! use jetcrab::vm::executor::traits::StackOperations;
//!
//! let mut stack = MyStack::new();
//! stack.push(Value::Number(5.0));
//! stack.push(Value::Number(3.0));
//! ArithmeticHandler::add(&mut stack)?;
//! // Stack now contains: [8.0]
//! ```

use crate::vm::executor::error_handler::ExecutionError;
use crate::vm::executor::traits::StackOperations;
use crate::vm::value::Value;

/// Handles arithmetic operations for the VM
pub struct ArithmeticHandler;

impl ArithmeticHandler {
    /// Adds the top two values on the stack
    ///
    /// Pops two values from the stack, adds them, and pushes the result.
    /// Supports numeric addition and string concatenation.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut stack = MyStack::new();
    /// stack.push(Value::Number(3.0));
    /// stack.push(Value::Number(5.0));
    /// ArithmeticHandler::add(&mut stack)?;
    /// assert_eq!(stack.pop(), Some(Value::Number(8.0)));
    /// ```
    pub fn add<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::String(a), Value::String(b)) => Value::String(a + &b),
            (Value::String(a), Value::Number(b)) => Value::String(a + &b.to_string()),
            (Value::Number(a), Value::String(b)) => Value::String(a.to_string() + &b),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot add non-numeric values".to_string(),
                ))
            }
        };

        stack.push(result);
        Ok(())
    }

    /// Subtracts the second value from the first value on the stack
    ///
    /// Pops two values from the stack, subtracts the second from the first,
    /// and pushes the result.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn subtract<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot subtract non-numeric values".to_string(),
                ))
            }
        };

        stack.push(result);
        Ok(())
    }

    /// Multiplies the top two values on the stack
    ///
    /// Pops two values from the stack, multiplies them, and pushes the result.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn multiply<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot multiply non-numeric values".to_string(),
                ))
            }
        };

        stack.push(result);
        Ok(())
    }

    /// Divides the first value by the second value on the stack
    ///
    /// Pops two values from the stack, divides the first by the second,
    /// and pushes the result. Handles division by zero.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    /// * `Err(ExecutionError::RuntimeError)` on division by zero
    pub fn divide<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match (a, b) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    return Err(ExecutionError::RuntimeError("Division by zero".to_string()));
                }
                Value::Number(a / b)
            }
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot divide non-numeric values".to_string(),
                ))
            }
        };

        stack.push(result);
        Ok(())
    }

    /// Computes the remainder of division
    ///
    /// Pops two values from the stack, computes a % b, and pushes the result.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    /// * `Err(ExecutionError::RuntimeError)` on division by zero
    pub fn modulo<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match (a, b) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    return Err(ExecutionError::RuntimeError("Modulo by zero".to_string()));
                }
                Value::Number(a % b)
            }
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot compute modulo of non-numeric values".to_string(),
                ))
            }
        };

        stack.push(result);
        Ok(())
    }

    /// Computes the power of the first value raised to the second value
    ///
    /// Pops two values from the stack, computes a^b, and pushes the result.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn power<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a.powf(b)),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot compute power of non-numeric values".to_string(),
                ))
            }
        };

        stack.push(result);
        Ok(())
    }

    /// Negates the top value on the stack
    ///
    /// Pops one value from the stack, negates it, and pushes the result.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if stack is empty
    pub fn negate<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match value {
            Value::Number(n) => Value::Number(-n),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot negate non-numeric value".to_string(),
                ))
            }
        };

        stack.push(result);
        Ok(())
    }

    /// Increments the top value on the stack by 1
    ///
    /// Pops one value from the stack, adds 1, and pushes the result.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if stack is empty
    pub fn increment<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match value {
            Value::Number(n) => Value::Number(n + 1.0),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot increment non-numeric value".to_string(),
                ))
            }
        };

        stack.push(result);
        Ok(())
    }

    /// Decrements the top value on the stack by 1
    ///
    /// Pops one value from the stack, subtracts 1, and pushes the result.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if stack is empty
    pub fn decrement<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match value {
            Value::Number(n) => Value::Number(n - 1.0),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot decrement non-numeric value".to_string(),
                ))
            }
        };

        stack.push(result);
        Ok(())
    }
}
