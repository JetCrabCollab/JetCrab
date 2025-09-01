//! # VM Execution Traits
//!
//! Defines the core traits that provide the interface for VM execution components.
//! These traits define the contract that implementations must fulfill for stack
//! operations, heap management, variable management, and instruction execution.
//!
//! ## Core Traits
//!
//! - **StackOperations**: Defines stack manipulation operations
//! - **HeapOperations**: Defines heap memory management operations
//! - **VariableManager**: Defines variable storage and retrieval operations
//! - **InstructionExecutor**: Defines the main execution interface
//!
//! ## Design Principles
//!
//! The traits follow these design principles:
//!
//! - **Generic Implementation**: Use trait bounds for flexibility
//! - **Clear Contracts**: Each trait has a well-defined responsibility
//! - **Error Handling**: All operations return Result types
//! - **Performance**: Optimized for common operations
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::vm::executor::traits::{StackOperations, HeapOperations, VariableManager};
//!
//! struct MyVM<S, H, V>
//! where
//!     S: StackOperations,
//!     H: HeapOperations,
//!     V: VariableManager,
//! {
//!     stack: S,
//!     heap: H,
//!     variables: V,
//! }
//! ```

use crate::vm::compiler::Bytecode;
use crate::vm::handle::HeapHandleId;
use crate::vm::types::{ArgIndex, ArraySize, LocalIndex};
use crate::vm::value::Value;

/// Defines operations for manipulating the VM's operand stack
///
/// This trait provides the interface for all stack operations including
/// pushing, popping, and inspecting stack contents.
pub trait StackOperations {
    /// Pushes a value onto the top of the stack
    ///
    /// # Arguments
    /// * `value` - The value to push onto the stack
    fn push(&mut self, value: Value);

    /// Pops a value from the top of the stack
    ///
    /// # Returns
    /// * `Some(value)` if the stack is not empty
    /// * `None` if the stack is empty
    fn pop(&mut self) -> Option<Value>;

    /// Peeks at the top value without removing it
    ///
    /// # Returns
    /// * `Some(&value)` if the stack is not empty
    /// * `None` if the stack is empty
    fn peek(&self) -> Option<&Value>;

    /// Clears all values from the stack
    fn clear(&mut self);

    /// Gets the current number of values on the stack
    ///
    /// # Returns
    /// * The number of values currently on the stack
    fn len(&self) -> usize;

    /// Checks if the stack is empty
    ///
    /// # Returns
    /// * `true` if the stack contains no values
    /// * `false` if the stack contains at least one value
    fn is_empty(&self) -> bool;

    /// Gets a mutable reference to the underlying stack
    ///
    /// This allows direct access to the stack for advanced operations
    /// that cannot be expressed through the trait interface.
    ///
    /// # Returns
    /// * A mutable reference to the underlying stack
    fn stack_mut(&mut self) -> &mut crate::vm::memory::stack::Stack;

    /// Gets the current size of the stack
    ///
    /// This is a convenience method that delegates to `len()`.
    ///
    /// # Returns
    /// * The number of values currently on the stack
    fn size(&self) -> usize {
        self.len()
    }

    /// Gets a value at a specific position in the stack
    ///
    /// # Arguments
    /// * `position` - The position from the bottom of the stack (0-based)
    ///
    /// # Returns
    /// * `Some(&value)` if the position is valid
    /// * `None` if the position is out of bounds
    fn get_at_position(&self, position: usize) -> Option<&Value>;
}

/// Defines operations for managing the VM's heap memory
///
/// This trait provides the interface for all heap operations including
/// object allocation, property access, and memory management.
pub trait HeapOperations {
    /// Allocates a new object on the heap
    ///
    /// # Returns
    /// * A handle to the newly allocated object
    fn alloc_object(&mut self) -> HeapHandleId;

    /// Allocates a new array on the heap
    ///
    /// # Returns
    /// * A handle to the newly allocated array
    fn alloc_array(&mut self) -> HeapHandleId;

    /// Allocates a new function on the heap
    ///
    /// # Arguments
    /// * `bytecode` - The function's bytecode
    /// * `arg_count` - The number of arguments the function accepts
    /// * `local_count` - The number of local variables the function uses
    ///
    /// # Returns
    /// * A handle to the newly allocated function
    fn alloc_function(
        &mut self,
        bytecode: Bytecode,
        arg_count: ArgIndex,
        local_count: LocalIndex,
    ) -> HeapHandleId;

    /// Gets a property value from an object
    ///
    /// # Arguments
    /// * `handle` - The handle to the object
    /// * `key` - The name of the property to retrieve
    ///
    /// # Returns
    /// * `Some(&value)` if the property exists
    /// * `None` if the property does not exist
    fn get_object_property(&self, handle: HeapHandleId, key: &str) -> Option<&Value>;

    /// Sets a property value on an object
    ///
    /// # Arguments
    /// * `handle` - The handle to the object
    /// * `key` - The name of the property to set
    /// * `value` - The value to assign to the property
    fn set_object_property(&mut self, handle: HeapHandleId, key: String, value: Value);

