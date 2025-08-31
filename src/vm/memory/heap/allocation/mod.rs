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
pub mod pool;
pub mod segregated;

pub use bump::BumpAllocator;
pub use cell::CellAllocator;
pub use free_list::FreeListAllocator;
pub use pool::PoolAllocator;
pub use segregated::SegregatedAllocator;

use crate::vm::handle::HeapHandleId;
use crate::vm::types::MemorySize;

/// Trait for memory allocators
pub trait Allocator {
    /// Allocate memory of the specified size
    fn allocate(&mut self, size: MemorySize) -> Option<HeapHandleId>;

    /// Deallocate memory at the specified handle
    fn deallocate(&mut self, handle: HeapHandleId) -> bool;

    /// Check if allocation is possible
    fn can_allocate(&self, size: MemorySize) -> bool;

    /// Get total allocated memory
    fn total_allocated(&self) -> MemorySize;

    /// Get total free memory
    fn total_free(&self) -> MemorySize;

    /// Get fragmentation percentage
    fn fragmentation(&self) -> f64;

    /// Get allocation statistics
    fn stats(&self) -> AllocationStats;
}

/// Allocation statistics
#[derive(Debug, Clone)]
pub struct AllocationStats {
    pub total_allocations: usize,
    pub total_deallocations: usize,
    pub current_allocations: usize,
    pub total_allocated_bytes: usize,
    pub total_deallocated_bytes: usize,
    pub peak_allocated_bytes: usize,
    pub fragmentation_percentage: f64,
    pub average_allocation_size: f64,
}

impl Default for AllocationStats {
    fn default() -> Self {
        Self {
            total_allocations: 0,
            total_deallocations: 0,
            current_allocations: 0,
            total_allocated_bytes: 0,
            total_deallocated_bytes: 0,
            peak_allocated_bytes: 0,
            fragmentation_percentage: 0.0,
            average_allocation_size: 0.0,
        }
    }
}

/// Memory allocation error
#[derive(Debug, thiserror::Error)]
pub enum AllocationError {
    #[error("Out of memory: requested {requested} bytes, available {available} bytes")]
    OutOfMemory { requested: usize, available: usize },

    #[error("Invalid handle: {handle:?}")]
    InvalidHandle { handle: HeapHandleId },

    #[error("Size too large: {size} bytes exceeds maximum {max} bytes")]
    SizeTooLarge { size: usize, max: usize },

    #[error("Alignment requirement not met: {alignment} bytes")]
    AlignmentRequirementNotMet { alignment: usize },

    #[error("Fragmentation too high: {fragmentation}%")]
    FragmentationTooHigh { fragmentation: f64 },
}

/// Memory alignment utilities
pub mod alignment {
    /// Align size to the next multiple of alignment
    pub fn align_up(size: usize, alignment: usize) -> usize {
        (size + alignment - 1) & !(alignment - 1)
    }

    /// Check if size is aligned
    pub fn is_aligned(size: usize, alignment: usize) -> bool {
        size % alignment == 0
    }

    /// Get alignment for a given size (power of 2)
    pub fn get_alignment(size: usize) -> usize {
        if size == 0 {
            return 1;
        }
        size & (!size + 1)
    }

    /// Standard memory alignments
    pub const ALIGN_8: usize = 8;
    pub const ALIGN_16: usize = 16;
    pub const ALIGN_32: usize = 32;
    pub const ALIGN_64: usize = 64;
    pub const ALIGN_128: usize = 128;
    pub const ALIGN_256: usize = 256;
    pub const ALIGN_512: usize = 512;
    pub const ALIGN_1024: usize = 1024;
    pub const ALIGN_4096: usize = 4096;
}

/// Smart allocator that chooses the best strategy based on object size
pub struct SmartAllocator {
    /// Bump allocator for new space (young generation)
    bump_allocator: BumpAllocator,
    /// Free list allocator for old space (old generation)
    free_list_allocator: FreeListAllocator,
    /// Cell allocator for small objects
    cell_allocator: CellAllocator,
    /// Statistics
    stats: AllocationStats,
}

impl SmartAllocator {
    /// Create a new smart allocator
    pub fn new(new_space_size: usize, old_space_size: usize, cell_count: usize) -> Self {
        Self {
            bump_allocator: BumpAllocator::new(new_space_size),
            free_list_allocator: FreeListAllocator::new(old_space_size),
            cell_allocator: CellAllocator::new(cell_count),
            stats: AllocationStats::default(),
        }
    }

    /// Get information about all allocators
    pub fn get_info(&self) -> AllocatorInfo {
        AllocatorInfo {
            new_space: self.bump_allocator.layout_info(),
            old_space: self.free_list_allocator.memory_info(),
            cell_space: self.cell_allocator.cell_info(),
        }
    }

    /// Reset new space allocator (after minor GC)
    pub fn reset_new_space(&mut self) {
        self.bump_allocator.reset();
    }

    /// Defragment old space
    pub fn defragment_old_space(&mut self) -> DefragmentationStats {
        self.free_list_allocator.defragment()
    }

    /// Compact cell space
    pub fn compact_cell_space(&mut self) -> CompactionStats {
        self.cell_allocator.compact()
    }
}

