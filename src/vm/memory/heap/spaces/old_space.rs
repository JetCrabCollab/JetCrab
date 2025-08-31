//! # Old Space
//!
//! Old generation memory space using mark & sweep garbage collection.
//! Optimized for long-lived objects with efficient memory reuse.
//!
//! ## Characteristics
//!
//! - **Mark & sweep GC**: Efficient collection for old objects
//! - **Free list allocation**: Reuses freed memory blocks
//! - **Fragmentation handling**: Manages memory fragmentation
//! - **Size classes**: Groups blocks by size for efficiency
//! - **Perfect for old objects**: Long-lived, infrequently allocated

use super::{MemorySpace, SpaceStats, SpaceType};
use crate::vm::handle::HeapHandleId;
use crate::vm::memory::heap::allocation::{
    Allocator, DefragmentationStats, FreeListAllocator, MemoryInfo,
};
use crate::vm::memory::heap::spaces::{CompactionStats, GcStats};
use crate::vm::types::MemorySize;
use crate::vm::value::Value;
use std::collections::HashMap;

/// Old space for old generation objects
pub struct OldSpace {
    /// Free list allocator for efficient memory reuse
    allocator: FreeListAllocator,
    /// Total size of the space
    total_size: usize,
    /// Statistics
    stats: SpaceStats,
    /// Object age tracking
    object_ages: HashMap<HeapHandleId, usize>,
    /// Maximum age before promotion
    max_age: usize,
    /// Current age counter
    age_counter: usize,
}

impl OldSpace {
    /// Create a new old space with the specified size
    pub fn new(size: usize) -> Self {
        let mut allocator = FreeListAllocator::new();
        // Add initial free block
        allocator.add_free_block(0, size);

        Self {
            allocator,
            total_size: size,
            stats: SpaceStats {
                space_type: SpaceType::OldSpace,
                total_size: size,
                allocated_size: 0,
                free_size: size,
                object_count: 0,
                fragmentation_percentage: 0.0,
                allocation_count: 0,
                deallocation_count: 0,
            },
            object_ages: HashMap::new(),
            max_age: 10, // Maximum age before promotion
            age_counter: 0,
        }
    }

    /// Get memory usage information
    pub fn memory_info(&self) -> MemoryInfo {
        self.allocator.memory_info()
    }

    /// Get object age information
    pub fn age_info(&self) -> AgeInfo {
        let total_objects = self.object_ages.len();
        let avg_age = if total_objects > 0 {
            let total_age: usize = self.object_ages.values().sum();
            total_age / total_objects
        } else {
            0
        };

        let max_age_found = self.object_ages.values().max().copied().unwrap_or(0);

        AgeInfo {
            total_objects,
            average_age: avg_age,
            max_age_found,
            max_age_threshold: self.max_age,
            age_counter: self.age_counter,
        }
    }

    /// Increment age for all objects
    pub fn increment_ages(&mut self) {
        for age in self.object_ages.values_mut() {
            *age += 1;
        }
        self.age_counter += 1;
    }

    /// Get objects that should be promoted
    pub fn get_promotable_objects(&self) -> Vec<HeapHandleId> {
        self.object_ages
            .iter()
            .filter(|(_, &age)| age >= self.max_age)
            .map(|(&handle, _)| handle)
            .collect()
    }

    /// Remove promoted objects
    pub fn remove_promoted_objects(&mut self, handles: &[HeapHandleId]) {
        for &handle in handles {
            self.object_ages.remove(&handle);
        }
    }

    /// Check if promotion is needed
    pub fn should_promote(&self) -> bool {
        !self.get_promotable_objects().is_empty()
    }

    /// Get fragmentation information
    pub fn fragmentation_info(&self) -> FragmentationInfo {
        let memory_info = self.allocator.memory_info();

        FragmentationInfo {
            total_size: memory_info.total_size,
            used_size: memory_info.used_size,
            free_size: memory_info.free_size,
            fragmentation_percentage: memory_info.fragmentation_percentage,
            free_block_count: memory_info.free_block_count,
            should_defragment: memory_info.fragmentation_percentage > 30.0,
        }
    }

    /// Check if defragmentation is needed
    pub fn should_defragment(&self) -> bool {
        self.fragmentation_info().should_defragment
    }

