//! # Incremental Garbage Collector
//!
//! Low-latency garbage collector that performs collection in small increments.
//! Minimizes pause times for interactive applications.
//!
//! ## Characteristics
//!
//! - **Incremental Collection**: Breaks collection into small steps
//! - **Short Pause Times**: Minimal interruption to application
//! - **Adaptive Scheduling**: Adjusts collection frequency based on workload
//! - **Write Barriers**: Tracks object mutations during collection
//! - **Perfect for**: Interactive applications, real-time systems

use crate::vm::handle::HeapHandleId;
use crate::vm::memory::heap::spaces::{GcStats, MemorySpace, SpaceType};
use std::collections::{HashMap, HashSet};

/// Incremental garbage collector
pub struct IncrementalGc {
    /// Collection statistics
    stats: IncrementalGcStats,
    /// Current collection phase
    current_phase: CollectionPhase,
    /// Collection progress
    progress: CollectionProgress,
    /// Write barrier tracking
    write_barriers: WriteBarrierTracker,
    /// Collection scheduling
    scheduling: CollectionScheduling,
    /// Collection count
    collection_count: usize,
}

/// Incremental GC statistics
#[derive(Debug, Clone)]
pub struct IncrementalGcStats {
    pub collections_performed: usize,
    pub total_increments: usize,
    pub total_objects_processed: usize,
    pub total_objects_marked: usize,
    pub total_objects_swept: usize,
    pub total_bytes_freed: usize,
    pub total_collection_time_ms: u64,
    pub average_increment_time_ms: u64,
    pub last_collection_time: Option<std::time::Instant>,
    pub pause_time_violations: usize,
}

/// Collection phases for incremental GC
#[derive(Debug, Clone, PartialEq)]
pub enum CollectionPhase {
    NotCollecting,
    Marking,
    Sweeping,
    Compacting,
    Completed,
}

/// Collection progress tracking
#[derive(Debug, Clone)]
pub struct CollectionProgress {
    pub phase_progress: f64, // 0.0 to 1.0
    pub total_progress: f64, // 0.0 to 1.0
    pub objects_processed: usize,
    pub objects_total: usize,
    pub current_increment: usize,
    pub total_increments: usize,
}

/// Write barrier tracker
#[derive(Debug, Clone)]
pub struct WriteBarrierTracker {
    pub barriers_installed: usize,
    pub barriers_triggered: usize,
    pub objects_tracked: HashSet<HeapHandleId>,
    pub mutation_count: usize,
}

/// Collection scheduling
#[derive(Debug, Clone)]
pub struct CollectionScheduling {
    pub target_pause_time_ms: u64,
    pub max_increment_time_ms: u64,
    pub adaptive_factor: f64,
    pub workload_estimate: WorkloadEstimate,
    pub last_adjustment: Option<std::time::Instant>,
}

/// Workload estimation
#[derive(Debug, Clone)]
pub struct WorkloadEstimate {
    pub allocation_rate: f64,     // objects per second
    pub mutation_rate: f64,       // mutations per second
    pub collection_pressure: f64, // 0.0 to 1.0
}

impl Default for IncrementalGcStats {
    fn default() -> Self {
        Self {
            collections_performed: 0,
            total_increments: 0,
            total_objects_processed: 0,
            total_objects_marked: 0,
            total_objects_swept: 0,
            total_bytes_freed: 0,
            total_collection_time_ms: 0,
            average_increment_time_ms: 0,
            last_collection_time: None,
            pause_time_violations: 0,
        }
    }
}

impl Default for CollectionProgress {
    fn default() -> Self {
        Self {
            phase_progress: 0.0,
            total_progress: 0.0,
            objects_processed: 0,
            objects_total: 0,
            current_increment: 0,
            total_increments: 0,
        }
    }
}

impl Default for WriteBarrierTracker {
    fn default() -> Self {
        Self {
            barriers_installed: 0,
            barriers_triggered: 0,
            objects_tracked: HashSet::new(),
            mutation_count: 0,
        }
    }
}

impl Default for CollectionScheduling {
    fn default() -> Self {
        Self {
            target_pause_time_ms: 5,  // 5ms target pause time
            max_increment_time_ms: 2, // 2ms max increment time
            adaptive_factor: 1.0,
            workload_estimate: WorkloadEstimate::default(),
            last_adjustment: None,
        }
    }
}

impl Default for WorkloadEstimate {
    fn default() -> Self {
        Self {
            allocation_rate: 1000.0,  // 1000 objects per second
            mutation_rate: 500.0,     // 500 mutations per second
            collection_pressure: 0.5, // Medium pressure
        }
    }
}

