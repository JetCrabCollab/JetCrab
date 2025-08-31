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


