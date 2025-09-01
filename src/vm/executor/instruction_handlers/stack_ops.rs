//! # Stack Operations Handler
//!
//! Handles all stack manipulation operations in the VM including pushing, popping,
//! duplicating, and other stack management operations.
//!
//! ## Operations Supported
//!
//! - **Basic Operations**: push, pop, peek
//! - **Duplication**: dup, dup2
//! - **Rearrangement**: swap, rot, over
//! - **Removal**: drop, drop2, clear
//! - **Information**: size, is_empty, depth
//! - **Advanced**: reserve, truncate
//!
//! ## Stack Semantics
//!
//! - **LIFO**: Last In, First Out stack behavior
//! - **Type Safety**: All operations maintain type safety
//! - **Bounds Checking**: Prevents stack underflow/overflow
//! - **Performance**: Optimized for common operations
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::vm::executor::instruction_handlers::StackOpsHandler;
//! use jetcrab::vm::executor::traits::StackOperations;
//! use jetcrab::vm::value::Value;
//! use jetcrab::vm::executor::stack_manager::StackManager;
//!
//! let mut stack = StackManager::new();
//! stack.push(Value::Number(42.0));
//! StackOpsHandler::dup(&mut stack).unwrap();
//! // Stack now contains: [42.0, 42.0]
//! ```

use crate::vm::executor::error_handler::ExecutionError;
use crate::vm::executor::traits::StackOperations;
use crate::vm::types::indices::StackIndex;
use crate::vm::value::Value;

/// Handles stack operations for the VM
pub struct StackOpsHandler;

