//! # Bump Allocator
//!
//! Ultra-fast O(1) memory allocator for new space (young generation).
//! Uses a simple bump pointer for extremely fast allocation.
//!
//! ## Characteristics
//!
//! - **O(1) allocation**: Single pointer increment
//! - **No fragmentation**: Contiguous memory layout
//! - **Cache-friendly**: Sequential memory access
//! - **No deallocation**: Memory is freed by copying GC
//! - **Perfect for young generation**: Short-lived objects

use super::alignment::{align_up, ALIGN_8};
use super::{AllocationError, AllocationStats, Allocator};
use crate::vm::handle::HeapHandleId;
use crate::vm::types::MemorySize;

/// Bump allocator for extremely fast allocation
pub struct BumpAllocator {
    /// Start of the memory region
    start: *mut u8,
    /// Current allocation pointer
    current: *mut u8,
    /// End of the memory region
    end: *mut u8,
    /// Total size of the memory region
    total_size: usize,
    /// Statistics
    stats: AllocationStats,
}

impl BumpAllocator {
    /// Create a new bump allocator with the specified size
    pub fn new(size: usize) -> Self {
        let layout = std::alloc::Layout::from_size_align(size, ALIGN_8).expect("Invalid layout");

        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            panic!("Failed to allocate memory for bump allocator");
        }

        let start = ptr;
        let current = ptr;
        let end = unsafe { ptr.add(size) };

        Self {
            start,
            current,
            end,
            total_size: size,
            stats: AllocationStats::default(),
        }
    }

    /// Create a new bump allocator with existing memory
    pub fn with_memory(memory: &mut [u8]) -> Self {
        let start = memory.as_mut_ptr();
        let current = start;
        let end = unsafe { start.add(memory.len()) };
        let total_size = memory.len();

        Self {
            start,
            current,
            end,
            total_size,
            stats: AllocationStats::default(),
        }
    }

    /// Reset the allocator (used after garbage collection)
    pub fn reset(&mut self) {
        self.current = self.start;
        self.stats.current_allocations = 0;
        self.stats.total_allocated_bytes = 0;
        self.stats.fragmentation_percentage = 0.0;
    }

    /// Get the current allocation pointer
    pub fn current_ptr(&self) -> *mut u8 {
        self.current
    }

    /// Get the remaining free space
    pub fn remaining_space(&self) -> usize {
        unsafe { self.end.offset_from(self.current) as usize }
    }

    /// Check if the allocator is empty
    pub fn is_empty(&self) -> bool {
        self.current == self.start
    }

    /// Check if the allocator is full
    pub fn is_full(&self) -> bool {
        self.current >= self.end
    }

    /// Get memory usage percentage
    pub fn usage_percentage(&self) -> f64 {
        let used = unsafe { self.current.offset_from(self.start) as usize };
        (used as f64 / self.total_size as f64) * 100.0
    }

    /// Get memory layout information
    pub fn layout_info(&self) -> LayoutInfo {
        let used = unsafe { self.current.offset_from(self.start) as usize };
        let free = self.total_size - used;

        LayoutInfo {
            total_size: self.total_size,
            used_size: used,
            free_size: free,
            usage_percentage: (used as f64 / self.total_size as f64) * 100.0,
            fragmentation_percentage: 0.0, // Bump allocator has no fragmentation
        }
    }
}

impl Allocator for BumpAllocator {
    fn allocate(&mut self, size: MemorySize) -> Option<HeapHandleId> {
        let aligned_size = align_up(size.as_usize(), ALIGN_8);

        if !self.can_allocate(aligned_size) {
            return None;
        }

        // Calculate handle based on offset from start
        let offset = unsafe { self.current.offset_from(self.start) as usize };
        let handle = HeapHandleId::new(offset);

        // Update current pointer
        self.current = unsafe { self.current.add(aligned_size) };

        // Update statistics
        self.stats.total_allocations += 1;
        self.stats.current_allocations += 1;
        self.stats.total_allocated_bytes += aligned_size;

        if self.stats.total_allocated_bytes > self.stats.peak_allocated_bytes {
            self.stats.peak_allocated_bytes = self.stats.total_allocated_bytes;
        }

        // Update average allocation size
        self.stats.average_allocation_size =
            self.stats.total_allocated_bytes as f64 / self.stats.total_allocations as f64;

        Some(handle)
    }

