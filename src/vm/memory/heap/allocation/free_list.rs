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

use super::{Allocator, AllocationStats, AllocationError};
use crate::vm::handle::HeapHandleId;
use crate::vm::types::MemorySize;
use super::alignment::{align_up, ALIGN_8};
use std::collections::HashMap;

/// Free block in memory
#[derive(Debug, Clone)]
struct FreeBlock {
    /// Start address of the free block
    start: usize,
    /// Size of the free block
    size: usize,
    /// Next free block in the list
    next: Option<usize>,
}

/// Free list allocator for efficient memory reuse
pub struct FreeListAllocator {
    /// Memory region
    memory: Vec<u8>,
    /// Free lists organized by size class
    free_lists: HashMap<usize, Option<usize>>,
    /// Map of free block addresses to block info
    free_blocks: HashMap<usize, FreeBlock>,
    /// Total size of the memory region
    total_size: usize,
    /// Statistics
    stats: AllocationStats,
    /// Size classes for efficient allocation
    size_classes: Vec<usize>,
}

impl FreeListAllocator {
    /// Create a new free list allocator with the specified size
    pub fn new(size: usize) -> Self {
        let mut size_classes = vec![
            8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768
        ];
        
        // Add size classes up to the total size
        let mut current = 16384;
        while current * 2 <= size {
            current *= 2;
            size_classes.push(current);
        }
        
        let mut free_lists = HashMap::new();
        for &size_class in &size_classes {
            free_lists.insert(size_class, None);
        }
        
        let mut allocator = Self {
            memory: vec![0; size],
            free_lists,
            free_blocks: HashMap::new(),
            total_size: size,
            stats: AllocationStats::default(),
            size_classes,
        };
        
        // Initialize with one large free block
        allocator.add_free_block(0, size);
        
        allocator
    }
    
    /// Add a free block to the appropriate free list
    fn add_free_block(&mut self, start: usize, size: usize) {
        let size_class = self.get_size_class(size);
        
        let block = FreeBlock {
            start,
            size,
            next: self.free_lists[&size_class],
        };
        
        self.free_blocks.insert(start, block);
        self.free_lists.insert(size_class, Some(start));
    }
    
    /// Remove a free block from the free list
    fn remove_free_block(&mut self, start: usize) -> Option<FreeBlock> {
        let block = self.free_blocks.remove(&start)?;
        let size_class = self.get_size_class(block.size);
        
        // Update the free list
        if let Some(&first_block) = self.free_lists.get(&size_class) {
            if first_block == start {
                self.free_lists.insert(size_class, block.next);
            } else {
                // Find and update the previous block
                let mut current = first_block;
                while let Some(block_info) = self.free_blocks.get(&current) {
                    if block_info.next == Some(start) {
                        let mut prev_block = block_info.clone();
                        prev_block.next = block.next;
                        self.free_blocks.insert(current, prev_block);
                        break;
                    }
                    current = block_info.next?;
                }
            }
        }
        
        Some(block)
    }
    
    /// Get the appropriate size class for a given size
    fn get_size_class(&self, size: usize) -> usize {
        for &size_class in &self.size_classes {
            if size_class >= size {
                return size_class;
            }
        }
        size
    }
    
    /// Find the best fit free block for a given size
    fn find_best_fit(&self, size: usize) -> Option<usize> {
        let size_class = self.get_size_class(size);
        
        // First try exact size class
        if let Some(&first_block) = self.free_lists.get(&size_class) {
            if first_block.is_some() {
                return first_block;
            }
        }
        
        // Then try larger size classes
        for &larger_class in &self.size_classes {
            if larger_class > size_class {
                if let Some(&first_block) = self.free_lists.get(&larger_class) {
                    if first_block.is_some() {
                        return first_block;
                    }
                }
            }
        }
        
        None
    }
    
    /// Split a free block if it's too large
    fn split_block(&mut self, start: usize, requested_size: usize) -> usize {
        let block = self.free_blocks.remove(&start).unwrap();
        let remaining_size = block.size - requested_size;
        
        if remaining_size >= 8 { // Minimum block size
            // Add the remaining part as a new free block
            let remaining_start = start + requested_size;
            self.add_free_block(remaining_start, remaining_size);
        }
        
        start
    }
    
