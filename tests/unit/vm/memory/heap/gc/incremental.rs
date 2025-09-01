use crate::vm::memory::heap::gc::incremental::*;
use crate::vm::memory::heap::spaces::{MemorySpace, SpaceStats, SpaceType};
use crate::vm::types::{HeapHandleId, MemorySize};
use crate::vm::value::Value;
use std::collections::HashMap;

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

    fn extract_object(&mut self, _handle: HeapHandleId) -> Option<Value> {
        Some(Value::Number(42.0))
    }

    fn allocate_object(&mut self, _data: Value) -> Option<HeapHandleId> {
        Some(HeapHandleId::new(999))
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