impl IncrementalGc {
    /// Create a new incremental garbage collector
    pub fn new() -> Self {
        Self {
            stats: IncrementalGcStats::default(),
            current_phase: CollectionPhase::NotCollecting,
            progress: CollectionProgress::default(),
            write_barriers: WriteBarrierTracker::default(),
            scheduling: CollectionScheduling::default(),
            collection_count: 0,
        }
    }

    /// Perform incremental garbage collection
    pub fn collect(
        &mut self,
        spaces: &mut HashMap<SpaceType, Box<dyn MemorySpace>>,
    ) -> Result<GcStats, String> {
        let start_time = std::time::Instant::now();

        // Check if we should start a new collection
        if self.current_phase == CollectionPhase::NotCollecting {
            if self.should_start_collection(spaces) {
                self.start_collection(spaces)?;
            } else {
                return Ok(GcStats {
                    objects_collected: 0,
                    bytes_freed: 0,
                    collection_time: 0,
                });
            }
        }

        // Perform one increment of collection
        let increment_result = self.perform_increment(spaces)?;

        // Check if collection is complete
        if self.progress.total_progress >= 1.0 {
            self.complete_collection();
        }

        // Update statistics
        self.update_stats(&increment_result, start_time);

        Ok(increment_result)
    }

    /// Start a new collection cycle
    fn start_collection(
        &mut self,
        spaces: &mut HashMap<SpaceType, Box<dyn MemorySpace>>,
    ) -> Result<(), String> {
        self.current_phase = CollectionPhase::Marking;
        self.progress = CollectionProgress::default();
        self.collection_count += 1;

        // Estimate total work
        let total_objects = self.estimate_total_objects(spaces);
        self.progress.objects_total = total_objects;

        // Install write barriers
        self.install_write_barriers(spaces);

        // Calculate total increments needed
        let estimated_time = self.estimate_collection_time(total_objects);
        let increments_needed =
            (estimated_time / self.scheduling.max_increment_time_ms as f64).ceil() as usize;
        self.progress.total_increments = increments_needed.max(1);

        Ok(())
    }

    /// Perform one increment of collection
    fn perform_increment(
        &mut self,
        spaces: &mut HashMap<SpaceType, Box<dyn MemorySpace>>,
    ) -> Result<GcStats, String> {
        let increment_start = std::time::Instant::now();
        let target_time = self.scheduling.max_increment_time_ms;

        match self.current_phase {
            CollectionPhase::Marking => {
                self.perform_marking_increment(spaces, target_time)?;
            }
            CollectionPhase::Sweeping => {
                self.perform_sweeping_increment(spaces, target_time)?;
            }
            CollectionPhase::Compacting => {
                self.perform_compacting_increment(spaces, target_time)?;
            }
            _ => {
                return Err("Invalid collection phase".to_string());
            }
        }

        // Check increment time
        let increment_time = increment_start.elapsed().as_millis() as u64;
        if increment_time > target_time {
            self.stats.pause_time_violations += 1;
        }

        // Update progress
        self.update_progress();

        // Check if phase is complete
        if self.progress.phase_progress >= 1.0 {
            self.advance_phase();
        }

        Ok(GcStats {
            objects_collected: 0,                   // Will be updated by specific phases
            bytes_freed: 0,                         // Will be updated by specific phases
            collection_time: increment_time * 1000, // Convert to microseconds
        })
    }

    /// Perform marking increment
    fn perform_marking_increment(
        &mut self,
        spaces: &mut HashMap<SpaceType, Box<dyn MemorySpace>>,
        target_time_ms: u64,
    ) -> Result<(), String> {
        let start_time = std::time::Instant::now();
        let mut objects_processed = 0;

        // Simulate marking work
        // In a real implementation, this would:
        // 1. Process objects from the marking worklist
        // 2. Mark objects as live
        // 3. Add referenced objects to worklist

        while start_time.elapsed().as_millis() < target_time_ms as u64 {
            // Simulate processing one object
            objects_processed += 1;

            // Check if we've processed enough objects
            if objects_processed >= self.progress.objects_total / self.progress.total_increments {
                break;
            }
        }

        self.progress.objects_processed += objects_processed;
        self.stats.total_objects_marked += objects_processed;

        Ok(())
    }