    /// Perform mark & sweep garbage collection
    pub fn collect(&mut self) -> GcStats {
        let start_time = std::time::Instant::now();

        // Get current usage before collection
        let before_usage = self.allocator.total_allocated().as_usize();
        let before_objects = self.object_ages.len();

        // Simple mark & sweep simulation
        // In a real implementation, this would traverse the object graph
        let mut objects_to_remove = Vec::new();

        // Simulate marking phase - mark some objects as dead
        for (&handle, &age) in &self.object_ages {
            // Simulate some objects becoming unreachable
            if age > self.max_age * 2 {
                objects_to_remove.push(handle);
            }
        }

        // Sweep phase - remove dead objects
        for handle in &objects_to_remove {
            if self.allocator.deallocate(handle.as_usize(), MemorySize::new(0)) {
                self.object_ages.remove(handle);
            }
        }

        // Calculate collection statistics
        let objects_collected = before_objects - self.object_ages.len();
        let bytes_freed = before_usage - self.allocator.total_allocated().as_usize();

        let end_time = std::time::Instant::now();
        let collection_time = end_time.duration_since(start_time).as_micros() as u64;

        // Update statistics
        self.stats.object_count = self.object_ages.len();
        self.stats.allocated_size = self.allocator.total_allocated().as_usize();
        self.stats.free_size = self.allocator.total_free().as_usize();
        self.stats.fragmentation_percentage = self.allocator.fragmentation();

        GcStats {
            objects_collected,
            bytes_freed,
            collection_time,
        }
    }

    /// Defragment the space
    pub fn defragment(&mut self) -> DefragmentationStats {
        self.allocator.defragment()
    }

    /// Compact the space
    pub fn compact(&mut self) -> CompactionStats {
        let start_time = std::time::Instant::now();
        let initial_fragmentation = self.allocator.fragmentation();

        // Perform defragmentation
        let defrag_stats = self.defragment();

        // Update statistics
        self.stats.fragmentation_percentage = self.allocator.fragmentation();

        let end_time = std::time::Instant::now();
        let duration = end_time.duration_since(start_time).as_micros() as u64;

        CompactionStats {
            duration_micros: duration,
            initial_fragmentation,
            final_fragmentation: self.allocator.fragmentation(),
            cells_moved: defrag_stats.blocks_moved,
        }
    }

    /// Get space efficiency
    pub fn efficiency(&self) -> f64 {
        let used = self.allocator.total_allocated().as_usize();
        let total = self.total_size;
        (used as f64 / total as f64) * 100.0
    }

    /// Get space health score
    pub fn health_score(&self) -> f64 {
        let efficiency = self.efficiency();
        let fragmentation = self.allocator.fragmentation();

        // Higher efficiency and lower fragmentation = better health
        let efficiency_score = efficiency / 100.0;
        let fragmentation_score = 1.0 - (fragmentation / 100.0);

        (efficiency_score + fragmentation_score) / 2.0 * 100.0
    }
}

impl MemorySpace for OldSpace {
    fn allocate(&mut self, size: MemorySize) -> Option<HeapHandleId> {
        if let Some(handle) = self.allocator.allocate(size) {
            // Track object age
            self.object_ages.insert(HeapHandleId::from(handle), 0);

            // Update statistics
            self.stats.allocated_size += size.bytes();
            self.stats.object_count += 1;
            self.stats.allocation_count += 1;

            // Update free space
            self.stats.free_size = self.allocator.total_free().bytes();

            Some(HeapHandleId::from(handle))
        } else {
            None
        }
    }

    fn deallocate(&mut self, handle: HeapHandleId) -> bool {
        if self
            .allocator
            .deallocate(handle.as_usize(), MemorySize::new(0))
        {
            // Remove age tracking
            self.object_ages.remove(&handle);

            // Update statistics
            self.stats.object_count = self.object_ages.len();
            self.stats.allocated_size = self.allocator.total_allocated().bytes();
            self.stats.free_size = self.allocator.total_free().bytes();
            self.stats.deallocation_count += 1;

            true
        } else {
            false
        }
    }

    fn can_allocate(&self, size: MemorySize) -> bool {
        self.allocator.can_allocate(size)
    }

    fn total_allocated(&self) -> MemorySize {
        self.allocator.total_allocated()
    }

    fn total_free(&self) -> MemorySize {
        self.allocator.total_free()
    }

    fn stats(&self) -> SpaceStats {
        self.stats.clone()
    }

    fn space_type(&self) -> SpaceType {
        SpaceType::OldSpace
    }

    fn extract_object(&mut self, handle: HeapHandleId) -> Option<Value> {
        // Extract object from allocator
        if let Some(object_data) = self.allocator.extract_object(handle.as_usize()) {
            // Remove age tracking
            self.object_ages.remove(&handle);

            // Update statistics
            self.stats.object_count = self.stats.object_count.saturating_sub(1);
            self.stats.allocated_size = self
                .stats
                .allocated_size
                .saturating_sub(object_data.size().unwrap_or(0));

            Some(object_data)
        } else {
            None
        }
    }

