//! # Memory Management System
//!
//! Simplified memory management system

use crate::vm::handle::HeapHandleId;
use crate::vm::types::MemorySize;

// Declare submodules
pub mod heap;
pub mod stack;
pub mod allocator;

// Re-export main types
pub use heap::Heap;
pub use stack::Stack;
pub use heap::ObjectType;

/// Memory management system that orchestrates all memory operations
pub struct MemoryManager {
    /// Heap for dynamic memory allocation
    heap: Heap,
    /// Stack for local variables and function calls
    stack: Stack,
    /// Memory statistics
    stats: MemoryStats,
}

/// Memory statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_memory: MemorySize,
    pub allocated_memory: MemorySize,
    pub free_memory: MemorySize,
    pub heap_efficiency: f64,
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self {
            total_memory: MemorySize::new(0),
            allocated_memory: MemorySize::new(0),
            free_memory: MemorySize::new(0),
            heap_efficiency: 0.0,
        }
    }
}

impl MemoryManager {
    /// Create a new memory manager
    pub fn new() -> Self {
        Self {
            heap: Heap::new(),
            stack: Stack::new(),
            stats: MemoryStats::default(),
        }
    }

    /// Allocate memory on the heap
    pub fn allocate(&mut self, size: MemorySize) -> Result<HeapHandleId, String> {
        let result = self
            .heap
            .allocate(size, ObjectType::Object);
        match result {
            Ok(handle) => Ok(handle),
            Err(_) => Err("Failed to allocate memory".to_string()),
        }
    }

    /// Deallocate memory from the heap
    pub fn deallocate(&mut self, handle: HeapHandleId) -> bool {
        let result = self.heap.deallocate(handle);
        result.is_ok()
    }

    /// Push a value onto the stack
    pub fn push(&mut self, value: crate::vm::value::Value) -> Result<(), String> {
        self.stack.push(value);
        Ok(())
    }

    /// Pop a value from the stack
    pub fn pop(&mut self) -> Option<crate::vm::value::Value> {
        self.stack.pop()
    }

    /// Get a mutable reference to the heap
    pub fn heap_mut(&mut self) -> &mut Heap {
        &mut self.heap
    }

    /// Get a reference to the stack
    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    /// Get a mutable reference to the stack
    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.stack
    }

    /// Get memory statistics
    pub fn get_stats(&self) -> &MemoryStats {
        &self.stats
    }

    /// Get heap information
    pub fn heap_info(&self) -> String {
        format!(
            "Heap: {} allocated, {} free",
            self.heap.total_allocated().as_usize(),
            self.heap.total_free().as_usize()
        )
    }

    /// Get stack information
    pub fn stack_info(&self) -> String {
        format!("Stack: {} items", self.stack.len())
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}
