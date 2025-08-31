//! # Memory Spaces
//!
//! Specialized memory spaces for different object types and allocation strategies.
//!
//! ## Memory Spaces
//!
//! - **New Space**: Young generation with copying GC
//! - **Old Space**: Old generation with mark & sweep GC
//! - **Large Object Space**: Objects > 1MB
//! - **Code Space**: Compiled bytecode
//! - **Cell Space**: Small objects (≤ 16 bytes)

pub mod cell_space;
pub mod code_space;
pub mod coordinator;
pub mod large_space;
pub mod new_space;
pub mod old_space;

pub use cell_space::CellSpace;
pub use code_space::CodeSpace;
pub use coordinator::{AllocationStrategies, PromotionPolicies, SpaceCoordinator};
pub use large_space::LargeObjectSpace;
pub use new_space::NewSpace;
pub use old_space::OldSpace;

use crate::vm::handle::HeapHandleId;
use crate::vm::types::MemorySize;
use crate::vm::value::Value;

/// Trait for memory spaces
pub trait MemorySpace {
    /// Allocate memory in this space
    fn allocate(&mut self, size: MemorySize) -> Option<HeapHandleId>;

    /// Deallocate memory in this space
    fn deallocate(&mut self, handle: HeapHandleId) -> bool;

    /// Check if allocation is possible
    fn can_allocate(&self, size: MemorySize) -> bool;

    /// Get total allocated memory
    fn total_allocated(&self) -> MemorySize;

    /// Get total free memory
    fn total_free(&self) -> MemorySize;

    /// Get space statistics
    fn stats(&self) -> SpaceStats;

    /// Get space type
    fn space_type(&self) -> SpaceType;

    /// Extract object data for promotion
    fn extract_object(&mut self, handle: HeapHandleId) -> Option<Value>;

    /// Allocate object with existing data
    fn allocate_object(&mut self, data: Value) -> Option<HeapHandleId>;
}

/// Memory space type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpaceType {
    NewSpace,
    OldSpace,
    LargeObjectSpace,
    CodeSpace,
    CellSpace,
}

/// Statistics for a memory space
#[derive(Debug, Clone)]
pub struct SpaceStats {
    pub space_type: SpaceType,
    pub total_size: usize,
    pub allocated_size: usize,
    pub free_size: usize,
    pub object_count: usize,
    pub fragmentation_percentage: f64,
    pub allocation_count: usize,
    pub deallocation_count: usize,
}

impl Default for SpaceStats {
    fn default() -> Self {
        Self {
            space_type: SpaceType::NewSpace,
            total_size: 0,
            allocated_size: 0,
            free_size: 0,
            object_count: 0,
            fragmentation_percentage: 0.0,
            allocation_count: 0,
            deallocation_count: 0,
        }
    }
}

/// Memory space manager
pub struct SpaceManager {
    /// New space for young generation
    new_space: NewSpace,
    /// Old space for old generation
    old_space: OldSpace,
    /// Large object space
    large_space: LargeObjectSpace,
    /// Code space
    code_space: CodeSpace,
    /// Cell space for small objects
    cell_space: CellSpace,
    /// Statistics
    stats: ManagerStats,
}

impl SpaceManager {
    /// Create a new space manager
    pub fn new(
        new_space_size: usize,
        old_space_size: usize,
        large_space_size: usize,
        code_space_size: usize,
        cell_count: usize,
    ) -> Self {
        Self {
            new_space: NewSpace::new(new_space_size),
            old_space: OldSpace::new(old_space_size),
            large_space: LargeObjectSpace::new(large_space_size),
            code_space: CodeSpace::new(code_space_size),
            cell_space: CellSpace::new(cell_count),
            stats: ManagerStats::default(),
        }
    }

    /// Allocate memory in the appropriate space
    pub fn allocate(&mut self, size: MemorySize, object_type: ObjectType) -> Option<HeapHandleId> {
        let size_bytes = size.as_usize();

        // Choose space based on size and type
        let result = match (size_bytes, object_type) {
            (size, _) if size <= 16 => {
                // Small objects go to cell space
                self.cell_space.allocate(MemorySize::new(size))
            }
            (size, ObjectType::Code) if size <= 1024 * 1024 => {
                // Code objects go to code space
                self.code_space.allocate(MemorySize::new(size))
            }
            (size, _) if size > 1024 * 1024 => {
                // Large objects go to large object space
                self.large_space.allocate(MemorySize::new(size))
            }
            (size, _) if size <= 1024 * 1024 => {
                // Medium objects go to new space first, then old space
                if let Some(handle) = self.new_space.allocate(MemorySize::new(size)) {
                    Some(handle)
                } else {
                    self.old_space.allocate(MemorySize::new(size))
                }
            }
            _ => None,
        };

        if result.is_some() {
            self.stats.total_allocations += 1;
            self.stats.current_allocations += 1;
        }

        result
    }

