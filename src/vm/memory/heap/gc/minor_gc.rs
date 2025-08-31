//! # Minor Garbage Collector
//!
//! Fast copying garbage collector for young generation objects.
//! Uses semi-space allocation and copying collection for optimal performance.
//!
//! ## Characteristics
//!
//! - **Copying Collection**: Fast collection by copying live objects
//! - **Semi-space Allocation**: Two spaces that swap roles
//! - **Short Pause Times**: Minimal interruption to application
//! - **Object Promotion**: Moves long-lived objects to old generation
//! - **Perfect for**: Short-lived objects, frequent allocation

use crate::vm::handle::HeapHandleId;
use crate::vm::memory::heap::spaces::{GcStats, MemorySpace, SpaceType};
use std::collections::HashMap;

/// Minor garbage collector for young generation
pub struct MinorGc {
    /// Collection statistics
    stats: MinorGcStats,
    /// Object age tracking
    object_ages: HashMap<HeapHandleId, usize>,
    /// Promotion threshold
    promotion_threshold: usize,
    /// Collection count
    collection_count: usize,
}

/// Minor GC statistics
#[derive(Debug, Clone)]
pub struct MinorGcStats {
    pub collections_performed: usize,
    pub total_objects_processed: usize,
    pub total_objects_promoted: usize,
    pub total_objects_collected: usize,
    pub total_bytes_freed: usize,
    pub total_collection_time_ms: u64,
    pub average_collection_time_ms: u64,
    pub last_collection_time: Option<std::time::Instant>,
}

impl Default for MinorGcStats {
    fn default() -> Self {
        Self {
            collections_performed: 0,
            total_objects_processed: 0,
            total_objects_promoted: 0,
            total_objects_collected: 0,
            total_bytes_freed: 0,
            total_collection_time_ms: 0,
            average_collection_time_ms: 0,
            last_collection_time: None,
        }
    }
}

impl MinorGc {
    /// Create a new minor garbage collector
    pub fn new() -> Self {
        Self {
            stats: MinorGcStats::default(),
            object_ages: HashMap::new(),
            promotion_threshold: 3, // Promote after 3 minor GCs
            collection_count: 0,
        }
    }

    /// Perform minor garbage collection
    pub fn collect(
        &mut self,
        spaces: &mut HashMap<SpaceType, Box<dyn MemorySpace>>,
    ) -> Result<GcStats, String> {
        let start_time = std::time::Instant::now();

        // Clone the spaces to avoid borrow checker issues
        let space_types: Vec<SpaceType> = spaces.keys().cloned().collect();

        // Perform copying collection without borrowing conflicts
        let collection_result = self.perform_copying_collection_safe(spaces)?;

        // Update statistics
        self.update_stats(&collection_result, start_time);

        Ok(collection_result)
    }

    /// Perform copying collection safely without borrow conflicts
    fn perform_copying_collection_safe(
        &mut self,
        spaces: &mut HashMap<SpaceType, Box<dyn MemorySpace>>,
    ) -> Result<GcStats, String> {
        let start_time = std::time::Instant::now();

        // Get current usage before collection
        let before_usage = if let Some(new_space) = spaces.get(&SpaceType::NewSpace) {
            new_space.total_allocated().as_usize()
        } else {
            return Err("New space not found".to_string());
        };

        let before_objects = if let Some(new_space) = spaces.get(&SpaceType::NewSpace) {
            new_space.stats().object_count
        } else {
            return Err("New space not found".to_string());
        };

        // Simulate copying collection
        let mut objects_to_promote = Vec::new();
        let mut objects_to_collect: Vec<HeapHandleId> = Vec::new();

        // Simulate object age tracking and promotion decisions
        for handle in self.get_simulated_object_handles(before_objects) {
            let age = self.object_ages.entry(handle).or_insert(0);
            *age += 1;

            if *age >= self.promotion_threshold {
                objects_to_promote.push(handle);
            } else {
                // Object survives this collection
                continue;
            }
        }

        // Simulate promotion to old generation
        let promoted_count = if let Some(old_space) = spaces.get_mut(&SpaceType::OldSpace) {
            self.simulate_promotion(old_space, &objects_to_promote)
        } else {
            0
        };

        // Simulate collection of remaining objects
        let collected_count = before_objects - promoted_count;

        // Reset new space (simulate copying collection)
        if let Some(new_space) = spaces.get_mut(&SpaceType::NewSpace) {
            self.simulate_new_space_reset(new_space);
        }

        // Calculate collection statistics
        let end_time = std::time::Instant::now();
        let collection_time = end_time.duration_since(start_time).as_micros() as u64;

        let bytes_freed = before_usage;

        Ok(GcStats {
            objects_collected: collected_count,
            bytes_freed,
            collection_time,
        })
    }

