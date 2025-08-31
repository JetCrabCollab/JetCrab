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
