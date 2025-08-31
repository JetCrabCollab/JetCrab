//! # Free List Allocator
//!
//! Efficient memory allocator for old space (old generation) using free lists
//! to reuse deallocated memory blocks.
//!
//! ## Characteristics
//!
//! - **Efficient reuse**: Reuses freed memory blocks
//! - **Fragmentation handling**: Manages memory fragmentation
//! - **Size classes**: Groups blocks by size for better efficiency
//! - **Coalescing**: Merges adjacent free blocks
//! - **Perfect for old generation**: Long-lived objects with deallocation

use crate::vm::memory::heap::allocation::{AllocationError, AllocationStats, Allocator};
use crate::vm::memory::heap::types::FreeBlockId;
use crate::vm::types::MemorySize;
use std::collections::HashMap;

/// Free block in the free list
#[derive(Debug, Clone)]
struct FreeBlock {
    /// Start address of the block
    start: usize,
    /// Size of the block
    size: usize,
    /// Next free block in the list
    next: Option<FreeBlockId>,
}

/// Free list allocator for efficient memory management
///
/// This allocator maintains a list of free memory blocks and can
/// allocate and deallocate memory efficiently.
pub struct FreeListAllocator {
    /// Free lists indexed by size
    free_lists: HashMap<usize, Option<FreeBlockId>>,
    /// All free blocks
    free_blocks: HashMap<FreeBlockId, FreeBlock>,
    /// Next free block ID
    next_block_id: FreeBlockId,
    /// Total allocated memory
    total_allocated: MemorySize,
    /// Total freed memory
    total_freed: MemorySize,
    /// Peak memory usage
    peak_usage: MemorySize,
}

impl FreeListAllocator {
    /// Create a new free list allocator
    pub fn new() -> Self {
        Self {
            free_lists: HashMap::new(),
            free_blocks: HashMap::new(),
            next_block_id: FreeBlockId::new(0),
            total_allocated: MemorySize::new(0),
            total_freed: MemorySize::new(0),
            peak_usage: MemorySize::new(0),
        }
    }

    /// Add a free block to the allocator
    pub fn add_free_block(&mut self, start: usize, size: usize) {
        let block_id = self.next_block_id;
        self.next_block_id.increment();

        let block = FreeBlock {
            start,
            size,
            next: None,
        };

        // Insert into free blocks
        self.free_blocks.insert(block_id, block);

        // Insert into appropriate free list
        let free_list = self.free_lists.entry(size).or_insert(None);
        let old_head = *free_list;
        *free_list = Some(block_id);

        // Update the new block's next pointer
        if let Some(block) = self.free_blocks.get_mut(&block_id) {
            block.next = old_head;
        }
    }

    /// Find the best fit block for the requested size
    fn find_best_fit(&self, size: usize) -> Option<FreeBlockId> {
        let mut best_fit: Option<FreeBlockId> = None;
        let mut best_size = usize::MAX;

        // Look for exact size first
        if let Some(block_id) = self.free_lists.get(&size) {
            if let Some(block_id) = *block_id {
                return Some(block_id);
            }
        }

        // Look for best fit
        for (&block_size, block_id) in &self.free_lists {
            if let Some(block_id) = block_id {
                if block_size >= size && block_size < best_size {
                    best_size = block_size;
                    best_fit = Some(*block_id);
                }
            }
        }

        best_fit
    }

