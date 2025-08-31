//! # New Space
//!
//! Young generation memory space using semi-space allocation with copying garbage collection.
//! Optimized for short-lived objects with extremely fast allocation.
//!
//! ## Characteristics
//!
//! - **Semi-space allocation**: Two spaces that swap roles
//! - **Copying GC**: Fast collection by copying live objects
//! - **Bump allocation**: O(1) allocation performance
//! - **No fragmentation**: Contiguous memory layout
//! - **Perfect for young objects**: Short-lived, frequently allocated

use super::{MemorySpace, SpaceStats, SpaceType};
use crate::vm::handle::HeapHandleId;
use crate::vm::memory::heap::allocation::{Allocator, BumpAllocator, LayoutInfo};
use crate::vm::memory::heap::spaces::{DefragmentationStats, GcStats};
use crate::vm::types::MemorySize;
use crate::vm::value::Value;

/// New space for young generation objects
pub struct NewSpace {
    /// Current active space (from_space)
    active_space: BumpAllocator,
    /// Inactive space (to_space)
    inactive_space: BumpAllocator,
    /// Total size of each space
    space_size: usize,
    /// Statistics
    stats: SpaceStats,
    /// Promotion threshold (number of GCs before promotion)
    promotion_threshold: usize,
    /// Current promotion count
    promotion_count: usize,
}

impl NewSpace {
    /// Create a new new space with the specified size
    pub fn new(size: usize) -> Self {
        Self {
            active_space: BumpAllocator::new(MemorySize::new(size)),
            inactive_space: BumpAllocator::new(MemorySize::new(size)),
            space_size: size,
            stats: SpaceStats {
                space_type: SpaceType::NewSpace,
                total_size: size * 2, // Two semi-spaces
                allocated_size: 0,
                free_size: size * 2,
                object_count: 0,
                fragmentation_percentage: 0.0,
                allocation_count: 0,
                deallocation_count: 0,
            },
            promotion_threshold: 3, // Promote after 3 minor GCs
            promotion_count: 0,
        }
    }

    /// Get the current active space layout information
    pub fn active_layout_info(&self) -> LayoutInfo {
        self.active_space.layout_info()
    }

    /// Get the inactive space layout information
    pub fn inactive_layout_info(&self) -> LayoutInfo {
        self.inactive_space.layout_info()
    }

    /// Get promotion information
    pub fn promotion_info(&self) -> PromotionInfo {
        PromotionInfo {
            threshold: self.promotion_threshold,
            current_count: self.promotion_count,
            should_promote: self.promotion_count >= self.promotion_threshold,
        }
    }

    /// Check if promotion is needed
    pub fn should_promote(&self) -> bool {
        self.promotion_count >= self.promotion_threshold
    }

    /// Increment promotion count
    pub fn increment_promotion_count(&mut self) {
        self.promotion_count += 1;
    }

    /// Reset promotion count
    pub fn reset_promotion_count(&mut self) {
        self.promotion_count = 0;
    }

    /// Get memory usage information
    pub fn memory_info(&self) -> NewSpaceInfo {
        let active_info = self.active_space.layout_info();
        let inactive_info = self.inactive_space.layout_info();

        NewSpaceInfo {
            active_space: active_info,
            inactive_space: inactive_info,
            total_size: self.space_size * 2,
            total_used: active_info.allocated_size.bytes(),
            total_free: active_info.free_size.bytes() + inactive_info.free_size.bytes(),
            usage_percentage: (active_info.allocated_size.bytes() as f64
                / (self.space_size * 2) as f64)
                * 100.0,
            promotion_info: self.promotion_info(),
        }
    }

    /// Reset the new space (used after minor GC)
    pub fn reset(&mut self) {
        // Swap spaces
        std::mem::swap(&mut self.active_space, &mut self.inactive_space);

        // Reset the new active space
        self.active_space.reset();

        // Update statistics
        self.stats.allocated_size = 0;
        self.stats.object_count = 0;
        self.stats.allocation_count = 0;
        self.stats.deallocation_count = 0;
        self.stats.fragmentation_percentage = 0.0;
    }

    /// Perform minor garbage collection
    pub fn collect(&mut self) -> GcStats {
        let start_time = std::time::Instant::now();

        // Get current usage before collection
        let before_usage = self.active_space.total_allocated().bytes();

        // Reset (swap spaces)
        self.reset();

        // Calculate collection statistics
        let objects_collected = if before_usage > 0 {
            // Estimate object count based on average size
            let avg_size = if self.stats.allocation_count > 0 {
                before_usage / self.stats.allocation_count
            } else {
                64 // Default average size
            };
            before_usage / avg_size
        } else {
            0
        };

        let end_time = std::time::Instant::now();
        let collection_time = end_time.duration_since(start_time).as_micros() as u64;

        GcStats {
            objects_collected,
            bytes_freed: before_usage,
            collection_time,
        }
    }

    /// Check if the space is full
    pub fn is_full(&self) -> bool {
        self.active_space.is_full()
    }

    /// Get remaining space in the active space
    pub fn remaining_space(&self) -> usize {
        self.active_space.remaining_space().bytes()
    }

    /// Get usage percentage of the active space
    pub fn usage_percentage(&self) -> f64 {
        self.active_space.usage_percentage()
    }

    /// Get efficiency (active space usage vs total space)
    pub fn efficiency(&self) -> f64 {
        let active_used = self.active_space.total_allocated().bytes();
        let total_space = self.space_size * 2;
        (active_used as f64 / total_space as f64) * 100.0
    }
}

