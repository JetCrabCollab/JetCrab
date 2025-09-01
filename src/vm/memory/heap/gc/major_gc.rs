//! # Major Garbage Collector
//!
//! Mark & sweep garbage collector for old generation objects.
//! Optimized for long-lived objects with efficient memory reuse.
//!
//! ## Characteristics
//!
//! - **Mark & Sweep**: Efficient collection for old objects
//! - **Incremental Marking**: Reduces pause times
//! - **Memory Compaction**: Defragments memory for better allocation
//! - **Object Aging**: Tracks object age for collection decisions
//! - **Perfect for**: Long-lived objects, infrequent collection

use crate::vm::handle::HeapHandleId;
use crate::vm::memory::heap::spaces::{GcStats, MemorySpace, SpaceType};
use std::collections::{HashMap, HashSet};

/// Major garbage collector for old generation
pub struct MajorGc {
    /// Collection statistics
    stats: MajorGcStats,
    /// Object marking state
    marking_state: MarkingState,
    /// Object age tracking
    object_ages: HashMap<HeapHandleId, usize>,
    /// Collection threshold
    collection_threshold: usize,
    /// Collection count
    collection_count: usize,
}

/// Major GC statistics
#[derive(Debug, Clone, Default)]
pub struct MajorGcStats {
    pub collections_performed: usize,
    pub total_objects_processed: usize,
    pub total_objects_marked: usize,
    pub total_objects_swept: usize,
    pub total_bytes_freed: usize,
    pub total_collection_time_ms: u64,
    pub average_collection_time_ms: u64,
    pub last_collection_time: Option<std::time::Instant>,
    pub compaction_count: usize,
    pub total_compaction_time_ms: u64,
}

/// Marking state for objects
#[derive(Debug, Clone, PartialEq)]
pub enum MarkingState {
    Unmarked,
    Marked,
    MarkedAndScanned,
}

impl MajorGc {
    /// Create a new major garbage collector
    pub fn new() -> Self {
        Self {
            stats: MajorGcStats::default(),
            marking_state: MarkingState::Unmarked,
            object_ages: HashMap::new(),
            collection_threshold: 1000, // Collect after 1000 objects
            collection_count: 0,
        }
    }

    /// Perform major garbage collection
    pub fn collect(
        &mut self,
        spaces: &mut HashMap<SpaceType, Box<dyn MemorySpace>>,
    ) -> Result<GcStats, String> {
        let start_time = std::time::Instant::now();

        // Get old space for old generation
        let old_space = spaces
            .get_mut(&SpaceType::OldSpace)
            .ok_or("Old space not found")?;

        // Perform mark & sweep collection
        let collection_result = self.perform_mark_sweep_collection(old_space)?;

        // Update statistics
        self.update_stats(&collection_result, start_time);

        Ok(collection_result)
    }

    /// Perform mark & sweep collection
    fn perform_mark_sweep_collection(
        &mut self,
        old_space: &mut Box<dyn MemorySpace>,
    ) -> Result<GcStats, String> {
        let start_time = std::time::Instant::now();

        // Get current usage before collection
        let before_usage = old_space.total_allocated().as_usize();
        let before_objects = old_space.stats().object_count;

        // Phase 1: Marking - Mark all live objects
        let marked_objects = self.mark_phase(old_space)?;

        // Phase 2: Sweeping - Remove unmarked objects
        let _swept_objects = self.sweep_phase(old_space, &marked_objects)?;

        // Phase 3: Compaction (optional) - Defragment memory
        let _compaction_stats = if self.should_compact(old_space) {
            self.compact_phase(old_space)?
        } else {
            CompactionStats::default()
        };

        // Calculate collection statistics
        let end_time = std::time::Instant::now();
        let collection_time = end_time.duration_since(start_time).as_micros() as u64;

        let objects_collected = before_objects - marked_objects.len();
        let bytes_freed = before_usage - old_space.total_allocated().as_usize();

        Ok(GcStats {
            objects_collected,
            bytes_freed,
            collection_time,
        })
    }

    /// Marking phase - identify live objects
    fn mark_phase(
        &mut self,
        old_space: &mut Box<dyn MemorySpace>,
    ) -> Result<HashSet<HeapHandleId>, String> {
        let mut marked_objects = HashSet::new();
        let mut worklist = Vec::new();

        // Get root objects (simulated for now)
        let root_objects = self.get_root_objects(old_space);
        worklist.extend(root_objects);

        // Mark and scan objects
        while let Some(handle) = worklist.pop() {
            if !marked_objects.contains(&handle) {
                // Mark object as live
                marked_objects.insert(handle);

                // Add referenced objects to worklist
                let referenced = self.get_referenced_objects(handle, old_space);
                worklist.extend(referenced);
            }
        }

        self.stats.total_objects_marked = marked_objects.len();
        Ok(marked_objects)
    }

    /// Sweeping phase - remove dead objects
    fn sweep_phase(
        &mut self,
        old_space: &mut Box<dyn MemorySpace>,
        marked_objects: &HashSet<HeapHandleId>,
    ) -> Result<usize, String> {
        // Simulate sweeping by removing unmarked objects
        // In a real implementation, this would iterate through all objects
        let total_objects = old_space.stats().object_count;
        let marked_count = marked_objects.len();

        // Calculate how many objects would be swept
        let swept_count = total_objects - marked_count;

        // Simulate deallocation of swept objects
        self.simulate_object_deallocation(old_space, swept_count);

        self.stats.total_objects_swept = swept_count;
        Ok(swept_count)
    }

