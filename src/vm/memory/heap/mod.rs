//! Advanced Generational Heap - High-performance memory management system
//! 
//! This module implements a V8-style generational heap with:
//! - Semi-space allocation for young generation
//! - Mark & sweep for old generation
//! - Specialized memory spaces
//! - Advanced garbage collection strategies
//! - Object shapes and string interning for optimization

pub mod spaces;
pub mod generational;
pub mod optimized_arrays;
pub mod object_shapes;
pub mod string_interning;

pub use allocation::{Allocator, SmartAllocator, AllocationStats, AllocationError};
pub use spaces::{MemorySpace, SpaceManager, SpaceType, SpaceStats, ObjectType};
pub use generational::{
    GenerationalHeap, SemiSpace, PointerSpace, DataSpace, LargeObjectSpace, 
    CodeSpace, CellSpace, PropertyCellSpace, MapSpace, ObjectShape, 
    PropertyDescriptor, PropertyAttributes, TypeInfo, OptimizedObject, 
    OldObject, LargeObject, CodeObject, Cell, ShapeId, StringId, 
    HeapStats, StringTable, ShapeTable, GcStats
};
pub use optimized_arrays::{OptimizedArray, ArrayRepresentation, ElementType};
pub use object_shapes::{ObjectShape, ShapeId, PropertyDescriptor, PropertyAttributes, PropertyType, ShapeTransitionManager};
pub use string_interning::{StringId, InternedString, StringInterningManager, StringInternTable, StringInternStats};

use crate::vm::handle::HeapHandleId;
use crate::vm::types::MemorySize;

/// Main heap implementation combining generational heap, allocators, and spaces
pub struct Heap {
    /// Generational heap core
    generational_heap: GenerationalHeap,
    /// Smart allocator for choosing best allocation strategy
    allocator: SmartAllocator,
    /// Space manager for specialized memory spaces
    space_manager: SpaceManager,
    /// Object shapes manager for property optimization
    shape_manager: ShapeTransitionManager,
    /// String interning manager for string deduplication
    string_manager: StringInterningManager,
    /// Heap statistics
    stats: HeapStats,
}

impl Heap {
    /// Create a new heap with default configuration
    pub fn new() -> Self {
        Self {
            generational_heap: GenerationalHeap::new(),
            allocator: SmartAllocator::new(),
            space_manager: SpaceManager::new(),
            shape_manager: ShapeTransitionManager::new(),
            string_manager: StringInterningManager::default(),
            stats: HeapStats::default(),
        }
    }

    /// Create a new heap with custom configuration
    pub fn with_config(
        initial_size: MemorySize,
        max_size: MemorySize,
        new_space_size: MemorySize,
        old_space_size: MemorySize,
    ) -> Self {
        Self {
            generational_heap: GenerationalHeap::with_config(initial_size, max_size, new_space_size, old_space_size),
            allocator: SmartAllocator::new(),
            space_manager: SpaceManager::new(),
            shape_manager: ShapeTransitionManager::new(),
            string_manager: StringInterningManager::default(),
            stats: HeapStats::default(),
        }
    }

    /// Allocate memory using the smart allocator
    pub fn allocate(&mut self, size: MemorySize, object_type: ObjectType) -> Result<HeapHandleId, AllocationError> {
        let handle = self.allocator.allocate(size, object_type)?;
        self.stats.total_allocations += 1;
        self.stats.total_allocated += size.bytes();
        Ok(handle)
    }

    /// Deallocate memory
    pub fn deallocate(&mut self, handle: HeapHandleId) -> Result<(), AllocationError> {
        self.allocator.deallocate(handle)?;
        self.stats.total_deallocations += 1;
        Ok(())
    }

    /// Get heap statistics
    pub fn stats(&self) -> &HeapStats {
        &self.stats
    }

    /// Get memory usage statistics
    pub fn memory_stats(&self) -> HeapMemoryStats {
        let total_allocated = self.stats.total_allocated;
        let total_freed = self.stats.total_freed;
        let current_usage = total_allocated.saturating_sub(total_freed);
        let fragmentation = self.calculate_fragmentation();

        HeapMemoryStats {
            total_allocated,
            total_freed,
            current_usage,
            fragmentation,
            allocation_count: self.stats.total_allocations,
            deallocation_count: self.stats.total_deallocations,
        }
    }

    /// Calculate memory fragmentation percentage
    fn calculate_fragmentation(&self) -> f64 {
        if self.stats.total_allocated == 0 {
            return 0.0;
        }

        let fragmented = self.stats.total_allocated.saturating_sub(self.stats.total_freed);
        (fragmented as f64 / self.stats.total_allocated as f64) * 100.0
    }

    /// Force garbage collection
    pub fn force_gc(&mut self) {
        self.generational_heap.force_gc();
        self.stats.gc_count += 1;
    }

    /// Minor garbage collection
    pub fn minor_gc(&mut self) -> GcStats {
        let stats = self.generational_heap.minor_gc();
        self.stats.minor_gc_count += 1;
        stats
    }

    /// Major garbage collection
    pub fn major_gc(&mut self, roots: &[HeapHandleId]) -> GcStats {
        let stats = self.generational_heap.major_gc(roots);
        self.stats.major_gc_count += 1;
        stats
    }

    /// Get object shape manager
    pub fn shape_manager(&self) -> &ShapeTransitionManager {
        &self.shape_manager
    }

    /// Get mutable object shape manager
    pub fn shape_manager_mut(&mut self) -> &mut ShapeTransitionManager {
        &mut self.shape_manager
    }

    /// Get string interning manager
    pub fn string_manager(&self) -> &StringInterningManager {
        &self.string_manager
    }

    /// Get mutable string interning manager
    pub fn string_manager_mut(&mut self) -> &mut StringInterningManager {
        &mut self.string_manager
    }

    /// Get space manager
    pub fn space_manager(&self) -> &SpaceManager {
        &self.space_manager
    }

    /// Get mutable space manager
    pub fn space_manager_mut(&mut self) -> &mut SpaceManager {
        &mut self.space_manager
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}

/// Heap memory statistics
#[derive(Debug, Clone)]
pub struct HeapMemoryStats {
    /// Total memory allocated
    pub total_allocated: usize,
    /// Total memory freed
    pub total_freed: usize,
    /// Current memory usage
    pub current_usage: usize,
    /// Memory fragmentation percentage
    pub fragmentation: f64,
    /// Total allocation count
    pub allocation_count: u64,
    /// Total deallocation count
    pub deallocation_count: u64,
}

/// Re-export types from sub-modules
pub use crate::vm::handle::HeapHandleId;