    /// Perform sweeping increment
    fn perform_sweeping_increment(
        &mut self,
        spaces: &mut HashMap<SpaceType, Box<dyn MemorySpace>>,
        target_time_ms: u64,
    ) -> Result<(), String> {
        let start_time = std::time::Instant::now();
        let mut objects_processed = 0;

        // Simulate sweeping work
        // In a real implementation, this would:
        // 1. Iterate through unmarked objects
        // 2. Deallocate dead objects
        // 3. Update free lists

        while start_time.elapsed().as_millis() < target_time_ms as u64 {
            objects_processed += 1;

            if objects_processed >= self.progress.objects_total / self.progress.total_increments {
                break;
            }
        }

        self.progress.objects_processed += objects_processed;
        self.stats.total_objects_swept += objects_processed;

        Ok(())
    }

    /// Perform compacting increment
    fn perform_compacting_increment(
        &mut self,
        spaces: &mut HashMap<SpaceType, Box<dyn MemorySpace>>,
        target_time_ms: u64,
    ) -> Result<(), String> {
        let start_time = std::time::Instant::now();
        let mut objects_processed = 0;

        // Simulate compaction work
        // In a real implementation, this would:
        // 1. Move live objects to contiguous memory
        // 2. Update object references
        // 3. Update allocation pointers

        while start_time.elapsed().as_millis() < target_time_ms as u64 {
            objects_processed += 1;

            if objects_processed >= self.progress.objects_total / self.progress.total_increments {
                break;
            }
        }

        self.progress.objects_processed += objects_processed;

        Ok(())
    }

    /// Advance to next collection phase
    fn advance_phase(&mut self) {
        match self.current_phase {
            CollectionPhase::Marking => {
                self.current_phase = CollectionPhase::Sweeping;
                self.progress.phase_progress = 0.0;
            }
            CollectionPhase::Sweeping => {
                if self.should_compact() {
                    self.current_phase = CollectionPhase::Compacting;
                } else {
                    self.current_phase = CollectionPhase::Completed;
                }
                self.progress.phase_progress = 0.0;
            }
            CollectionPhase::Compacting => {
                self.current_phase = CollectionPhase::Completed;
                self.progress.phase_progress = 0.0;
            }
            _ => {}
        }
    }

    /// Complete collection cycle
    fn complete_collection(&mut self) {
        self.current_phase = CollectionPhase::NotCollecting;
        self.progress.total_progress = 1.0;

        // Remove write barriers
        self.remove_write_barriers();

        // Update workload estimates
        self.update_workload_estimates();

        // Adjust scheduling if needed
        self.adjust_scheduling();
    }

    /// Update collection progress
    fn update_progress(&mut self) {
        // Update phase progress
        let phase_objects = self.progress.objects_processed;
        let phase_total = self.progress.objects_total / self.progress.total_increments;

        if phase_total > 0 {
            self.progress.phase_progress = (phase_objects as f64 / phase_total as f64).min(1.0);
        }

        // Update total progress
        let completed_phases = match self.current_phase {
            CollectionPhase::Marking => 0,
            CollectionPhase::Sweeping => 1,
            CollectionPhase::Compacting => 2,
            CollectionPhase::Completed => 3,
            _ => 0,
        };

        let phase_weight = 1.0 / 3.0; // Equal weight for each phase
        self.progress.total_progress = (completed_phases as f64 * phase_weight)
            + (self.progress.phase_progress * phase_weight);
    }

    /// Check if collection should start
    fn should_start_collection(&self, spaces: &HashMap<SpaceType, Box<dyn MemorySpace>>) -> bool {
        // Check memory pressure
        let total_usage = self.calculate_total_usage(spaces);
        let total_size = self.calculate_total_size(spaces);

        if total_size > 0 {
            let usage_percentage = (total_usage as f64 / total_size as f64) * 100.0;
            return usage_percentage > 70.0; // Start collection at 70% usage
        }

        false
    }

    /// Estimate total objects to process
    fn estimate_total_objects(&self, spaces: &HashMap<SpaceType, Box<dyn MemorySpace>>) -> usize {
        spaces
            .values()
            .map(|space| space.stats().object_count)
            .sum()
    }

    /// Estimate collection time
    fn estimate_collection_time(&self, object_count: usize) -> f64 {
        // Simple estimation: 1 microsecond per object
        object_count as f64 * 0.001 // Convert to milliseconds
    }

    /// Install write barriers
    fn install_write_barriers(&mut self, _spaces: &mut HashMap<SpaceType, Box<dyn MemorySpace>>) {
        // In a real implementation, this would install write barriers
        // For now, just simulate
        self.write_barriers.barriers_installed += 1000;
    }

    /// Remove write barriers
    fn remove_write_barriers(&mut self) {
        self.write_barriers.barriers_installed = 0;
        self.write_barriers.barriers_triggered = 0;
        self.write_barriers.objects_tracked.clear();
    }

    /// Check if compaction is needed
    fn should_compact(&self) -> bool {
        // Simple heuristic: compact if fragmentation is high
        self.scheduling.workload_estimate.collection_pressure > 0.7
    }