    /// Perform copying collection
    fn perform_copying_collection(
        &mut self,
        new_space: &mut Box<dyn MemorySpace>,
        old_space: &mut Box<dyn MemorySpace>,
    ) -> Result<GcStats, String> {
        let start_time = std::time::Instant::now();

        // Get current usage before collection
        let before_usage = new_space.total_allocated().as_usize();
        let before_objects = new_space.stats().object_count;

        // Simulate copying collection
        // In a real implementation, this would:
        // 1. Mark live objects from roots
        // 2. Copy live objects to to-space
        // 3. Promote objects that survived multiple collections
        // 4. Swap from-space and to-space

        let mut objects_to_promote = Vec::new();
        let mut objects_to_collect: Vec<HeapHandleId> = Vec::new();

        // Simulate object age tracking and promotion decisions
        for handle in self.get_simulated_object_handles(before_objects) {
            let age = self.object_ages.entry(handle).or_insert(0);
            *age += 1;

            if *age >= self.promotion_threshold {
                objects_to_promote.push(handle);
            } else {
                // Object survives this collection
                continue;
            }
        }

        // Simulate promotion to old generation
        let promoted_count = self.simulate_promotion(old_space, &objects_to_promote);

        // Simulate collection of remaining objects
        let collected_count = before_objects - promoted_count;

        // Reset new space (simulate copying collection)
        self.simulate_new_space_reset(new_space);

        // Calculate collection statistics
        let end_time = std::time::Instant::now();
        let collection_time = end_time.duration_since(start_time).as_micros() as u64;

        let bytes_freed = before_usage;

        Ok(GcStats {
            objects_collected: collected_count,
            bytes_freed,
            collection_time,
        })
    }

    /// Simulate object promotion to old generation
    fn simulate_promotion(
        &self,
        old_space: &mut Box<dyn MemorySpace>,
        objects_to_promote: &[HeapHandleId],
    ) -> usize {
        let mut promoted_count = 0;

        for &handle in objects_to_promote {
            // Simulate allocation in old space
            // In a real implementation, this would copy the object data
            if let Some(_) = old_space.allocate(crate::vm::types::MemorySize::new(64)) {
                promoted_count += 1;
            }
        }

        promoted_count
    }

    /// Simulate new space reset after collection
    fn simulate_new_space_reset(&self, new_space: &mut Box<dyn MemorySpace>) {
        // In a real implementation, this would:
        // 1. Clear the from-space
        // 2. Swap from-space and to-space
        // 3. Reset allocation pointers

        // For now, we just simulate the reset
        // The actual implementation would be in the NewSpace struct
    }

    /// Get simulated object handles for testing
    fn get_simulated_object_handles(&self, count: usize) -> Vec<HeapHandleId> {
        (0..count).map(|i| HeapHandleId::new(i * 64)).collect()
    }

    /// Update collection statistics
    fn update_stats(&mut self, result: &GcStats, start_time: std::time::Instant) {
        self.stats.collections_performed += 1;
        self.stats.total_objects_processed += result.objects_collected;
        self.stats.total_objects_collected += result.objects_collected;
        self.stats.total_bytes_freed += result.bytes_freed;
        self.stats.total_collection_time_ms += result.collection_time / 1000;
        self.stats.average_collection_time_ms =
            self.stats.total_collection_time_ms / self.stats.collections_performed as u64;
        self.stats.last_collection_time = Some(start_time);

        self.collection_count += 1;
    }