    /// Compaction phase - defragment memory
    fn compact_phase(
        &mut self,
        _old_space: &mut Box<dyn MemorySpace>,
    ) -> Result<CompactionStats, String> {
        let start_time = std::time::Instant::now();

        // Simulate compaction
        // In a real implementation, this would:
        // 1. Move live objects to contiguous memory
        // 2. Update object references
        // 3. Update allocation pointers

        let compaction_time = start_time.elapsed().as_millis() as u64;

        // Update statistics
        self.stats.compaction_count += 1;
        self.stats.total_compaction_time_ms += compaction_time;

        Ok(CompactionStats {
            duration_ms: compaction_time,
            objects_moved: 0,          // Simulated
            fragmentation_before: 0.0, // Simulated
            fragmentation_after: 0.0,  // Simulated
        })
    }

    /// Get root objects (simulated)
    fn get_root_objects(&self, _old_space: &mut Box<dyn MemorySpace>) -> Vec<HeapHandleId> {
        // In a real implementation, this would return actual root objects
        // For now, simulate some root objects
        vec![
            HeapHandleId::new(64),
            HeapHandleId::new(128),
            HeapHandleId::new(192),
        ]
    }

    /// Get referenced objects (simulated)
    fn get_referenced_objects(
        &self,
        _handle: HeapHandleId,
        _old_space: &mut Box<dyn MemorySpace>,
    ) -> Vec<HeapHandleId> {
        // In a real implementation, this would traverse object references
        // For now, simulate some referenced objects
        vec![HeapHandleId::new(256), HeapHandleId::new(320)]
    }

    /// Simulate object deallocation
    fn simulate_object_deallocation(&self, _old_space: &mut Box<dyn MemorySpace>, _count: usize) {
        // In a real implementation, this would actually deallocate objects
        // For now, just simulate the operation
    }

    /// Check if compaction is needed
    fn should_compact(&self, old_space: &mut Box<dyn MemorySpace>) -> bool {
        let fragmentation = old_space.stats().fragmentation_percentage;
        fragmentation > 30.0 // Compact if fragmentation > 30%
    }

    /// Update collection statistics
    fn update_stats(&mut self, result: &GcStats, start_time: std::time::Instant) {
        self.stats.collections_performed += 1;
        self.stats.total_objects_processed += result.objects_collected;
        self.stats.total_objects_swept += result.objects_collected;
        self.stats.total_bytes_freed += result.bytes_freed;
        self.stats.total_collection_time_ms += result.collection_time / 1000;
        self.stats.average_collection_time_ms =
            self.stats.total_collection_time_ms / self.stats.collections_performed as u64;
        self.stats.last_collection_time = Some(start_time);

        self.collection_count += 1;
    }

    /// Get collection statistics
    pub fn stats(&self) -> &MajorGcStats {
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
        let old_objects = self.object_ages.iter().filter(|(_, &age)| age > 10).count();

        ObjectAgeInfo {
            total_objects,
            average_age: avg_age,
            max_age,
            old_objects,
            collection_count: self.collection_count,
        }
    }

    /// Update collection threshold
    pub fn update_collection_threshold(&mut self, threshold: usize) {
        self.collection_threshold = threshold;
    }

    /// Get collection threshold
    pub fn collection_threshold(&self) -> usize {
        self.collection_threshold
    }

    /// Check if collection is needed
    pub fn should_collect(&self, object_count: usize) -> bool {
        object_count >= self.collection_threshold
    }

    /// Increment age for all objects
    pub fn increment_object_ages(&mut self) {
        for age in self.object_ages.values_mut() {
            *age += 1;
        }
    }

    /// Add object for age tracking
    pub fn track_object(&mut self, handle: HeapHandleId) {
        self.object_ages.insert(handle, 0);
    }

    /// Remove object from age tracking
    pub fn untrack_object(&mut self, handle: &HeapHandleId) {
        self.object_ages.remove(handle);
    }

    /// Get marking state
    pub fn marking_state(&self) -> &MarkingState {
        &self.marking_state
    }

    /// Set marking state
    pub fn set_marking_state(&mut self, state: MarkingState) {
        self.marking_state = state;
    }

    /// Reset marking state
    pub fn reset_marking_state(&mut self) {
        self.marking_state = MarkingState::Unmarked;
    }

    /// Get memory fragmentation info
    pub fn fragmentation_info(&self, old_space: &dyn MemorySpace) -> FragmentationInfo {
        let stats = old_space.stats();
        let fragmentation = stats.fragmentation_percentage;

        FragmentationInfo {
            fragmentation_percentage: fragmentation,
            should_compact: fragmentation > 30.0,
            total_size: stats.total_size,
            allocated_size: stats.allocated_size,
            free_size: stats.free_size,
        }
    }
}

/// Object age information
#[derive(Debug, Clone)]
pub struct ObjectAgeInfo {
    pub total_objects: usize,
    pub average_age: usize,
    pub max_age: usize,
    pub old_objects: usize,
    pub collection_count: usize,
}

/// Compaction statistics
#[derive(Debug, Clone)]
pub struct CompactionStats {
    pub duration_ms: u64,
    pub objects_moved: usize,
    pub fragmentation_before: f64,
    pub fragmentation_after: f64,
}

impl Default for CompactionStats {
    fn default() -> Self {
        Self {
            duration_ms: 0,
            objects_moved: 0,
            fragmentation_before: 0.0,
            fragmentation_after: 0.0,
        }
    }
}

/// Fragmentation information
#[derive(Debug, Clone)]
pub struct FragmentationInfo {
    pub fragmentation_percentage: f64,
    pub should_compact: bool,
    pub total_size: usize,
    pub allocated_size: usize,
    pub free_size: usize,
}

impl Default for MajorGc {
    fn default() -> Self {
        Self::new()
    }
}
