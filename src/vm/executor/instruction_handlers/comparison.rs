//! # Comparison Handler
//!
//! Handles all comparison and logical operations in the VM including equality checks,
//! relational comparisons, and logical operations.
//!
//! ## Operations Supported
//!
//! - **Equality**: equal, not_equal, strict_equal, strict_not_equal
//! - **Relational**: less_than, less_equal, greater_than, greater_equal
//! - **Logical**: logical_and, logical_or, logical_not
//! - **Bitwise**: bitwise_and, bitwise_or, bitwise_xor, bitwise_not
//!
//! ## Comparison Rules
//!
//! - **Loose Equality**: Uses JavaScript-like type coercion
//! - **Strict Equality**: No type coercion, exact value comparison
//! - **Relational**: Numeric comparison with type coercion
//! - **Logical**: Boolean operations with truthy/falsy conversion
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::vm::executor::instruction_handlers::ComparisonHandler;
//! use jetcrab::vm::executor::traits::StackOperations;
//! use jetcrab::vm::value::Value;
//! use jetcrab::vm::executor::stack_manager::StackManager;
//!
//! let mut stack = StackManager::new();
//! stack.push(Value::Number(5.0));
//! stack.push(Value::Number(3.0));
//! ComparisonHandler::greater_than(&mut stack).unwrap();
//! // Stack now contains: [true]
//! ```

use crate::vm::executor::error_handler::ExecutionError;
use crate::vm::executor::traits::StackOperations;
use crate::vm::value::Value;

/// Handles comparison and logical operations for the VM
pub struct ComparisonHandler;