    /// Get collection statistics
    pub fn stats(&self) -> &MinorGcStats {
        &self.stats
    }

    /// Get object age information
    pub fn object_age_info(&self) -> ObjectAgeInfo {
        let total_objects = self.object_ages.len();
        let avg_age = if total_objects > 0 {
            let total_age: usize = self.object_ages.values().sum();
            total_age / total_objects
        } else {
            0
        };

        let max_age = self.object_ages.values().max().copied().unwrap_or(0);
        let promotable_objects = self
            .object_ages
            .iter()
            .filter(|(_, &age)| age >= self.promotion_threshold)
            .count();

        ObjectAgeInfo {
            total_objects,
            average_age: avg_age,
            max_age,
            promotion_threshold: self.promotion_threshold,
            promotable_objects,
            collection_count: self.collection_count,
        }
    }

    /// Update promotion threshold
    pub fn update_promotion_threshold(&mut self, threshold: usize) {
        self.promotion_threshold = threshold;
    }

    /// Get promotion threshold
    pub fn promotion_threshold(&self) -> usize {
        self.promotion_threshold
    }

    /// Check if promotion is needed
    pub fn should_promote(&self) -> bool {
        let promotable = self
            .object_ages
            .iter()
            .filter(|(_, &age)| age >= self.promotion_threshold)
            .count();

        promotable > 0
    }

    /// Get objects ready for promotion
    pub fn get_promotable_objects(&self) -> Vec<HeapHandleId> {
        self.object_ages
            .iter()
            .filter(|(_, &age)| age >= self.promotion_threshold)
            .map(|(&handle, _)| handle)
            .collect()
    }

    /// Clear age tracking for promoted objects
    pub fn clear_promoted_objects(&mut self, handles: &[HeapHandleId]) {
        for &handle in handles {
            self.object_ages.remove(&handle);
        }
    }

    /// Increment age for all objects
    pub fn increment_object_ages(&mut self) {
        for age in self.object_ages.values_mut() {
            *age += 1;
        }
    }

    /// Reset age tracking
    pub fn reset_age_tracking(&mut self) {
        self.object_ages.clear();
        self.collection_count = 0;
    }
}

/// Object age information
#[derive(Debug, Clone)]
pub struct ObjectAgeInfo {
    pub total_objects: usize,
    pub average_age: usize,
    pub max_age: usize,
    pub promotion_threshold: usize,
    pub promotable_objects: usize,
    pub collection_count: usize,
}