impl StackOpsHandler {
    /// Pushes a value onto the stack
    ///
    /// Adds a value to the top of the stack.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `value` - The value to push onto the stack
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError)` on failure
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::instruction_handlers::stack_ops::StackOpsHandler;
    /// use jetcrab::vm::executor::stack_manager::StackManager;
    /// use jetcrab::vm::executor::traits::StackOperations;
    /// use jetcrab::vm::value::Value;
    ///
    /// let mut stack = StackManager::new();
    /// StackOpsHandler::push(&mut stack, Value::Number(42.0)).unwrap();
    /// assert_eq!(stack.peek(), Some(&Value::Number(42.0)));
    /// ```
    pub fn push<S>(stack: &mut S, value: Value) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        stack.push(value);
        Ok(())
    }

    /// Pops a value from the top of the stack
    ///
    /// Removes and returns the top value from the stack.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if stack is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::instruction_handlers::stack_ops::StackOpsHandler;
    /// use jetcrab::vm::executor::stack_manager::StackManager;
    /// use jetcrab::vm::executor::traits::StackOperations;
    /// use jetcrab::vm::value::Value;
    ///
    /// let mut stack = StackManager::new();
    /// stack.push(Value::Number(42.0));
    /// StackOpsHandler::pop(&mut stack).unwrap();
    /// assert!(stack.is_empty());
    /// ```
    pub fn pop<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        Ok(())
    }

    /// Duplicates the top value on the stack
    ///
    /// Pops the top value and pushes it twice, effectively duplicating it.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if stack is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::instruction_handlers::stack_ops::StackOpsHandler;
    /// use jetcrab::vm::executor::stack_manager::StackManager;
    /// use jetcrab::vm::executor::traits::StackOperations;
    /// use jetcrab::vm::value::Value;
    ///
    /// let mut stack = StackManager::new();
    /// stack.push(Value::Number(42.0));
    /// StackOpsHandler::dup(&mut stack).unwrap();
    /// assert_eq!(stack.size(), 2);
    /// assert_eq!(stack.peek(), Some(&Value::Number(42.0)));
    /// ```
    pub fn dup<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let value = stack.peek().ok_or(ExecutionError::StackUnderflow)?.clone();
        stack.push(value);
        Ok(())
    }

    /// Duplicates the top two values on the stack
    ///
    /// Pops the top two values and pushes them twice, effectively duplicating them.
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
    /// use jetcrab::vm::executor::instruction_handlers::stack_ops::StackOpsHandler;
    /// use jetcrab::vm::executor::stack_manager::StackManager;
    /// use jetcrab::vm::executor::traits::StackOperations;
    /// use jetcrab::vm::value::Value;
    ///
    /// let mut stack = StackManager::new();
    /// stack.push(Value::Number(1.0));
    /// stack.push(Value::Number(2.0));
    /// StackOpsHandler::dup2(&mut stack).unwrap();
    /// assert_eq!(stack.size(), 4);
    /// ```
    pub fn dup2<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        if stack.len() < 2 {
            return Err(ExecutionError::StackUnderflow);
        }

        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        stack.push(a.clone());
        stack.push(b.clone());
        stack.push(a);
        stack.push(b);

        Ok(())
    }

    /// Swaps the top two values on the stack
    ///
    /// Exchanges the positions of the top two values on the stack.
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
    /// use jetcrab::vm::executor::instruction_handlers::stack_ops::StackOpsHandler;
    /// use jetcrab::vm::executor::stack_manager::StackManager;
    /// use jetcrab::vm::executor::traits::StackOperations;
    /// use jetcrab::vm::value::Value;
    ///
    /// let mut stack = StackManager::new();
    /// stack.push(Value::Number(1.0));
    /// stack.push(Value::Number(2.0));
    /// StackOpsHandler::swap(&mut stack).unwrap();
    /// assert_eq!(stack.pop(), Some(Value::Number(1.0)));
    /// assert_eq!(stack.pop(), Some(Value::Number(2.0)));
    /// ```
    pub fn swap<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        if stack.len() < 2 {
            return Err(ExecutionError::StackUnderflow);
        }

        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        stack.push(b);
        stack.push(a);

        Ok(())
    }

    /// Rotates the top three values on the stack
    ///
    /// Moves the third value to the top, shifting the others down.
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
    /// use jetcrab::vm::executor::instruction_handlers::stack_ops::StackOpsHandler;
    /// use jetcrab::vm::executor::stack_manager::StackManager;
    /// use jetcrab::vm::executor::traits::StackOperations;
    /// use jetcrab::vm::value::Value;
    ///
    /// let mut stack = StackManager::new();
    /// stack.push(Value::Number(1.0));
    /// stack.push(Value::Number(2.0));
    /// stack.push(Value::Number(3.0));
    /// StackOpsHandler::rot(&mut stack).unwrap();
    /// assert_eq!(stack.pop(), Some(Value::Number(1.0)));
    /// ```
    pub fn rot<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        if stack.len() < 3 {
            return Err(ExecutionError::StackUnderflow);
        }

        let c = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        stack.push(b);
        stack.push(c);
        stack.push(a);

        Ok(())
    }

    /// Copies the second value to the top of the stack
    ///
    /// Pushes a copy of the second value without removing it.
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
    /// use jetcrab::vm::executor::instruction_handlers::stack_ops::StackOpsHandler;
    /// use jetcrab::vm::executor::stack_manager::StackManager;
    /// use jetcrab::vm::executor::traits::StackOperations;
    /// use jetcrab::vm::value::Value;
    ///
    /// let mut stack = StackManager::new();
    /// stack.push(Value::Number(1.0));
    /// stack.push(Value::Number(2.0));
    /// StackOpsHandler::over(&mut stack).unwrap();
    /// assert_eq!(stack.size(), 3);
    /// ```
    pub fn over<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        if stack.len() < 2 {
            return Err(ExecutionError::StackUnderflow);
        }

        let values = stack.stack_mut();
        let second_value = values
            .get(StackIndex::new(values.size() - 2))
            .unwrap()
            .clone();
        stack.push(second_value);

        Ok(())
    }

    /// Removes the top value from the stack
    ///
    /// Pops and discards the top value from the stack.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if stack is empty
    pub fn drop<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        Ok(())
    }

    /// Removes the top two values from the stack
    ///
    /// Pops and discards the top two values from the stack.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn drop2<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        if stack.len() < 2 {
            return Err(ExecutionError::StackUnderflow);
        }

        stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        Ok(())
    }

    /// Clears all values from the stack
    ///
    /// Removes all values from the stack, leaving it empty.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    pub fn clear<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        stack.clear();
        Ok(())
    }

    /// Gets the current size of the stack
    ///
    /// Pushes the number of values currently on the stack.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::instruction_handlers::stack_ops::StackOpsHandler;
    /// use jetcrab::vm::executor::stack_manager::StackManager;
    /// use jetcrab::vm::executor::traits::StackOperations;
    /// use jetcrab::vm::value::Value;
    ///
    /// let mut stack = StackManager::new();
    /// stack.push(Value::Number(1.0));
    /// stack.push(Value::Number(2.0));
    /// StackOpsHandler::size(&mut stack).unwrap();
    /// assert_eq!(stack.pop(), Some(Value::Number(2.0)));
    /// ```
    pub fn size<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let size = stack.size();
        stack.push(Value::Number(size as f64));
        Ok(())
    }

    /// Checks if the stack is empty
    ///
    /// Pushes a boolean indicating whether the stack is empty.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::instruction_handlers::stack_ops::StackOpsHandler;
    /// use jetcrab::vm::executor::stack_manager::StackManager;
    /// use jetcrab::vm::executor::traits::StackOperations;
    /// use jetcrab::vm::value::Value;
    ///
    /// let mut stack = StackManager::new();
    /// StackOpsHandler::is_empty(&mut stack).unwrap();
    /// assert_eq!(stack.pop(), Some(Value::Boolean(true)));
    /// ```
    pub fn is_empty<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let is_empty = stack.is_empty();
        stack.push(Value::Boolean(is_empty));
        Ok(())
    }

    /// Peeks at the top value without removing it
    ///
    /// Pushes a copy of the top value without modifying the stack.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if stack is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::instruction_handlers::stack_ops::StackOpsHandler;
    /// use jetcrab::vm::executor::stack_manager::StackManager;
    /// use jetcrab::vm::executor::traits::StackOperations;
    /// use jetcrab::vm::value::Value;
    ///
    /// let mut stack = StackManager::new();
    /// stack.push(Value::Number(42.0));
    /// StackOpsHandler::peek(&mut stack).unwrap();
    /// assert_eq!(stack.size(), 2);
    /// assert_eq!(stack.pop(), Some(Value::Number(42.0)));
    /// assert_eq!(stack.pop(), Some(Value::Number(42.0)));
    /// ```
    pub fn peek<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let value = stack.peek().ok_or(ExecutionError::StackUnderflow)?.clone();
        stack.push(value);
        Ok(())
    }

    /// Gets the depth of the stack
    ///
    /// Pushes the current depth (number of values) on the stack.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    ///
    /// # Returns
    /// * `Ok(())` on success
    pub fn depth<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let depth = stack.len();
        stack.push(Value::Number(depth as f64));
        Ok(())
    }

    /// Reserves space on the stack
    ///
    /// Ensures the stack has at least the specified capacity.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `capacity` - The minimum capacity to reserve
    ///
    /// # Returns
    /// * `Ok(())` on success
    pub fn reserve<S>(stack: &mut S, capacity: usize) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let stack_mut = stack.stack_mut();
        stack_mut.values.reserve(capacity);
        Ok(())
    }

    /// Truncates the stack to a specific size
    ///
    /// Removes values from the top of the stack until it reaches the specified size.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `size` - The target size for the stack
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if target size is larger than current size
    pub fn truncate<S>(stack: &mut S, size: usize) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let current_size = stack.len();
        if size > current_size {
            return Err(ExecutionError::StackUnderflow);
        }

        let to_remove = current_size - size;
        for _ in 0..to_remove {
            stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        }

        Ok(())
    }
}