    /// Deallocate memory from the appropriate space
    pub fn deallocate(&mut self, handle: HeapHandleId) -> bool {
        // Try each space until we find the right one
        if self.cell_space.deallocate(handle)
            || self.new_space.deallocate(handle)
            || self.old_space.deallocate(handle)
            || self.large_space.deallocate(handle)
            || self.code_space.deallocate(handle)
        {
            self.stats.total_deallocations += 1;
            self.stats.current_allocations = self.stats.current_allocations.saturating_sub(1);
            true
        } else {
            false
        }
    }

    /// Get information about all spaces
    pub fn get_info(&self) -> ManagerInfo {
        ManagerInfo {
            new_space: self.new_space.stats(),
            old_space: self.old_space.stats(),
            large_space: self.large_space.stats(),
            code_space: self.code_space.stats(),
            cell_space: self.cell_space.stats(),
            manager: self.stats.clone(),
        }
    }

    /// Reset new space (after minor GC)
    pub fn reset_new_space(&mut self) {
        self.new_space.reset();
    }

    /// Perform minor garbage collection
    pub fn minor_gc(&mut self) -> GcStats {
        self.new_space.collect()
    }

    /// Perform major garbage collection
    pub fn major_gc(&mut self) -> GcStats {
        let old_stats = self.old_space.collect();
        let large_stats = self.large_space.collect();
        let code_stats = self.code_space.collect();

        GcStats {
            objects_collected: old_stats.objects_collected
                + large_stats.objects_collected
                + code_stats.objects_collected,
            bytes_freed: old_stats.bytes_freed + large_stats.bytes_freed + code_stats.bytes_freed,
            collection_time: old_stats.collection_time
                + large_stats.collection_time
                + code_stats.collection_time,
        }
    }

    /// Defragment old space
    pub fn defragment_old_space(&mut self) -> DefragmentationStats {
        let defrag_stats = self.old_space.defragment();
        DefragmentationStats {
            duration_micros: 0,
            initial_fragmentation: 0.0,
            final_fragmentation: 0.0,
            blocks_moved: defrag_stats.blocks_merged,
        }
    }

    /// Compact cell space
    pub fn compact_cell_space(&mut self) -> CompactionStats {
        let compact_stats = self.cell_space.compact();
        CompactionStats {
            duration_micros: 0,
            initial_fragmentation: 0.0,
            final_fragmentation: 0.0,
            cells_moved: compact_stats.objects_moved,
        }
    }
}

/// Object type for space selection
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    Object,
    Array,
    Function,
    String,
    Number,
    Boolean,
    Code,
    Large,
}

/// Information about all spaces
#[derive(Debug, Clone)]
pub struct ManagerInfo {
    pub new_space: SpaceStats,
    pub old_space: SpaceStats,
    pub large_space: SpaceStats,
    pub code_space: SpaceStats,
    pub cell_space: SpaceStats,
    pub manager: ManagerStats,
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStats {
    pub total_allocations: usize,
    pub total_deallocations: usize,
    pub current_allocations: usize,
    pub total_allocated_bytes: usize,
    pub total_deallocated_bytes: usize,
    pub peak_allocated_bytes: usize,
}

impl Default for ManagerStats {
    fn default() -> Self {
        Self {
            total_allocations: 0,
            total_deallocations: 0,
            current_allocations: 0,
            total_allocated_bytes: 0,
            total_deallocated_bytes: 0,
            peak_allocated_bytes: 0,
        }
    }
}

/// Garbage collection statistics
#[derive(Debug, Clone)]
pub struct GcStats {
    pub objects_collected: usize,
    pub bytes_freed: usize,
    pub collection_time: u64, // microseconds
}

/// Defragmentation statistics
#[derive(Debug, Clone)]
pub struct DefragmentationStats {
    pub duration_micros: u64,
    pub initial_fragmentation: f64,
    pub final_fragmentation: f64,
    pub blocks_moved: usize,
}

/// Compaction statistics
#[derive(Debug, Clone)]
pub struct CompactionStats {
    pub duration_micros: u64,
    pub initial_fragmentation: f64,
    pub final_fragmentation: f64,
    pub cells_moved: usize,
}