    /// Coalesce adjacent free blocks
    fn coalesce_blocks(&mut self) {
        let mut addresses: Vec<usize> = self.free_blocks.keys().cloned().collect();
        addresses.sort();
        
        let mut i = 0;
        while i < addresses.len() - 1 {
            let current_addr = addresses[i];
            let next_addr = addresses[i + 1];
            
            if let Some(current_block) = self.free_blocks.get(&current_addr) {
                if current_addr + current_block.size == next_addr {
                    // Blocks are adjacent, coalesce them
                    let current_size = current_block.size;
                    let next_block = self.free_blocks.remove(&next_addr).unwrap();
                    let combined_size = current_size + next_block.size;
                    
                    // Remove current block
                    self.remove_free_block(current_addr);
                    
                    // Add combined block
                    self.add_free_block(current_addr, combined_size);
                    
                    // Update addresses list
                    addresses.remove(i + 1);
                    continue;
                }
            }
            i += 1;
        }
    }
    
    /// Get memory usage information
    pub fn memory_info(&self) -> MemoryInfo {
        let total_free = self.free_blocks.values().map(|b| b.size).sum::<usize>();
        let total_used = self.total_size - total_free;
        
        MemoryInfo {
            total_size: self.total_size,
            used_size: total_used,
            free_size: total_free,
            usage_percentage: (total_used as f64 / self.total_size as f64) * 100.0,
            fragmentation_percentage: self.calculate_fragmentation(),
            free_block_count: self.free_blocks.len(),
        }
    }
    
    /// Calculate fragmentation percentage
    fn calculate_fragmentation(&self) -> f64 {
        if self.free_blocks.is_empty() {
            return 0.0;
        }
        
        let total_free = self.free_blocks.values().map(|b| b.size).sum::<usize>();
        let largest_free = self.free_blocks.values().map(|b| b.size).max().unwrap_or(0);
        
        if total_free == 0 {
            0.0
        } else {
            (1.0 - (largest_free as f64 / total_free as f64)) * 100.0
        }
    }
    
    /// Defragment memory by compacting free blocks
    pub fn defragment(&mut self) -> DefragmentationStats {
        let start_time = std::time::Instant::now();
        let initial_fragmentation = self.calculate_fragmentation();
        
        // Simple defragmentation: move all free blocks to the end
        let mut free_blocks: Vec<(usize, usize)> = self.free_blocks
            .iter()
            .map(|(&start, block)| (start, block.size))
            .collect();
        
        free_blocks.sort_by_key(|&(start, _)| start);
        
        // Calculate new positions
        let mut new_positions = Vec::new();
        let mut current_pos = 0;
        
        for (start, size) in &free_blocks {
            if *start > current_pos {
                // There's a gap, move data
                let gap_size = start - current_pos;
                if gap_size > 0 {
                    // Move data from start to current_pos
                    self.memory.copy_within(*start..*start + gap_size, current_pos);
                }
            }
            current_pos += size;
        }
        
        // Rebuild free lists
        self.free_lists.clear();
        for &size_class in &self.size_classes {
            self.free_lists.insert(size_class, None);
        }
        self.free_blocks.clear();
        
        // Add one large free block at the end
        let total_free: usize = free_blocks.iter().map(|(_, size)| size).sum();
        if total_free > 0 {
            self.add_free_block(self.total_size - total_free, total_free);
        }
        
        let end_time = std::time::Instant::now();
        let duration = end_time.duration_since(start_time);
        
        DefragmentationStats {
            duration_micros: duration.as_micros() as u64,
            initial_fragmentation,
            final_fragmentation: self.calculate_fragmentation(),
            blocks_moved: free_blocks.len(),
        }
    }
}

impl Allocator for FreeListAllocator {
    fn allocate(&mut self, size: MemorySize) -> Option<HeapHandleId> {
        let aligned_size = align_up(size.as_usize(), ALIGN_8);
        
        if !self.can_allocate(aligned_size) {
            return None;
        }
        
        // Find best fit block
        if let Some(block_start) = self.find_best_fit(aligned_size) {
            let block = self.remove_free_block(block_start).unwrap();
            
            // Split block if necessary
            let actual_start = if block.size > aligned_size {
                self.split_block(block_start, aligned_size)
            } else {
                block_start
            };
            
            // Update statistics
            self.stats.total_allocations += 1;
            self.stats.current_allocations += 1;
            self.stats.total_allocated_bytes += aligned_size;
            
            if self.stats.total_allocated_bytes > self.stats.peak_allocated_bytes {
                self.stats.peak_allocated_bytes = self.stats.total_allocated_bytes;
            }
            
            self.stats.average_allocation_size = 
                self.stats.total_allocated_bytes as f64 / self.stats.total_allocations as f64;
            
            Some(HeapHandleId::new(actual_start))
        } else {
            None
        }
    }
    
