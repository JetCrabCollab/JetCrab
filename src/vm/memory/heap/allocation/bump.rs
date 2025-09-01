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

use crate::vm::memory::heap::allocation::Allocator;
use crate::vm::types::MemorySize;

/// Bump allocator for fast, simple memory allocation
///
/// This allocator is very fast but doesn't support deallocation.
/// It's ideal for temporary allocations or when all memory is freed at once.
pub struct BumpAllocator {
    /// Start of the memory region
    start: *mut u8,
    /// Current allocation pointer
    current: *mut u8,
    /// End of the memory region
    end: *mut u8,
    /// Total allocated memory
    total_allocated: MemorySize,
    /// Peak memory usage
    peak_usage: MemorySize,
}

impl BumpAllocator {
    /// Create a new bump allocator with the specified size
    pub fn new(size: MemorySize) -> Self {
        let size_bytes = size.bytes();
        let layout = std::alloc::Layout::from_size_align(size_bytes, 8).expect("Invalid layout");

        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            panic!("Failed to allocate memory for bump allocator");
        }

        let start = ptr;
        let current = ptr;
        let end = unsafe { ptr.add(size_bytes) };

        Self {
            start,
            current,
            end,
            total_allocated: MemorySize::new(0),
            peak_usage: MemorySize::new(0),
        }
    }

    /// Reset the allocator to free all memory
    pub fn reset(&mut self) {
        self.current = self.start;
        self.total_allocated = MemorySize::new(0);
    }

    /// Get the current allocation pointer
    pub fn current_ptr(&self) -> *mut u8 {
        self.current
    }

    /// Get the remaining free memory
    pub fn remaining(&self) -> MemorySize {
        let used = unsafe { self.current.offset_from(self.start) as usize };
        let total = unsafe { self.end.offset_from(self.start) as usize };
        MemorySize::new(total.saturating_sub(used))
    }
}

impl Allocator for BumpAllocator {
    fn allocate(&mut self, size: MemorySize) -> Option<usize> {
        if !self.can_allocate(size) {
            return None;
        }

        let aligned_size = align_up(size.bytes(), 8);
        let address = self.current as usize;

        unsafe {
            self.current = self.current.add(aligned_size);
        }

        self.total_allocated = MemorySize::new(self.total_allocated.bytes() + aligned_size);

        if self.total_allocated.bytes() > self.peak_usage.bytes() {
            self.peak_usage = self.total_allocated;
        }

        Some(address)
    }

    fn deallocate(&mut self, _address: usize, _size: MemorySize) -> bool {
        // Bump allocator doesn't support deallocation
        false
    }

    fn can_allocate(&self, size: MemorySize) -> bool {
        let aligned_size = align_up(size.bytes(), 8);
        let remaining = unsafe { self.end.offset_from(self.current) as usize };
        aligned_size <= remaining
    }

    fn total_allocated(&self) -> MemorySize {
        self.total_allocated
    }

    fn total_free(&self) -> MemorySize {
        self.remaining()
    }

    fn fragmentation(&self) -> f64 {
        // Bump allocator has no fragmentation
        0.0
    }
}

impl Drop for BumpAllocator {
    fn drop(&mut self) {
        let size = unsafe { self.end.offset_from(self.start) as usize };
        let layout = std::alloc::Layout::from_size_align(size, 8).expect("Invalid layout");

        unsafe {
            std::alloc::dealloc(self.start, layout);
        }
    }
}

/// Align a size to the specified alignment
fn align_up(size: usize, alignment: usize) -> usize {
    (size + alignment - 1) & !(alignment - 1)
}