    /// Update workload estimates
    fn update_workload_estimates(&mut self) {
        // In a real implementation, this would analyze actual workload
        // For now, just simulate some updates
        let mut workload = &mut self.scheduling.workload_estimate;
        workload.allocation_rate *= 0.95; // Slight decrease
        workload.mutation_rate *= 0.95; // Slight decrease
        workload.collection_pressure *= 0.8; // Reduce pressure after collection
    }

    /// Adjust collection scheduling
    fn adjust_scheduling(&mut self) {
        let now = std::time::Instant::now();

        // Adjust every 10 collections
        if self.collection_count % 10 == 0 {
            if let Some(last_adjustment) = self.scheduling.last_adjustment {
                let time_since = now.duration_since(last_adjustment).as_secs();

                if time_since > 60 {
                    // Adjust every minute
                    self.adjust_scheduling_parameters();
                    self.scheduling.last_adjustment = Some(now);
                }
            } else {
                self.scheduling.last_adjustment = Some(now);
            }
        }
    }

    /// Adjust scheduling parameters
    fn adjust_scheduling_parameters(&mut self) {
        let pressure = self.scheduling.workload_estimate.collection_pressure;

        if pressure > 0.8 {
            // High pressure: reduce pause times
            self.scheduling.max_increment_time_ms =
                (self.scheduling.max_increment_time_ms as f64 * 0.9) as u64;
        } else if pressure < 0.3 {
            // Low pressure: increase pause times for efficiency
            self.scheduling.max_increment_time_ms =
                (self.scheduling.max_increment_time_ms as f64 * 1.1) as u64;
        }

        // Ensure bounds
        self.scheduling.max_increment_time_ms =
            self.scheduling.max_increment_time_ms.max(1).min(10);
    }

    /// Calculate total memory usage
    fn calculate_total_usage(&self, spaces: &HashMap<SpaceType, Box<dyn MemorySpace>>) -> usize {
        spaces
            .values()
            .map(|space| space.total_allocated().as_usize())
            .sum()
    }

    /// Calculate total memory size
    fn calculate_total_size(&self, spaces: &HashMap<SpaceType, Box<dyn MemorySpace>>) -> usize {
        spaces.values().map(|space| space.stats().total_size).sum()
    }

    /// Update collection statistics
    fn update_stats(&mut self, result: &GcStats, start_time: std::time::Instant) {
        self.stats.total_increments += 1;
        self.stats.total_collection_time_ms += result.collection_time / 1000;
        self.stats.average_increment_time_ms =
            self.stats.total_collection_time_ms / self.stats.total_increments as u64;

        if self.current_phase == CollectionPhase::NotCollecting {
            self.stats.collections_performed += 1;
            self.stats.last_collection_time = Some(start_time);
        }
    }

    /// Get collection statistics
    pub fn stats(&self) -> &IncrementalGcStats {
        &self.stats
    }

    /// Get current collection phase
    pub fn current_phase(&self) -> &CollectionPhase {
        &self.current_phase
    }

    /// Get collection progress
    pub fn progress(&self) -> &CollectionProgress {
        &self.progress
    }

    /// Get write barrier information
    pub fn write_barrier_info(&self) -> &WriteBarrierTracker {
        &self.write_barriers
    }

    /// Get scheduling information
    pub fn scheduling_info(&self) -> &CollectionScheduling {
        &self.scheduling
    }

    /// Check if collection is active
    pub fn is_collecting(&self) -> bool {
        self.current_phase != CollectionPhase::NotCollecting
    }

    /// Get collection completion percentage
    pub fn completion_percentage(&self) -> f64 {
        self.progress.total_progress * 100.0
    }

    /// Update target pause time
    pub fn update_target_pause_time(&mut self, target_ms: u64) {
        self.scheduling.target_pause_time_ms = target_ms;
    }

    /// Update max increment time
    pub fn update_max_increment_time(&mut self, max_ms: u64) {
        self.scheduling.max_increment_time_ms = max_ms;
    }
}

impl Default for IncrementalGc {
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
    fn test_incremental_gc_new() {
        let gc = IncrementalGc::new();
        assert_eq!(gc.stats.collections_performed, 0);
        assert_eq!(gc.current_phase(), &CollectionPhase::NotCollecting);
        assert!(!gc.is_collecting());
        assert_eq!(gc.completion_percentage(), 0.0);
    }