    /// Sets an element in an array
    ///
    /// # Arguments
    /// * `handle` - The handle to the array
    /// * `index` - The index where to set the element
    /// * `value` - The value to assign to the array element
    fn set_array_element(&mut self, handle: HeapHandleId, index: ArraySize, value: Value);

    /// Gets a reference to the underlying heap
    ///
    /// This allows direct access to the heap for advanced operations
    /// that cannot be expressed through the trait interface.
    ///
    /// # Returns
    /// * A reference to the underlying heap
    fn get_heap(&self) -> &crate::vm::memory::heap::Heap;

    /// Gets an element from an array
    ///
    /// # Arguments
    /// * `handle` - The handle to the array
    /// * `index` - The index of the element to retrieve
    ///
    /// # Returns
    /// * `Some(&value)` if the element exists at the given index
    /// * `None` if the index is out of bounds
    fn get_array_element(&self, _handle: HeapHandleId, _index: ArraySize) -> Option<&Value> {
        None
    }

    /// Checks if an object has a specific property
    ///
    /// # Arguments
    /// * `handle` - The handle to the object
    /// * `key` - The name of the property to check
    ///
    /// # Returns
    /// * `true` if the property exists
    /// * `false` if the property does not exist
    fn has_object_property(&self, handle: HeapHandleId, key: &str) -> bool {
        self.get_object_property(handle, key).is_some()
    }

    /// Gets the length of an array
    ///
    /// # Arguments
    /// * `handle` - The handle to the array
    ///
    /// # Returns
    /// * The number of elements in the array
    fn get_array_length(&self, _handle: HeapHandleId) -> usize {
        0
    }
}

/// Defines operations for managing VM variables
///
/// This trait provides the interface for all variable operations including
/// local variables, global variables, and function arguments.
pub trait VariableManager {
    /// Gets a local variable by index
    ///
    /// # Arguments
    /// * `idx` - The index of the local variable
    ///
    /// # Returns
    /// * `Some(&value)` if the variable exists
    /// * `None` if the variable does not exist
    fn get_local(&self, idx: usize) -> Option<&Value>;

    /// Sets a local variable by index
    ///
    /// # Arguments
    /// * `idx` - The index of the local variable
    /// * `value` - The value to assign to the variable
    fn set_local(&mut self, idx: usize, value: Value);

    /// Gets a global variable by index
    ///
    /// # Arguments
    /// * `idx` - The index of the global variable
    ///
    /// # Returns
    /// * `Some(&value)` if the variable exists
    /// * `None` if the variable does not exist
    fn get_global(&self, idx: usize) -> Option<&Value>;

    /// Sets a global variable by index
    ///
    /// # Arguments
    /// * `idx` - The index of the global variable
    /// * `value` - The value to assign to the variable
    fn set_global(&mut self, idx: usize, value: Value);

    /// Gets a mutable reference to a local variable by index
    ///
    /// # Arguments
    /// * `idx` - The index of the local variable
    ///
    /// # Returns
    /// * `Some(&mut value)` if the variable exists
    /// * `None` if the variable does not exist
    fn get_local_mut(&mut self, idx: usize) -> Option<&mut Value>;

    /// Gets a mutable reference to a global variable by index
    ///
    /// # Arguments
    /// * `idx` - The index of the global variable
    ///
    /// # Returns
    /// * `Some(&mut value)` if the variable exists
    /// * `None` if the variable does not exist
    fn get_global_mut(&mut self, idx: usize) -> Option<&mut Value>;

    /// Gets a function argument by index
    ///
    /// This is a convenience method that delegates to `get_local()`.
    ///
    /// # Arguments
    /// * `idx` - The index of the argument
    ///
    /// # Returns
    /// * `Some(&value)` if the argument exists
    /// * `None` if the argument does not exist
    fn get_argument(&self, idx: usize) -> Option<&Value> {
        self.get_local(idx)
    }

    /// Sets a function argument by index
    ///
    /// This is a convenience method that delegates to `set_local()`.
    ///
    /// # Arguments
    /// * `idx` - The index of the argument
    /// * `value` - The value to assign to the argument
    fn set_argument(&mut self, idx: usize, value: Value) {
        self.set_local(idx, value);
    }

    /// Gets a variable by name
    ///
    /// # Arguments
    /// * `name` - The name of the variable
    ///
    /// # Returns
    /// * `Some(&value)` if the variable exists
    /// * `None` if the variable does not exist
    fn get_variable(&self, name: &str) -> Option<&Value>;

    /// Sets a variable by name
    ///
    /// # Arguments
    /// * `name` - The name of the variable
    /// * `value` - The value to assign to the variable
    fn set_variable(&mut self, name: &str, value: Value);
}

/// Defines the main interface for VM instruction execution
///
/// This trait provides the core execution interface that the VM uses
/// to run bytecode programs.
pub trait InstructionExecutor {
    /// Executes a bytecode program
    ///
    /// # Arguments
    /// * `bytecode` - The bytecode to execute
    /// * `constants` - The constant pool for the bytecode
    ///
    /// # Returns
    /// * `Ok(())` on successful execution
    /// * `Err(ExecutionError)` if execution fails
    fn execute(
        &mut self,
        bytecode: &Bytecode,
        constants: &[Value],
    ) -> Result<(), crate::vm::executor::error_handler::ExecutionError>;
}
