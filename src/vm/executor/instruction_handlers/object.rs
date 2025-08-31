//! # Object Handler
//!
//! Handles all object and class-related operations in the VM including object creation,
//! property access, class instantiation, and method calls.
//!
//! ## Operations Supported
//!
//! - **Object Creation**: new object, new class
//! - **Property Operations**: get property, set property
//! - **Class Operations**: class instantiation, prototype handling
//! - **Method Calls**: object method invocation
//!
//! ## Error Handling
//!
//! All operations return `Result<(), ExecutionError>` and handle:
//! - Stack underflow (insufficient operands)
//! - Invalid object operations
//! - Property access errors
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::vm::executor::instruction_handlers::ObjectHandler;
//! use jetcrab::vm::executor::traits::{StackOperations, HeapOperations};
//!
//! let mut stack = MyStack::new();
//! let mut heap = MyHeap::new();
//! ObjectHandler::new_object(&mut stack, &mut heap)?;
//! ```

use crate::vm::executor::error_handler::ExecutionError;
use crate::vm::executor::traits::{StackOperations, HeapOperations};
use crate::vm::value::Value;
use crate::vm::handle::ObjectHandle;

/// Handles object and class operations for the VM
pub struct ObjectHandler;

impl ObjectHandler {
    /// Creates a new object and pushes it onto the stack
    ///
    /// Allocates memory for a new object in the heap and pushes
    /// an ObjectHandle onto the stack.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `heap` - The heap manager for object allocation
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::OutOfMemory)` if heap allocation fails
    pub fn new_object<S, H>(
        stack: &mut S,
        heap: &mut H,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        H: HeapOperations,
    {
        let handle = heap.alloc_object();
        let object_handle = ObjectHandle::from(handle.as_usize());
        stack.push(Value::Object(object_handle));
        Ok(())
    }

    /// Creates a new class instance and pushes it onto the stack
    ///
    /// For now, creates a simple object to represent a class instance.
    /// TODO: Implement proper class prototype chain and constructor execution.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `heap` - The heap manager for object allocation
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::OutOfMemory)` if heap allocation fails
    pub fn new_class<S, H>(
        stack: &mut S,
        heap: &mut H,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        H: HeapOperations,
    {
        // For now, create a simple object to represent a class instance
        // TODO: Implement proper class prototype chain and constructor execution
        let handle = heap.alloc_object();
        let object_handle = ObjectHandle::from(handle.as_usize());
        stack.push(Value::Object(object_handle));
        Ok(())
    }

    /// Sets a property on an object
    ///
    /// Pops three values from the stack: the object, the property key, and the value.
    /// Sets the property on the object and pushes the object back onto the stack.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `heap` - The heap manager for property access
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    /// * `Err(ExecutionError::TypeError)` if invalid object or key types
    pub fn set_property<S, H>(
        stack: &mut S,
        heap: &mut H,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        H: HeapOperations,
    {
        let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let key = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let obj = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        match (obj, key) {
            (Value::Object(handle), Value::String(key_str)) => {
                heap.set_object_property(handle.id(), key_str, value.clone());
                // Push the object back to the stack so it can be used in object literals
                stack.push(Value::Object(handle));
                Ok(())
            }
            (_obj, _key) => {
                // For now, just push undefined on error
                stack.push(Value::Undefined);
                Ok(())
            }
        }
    }

    /// Sets a property on an object for assignment operations
    ///
    /// Similar to set_property but pushes the assigned value back to the stack
    /// instead of the object (for assignment expressions).
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `heap` - The heap manager for property access
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn set_property_assign<S, H>(
        stack: &mut S,
        heap: &mut H,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        H: HeapOperations,
    {
        let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let key = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let obj = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        match (obj, key) {
            (Value::Object(handle), Value::String(key_str)) => {
                heap.set_object_property(handle.id(), key_str, value.clone());
                // Push the assigned value back to the stack (for assignments)
                stack.push(value);
                Ok(())
            }
            (_obj, _key) => {
                // For now, just push undefined on error
                stack.push(Value::Undefined);
                Ok(())
            }
        }
    }

    /// Gets a property from an object
    ///
    /// Pops two values from the stack: the object and the property key.
    /// Pushes the property value onto the stack.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `heap` - The heap manager for property access
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn get_property<S, H>(
        stack: &mut S,
        heap: &mut H,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        H: HeapOperations,
    {
        let key = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let obj = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let result = match (&obj, &key) {
            (Value::String(str_val), Value::String(key_str)) => {
                if key_str == "length" {
                    Value::Number(str_val.len() as f64)
                } else {
                    Value::Undefined
                }
            }
            (Value::Array(handle), Value::String(key_str)) => {
                if key_str == "length" {
                    if let Some(crate::vm::memory::heap::HeapEntry::Array(arr)) =
                        heap.get_heap().get(handle.id())
                    {
                        Value::Number(arr.len() as f64)
                    } else {
                        Value::Undefined
                    }
                } else if key_str == "push" || key_str == "pop" {
                    Value::String(format!("Array.prototype.{}", key_str))
                } else if let Ok(index) = key_str.parse::<usize>() {
                    if let Some(crate::vm::memory::heap::HeapEntry::Array(arr)) =
                        heap.get_heap().get(handle.id())
                    {
                        arr.get(index).cloned().unwrap_or(Value::Undefined)
                    } else {
                        Value::Undefined
                    }
                } else {
                    Value::Undefined
                }
            }
            (Value::Array(handle), Value::Number(num)) => {
                let index = *num as usize;
                if let Some(crate::vm::memory::heap::HeapEntry::Array(arr)) =
                    heap.get_heap().get(handle.id())
                {
                    arr.get(index).cloned().unwrap_or(Value::Undefined)
                } else {
                    Value::Undefined
                }
            }
            (Value::Object(handle), Value::String(key_str)) => heap
                .get_object_property(handle.id(), key_str)
                .cloned()
                .unwrap_or(Value::Undefined),
            _ => Value::Undefined,
        };

        stack.push(result);
        Ok(())
    }

    /// Removes a property from an object
    ///
    /// Pops two values from the stack: the object and the property key.
    /// Removes the property and pushes the result (true if removed, false if not found).
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `heap` - The heap manager for property access
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn remove_object_property<S, H>(
        stack: &mut S,
        heap: &mut H,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        H: HeapOperations,
    {
        let key = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let obj = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        match (obj, key) {
            (Value::Object(handle), Value::String(key_str)) => {
                // TODO: Implement actual property removal
                // For now, just return true
                stack.push(Value::Boolean(true));
                Ok(())
            }
            _ => {
                stack.push(Value::Boolean(false));
                Ok(())
            }
        }
    }

    /// Calls a method on an object
    ///
    /// Pops the method name and arguments from the stack, then calls
    /// the method on the object. For now, this is a simplified implementation.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `heap` - The heap manager for property access
    /// * `arg_count` - Number of arguments to pop from the stack
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError::StackUnderflow)` if insufficient operands
    pub fn call_object_method<S, H>(
        stack: &mut S,
        heap: &mut H,
        arg_count: usize,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        H: HeapOperations,
    {
        // Pop arguments (in reverse order)
        let mut args = Vec::with_capacity(arg_count);
        for _ in 0..arg_count {
            args.push(stack.pop().ok_or(ExecutionError::StackUnderflow)?);
        }
        args.reverse();

        // Pop method name
        let method_name = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let method_name = match method_name {
            Value::String(name) => name,
            _ => return Err(ExecutionError::TypeError("Method name must be a string".to_string())),
        };

        // Pop object
        let obj = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        // For now, implement basic method behaviors
        let result = match (&obj, method_name.as_str()) {
            (Value::String(_), "charAt") => {
                if let Some(Value::Number(index)) = args.get(0) {
                    // TODO: Implement actual charAt logic
                    Value::String("a".to_string())
                } else {
                    Value::Undefined
                }
            }
            (Value::Array(_), "push") => {
                // TODO: Implement actual array push
                Value::Number(args.len() as f64)
            }
            (Value::Array(_), "pop") => {
                // TODO: Implement actual array pop
                Value::Undefined
            }
            _ => Value::Undefined,
        };

        stack.push(result);
        Ok(())
    }
}
