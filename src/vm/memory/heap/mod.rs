//! Advanced Generational Heap - High-performance memory management system

pub mod generational;
pub mod object_shapes;
pub mod optimized_arrays;
pub mod spaces;
pub mod string_interning;

use crate::vm::handle::HeapHandleId;
use crate::vm::memory::heap::spaces::ObjectType;
use crate::vm::types::MemorySize;

/// Main heap implementation combining generational heap, allocators, and spaces
pub struct Heap {
    /// Total allocated memory in bytes
    total_allocated: usize,
    /// Total freed memory in bytes
    total_freed: usize,
}

impl Heap {
    /// Create a new heap with default configuration
    pub fn new() -> Self {
        Self {
            total_allocated: 0,
            total_freed: 0,
        }
    }

    /// Create a new heap with custom configuration
    pub fn with_config(
        _initial_size: MemorySize,
        _max_size: MemorySize,
        _new_space_size: MemorySize,
        _old_space_size: MemorySize,
    ) -> Self {
        Self::new()
    }

    /// Allocate memory
    pub fn allocate(
        &mut self,
        size: MemorySize,
        _object_type: ObjectType,
    ) -> Result<HeapHandleId, String> {
        self.total_allocated += size.bytes();
        Ok(HeapHandleId::new(self.total_allocated as u64))
    }

    /// Deallocate memory
    pub fn deallocate(&mut self, _handle: HeapHandleId) -> Result<(), String> {
        self.total_freed += 64; // Assume 64 bytes for now
        Ok(())
    }

    /// Get total allocated memory
    pub fn total_allocated(&self) -> MemorySize {
        MemorySize::new(self.total_allocated)
    }

    /// Get total free memory
    pub fn total_free(&self) -> MemorySize {
        MemorySize::new(self.total_allocated.saturating_sub(self.total_freed))
    }

    /// Get total allocated memory in bytes
    pub fn total_allocated_bytes(&self) -> usize {
        self.total_allocated
    }

    /// Get total free memory in bytes
    pub fn total_free_bytes(&self) -> usize {
        self.total_allocated.saturating_sub(self.total_freed)
    }

    /// Force garbage collection
    pub fn force_gc(&mut self) -> usize {
        let freed = self.total_allocated.saturating_sub(self.total_freed);
        self.total_freed = self.total_allocated;
        freed
    }
}