    fn deallocate(&mut self, _handle: HeapHandleId) -> bool {
        // Bump allocator doesn't support deallocation
        // Memory is freed by copying garbage collection
        false
    }

    fn can_allocate(&self, size: usize) -> bool {
        let aligned_size = align_up(size, ALIGN_8);
        unsafe { self.current.add(aligned_size) <= self.end }
    }

    fn total_allocated(&self) -> MemorySize {
        let used = unsafe { self.current.offset_from(self.start) as usize };
        MemorySize::new(used)
    }

    fn total_free(&self) -> MemorySize {
        let used = unsafe { self.current.offset_from(self.start) as usize };
        MemorySize::new(self.total_size - used)
    }

    fn fragmentation(&self) -> f64 {
        // Bump allocator has no fragmentation
        0.0
    }

    fn stats(&self) -> AllocationStats {
        self.stats.clone()
    }
}

/// Memory layout information
#[derive(Debug, Clone)]
pub struct LayoutInfo {
    pub total_size: usize,
    pub used_size: usize,
    pub free_size: usize,
    pub usage_percentage: f64,
    pub fragmentation_percentage: f64,
}

impl Default for BumpAllocator {
    fn default() -> Self {
        Self::new(1024 * 1024) // 1MB default
    }
}

impl Drop for BumpAllocator {
    fn drop(&mut self) {
        if !self.start.is_null() {
            let layout = std::alloc::Layout::from_size_align(self.total_size, ALIGN_8)
                .expect("Invalid layout");
            unsafe {
                std::alloc::dealloc(self.start, layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bump_allocator_new() {
        let mut allocator = BumpAllocator::new(1024);
        assert_eq!(allocator.total_size, 1024);
        assert_eq!(allocator.remaining_space(), 1024);
        assert!(allocator.is_empty());
        assert!(!allocator.is_full());
    }

    #[test]
    fn test_bump_allocator_allocate() {
        let mut allocator = BumpAllocator::new(1024);

        let handle1 = allocator.allocate(MemorySize::new(64));
        assert!(handle1.is_some());
        assert_eq!(allocator.remaining_space(), 1024 - 64);
        assert_eq!(allocator.total_allocated().as_usize(), 64);

        let handle2 = allocator.allocate(MemorySize::new(128));
        assert!(handle2.is_some());
        assert_eq!(allocator.remaining_space(), 1024 - 64 - 128);
        assert_eq!(allocator.total_allocated().as_usize(), 64 + 128);
    }

    #[test]
    fn test_bump_allocator_alignment() {
        let mut allocator = BumpAllocator::new(1024);

        // Allocate 7 bytes (should be aligned to 8)
        let handle = allocator.allocate(MemorySize::new(7));
        assert!(handle.is_some());
        assert_eq!(allocator.total_allocated().as_usize(), 8); // Aligned to 8
    }

    #[test]
    fn test_bump_allocator_reset() {
        let mut allocator = BumpAllocator::new(1024);

        allocator.allocate(MemorySize::new(64));
        assert_eq!(allocator.total_allocated().as_usize(), 64);

        allocator.reset();
        assert_eq!(allocator.total_allocated().as_usize(), 0);
        assert_eq!(allocator.remaining_space(), 1024);
        assert!(allocator.is_empty());
    }

    #[test]
    fn test_bump_allocator_out_of_memory() {
        let mut allocator = BumpAllocator::new(64);

        // First allocation should succeed
        let handle1 = allocator.allocate(MemorySize::new(32));
        assert!(handle1.is_some());

        // Second allocation should fail (not enough space)
        let handle2 = allocator.allocate(MemorySize::new(64));
        assert!(handle2.is_none());
    }

    #[test]
    fn test_bump_allocator_stats() {
        let mut allocator = BumpAllocator::new(1024);

        allocator.allocate(MemorySize::new(64));
        allocator.allocate(MemorySize::new(128));

        let stats = allocator.stats();
        assert_eq!(stats.total_allocations, 2);
        assert_eq!(stats.current_allocations, 2);
        assert_eq!(stats.total_allocated_bytes, 64 + 128);
        assert_eq!(stats.fragmentation_percentage, 0.0);
    }
}
