use crate::vm::memory::heap::gc::minor_gc::*;
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
