//! # Garbage Collection System
//!
//! Advanced garbage collection system with multiple strategies and optimizations.
//! Provides generational collection, incremental collection, and background collection.
//!
//! ## Features
//!
//! - **Generational Collection**: Separate strategies for young and old objects
//! - **Incremental Collection**: Short pause times for interactive applications
//! - **Background Collection**: Non-blocking collection in background threads
//! - **Adaptive Collection**: Automatically adjusts collection frequency
//! - **Memory Pressure Detection**: Triggers collection based on memory pressure

pub mod background;
pub mod incremental;
pub mod major_gc;
pub mod minor_gc;

pub use background::BackgroundGc;
pub use incremental::IncrementalGc;
pub use major_gc::MajorGc;
pub use minor_gc::MinorGc;

use crate::vm::handle::HeapHandleId;
use crate::vm::memory::heap::spaces::{MemorySpace, SpaceType};
use std::collections::HashMap;

/// Main garbage collector that orchestrates all collection strategies
pub struct GarbageCollector {
    /// Minor GC for young generation
    minor_gc: MinorGc,
    /// Major GC for old generation
    major_gc: MajorGc,
    /// Incremental GC for low-latency applications
    incremental_gc: IncrementalGc,
    /// Background GC for non-blocking collection
    background_gc: BackgroundGc,
    /// Collection statistics
    stats: GcStats,
    /// Collection triggers and thresholds
    triggers: GcTriggers,
    /// Memory spaces being managed
    spaces: HashMap<SpaceType, Box<dyn MemorySpace>>,
}

/// Garbage collection statistics
#[derive(Debug, Clone)]
pub struct GcStats {
    pub total_collections: usize,
    pub minor_collections: usize,
    pub major_collections: usize,
    pub incremental_collections: usize,
    pub background_collections: usize,
    pub total_objects_collected: usize,
    pub total_bytes_freed: usize,
    pub total_collection_time_ms: u64,
    pub average_collection_time_ms: u64,
    pub last_collection_time: Option<std::time::Instant>,
}

/// Garbage collection triggers and thresholds
#[derive(Debug, Clone)]
pub struct GcTriggers {
    /// Memory pressure threshold (percentage)
    pub memory_pressure_threshold: f64,
    /// Young generation fill threshold
    pub young_gen_threshold: f64,
    /// Old generation fill threshold
    pub old_gen_threshold: f64,
    /// Time-based collection interval (seconds)
    pub time_based_interval: u64,
    /// Last time-based collection
    pub last_time_based_collection: Option<std::time::Instant>,
    /// Collection frequency multiplier
    pub frequency_multiplier: f64,
}

/// Garbage collection result
#[derive(Debug, Clone)]
pub struct GcResult {
    /// Type of collection performed
    pub collection_type: CollectionType,
    /// Objects collected
    pub objects_collected: usize,
    /// Bytes freed
    pub bytes_freed: usize,
    /// Collection duration
    pub duration_ms: u64,
    /// Whether collection was successful
    pub success: bool,
    /// Error message if collection failed
    pub error: Option<String>,
}

/// Types of garbage collection
#[derive(Debug, Clone, PartialEq)]
pub enum CollectionType {
    Minor,
    Major,
    Incremental,
    Background,
    Full,
}