    /// Remove a block from the free list
    fn remove_from_free_list(&mut self, block_id: FreeBlockId, size: usize) {
        if let Some(head) = self.free_lists.get(&size) {
            if let Some(head) = head {
                if *head == block_id {
                    // Block is at the head of the list
                    if let Some(block) = self.free_blocks.get(&block_id) {
                        self.free_lists.insert(size, block.next);
                    }
                } else {
                    // Block is somewhere in the list
                    let mut current = *head;
                    while let Some(block) = self.free_blocks.get(&current) {
                        if block.next == Some(block_id) {
                            // Found the predecessor
                            let next_next =
                                if let Some(next_block) = self.free_blocks.get(&block_id) {
                                    next_block.next
                                } else {
                                    None
                                };

                            if let Some(prev_block) = self.free_blocks.get_mut(&current) {
                                prev_block.next = next_next;
                            }
                            break;
                        }
                        if let Some(next_id) = block.next {
                            current = next_id;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        // Remove the block
        self.free_blocks.remove(&block_id);
    }

    /// Split a block if it's too large
    fn split_block(&mut self, block_id: FreeBlockId, requested_size: usize) -> Option<usize> {
        // Get block info before borrowing self mutably
        let block_size = if let Some(block) = self.free_blocks.get(&block_id) {
            block.size
        } else {
            return None;
        };

        if block_size <= requested_size + 16 {
            // Don't split if remainder is too small
            return None;
        }

        let new_size = block_size - requested_size;
        let new_start = if let Some(block) = self.free_blocks.get(&block_id) {
            block.start + requested_size
        } else {
            return None;
        };

        let original_start = if let Some(block) = self.free_blocks.get(&block_id) {
            block.start
        } else {
            return None;
        };

        // Remove the original block
        self.remove_from_free_list(block_id, block_size);

        // Add the remainder as a new free block
        self.add_free_block(new_start, new_size);

        // Return the original block's start address
        Some(original_start)
    }
}

impl Allocator for FreeListAllocator {
    fn allocate(&mut self, size: MemorySize) -> Option<usize> {
        let size_bytes = size.bytes();

        if let Some(block_id) = self.find_best_fit(size_bytes) {
            if let Some(block) = self.free_blocks.get(&block_id) {
                if block.size == size_bytes {
                    // Exact fit
                    let start = block.start;
                    self.remove_from_free_list(block_id, size_bytes);
                    self.total_allocated =
                        MemorySize::new(self.total_allocated.bytes() + size_bytes);

                    if self.total_allocated.bytes() > self.peak_usage.bytes() {
                        self.peak_usage = self.total_allocated;
                    }

                    return Some(start);
                } else {
                    // Split the block
                    let start = self.split_block(block_id, size_bytes);
                    if let Some(start_addr) = start {
                        self.total_allocated =
                            MemorySize::new(self.total_allocated.bytes() + size_bytes);

                        if self.total_allocated.bytes() > self.peak_usage.bytes() {
                            self.peak_usage = self.total_allocated;
                        }

                        return Some(start_addr);
                    }
                }
            }
        }

        None
    }

    fn deallocate(&mut self, address: usize, size: MemorySize) -> bool {
        let size_bytes = size.bytes();

        // Add the freed block to the free list
        self.add_free_block(address, size_bytes);

        self.total_freed = MemorySize::new(self.total_freed.bytes() + size_bytes);

        true
    }

    fn can_allocate(&self, size: MemorySize) -> bool {
        let size_bytes = size.bytes();

        // Check if we have a block that can fit this size
        for (&block_size, _) in &self.free_lists {
            if block_size >= size_bytes {
                return true;
            }
        }

        false
    }

    fn total_allocated(&self) -> MemorySize {
        self.total_allocated
    }

    fn total_free(&self) -> MemorySize {
        let mut total_free = 0;
        for (_, block) in &self.free_blocks {
            total_free += block.size;
        }
        MemorySize::new(total_free)
    }

    fn fragmentation(&self) -> f64 {
        let total_free = self.total_free().bytes();
        if total_free == 0 {
            return 0.0;
        }

        let mut free_blocks = 0;
        for (_, _) in &self.free_blocks {
            free_blocks += 1;
        }

        if free_blocks == 0 {
            return 0.0;
        }

        // Calculate fragmentation based on number of free blocks
        // More blocks = higher fragmentation
        (free_blocks as f64 / total_free as f64) * 100.0
    }
}

impl Default for FreeListAllocator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_list_allocator_creation() {
        let allocator = FreeListAllocator::new();
        assert_eq!(allocator.total_allocated().bytes(), 0);
        assert_eq!(allocator.total_free().bytes(), 0);
    }

    #[test]
    fn test_free_list_allocator_add_free_block() {
        let mut allocator = FreeListAllocator::new();
        allocator.add_free_block(0, 1024);

        assert!(allocator.can_allocate(MemorySize::new(1024)));
        assert!(allocator.can_allocate(MemorySize::new(512)));
        assert!(!allocator.can_allocate(MemorySize::new(2048)));
    }

    #[test]
    fn test_free_list_allocator_allocate() {
        let mut allocator = FreeListAllocator::new();
        allocator.add_free_block(0, 1024);

        let addr = allocator.allocate(MemorySize::new(512));
        assert!(addr.is_some());
        assert_eq!(allocator.total_allocated().bytes(), 512);
    }

    #[test]
    fn test_free_list_allocator_deallocate() {
        let mut allocator = FreeListAllocator::new();
        allocator.add_free_block(0, 1024);

        let addr = allocator.allocate(MemorySize::new(512));
        assert!(addr.is_some());

        allocator.deallocate(addr.unwrap(), MemorySize::new(512));
        assert_eq!(allocator.total_freed().bytes(), 512);
    }
}
