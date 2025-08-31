//! # Memory Management System
//!
//! Simplified memory management system

use crate::vm::handle::HeapHandleId;
use crate::vm::types::MemorySize;
use crate::vm::memory::heap::spaces::MemorySpace;

// Declare submodules
pub mod allocator;
pub mod heap;
pub mod stack;

// Re-export main types
pub use heap::gc::GarbageCollector;
pub use heap::spaces::SpaceCoordinator;
pub use heap::Heap;
pub use heap::ObjectType;
pub use stack::Stack;

/// Memory management system that orchestrates all memory operations
pub struct MemoryManager {
    /// Garbage collector for automatic memory management
    gc: GarbageCollector,
    /// Space coordinator for managing different memory spaces
    space_coordinator: SpaceCoordinator,
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
        let mut gc = GarbageCollector::new();
        let mut space_coordinator = SpaceCoordinator::new();

        // Register default spaces
        space_coordinator.register_space(
            heap::spaces::SpaceType::NewSpace,
            Box::new(heap::spaces::NewSpace::new(64 * 1024 * 1024)),
        );
        space_coordinator.register_space(
            heap::spaces::SpaceType::OldSpace,
            Box::new(heap::spaces::OldSpace::new(128 * 1024 * 1024)),
        );
        space_coordinator.register_space(
            heap::spaces::SpaceType::LargeObjectSpace,
            Box::new(heap::spaces::LargeObjectSpace::new(256 * 1024 * 1024)),
        );
        space_coordinator.register_space(
            heap::spaces::SpaceType::CodeSpace,
            Box::new(heap::spaces::CodeSpace::new(32 * 1024 * 1024)),
        );
        space_coordinator.register_space(
            heap::spaces::SpaceType::CellSpace,
            Box::new(heap::spaces::CellSpace::new(1024)),
        );

        // Register spaces with GC
        for space_type in space_coordinator.get_spaces() {
            let space: Box<dyn MemorySpace> = match space_type {
                heap::spaces::SpaceType::NewSpace => Box::new(heap::spaces::NewSpace::new(64 * 1024 * 1024)),
                heap::spaces::SpaceType::OldSpace => Box::new(heap::spaces::OldSpace::new(128 * 1024 * 1024)),
                heap::spaces::SpaceType::LargeObjectSpace => {
                    Box::new(heap::spaces::LargeObjectSpace::new(256 * 1024 * 1024))
                }
                heap::spaces::SpaceType::CodeSpace => Box::new(heap::spaces::CodeSpace::new(32 * 1024 * 1024)),
                heap::spaces::SpaceType::CellSpace => Box::new(heap::spaces::CellSpace::new(1024)),
            };
            gc.register_space(space_type, space);
        }

        Self {
            gc,
            space_coordinator,
            stack: Stack::new(),
            stats: MemoryStats::default(),
        }
    }

    /// Allocate memory using space coordinator
    pub fn allocate(
        &mut self,
        size: MemorySize,
        object_type: heap::spaces::ObjectType,
    ) -> Result<HeapHandleId, String> {
        if let Some(handle) = self.space_coordinator.allocate(size, object_type) {
            Ok(handle)
        } else {
            Err("Failed to allocate memory".to_string())
        }
    }

    /// Deallocate memory using space coordinator
    pub fn deallocate(&mut self, handle: HeapHandleId) -> bool {
        self.space_coordinator.deallocate(handle)
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

    /// Get a mutable reference to the garbage collector
    pub fn gc_mut(&mut self) -> &mut GarbageCollector {
        &mut self.gc
    }

    /// Get a reference to the space coordinator
    pub fn space_coordinator(&self) -> &SpaceCoordinator {
        &self.space_coordinator
    }

    /// Get a mutable reference to the space coordinator
    pub fn space_coordinator_mut(&mut self) -> &mut SpaceCoordinator {
        &mut self.space_coordinator
    }

    /// Get a reference to the stack
    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    /// Get a mutable reference to the stack
    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.stack
    }

    /// Perform garbage collection
    pub fn collect(&mut self) -> Result<(), String> {
        if let Some(collection_type) = self.gc.should_collect() {
            let result = self.gc.collect();
            if !result.success {
                return Err(result.error.unwrap_or_else(|| "GC failed".to_string()));
            }
        }
        Ok(())
    }

    /// Update memory statistics
    pub fn update_stats(&mut self) {
        let total_allocated = self.space_coordinator.get_total_memory_usage();
        let total_free = self.space_coordinator.get_total_free_memory();
        let total_memory = total_allocated + total_free;

        self.stats.total_memory = total_memory;
        self.stats.allocated_memory = total_allocated;
        self.stats.free_memory = total_free;

        if total_memory.bytes() > 0 {
            self.stats.heap_efficiency =
                total_allocated.bytes() as f64 / total_memory.bytes() as f64;
        }
    }

    /// Get memory statistics
    pub fn get_stats(&self) -> &MemoryStats {
        &self.stats
    }

    /// Get heap information
    pub fn heap_info(&self) -> String {
        let total_allocated = self.space_coordinator.get_total_memory_usage();
        let total_free = self.space_coordinator.get_total_free_memory();
        format!(
            "Heap: {} allocated, {} free",
            total_allocated.as_usize(),
            total_free.as_usize()
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