impl Allocator for SmartAllocator {
    fn allocate(&mut self, size: MemorySize) -> Option<HeapHandleId> {
        let size_bytes = size.as_usize();

        // Choose the best allocator based on size
        if size_bytes <= cell::CELL_SIZE {
            // Small objects go to cell allocator
            self.cell_allocator.allocate(size)
        } else if size_bytes <= 1024 * 1024 {
            // Medium objects go to bump allocator (new space)
            if let Some(handle) = self.bump_allocator.allocate(size) {
                // Update statistics
                self.stats.total_allocations += 1;
                self.stats.current_allocations += 1;
                self.stats.total_allocated_bytes += size_bytes;

                if self.stats.total_allocated_bytes > self.stats.peak_allocated_bytes {
                    self.stats.peak_allocated_bytes = self.stats.total_allocated_bytes;
                }

                Some(handle)
            } else {
                // New space full, try old space
                self.free_list_allocator.allocate(size)
            }
        } else {
            // Large objects go to free list allocator (old space)
            self.free_list_allocator.allocate(size)
        }
    }

    fn deallocate(&mut self, handle: HeapHandleId) -> bool {
        let handle_addr = handle.as_usize();

        // Try cell allocator first (small objects)
        if self.cell_allocator.deallocate(handle) {
            self.stats.total_deallocations += 1;
            self.stats.current_allocations = self.stats.current_allocations.saturating_sub(1);
            return true;
        }

        // Try free list allocator (old space)
        if self.free_list_allocator.deallocate(handle) {
            self.stats.total_deallocations += 1;
            self.stats.current_allocations = self.stats.current_allocations.saturating_sub(1);
            return true;
        }

        // Bump allocator doesn't support deallocation
        false
    }

    fn can_allocate(&self, size: MemorySize) -> bool {
        let size_bytes = size.as_usize();

        if size_bytes <= cell::CELL_SIZE {
            self.cell_allocator.can_allocate(size_bytes)
        } else if size_bytes <= 1024 * 1024 {
            self.bump_allocator.can_allocate(size_bytes)
                || self.free_list_allocator.can_allocate(size_bytes)
        } else {
            self.free_list_allocator.can_allocate(size_bytes)
        }
    }

    fn total_allocated(&self) -> MemorySize {
        let total = self.bump_allocator.total_allocated().as_usize()
            + self.free_list_allocator.total_allocated().as_usize()
            + self.cell_allocator.total_allocated().as_usize();
        MemorySize::new(total)
    }

    fn total_free(&self) -> MemorySize {
        let total = self.bump_allocator.total_free().as_usize()
            + self.free_list_allocator.total_free().as_usize()
            + self.cell_allocator.total_free().as_usize();
        MemorySize::new(total)
    }

    fn fragmentation(&self) -> f64 {
        // Weighted average based on space usage
        let new_weight = self.bump_allocator.total_allocated().as_usize() as f64;
        let old_weight = self.free_list_allocator.total_allocated().as_usize() as f64;
        let cell_weight = self.cell_allocator.total_allocated().as_usize() as f64;
        let total_weight = new_weight + old_weight + cell_weight;

        if total_weight == 0.0 {
            0.0
        } else {
            (new_weight * self.bump_allocator.fragmentation()
                + old_weight * self.free_list_allocator.fragmentation()
                + cell_weight * self.cell_allocator.fragmentation())
                / total_weight
        }
    }

    fn stats(&self) -> AllocationStats {
        // Combine statistics from all allocators
        let mut combined_stats = self.stats.clone();

        let bump_stats = self.bump_allocator.stats();
        let free_list_stats = self.free_list_allocator.stats();
        let cell_stats = self.cell_allocator.stats();

        combined_stats.total_allocations = bump_stats.total_allocations
            + free_list_stats.total_allocations
            + cell_stats.total_allocations;

        combined_stats.total_deallocations = bump_stats.total_deallocations
            + free_list_stats.total_deallocations
            + cell_stats.total_deallocations;

        combined_stats.current_allocations = bump_stats.current_allocations
            + free_list_stats.current_allocations
            + cell_stats.current_allocations;

        combined_stats.total_allocated_bytes = bump_stats.total_allocated_bytes
            + free_list_stats.total_allocated_bytes
            + cell_stats.total_allocated_bytes;

        combined_stats.total_deallocated_bytes = bump_stats.total_deallocated_bytes
            + free_list_stats.total_deallocated_bytes
            + cell_stats.total_deallocated_bytes;

        combined_stats.peak_allocated_bytes = combined_stats.total_allocated_bytes;

        if combined_stats.total_allocations > 0 {
            combined_stats.average_allocation_size = combined_stats.total_allocated_bytes as f64
                / combined_stats.total_allocations as f64;
        }

        combined_stats
    }
}

/// Information about all allocators
#[derive(Debug, Clone)]
pub struct AllocatorInfo {
    pub new_space: bump::LayoutInfo,
    pub old_space: free_list::MemoryInfo,
    pub cell_space: cell::CellInfo,
}

/// Re-export types for convenience
pub use bump::{BumpAllocator, LayoutInfo};
pub use cell::{CellAllocator, CellInfo, CompactionStats, CELL_SIZE};
pub use free_list::{DefragmentationStats, FreeListAllocator, MemoryInfo};
