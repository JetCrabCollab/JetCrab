//! # Heap Manager
//!
//! Provides concrete implementation of heap operations for the VM executor.
//! Manages object allocation, property access, and memory management
//! through the `HeapOperations` trait implementation.
//!
//! ## Overview
//!
//! The heap manager wraps the low-level `Heap` implementation and provides
//! a high-level interface for heap operations including:
//!
//! - **Object Allocation**: Creating objects, arrays, and functions
//! - **Property Access**: Getting and setting object properties
//! - **Array Operations**: Manipulating array elements
//! - **Memory Management**: Interfacing with garbage collection
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::vm::executor::heap_manager::HeapManager;
//! use jetcrab::vm::executor::traits::HeapOperations;
//! use jetcrab::vm::value::Value;
//!
//! let mut heap_manager = HeapManager::new();
//! let object_id = heap_manager.alloc_object();
//! heap_manager.set_object_property(object_id, "key".to_string(), Value::Number(42.0));
//! ```

use super::HeapOperations;
use crate::vm::bytecode::Bytecode;
use crate::vm::handle::HeapHandleId;
use crate::vm::heap::Heap;
use crate::vm::types::ArraySize;
use crate::vm::value::Value;

/// Concrete implementation of heap operations for the VM
///
/// Wraps the low-level Heap and provides high-level heap management
/// functionality for object allocation and memory operations.
pub struct HeapManager {
    heap: Heap,
}

impl HeapManager {
    /// Creates a new heap manager with an empty heap
    ///
    /// Initializes the heap manager with a fresh heap ready
    /// for object allocation and memory management.
    pub fn new() -> Self {
        Self { heap: Heap::new() }
    }

    /// Gets read-only access to the underlying heap
    ///
    /// Provides access to the heap for inspection and debugging
    /// without allowing modifications.
    pub fn heap(&self) -> &Heap {
        &self.heap
    }

    /// Gets mutable access to the underlying heap
    ///
    /// Provides direct access to the heap for advanced operations
    /// and testing purposes.
    pub fn heap_mut(&mut self) -> &mut Heap {
        &mut self.heap
    }
}

impl HeapOperations for HeapManager {
    fn alloc_object(&mut self) -> HeapHandleId {
        self.heap.alloc_object()
    }

    fn alloc_array(&mut self) -> HeapHandleId {
        self.heap.alloc_array()
    }

    fn alloc_function(
        &mut self,
        bytecode: Bytecode,
        arg_count: crate::vm::types::ArgIndex,
        local_count: crate::vm::types::LocalIndex,
    ) -> HeapHandleId {
        self.heap.alloc_function(bytecode, arg_count, local_count)
    }

    fn get_object_property(&self, handle: HeapHandleId, key: &str) -> Option<&Value> {
        self.heap.get_object_property(handle, key)
    }

    fn set_object_property(&mut self, handle: HeapHandleId, key: String, value: Value) {
        self.heap.set_object_property(handle, key, value);
    }

    fn set_array_element(&mut self, handle: HeapHandleId, index: ArraySize, value: Value) {
        self.heap.set_array_element(handle, index, value);
    }

    fn get_heap(&self) -> &crate::vm::heap::Heap {
        &self.heap
    }
}
