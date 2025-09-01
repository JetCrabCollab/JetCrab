//! # Stack Manager
//!
//! Provides concrete implementation of stack operations for the VM executor.
//! Manages the execution stack and implements the `StackOperations` trait
//! to provide stack manipulation functionality.
//!
//! ## Overview
//!
//! The stack manager wraps the low-level `Stack` implementation and provides
//! a high-level interface for stack operations including:
//!
//! - **Push/Pop Operations**: Adding and removing values
//! - **Inspection**: Peeking at values without modification
//! - **Management**: Clearing, sizing, and state checking
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::vm::executor::stack_manager::StackManager;
//! use jetcrab::vm::executor::traits::StackOperations;
//! use jetcrab::vm::value::Value;
//!
//! let mut stack_manager = StackManager::new();
//! stack_manager.push(Value::Number(42.0));
//! let value = stack_manager.pop();
//! ```

use super::StackOperations;
use crate::vm::memory::stack::Stack;
use crate::vm::value::Value;

/// Concrete implementation of stack operations for the VM
///
/// Wraps the low-level Stack and provides high-level stack management
/// functionality for the execution engine.
pub struct StackManager {
    stack: Stack,
}

impl Default for StackManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StackManager {
    /// Creates a new stack manager with an empty stack
    ///
    /// Initializes the stack manager with a fresh, empty stack
    /// ready for VM operations.
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
        }
    }

    /// Gets read-only access to the underlying stack
    ///
    /// Provides access to the stack for inspection and debugging
    /// without allowing modifications.
    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    /// Gets mutable access to the underlying stack
    ///
    /// Provides direct access to the stack for advanced operations
    /// and testing purposes.
    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.stack
    }
}

impl StackOperations for StackManager {
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    fn peek(&self) -> Option<&Value> {
        self.stack.values.last()
    }

    fn clear(&mut self) {
        self.stack.values.clear();
    }

    fn len(&self) -> usize {
        self.stack.values.len()
    }

    fn is_empty(&self) -> bool {
        self.stack.values.is_empty()
    }

    fn stack_mut(&mut self) -> &mut crate::vm::memory::stack::Stack {
        &mut self.stack
    }

    fn get_at_position(&self, position: usize) -> Option<&Value> {
        self.stack.values.get(position)
    }
}
