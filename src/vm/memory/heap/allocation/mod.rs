//! # Memory Allocation Strategies
//!
//! Specialized allocation strategies for different memory spaces and object types.
//!
//! ## Allocation Strategies
//!
//! - **Bump Allocator**: Fast O(1) allocation for new space (young generation)
//! - **Free List Allocator**: Efficient reuse for old space (old generation)
//! - **Cell Allocator**: Small object optimization (≤ 16 bytes)
//! - **Pool Allocator**: Fixed-size object allocation
//! - **Segregated Allocator**: Size-class based allocation

pub mod bump;
pub mod cell;
pub mod free_list;

pub use bump::BumpAllocator;
pub use cell::CellAllocator;
pub use free_list::FreeListAllocator;

use crate::vm::types::MemorySize;

/// Trait for memory allocators
pub trait Allocator {
    /// Allocate memory of the specified size
    fn allocate(&mut self, size: MemorySize) -> Option<usize>;

    /// Deallocate memory at the specified address
    fn deallocate(&mut self, address: usize, size: MemorySize) -> bool;

    /// Check if the allocator can allocate the specified size
    fn can_allocate(&self, size: MemorySize) -> bool;

    /// Get total allocated memory
    fn total_allocated(&self) -> MemorySize;

    /// Get total free memory
    fn total_free(&self) -> MemorySize;

    /// Get fragmentation percentage
    fn fragmentation(&self) -> f64;
    
    /// Extract object data for promotion (returns None if not supported)
    fn extract_object(&mut self, _handle: usize) -> Option<crate::vm::value::Value> {
        None
    }
    
    /// Allocate object with existing data (returns None if not supported)
    fn allocate_object(&mut self, _data: crate::vm::value::Value) -> Option<usize> {
        None
    }
    
    /// Get layout information
    fn layout_info(&self) -> LayoutInfo {
        LayoutInfo {
            total_size: self.total_allocated() + self.total_free(),
            allocated_size: self.total_allocated(),
            free_size: self.total_free(),
            fragmentation: self.fragmentation(),
            alignment: 8,
        }
    }
    
    /// Check if allocator is full
    fn is_full(&self) -> bool {
        self.total_free().bytes() == 0
    }
    
    /// Get remaining space
    fn remaining_space(&self) -> MemorySize {
        self.total_free()
    }
    
    /// Get usage percentage
    fn usage_percentage(&self) -> f64 {
        let total = self.total_allocated().bytes() + self.total_free().bytes();
        if total == 0 {
            0.0
        } else {
            (self.total_allocated().bytes() as f64 / total as f64) * 100.0
        }
    }
}

/// Allocation statistics
#[derive(Debug, Clone)]
pub struct AllocationStats {
    pub total_allocations: usize,
    pub total_deallocations: usize,
    pub total_allocated: MemorySize,
    pub total_freed: MemorySize,
    pub peak_usage: MemorySize,
    pub current_usage: MemorySize,
}

/// Allocation error types
#[derive(Debug, Clone)]
pub enum AllocationError {
    OutOfMemory,
    InvalidSize,
    InvalidAddress,
    Fragmentation,
}

/// Compaction statistics
#[derive(Debug, Clone)]
pub struct CompactionStats {
    pub objects_moved: usize,
    pub memory_compacted: MemorySize,
    pub fragmentation_before: f64,
    pub fragmentation_after: f64,
}

/// Defragmentation statistics
#[derive(Debug, Clone)]
pub struct DefragmentationStats {
    pub blocks_merged: usize,
    pub memory_defragmented: MemorySize,
    pub free_blocks_before: usize,
    pub free_blocks_after: usize,
}

/// Layout information for memory spaces
#[derive(Debug, Clone)]
pub struct LayoutInfo {
    pub total_size: MemorySize,
    pub allocated_size: MemorySize,
    pub free_size: MemorySize,
    pub fragmentation: f64,
    pub alignment: usize,
}

/// Memory information for allocators
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total_size: MemorySize,
    pub allocated_size: MemorySize,
    pub free_size: MemorySize,
    pub fragmentation: f64,
    pub block_count: usize,
    pub average_block_size: MemorySize,
}

/// Cell information for cell-based allocators
#[derive(Debug, Clone)]
pub struct CellInfo {
    pub total_cells: usize,
    pub allocated_cells: usize,
    pub free_cells: usize,
    pub cell_size: usize,
    pub fragmentation: f64,
    pub efficiency: f64,
}