    fn allocate_object(&mut self, data: Value) -> Option<HeapHandleId> {
        let size = MemorySize::new(data.size().unwrap_or(64));
        if let Some(handle) = self.allocator.allocate_object(data) {
            // Track object age
            self.object_ages.insert(HeapHandleId::from(handle), 0);

            // Update statistics
            self.stats.allocated_size += size.bytes();
            self.stats.object_count += 1;
            self.stats.allocation_count += 1;

            // Update free space
            self.stats.free_size = self.allocator.total_free().bytes();

            Some(HeapHandleId::from(handle))
        } else {
            None
        }
    }
}

/// Information about object ages
#[derive(Debug, Clone)]
pub struct AgeInfo {
    pub total_objects: usize,
    pub average_age: usize,
    pub max_age_found: usize,
    pub max_age_threshold: usize,
    pub age_counter: usize,
}

/// Information about fragmentation
#[derive(Debug, Clone)]
pub struct FragmentationInfo {
    pub total_size: usize,
    pub used_size: usize,
    pub free_size: usize,
    pub fragmentation_percentage: f64,
    pub free_block_count: usize,
    pub should_defragment: bool,
}

impl Default for OldSpace {
    fn default() -> Self {
        Self::new(256 * 1024 * 1024) // 256MB default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_old_space_new() {
        let old_space = OldSpace::new(1024);
        assert_eq!(old_space.total_size, 1024);
        assert_eq!(old_space.total_allocated().as_usize(), 0);
        assert_eq!(old_space.total_free().as_usize(), 1024);
        assert_eq!(old_space.stats.object_count, 0);
    }

    #[test]
    fn test_old_space_allocate() {
        let mut old_space = OldSpace::new(1024);

        let handle = old_space.allocate(MemorySize::new(64));
        assert!(handle.is_some());
        assert_eq!(old_space.total_allocated().as_usize(), 64);
        assert_eq!(old_space.stats.object_count, 1);
        assert_eq!(old_space.stats.allocation_count, 1);

        // Check age tracking
        assert_eq!(old_space.object_ages.get(&handle.unwrap()), Some(&0));
    }

    #[test]
    fn test_old_space_deallocate() {
        let mut old_space = OldSpace::new(1024);

        let handle = old_space.allocate(MemorySize::new(64)).unwrap();
        assert_eq!(old_space.stats.object_count, 1);

        assert!(old_space.deallocate(handle));
        assert_eq!(old_space.stats.object_count, 0);
        assert_eq!(old_space.stats.deallocation_count, 1);

        // Age tracking should be removed
        assert!(old_space.object_ages.is_empty());
    }

    #[test]
    fn test_old_space_age_tracking() {
        let mut old_space = OldSpace::new(1024);

        let handle = old_space.allocate(MemorySize::new(64)).unwrap();

        // Increment ages
        old_space.increment_ages();
        old_space.increment_ages();

        assert_eq!(old_space.object_ages.get(&handle), Some(&2));
        assert_eq!(old_space.age_counter, 2);
    }

    #[test]
    fn test_old_space_promotion() {
        let mut old_space = OldSpace::new(1024);

        let handle = old_space.allocate(MemorySize::new(64)).unwrap();

        // Age the object beyond promotion threshold
        for _ in 0..15 {
            old_space.increment_ages();
        }

        assert!(old_space.should_promote());

        let promotable = old_space.get_promotable_objects();
        assert_eq!(promotable.len(), 1);
        assert_eq!(promotable[0], handle);

        // Remove promoted object
        old_space.remove_promoted_objects(&promotable);
        assert!(old_space.object_ages.is_empty());
    }

    #[test]
    fn test_old_space_collect() {
        let mut old_space = OldSpace::new(1024);

        // Allocate some objects
        old_space.allocate(MemorySize::new(64));
        old_space.allocate(MemorySize::new(128));

        let before_objects = old_space.stats.object_count;

        // Perform collection
        let stats = old_space.collect();

        assert!(stats.collection_time > 0);
        assert_eq!(
            old_space.stats.object_count,
            before_objects - stats.objects_collected
        );
    }

    #[test]
    fn test_old_space_defragment() {
        let mut old_space = OldSpace::new(1024);

        // Allocate and deallocate to create fragmentation
        let handle1 = old_space.allocate(MemorySize::new(64)).unwrap();
        let handle2 = old_space.allocate(MemorySize::new(128)).unwrap();

        old_space.deallocate(handle1);

        let initial_fragmentation = old_space.allocator.fragmentation();

        // Defragment
        let stats = old_space.defragment();
        assert!(stats.final_fragmentation < stats.initial_fragmentation);
    }
}
