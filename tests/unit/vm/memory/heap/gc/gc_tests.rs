use crate::vm::memory::heap::gc::*;
use crate::vm::memory::heap::spaces::{MemorySpace, SpaceStats, SpaceType};
use crate::vm::types::{HeapHandleId, MemorySize};
use crate::vm::value::Value;

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

    fn extract_object(&mut self, _handle: HeapHandleId) -> Option<Value> {
        Some(Value::Number(42.0))
    }

    fn allocate_object(&mut self, _data: Value) -> Option<HeapHandleId> {
        Some(HeapHandleId::new(999))
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