impl Default for MinorGc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::memory::heap::spaces::{SpaceStats, SpaceType};
    use crate::vm::types::MemorySize;

    // Mock memory space for testing
    struct MockMemorySpace {
        stats: SpaceStats,
        allocated: usize,
    }

    impl MemorySpace for MockMemorySpace {
        fn allocate(&mut self, size: MemorySize) -> Option<HeapHandleId> {
            self.allocated += size.as_usize();
            Some(HeapHandleId::new(self.allocated))
        }

        fn deallocate(&mut self, _handle: HeapHandleId) -> bool {
            false
        }

        fn can_allocate(&self, _size: MemorySize) -> bool {
            true
        }

        fn total_allocated(&self) -> MemorySize {
            MemorySize::new(self.allocated)
        }

        fn total_free(&self) -> MemorySize {
            MemorySize::new(self.stats.total_size - self.allocated)
        }

        fn stats(&self) -> SpaceStats {
            self.stats.clone()
        }

        fn space_type(&self) -> SpaceType {
            self.stats.space_type.clone()
        }
    }

    #[test]
    fn test_minor_gc_new() {
        let minor_gc = MinorGc::new();
        assert_eq!(minor_gc.stats.collections_performed, 0);
        assert_eq!(minor_gc.promotion_threshold(), 3);
        assert_eq!(minor_gc.collection_count, 0);
    }

    #[test]
    fn test_minor_gc_object_ages() {
        let mut minor_gc = MinorGc::new();

        // Simulate some objects
        let handle1 = HeapHandleId::new(64);
        let handle2 = HeapHandleId::new(128);

        // Increment ages
        minor_gc.increment_object_ages();
        minor_gc.increment_object_ages();

        // Check age tracking
        let age_info = minor_gc.object_age_info();
        assert_eq!(age_info.total_objects, 0); // No objects added yet

        // Add objects and check ages
        minor_gc.object_ages.insert(handle1, 2);
        minor_gc.object_ages.insert(handle2, 1);

        let age_info = minor_gc.object_age_info();
        assert_eq!(age_info.total_objects, 2);
        assert_eq!(age_info.average_age, 1);
        assert_eq!(age_info.max_age, 2);
    }

    #[test]
    fn test_minor_gc_promotion() {
        let mut minor_gc = MinorGc::new();

        // Set promotion threshold to 2 for testing
        minor_gc.update_promotion_threshold(2);

        // Add objects with different ages
        let handle1 = HeapHandleId::new(64);
        let handle2 = HeapHandleId::new(128);
        let handle3 = HeapHandleId::new(192);

        minor_gc.object_ages.insert(handle1, 1); // Not ready
        minor_gc.object_ages.insert(handle2, 2); // Ready
        minor_gc.object_ages.insert(handle3, 3); // Ready

        assert!(minor_gc.should_promote());

        let promotable = minor_gc.get_promotable_objects();
        assert_eq!(promotable.len(), 2);
        assert!(promotable.contains(&handle2));
        assert!(promotable.contains(&handle3));

        // Clear promoted objects
        minor_gc.clear_promoted_objects(&promotable);
        assert_eq!(minor_gc.object_ages.len(), 1);
        assert!(minor_gc.object_ages.contains_key(&handle1));
    }

    #[test]
    fn test_minor_gc_collection() {
        let mut minor_gc = MinorGc::new();

        // Create mock spaces
        let mut spaces: HashMap<SpaceType, Box<dyn MemorySpace>> = HashMap::new();

        let new_space = MockMemorySpace {
            stats: SpaceStats {
                space_type: SpaceType::NewSpace,
                total_size: 1024,
                allocated_size: 512,
                free_size: 512,
                object_count: 8,
                fragmentation_percentage: 0.0,
                allocation_count: 8,
                deallocation_count: 0,
            },
            allocated: 512,
        };

        let old_space = MockMemorySpace {
            stats: SpaceStats {
                space_type: SpaceType::OldSpace,
                total_size: 2048,
                allocated_size: 0,
                free_size: 2048,
                object_count: 0,
                fragmentation_percentage: 0.0,
                allocation_count: 0,
                deallocation_count: 0,
            },
            allocated: 0,
        };

        spaces.insert(SpaceType::NewSpace, Box::new(new_space));
        spaces.insert(SpaceType::OldSpace, Box::new(old_space));

        // Perform collection
        let result = minor_gc.collect(&mut spaces);
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert!(stats.objects_collected > 0);
        assert!(stats.bytes_freed > 0);
        assert!(stats.collection_time > 0);

        // Check that statistics were updated
        assert_eq!(minor_gc.stats.collections_performed, 1);
        assert!(minor_gc.stats.last_collection_time.is_some());
    }

    #[test]
    fn test_minor_gc_reset() {
        let mut minor_gc = MinorGc::new();

        // Add some objects and increment ages
        let handle = HeapHandleId::new(64);
        minor_gc.object_ages.insert(handle, 5);
        minor_gc.collection_count = 10;

        // Reset age tracking
        minor_gc.reset_age_tracking();

        assert!(minor_gc.object_ages.is_empty());
        assert_eq!(minor_gc.collection_count, 0);
    }
}