    #[test]
    fn test_incremental_gc_scheduling() {
        let mut gc = IncrementalGc::new();

        // Update scheduling parameters
        gc.update_target_pause_time(10);
        gc.update_max_increment_time(5);

        let scheduling = gc.scheduling_info();
        assert_eq!(scheduling.target_pause_time_ms, 10);
        assert_eq!(scheduling.max_increment_time_ms, 5);
    }

    #[test]
    fn test_incremental_gc_collection_start() {
        let mut gc = IncrementalGc::new();

        // Create mock spaces with high usage
        let mut spaces: HashMap<SpaceType, Box<dyn MemorySpace>> = HashMap::new();

        let space = MockMemorySpace {
            stats: SpaceStats {
                space_type: SpaceType::NewSpace,
                total_size: 1000,
                allocated_size: 800, // 80% usage
                free_size: 200,
                object_count: 100,
                fragmentation_percentage: 0.0,
                allocation_count: 100,
                deallocation_count: 0,
            },
            allocated: 800,
        };

        spaces.insert(SpaceType::NewSpace, Box::new(space));

        // Should start collection due to high usage
        assert!(gc.should_start_collection(&spaces));
    }

    #[test]
    fn test_incremental_gc_collection_cycle() {
        let mut gc = IncrementalGc::new();

        // Create mock spaces
        let mut spaces: HashMap<SpaceType, Box<dyn MemorySpace>> = HashMap::new();

        let space = MockMemorySpace {
            stats: SpaceStats {
                space_type: SpaceType::NewSpace,
                total_size: 1000,
                allocated_size: 800,
                free_size: 200,
                object_count: 100,
                fragmentation_percentage: 0.0,
                allocation_count: 100,
                deallocation_count: 0,
            },
            allocated: 800,
        };

        spaces.insert(SpaceType::NewSpace, Box::new(space));

        // Start collection
        gc.start_collection(&mut spaces).unwrap();
        assert!(gc.is_collecting());
        assert_eq!(gc.current_phase(), &CollectionPhase::Marking);

        // Perform increments until completion
        let mut increment_count = 0;
        while gc.is_collecting() && increment_count < 100 {
            let result = gc.collect(&mut spaces);
            assert!(result.is_ok());
            increment_count += 1;
        }

        // Collection should be complete
        assert!(!gc.is_collecting());
        assert_eq!(gc.completion_percentage(), 100.0);
        assert_eq!(gc.current_phase(), &CollectionPhase::NotCollecting);
    }

    #[test]
    fn test_incremental_gc_progress_tracking() {
        let mut gc = IncrementalGc::new();

        // Create mock spaces
        let mut spaces: HashMap<SpaceType, Box<dyn MemorySpace>> = HashMap::new();

        let space = MockMemorySpace {
            stats: SpaceStats {
                space_type: SpaceType::NewSpace,
                total_size: 1000,
                allocated_size: 800,
                free_size: 200,
                object_count: 100,
                fragmentation_percentage: 0.0,
                allocation_count: 100,
                deallocation_count: 0,
            },
            allocated: 800,
        };

        spaces.insert(SpaceType::NewSpace, Box::new(space));

        // Start collection
        gc.start_collection(&mut spaces).unwrap();

        // Check initial progress
        let progress = gc.progress();
        assert_eq!(progress.total_progress, 0.0);
        assert_eq!(progress.phase_progress, 0.0);

        // Perform one increment
        let result = gc.collect(&mut spaces);
        assert!(result.is_ok());

        // Progress should have increased
        let progress = gc.progress();
        assert!(progress.total_progress > 0.0);
        assert!(progress.phase_progress > 0.0);
    }

    #[test]
    fn test_incremental_gc_write_barriers() {
        let mut gc = IncrementalGc::new();

        // Create mock spaces
        let mut spaces: HashMap<SpaceType, Box<dyn MemorySpace>> = HashMap::new();

        let space = MockMemorySpace {
            stats: SpaceStats {
                space_type: SpaceType::NewSpace,
                total_size: 1000,
                allocated_size: 800,
                free_size: 200,
                object_count: 100,
                fragmentation_percentage: 0.0,
                allocation_count: 100,
                deallocation_count: 0,
            },
            allocated: 800,
        };

        spaces.insert(SpaceType::NewSpace, Box::new(space));

        // Start collection (installs write barriers)
        gc.start_collection(&mut spaces).unwrap();

        let barrier_info = gc.write_barrier_info();
        assert!(barrier_info.barriers_installed > 0);

        // Complete collection (removes write barriers)
        while gc.is_collecting() {
            let _ = gc.collect(&mut spaces);
        }

        let barrier_info = gc.write_barrier_info();
        assert_eq!(barrier_info.barriers_installed, 0);
        assert_eq!(barrier_info.barriers_triggered, 0);
    }
}