/// Memory pressure levels
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MemoryPressure {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for GcStats {
    fn default() -> Self {
        Self {
            total_collections: 0,
            minor_collections: 0,
            major_collections: 0,
            incremental_collections: 0,
            background_collections: 0,
            total_objects_collected: 0,
            total_bytes_freed: 0,
            total_collection_time_ms: 0,
            average_collection_time_ms: 0,
            last_collection_time: None,
        }
    }
}

impl Default for GcTriggers {
    fn default() -> Self {
        Self {
            memory_pressure_threshold: 80.0,
            young_gen_threshold: 75.0,
            old_gen_threshold: 85.0,
            time_based_interval: 30, // 30 seconds
            last_time_based_collection: None,
            frequency_multiplier: 1.0,
        }
    }
}

impl GarbageCollector {
    /// Create a new garbage collector
    pub fn new() -> Self {
        Self {
            minor_gc: MinorGc::new(),
            major_gc: MajorGc::new(),
            incremental_gc: IncrementalGc::new(),
            background_gc: BackgroundGc::new(),
            stats: GcStats::default(),
            triggers: GcTriggers::default(),
            spaces: HashMap::new(),
        }
    }

    /// Register a memory space
    pub fn register_space(&mut self, space_type: SpaceType, space: Box<dyn MemorySpace>) {
        self.spaces.insert(space_type, space);
    }

    /// Check if collection is needed
    pub fn should_collect(&self) -> Option<CollectionType> {
        // Check memory pressure
        if let Some(pressure) = self.check_memory_pressure() {
            match pressure {
                MemoryPressure::Critical => return Some(CollectionType::Full),
                MemoryPressure::High => return Some(CollectionType::Major),
                MemoryPressure::Medium => return Some(CollectionType::Minor),
                MemoryPressure::Low => {}
            }
        }

        // Check young generation threshold
        if self.check_young_gen_threshold() {
            return Some(CollectionType::Minor);
        }

        // Check old generation threshold
        if self.check_old_gen_threshold() {
            return Some(CollectionType::Major);
        }

        // Check time-based collection
        if self.check_time_based_collection() {
            return Some(CollectionType::Incremental);
        }

        None
    }

    /// Perform garbage collection
    pub fn collect(&mut self) -> GcResult {
        let collection_type = self.should_collect().unwrap_or(CollectionType::Minor);
        let start_time = std::time::Instant::now();

        let result = match collection_type {
            CollectionType::Minor => self.perform_minor_collection(),
            CollectionType::Major => self.perform_major_collection(),
            CollectionType::Incremental => self.perform_incremental_collection(),
            CollectionType::Background => self.perform_background_collection(),
            CollectionType::Full => self.perform_full_collection(),
        };

        // Update statistics
        self.update_stats(&result, start_time);

        result
    }

    /// Perform minor collection
    fn perform_minor_collection(&mut self) -> GcResult {
        match self.minor_gc.collect(&mut self.spaces) {
            Ok(stats) => {
                self.stats.minor_collections += 1;
                GcResult {
                    collection_type: CollectionType::Minor,
                    objects_collected: stats.objects_collected,
                    bytes_freed: stats.bytes_freed,
                    duration_ms: stats.collection_time / 1000,
                    success: true,
                    error: None,
                }
            }
            Err(e) => GcResult {
                collection_type: CollectionType::Minor,
                objects_collected: 0,
                bytes_freed: 0,
                duration_ms: 0,
                success: false,
                error: Some(e.to_string()),
            },
        }
    }

    /// Perform major collection
    fn perform_major_collection(&mut self) -> GcResult {
        match self.major_gc.collect(&mut self.spaces) {
            Ok(stats) => {
                self.stats.major_collections += 1;
                GcResult {
                    collection_type: CollectionType::Major,
                    objects_collected: stats.objects_collected,
                    bytes_freed: stats.bytes_freed,
                    duration_ms: stats.collection_time / 1000,
                    success: true,
                    error: None,
                }
            }
            Err(e) => GcResult {
                collection_type: CollectionType::Major,
                objects_collected: 0,
                bytes_freed: 0,
                duration_ms: 0,
                success: false,
                error: Some(e.to_string()),
            },
        }
    }

    /// Perform incremental collection
    fn perform_incremental_collection(&mut self) -> GcResult {
        match self.incremental_gc.collect(&mut self.spaces) {
            Ok(stats) => {
                self.stats.incremental_collections += 1;
                GcResult {
                    collection_type: CollectionType::Incremental,
                    objects_collected: stats.objects_collected,
                    bytes_freed: stats.bytes_freed,
                    duration_ms: stats.collection_time / 1000,
                    success: true,
                    error: None,
                }
            }
            Err(e) => GcResult {
                collection_type: CollectionType::Incremental,
                objects_collected: 0,
                bytes_freed: 0,
                duration_ms: 0,
                success: false,
                error: Some(e.to_string()),
            },
        }
    }

    /// Perform background collection
    fn perform_background_collection(&mut self) -> GcResult {
        match self.background_gc.collect(&mut self.spaces) {
            Ok(stats) => {
                self.stats.background_collections += 1;
                GcResult {
                    collection_type: CollectionType::Background,
                    objects_collected: stats.objects_collected,
                    bytes_freed: stats.bytes_freed,
                    duration_ms: stats.collection_time / 1000,
                    success: true,
                    error: None,
                }
            }
            Err(e) => GcResult {
                collection_type: CollectionType::Background,
                objects_collected: 0,
                bytes_freed: 0,
                duration_ms: 0,
                success: false,
                error: Some(e.to_string()),
            },
        }
    }

    /// Perform full collection
    fn perform_full_collection(&mut self) -> GcResult {
        let start_time = std::time::Instant::now();

        // Perform all collection types
        let minor_result = self.perform_minor_collection();
        let major_result = self.perform_major_collection();

        let total_objects = minor_result.objects_collected + major_result.objects_collected;
        let total_bytes = minor_result.bytes_freed + major_result.bytes_freed;
        let duration = start_time.elapsed().as_millis() as u64;

        GcResult {
            collection_type: CollectionType::Full,
            objects_collected: total_objects,
            bytes_freed: total_bytes,
            duration_ms: duration,
            success: minor_result.success && major_result.success,
            error: if !minor_result.success || !major_result.success {
                Some("Partial collection failure".to_string())
            } else {
                None
            },
        }
    }

    /// Check memory pressure
    fn check_memory_pressure(&self) -> Option<MemoryPressure> {
        // Calculate total memory usage across all spaces
        let mut total_used = 0;
        let mut total_size = 0;

        for space in self.spaces.values() {
            total_used += space.total_allocated().as_usize();
            total_size += space.stats().total_size;
        }

        if total_size == 0 {
            return None;
        }

        let usage_percentage = (total_used as f64 / total_size as f64) * 100.0;

        if usage_percentage >= 95.0 {
            Some(MemoryPressure::Critical)
        } else if usage_percentage >= 85.0 {
            Some(MemoryPressure::High)
        } else if usage_percentage >= 70.0 {
            Some(MemoryPressure::Medium)
        } else if usage_percentage >= 50.0 {
            Some(MemoryPressure::Low)
        } else {
            None
        }
    }

    /// Check young generation threshold
    fn check_young_gen_threshold(&self) -> bool {
        if let Some(space) = self.spaces.get(&SpaceType::NewSpace) {
            let usage = space.total_allocated().as_usize();
            let total = space.stats().total_size;
            if total > 0 {
                let usage_percentage = (usage as f64 / total as f64) * 100.0;
                return usage_percentage >= self.triggers.young_gen_threshold;
            }
        }
        false
    }

    /// Check old generation threshold
    fn check_old_gen_threshold(&self) -> bool {
        if let Some(space) = self.spaces.get(&SpaceType::OldSpace) {
            let usage = space.total_allocated().as_usize();
            let total = space.stats().total_size;
            if total > 0 {
                let usage_percentage = (usage as f64 / total as f64) * 100.0;
                return usage_percentage >= self.triggers.old_gen_threshold;
            }
        }
        false
    }

    /// Check time-based collection
    fn check_time_based_collection(&self) -> bool {
        if let Some(last_collection) = self.triggers.last_time_based_collection {
            let elapsed = std::time::Instant::now()
                .duration_since(last_collection)
                .as_secs();
            elapsed >= self.triggers.time_based_interval
        } else {
            true
        }
    }

    /// Update collection statistics
    fn update_stats(&mut self, result: &GcResult, start_time: std::time::Instant) {
        self.stats.total_collections += 1;
        self.stats.total_objects_collected += result.objects_collected;
        self.stats.total_bytes_freed += result.bytes_freed;
        self.stats.total_collection_time_ms += result.duration_ms;
        self.stats.average_collection_time_ms =
            self.stats.total_collection_time_ms / self.stats.total_collections as u64;
        self.stats.last_collection_time = Some(start_time);

        // Update time-based collection trigger
        if result.success {
            self.triggers.last_time_based_collection = Some(std::time::Instant::now());
        }
    }

    /// Get collection statistics
    pub fn stats(&self) -> &GcStats {
        &self.stats
    }

    /// Get collection triggers
    pub fn triggers(&self) -> &GcTriggers {
        &self.triggers
    }

    /// Update collection triggers
    pub fn update_triggers(&mut self, triggers: GcTriggers) {
        self.triggers = triggers;
    }

    /// Get memory pressure information
    pub fn memory_pressure_info(&self) -> MemoryPressureInfo {
        let pressure = self.check_memory_pressure().unwrap_or(MemoryPressure::Low);
        let mut space_info = HashMap::new();

        for (space_type, space) in &self.spaces {
            let usage = space.total_allocated().as_usize();
            let total = space.stats().total_size;
            let usage_percentage = if total > 0 {
                (usage as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            space_info.insert(
                space_type.clone(),
                SpacePressureInfo {
                    usage_bytes: usage,
                    total_bytes: total,
                    usage_percentage,
                },
            );
        }

        MemoryPressureInfo {
            overall_pressure: pressure,
            space_pressure: space_info,
        }
    }

    /// Start background collection
    pub fn start_background_collection(&mut self) {
        self.background_gc.start();
    }

    /// Stop background collection
    pub fn stop_background_collection(&mut self) {
        self.background_gc.stop();
    }

    /// Check if background collection is running
    pub fn is_background_collection_running(&self) -> bool {
        self.background_gc.is_running()
    }
}

/// Memory pressure information
#[derive(Debug, Clone)]
pub struct MemoryPressureInfo {
    pub overall_pressure: MemoryPressure,
    pub space_pressure: HashMap<SpaceType, SpacePressureInfo>,
}

/// Space pressure information
#[derive(Debug, Clone)]
pub struct SpacePressureInfo {
    pub usage_bytes: usize,
    pub total_bytes: usize,
    pub usage_percentage: f64,
}

impl Default for GarbageCollector {
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
    }

    impl MemorySpace for MockMemorySpace {
        fn allocate(&mut self, _size: MemorySize) -> Option<HeapHandleId> {
            None
        }

        fn deallocate(&mut self, _handle: HeapHandleId) -> bool {
            false
        }

        fn can_allocate(&self, _size: MemorySize) -> bool {
            false
        }

        fn total_allocated(&self) -> MemorySize {
            MemorySize::new(self.stats.allocated_size)
        }

        fn total_free(&self) -> MemorySize {
            MemorySize::new(self.stats.free_size)
        }

        fn stats(&self) -> SpaceStats {
            self.stats.clone()
        }

        fn space_type(&self) -> SpaceType {
            self.stats.space_type.clone()
        }
    }

    #[test]
    fn test_garbage_collector_new() {
        let gc = GarbageCollector::new();
        assert_eq!(gc.stats.total_collections, 0);
        assert_eq!(gc.stats.minor_collections, 0);
        assert_eq!(gc.stats.major_collections, 0);
    }

    #[test]
    fn test_garbage_collector_register_space() {
        let mut gc = GarbageCollector::new();

        let mock_space = MockMemorySpace {
            stats: SpaceStats {
                space_type: SpaceType::NewSpace,
                total_size: 1024,
                allocated_size: 0,
                free_size: 1024,
                object_count: 0,
                fragmentation_percentage: 0.0,
                allocation_count: 0,
                deallocation_count: 0,
            },
        };

        gc.register_space(SpaceType::NewSpace, Box::new(mock_space));
        assert_eq!(gc.spaces.len(), 1);
    }

    #[test]
    fn test_garbage_collector_memory_pressure() {
        let mut gc = GarbageCollector::new();

        // Register a space with high usage
        let mock_space = MockMemorySpace {
            stats: SpaceStats {
                space_type: SpaceType::NewSpace,
                total_size: 1000,
                allocated_size: 900, // 90% usage
                free_size: 100,
                object_count: 0,
                fragmentation_percentage: 0.0,
                allocation_count: 0,
                deallocation_count: 0,
            },
        };

        gc.register_space(SpaceType::NewSpace, Box::new(mock_space));

        let pressure_info = gc.memory_pressure_info();
        assert_eq!(pressure_info.overall_pressure, MemoryPressure::High);
    }

    #[test]
    fn test_garbage_collector_triggers() {
        let mut gc = GarbageCollector::new();

        // Update triggers
        let mut triggers = GcTriggers::default();
        triggers.memory_pressure_threshold = 70.0;
        triggers.young_gen_threshold = 60.0;

        gc.update_triggers(triggers);

        let updated_triggers = gc.triggers();
        assert_eq!(updated_triggers.memory_pressure_threshold, 70.0);
        assert_eq!(updated_triggers.young_gen_threshold, 60.0);
    }
}