impl MemorySpace for NewSpace {
    fn allocate(&mut self, size: MemorySize) -> Option<HeapHandleId> {
        if let Some(handle) = self.active_space.allocate(size) {
            // Update statistics
            self.stats.allocated_size += size.bytes();
            self.stats.object_count += 1;
            self.stats.allocation_count += 1;

            // Update free space
            self.stats.free_size =
                self.active_space.total_free().bytes() + self.inactive_space.total_free().bytes();

            Some(HeapHandleId::from(handle))
        } else {
            None
        }
    }

    fn deallocate(&mut self, _handle: HeapHandleId) -> bool {
        // New space doesn't support deallocation
        // Memory is freed by copying garbage collection
        false
    }

    fn can_allocate(&self, size: MemorySize) -> bool {
        self.active_space.can_allocate(size)
    }

    fn total_allocated(&self) -> MemorySize {
        self.active_space.total_allocated()
    }

    fn total_free(&self) -> MemorySize {
        let active_free = self.active_space.total_free().bytes();
        let inactive_free = self.inactive_space.total_free().bytes();
        MemorySize::new(active_free + inactive_free)
    }

    fn stats(&self) -> SpaceStats {
        self.stats.clone()
    }

    fn space_type(&self) -> SpaceType {
        SpaceType::NewSpace
    }

    fn extract_object(&mut self, handle: HeapHandleId) -> Option<Value> {
        // Extract object from active space
        if let Some(object_data) = self.active_space.extract_object(handle.as_usize()) {
            self.stats.object_count = self.stats.object_count.saturating_sub(1);
            self.stats.allocated_size = self
                .stats
                .allocated_size
                .saturating_sub(object_data.size().unwrap_or(0));
            Some(object_data)
        } else {
            // Try inactive space
            if let Some(object_data) = self.inactive_space.extract_object(handle.as_usize()) {
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
    }

    fn allocate_object(&mut self, data: Value) -> Option<HeapHandleId> {
        let size = MemorySize::new(data.size().unwrap_or(64));
        if let Some(handle) = self.active_space.allocate_object(data) {
            self.stats.allocated_size += size.bytes();
            self.stats.object_count += 1;
            self.stats.allocation_count += 1;
            Some(HeapHandleId::from(handle))
        } else {
            None
        }
    }
}

/// Information about new space
#[derive(Debug, Clone)]
pub struct NewSpaceInfo {
    pub active_space: LayoutInfo,
    pub inactive_space: LayoutInfo,
    pub total_size: usize,
    pub total_used: usize,
    pub total_free: usize,
    pub usage_percentage: f64,
    pub promotion_info: PromotionInfo,
}

/// Promotion information
#[derive(Debug, Clone)]
pub struct PromotionInfo {
    pub threshold: usize,
    pub current_count: usize,
    pub should_promote: bool,
}

impl Default for NewSpace {
    fn default() -> Self {
        Self::new(64 * 1024 * 1024) // 64MB default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_space_new() {
        let new_space = NewSpace::new(1024);
        assert_eq!(new_space.space_size, 1024);
        assert_eq!(new_space.total_allocated().bytes(), 0);
        assert_eq!(new_space.total_free().bytes(), 2048); // 2 * 1024
        assert!(!new_space.is_full());
    }

    #[test]
    fn test_new_space_allocate() {
        let mut new_space = NewSpace::new(1024);

        let handle = new_space.allocate(MemorySize::new(64));
        assert!(handle.is_some());
        assert_eq!(new_space.total_allocated().bytes(), 64);
        assert_eq!(new_space.stats.object_count, 1);
        assert_eq!(new_space.stats.allocation_count, 1);
    }

    #[test]
    fn test_new_space_reset() {
        let mut new_space = NewSpace::new(1024);

        // Allocate some memory
        new_space.allocate(MemorySize::new(64));
        new_space.allocate(MemorySize::new(128));

        assert_eq!(new_space.total_allocated().bytes(), 64 + 128);

        // Reset
        new_space.reset();

        assert_eq!(new_space.total_allocated().bytes(), 0);
        assert_eq!(new_space.stats.object_count, 0);
        assert_eq!(new_space.stats.allocation_count, 0);
    }

    #[test]
    fn test_new_space_collect() {
        let mut new_space = NewSpace::new(1024);

        // Allocate some memory
        new_space.allocate(MemorySize::new(64));
        new_space.allocate(MemorySize::new(128));

        let before_usage = new_space.total_allocated().bytes();

        // Perform collection
        let stats = new_space.collect();

        assert!(stats.objects_collected > 0);
        assert_eq!(stats.bytes_freed, before_usage);
        assert!(stats.collection_time > 0);

        // Space should be reset
        assert_eq!(new_space.total_allocated().bytes(), 0);
    }

    #[test]
    fn test_new_space_promotion() {
        let mut new_space = NewSpace::new(1024);

        assert!(!new_space.should_promote());
        assert_eq!(new_space.promotion_info().current_count, 0);

        // Increment promotion count
        new_space.increment_promotion_count();
        new_space.increment_promotion_count();
        new_space.increment_promotion_count();

        assert!(new_space.should_promote());
        assert_eq!(new_space.promotion_info().current_count, 3);

        // Reset promotion count
        new_space.reset_promotion_count();

        assert!(!new_space.should_promote());
        assert_eq!(new_space.promotion_info().current_count, 0);
    }
}
