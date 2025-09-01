use crate::vm::memory::heap::gc::major_gc::*;
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
fn test_major_gc_new() {
    let major_gc = MajorGc::new();
    assert_eq!(major_gc.stats.collections_performed, 0);
    assert_eq!(major_gc.collection_threshold(), 1000);
    assert_eq!(major_gc.collection_count, 0);
    assert_eq!(major_gc.marking_state(), &MarkingState::Unmarked);
}

#[test]
fn test_major_gc_object_tracking() {
    let mut major_gc = MajorGc::new();

    let handle1 = HeapHandleId::new(64);
    let handle2 = HeapHandleId::new(128);

    // Track objects
    major_gc.track_object(handle1);
    major_gc.track_object(handle2);

    // Increment ages
    major_gc.increment_object_ages();
    major_gc.increment_object_ages();

    let age_info = major_gc.object_age_info();
    assert_eq!(age_info.total_objects, 2);
    assert_eq!(age_info.average_age, 2);
    assert_eq!(age_info.max_age, 2);

    // Untrack object
    major_gc.untrack_object(&handle1);
    assert_eq!(major_gc.object_ages.len(), 1);
    assert!(major_gc.object_ages.contains_key(&handle2));
}

#[test]
fn test_major_gc_collection_threshold() {
    let mut major_gc = MajorGc::new();

    // Update threshold
    major_gc.update_collection_threshold(500);
    assert_eq!(major_gc.collection_threshold(), 500);

    // Check collection need
    assert!(!major_gc.should_collect(400));
    assert!(major_gc.should_collect(600));
}

#[test]
fn test_major_gc_marking_state() {
    let mut major_gc = MajorGc::new();

    // Test state transitions
    assert_eq!(major_gc.marking_state(), &MarkingState::Unmarked);

    major_gc.set_marking_state(MarkingState::Marked);
    assert_eq!(major_gc.marking_state(), &MarkingState::Marked);

    major_gc.set_marking_state(MarkingState::MarkedAndScanned);
    assert_eq!(major_gc.marking_state(), &MarkingState::MarkedAndScanned);

    major_gc.reset_marking_state();
    assert_eq!(major_gc.marking_state(), &MarkingState::Unmarked);
}

#[test]
fn test_major_gc_collection() {
    let mut major_gc = MajorGc::new();

    // Create mock old space
    let mut spaces: HashMap<SpaceType, Box<dyn MemorySpace>> = HashMap::new();

    let old_space = MockMemorySpace {
        stats: SpaceStats {
            space_type: SpaceType::OldSpace,
            total_size: 2048,
            allocated_size: 1024,
            free_size: 1024,
            object_count: 16,
            fragmentation_percentage: 25.0,
            allocation_count: 16,
            deallocation_count: 0,
        },
        allocated: 1024,
    };

    spaces.insert(SpaceType::OldSpace, Box::new(old_space));

    // Perform collection
    let result = major_gc.collect(&mut spaces);
    assert!(result.is_ok());

    let stats = result.unwrap();
    assert!(stats.objects_collected >= 0);
    assert!(stats.bytes_freed >= 0);
    assert!(stats.collection_time > 0);

    // Check that statistics were updated
    assert_eq!(major_gc.stats.collections_performed, 1);
    assert!(major_gc.stats.last_collection_time.is_some());
}

#[test]
fn test_major_gc_fragmentation() {
    let mut major_gc = MajorGc::new();

    let old_space = MockMemorySpace {
        stats: SpaceStats {
            space_type: SpaceType::OldSpace,
            total_size: 2048,
            allocated_size: 1024,
            free_size: 1024,
            object_count: 16,
            fragmentation_percentage: 35.0, // High fragmentation
            allocation_count: 16,
            deallocation_count: 0,
        },
        allocated: 1024,
    };

    let old_space_box: Box<dyn MemorySpace> = Box::new(old_space);
    let fragmentation_info = major_gc.fragmentation_info(&old_space_box);
    assert_eq!(fragmentation_info.fragmentation_percentage, 35.0);
    assert!(fragmentation_info.should_compact);
    assert_eq!(fragmentation_info.total_size, 2048);
    assert_eq!(fragmentation_info.allocated_size, 1024);
}