    fn deallocate(&mut self, handle: HeapHandleId) -> bool {
        let start = handle.as_usize();
        
        // Find the size of the allocated block
        // For simplicity, we'll assume it's the size class
        let mut found_size = None;
        for &size_class in &self.size_classes {
            if start % size_class == 0 {
                found_size = Some(size_class);
                break;
            }
        }
        
        if let Some(size) = found_size {
            // Add the block back to free lists
            self.add_free_block(start, size);
            
            // Try to coalesce with adjacent blocks
            self.coalesce_blocks();
            
            // Update statistics
            self.stats.total_deallocations += 1;
            self.stats.current_allocations = self.stats.current_allocations.saturating_sub(1);
            self.stats.total_deallocated_bytes += size;
            
            true
        } else {
            false
        }
    }
    
    fn can_allocate(&self, size: usize) -> bool {
        let aligned_size = align_up(size, ALIGN_8);
        
        // Check if we have a block large enough
        for &size_class in &self.size_classes {
            if size_class >= aligned_size {
                if let Some(&first_block) = self.free_lists.get(&size_class) {
                    if first_block.is_some() {
                        return true;
                    }
                }
            }
        }
        
        false
    }
    
    fn total_allocated(&self) -> MemorySize {
        let total_free: usize = self.free_blocks.values().map(|b| b.size).sum();
        MemorySize::new(self.total_size - total_free)
    }
    
    fn total_free(&self) -> MemorySize {
        let total_free: usize = self.free_blocks.values().map(|b| b.size).sum();
        MemorySize::new(total_free)
    }
    
    fn fragmentation(&self) -> f64 {
        self.calculate_fragmentation()
    }
    
    fn stats(&self) -> AllocationStats {
        self.stats.clone()
    }
}

/// Memory information
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total_size: usize,
    pub used_size: usize,
    pub free_size: usize,
    pub usage_percentage: f64,
    pub fragmentation_percentage: f64,
    pub free_block_count: usize,
}

/// Defragmentation statistics
#[derive(Debug, Clone)]
pub struct DefragmentationStats {
    pub duration_micros: u64,
    pub initial_fragmentation: f64,
    pub final_fragmentation: f64,
    pub blocks_moved: usize,
}

impl Default for FreeListAllocator {
    fn default() -> Self {
        Self::new(1024 * 1024) // 1MB default
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_free_list_allocator_new() {
        let allocator = FreeListAllocator::new(1024);
        assert_eq!(allocator.total_size, 1024);
        assert_eq!(allocator.total_free().as_usize(), 1024);
        assert_eq!(allocator.total_allocated().as_usize(), 0);
    }
    
    #[test]
    fn test_free_list_allocator_allocate() {
        let mut allocator = FreeListAllocator::new(1024);
        
        let handle = allocator.allocate(MemorySize::new(64));
        assert!(handle.is_some());
        assert_eq!(allocator.total_allocated().as_usize(), 64);
        assert_eq!(allocator.total_free().as_usize(), 1024 - 64);
    }
    
    #[test]
    fn test_free_list_allocator_deallocate() {
        let mut allocator = FreeListAllocator::new(1024);
        
        let handle = allocator.allocate(MemorySize::new(64)).unwrap();
        assert_eq!(allocator.total_allocated().as_usize(), 64);
        
        assert!(allocator.deallocate(handle));
        assert_eq!(allocator.total_allocated().as_usize(), 0);
        assert_eq!(allocator.total_free().as_usize(), 1024);
    }
    
    #[test]
    fn test_free_list_allocator_fragmentation() {
        let mut allocator = FreeListAllocator::new(1024);
        
        // Allocate and deallocate to create fragmentation
        let handle1 = allocator.allocate(MemorySize::new(64)).unwrap();
        let handle2 = allocator.allocate(MemorySize::new(128)).unwrap();
        
        allocator.deallocate(handle1);
        
        let fragmentation = allocator.fragmentation();
        assert!(fragmentation > 0.0);
        
        // Defragment
        let stats = allocator.defragment();
        assert!(stats.final_fragmentation < stats.initial_fragmentation);
    }
}