impl ComparisonHandler {
    /// Checks if two values are equal using loose equality
    ///
    /// Pops two values from the stack, compares them using JavaScript-like
    /// type coercion, and pushes the boolean result.
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
    /// use jetcrab::vm::executor::instruction_handlers::ComparisonHandler;
    /// use jetcrab::vm::executor::stack_manager::StackManager;
    /// use jetcrab::vm::executor::traits::StackOperations;
    /// use jetcrab::vm::value::Value;
    ///
    /// let mut stack = StackManager::new();
    /// stack.push(Value::Number(5.0));
    /// stack.push(Value::String("5".to_string()));
    /// ComparisonHandler::equal(&mut stack).unwrap();
    /// assert_eq!(stack.pop(), Some(Value::Boolean(true)));
    /// ```
    pub fn equal<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match (a, b) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Undefined, Value::Undefined) => true,
            (Value::Number(a), Value::String(b)) => a.to_string() == b,
            (Value::String(a), Value::Number(b)) => a == b.to_string(),
            (Value::Number(a), Value::Boolean(b)) => a == if b { 1.0 } else { 0.0 },
            (Value::Boolean(a), Value::Number(b)) => (if a { 1.0 } else { 0.0 }) == b,
            _ => false,
        };

        stack.push(Value::Boolean(result));
        Ok(())
    }

    /// Checks if two values are not equal using loose equality
    ///
    /// Pops two values from the stack, compares them using JavaScript-like
    /// type coercion, and pushes the boolean result.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn not_equal<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match (a, b) {
            (Value::Number(a), Value::Number(b)) => a != b,
            (Value::String(a), Value::String(b)) => a != b,
            (Value::Boolean(a), Value::Boolean(b)) => a != b,
            (Value::Null, Value::Null) => false,
            (Value::Undefined, Value::Undefined) => false,
            (Value::Number(a), Value::String(b)) => a.to_string() != b,
            (Value::String(a), Value::Number(b)) => a != b.to_string(),
            (Value::Number(a), Value::Boolean(b)) => a != if b { 1.0 } else { 0.0 },
            (Value::Boolean(a), Value::Number(b)) => (if a { 1.0 } else { 0.0 }) != b,
            _ => true,
        };

        stack.push(Value::Boolean(result));
        Ok(())
    }

    /// Checks if two values are strictly equal
    ///
    /// Pops two values from the stack, compares them without type coercion,
    /// and pushes the boolean result.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn strict_equal<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = a == b;
        stack.push(Value::Boolean(result));
        Ok(())
    }

    /// Checks if two values are strictly not equal
    ///
    /// Pops two values from the stack, compares them without type coercion,
    /// and pushes the boolean result.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn strict_not_equal<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = a != b;
        stack.push(Value::Boolean(result));
        Ok(())
    }

    /// Checks if the first value is less than the second
    ///
    /// Pops two values from the stack, compares them, and pushes the boolean result.
    /// Supports numeric comparison with type coercion.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn less_than<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match (a, b) {
            (Value::Number(a), Value::Number(b)) => a < b,
            (Value::String(a), Value::String(b)) => a < b,
            (Value::Number(a), Value::String(b)) => a < b.parse::<f64>().unwrap_or(f64::NAN),
            (Value::String(a), Value::Number(b)) => a.parse::<f64>().unwrap_or(f64::NAN) < b,
            _ => false,
        };

        stack.push(Value::Boolean(result));
        Ok(())
    }

    /// Checks if the first value is less than or equal to the second
    ///
    /// Pops two values from the stack, compares them, and pushes the boolean result.
    /// Supports numeric comparison with type coercion.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn less_equal<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match (a, b) {
            (Value::Number(a), Value::Number(b)) => a <= b,
            (Value::String(a), Value::String(b)) => a <= b,
            (Value::Number(a), Value::String(b)) => a <= b.parse::<f64>().unwrap_or(f64::NAN),
            (Value::String(a), Value::Number(b)) => a.parse::<f64>().unwrap_or(f64::NAN) <= b,
            _ => false,
        };

        stack.push(Value::Boolean(result));
        Ok(())
    }

    /// Checks if the first value is greater than the second
    ///
    /// Pops two values from the stack, compares them, and pushes the boolean result.
    /// Supports numeric comparison with type coercion.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn greater_than<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match (a, b) {
            (Value::Number(a), Value::Number(b)) => a > b,
            (Value::String(a), Value::String(b)) => a > b,
            (Value::Number(a), Value::String(b)) => a > b.parse::<f64>().unwrap_or(f64::NAN),
            (Value::String(a), Value::Number(b)) => a.parse::<f64>().unwrap_or(f64::NAN) > b,
            _ => false,
        };

        stack.push(Value::Boolean(result));
        Ok(())
    }

    /// Checks if the first value is greater than or equal to the second
    ///
    /// Pops two values from the stack, compares them, and pushes the boolean result.
    /// Supports numeric comparison with type coercion.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn greater_equal<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match (a, b) {
            (Value::Number(a), Value::Number(b)) => a >= b,
            (Value::String(a), Value::String(b)) => a >= b,
            (Value::Number(a), Value::String(b)) => a >= b.parse::<f64>().unwrap_or(f64::NAN),
            (Value::String(a), Value::Number(b)) => a.parse::<f64>().unwrap_or(f64::NAN) >= b,
            _ => false,
        };

        stack.push(Value::Boolean(result));
        Ok(())
    }

    /// Performs logical AND operation
    ///
    /// Pops two values from the stack, performs logical AND, and pushes the result.
    /// Uses JavaScript-like truthy/falsy conversion.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn logical_and<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = if is_truthy(&a) { b } else { a };
        stack.push(result);
        Ok(())
    }

    /// Performs logical OR operation
    ///
    /// Pops two values from the stack, performs logical OR, and pushes the result.
    /// Uses JavaScript-like truthy/falsy conversion.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn logical_or<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = if is_truthy(&a) { a } else { b };
        stack.push(result);
        Ok(())
    }

    /// Performs logical NOT operation
    ///
    /// Pops one value from the stack, performs logical NOT, and pushes the result.
    /// Uses JavaScript-like truthy/falsy conversion.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if stack is empty
    pub fn logical_not<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let result = !is_truthy(&value);
        stack.push(Value::Boolean(result));
        Ok(())
    }

    /// Performs bitwise AND operation
    ///
    /// Pops two values from the stack, performs bitwise AND, and pushes the result.
    /// Converts values to integers before operation.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn bitwise_and<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number((a as i64 & b as i64) as f64),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot perform bitwise AND on non-numeric values".to_string(),
                ))
            }
        };

        stack.push(result);
        Ok(())
    }

    /// Performs bitwise OR operation
    ///
    /// Pops two values from the stack, performs bitwise OR, and pushes the result.
    /// Converts values to integers before operation.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn bitwise_or<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number((a as i64 | b as i64) as f64),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot perform bitwise OR on non-numeric values".to_string(),
                ))
            }
        };

        stack.push(result);
        Ok(())
    }

    /// Performs bitwise XOR operation
    ///
    /// Pops two values from the stack, performs bitwise XOR, and pushes the result.
    /// Converts values to integers before operation.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn bitwise_xor<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number((a as i64 ^ b as i64) as f64),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot perform bitwise XOR on non-numeric values".to_string(),
                ))
            }
        };

        stack.push(result);
        Ok(())
    }

    /// Performs bitwise NOT operation
    ///
    /// Pops one value from the stack, performs bitwise NOT, and pushes the result.
    /// Converts value to integer before operation.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if stack is empty
    pub fn bitwise_not<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match value {
            Value::Number(n) => Value::Number(!(n as i64) as f64),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot perform bitwise NOT on non-numeric value".to_string(),
                ))
            }
        };

        stack.push(result);
        Ok(())
    }
}

/// Determines if a value is truthy according to JavaScript rules
///
/// # Arguments
/// * `value` - The value to check
///
/// # Returns
/// * `true` if the value is truthy
/// * `false` if the value is falsy
fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Boolean(b) => *b,
        Value::Number(n) => *n != 0.0 && !n.is_nan(),
        Value::String(s) => !s.is_empty(),
        Value::Null => false,
        Value::Undefined => false,
        Value::Object(_) => true,
        Value::Array(_) => true,
        Value::Function(_) => true,
    }
}
