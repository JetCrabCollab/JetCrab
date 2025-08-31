use crate::vm::memory::heap::gc::background::*;
use crate::vm::memory::heap::spaces::{MemorySpace, SpaceStats, SpaceType};
use crate::vm::types::{HeapHandleId, MemorySize};
use crate::vm::value::Value;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

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
fn test_background_gc_new() {
    let gc = BackgroundGc::new();
    assert_eq!(gc.stats.collections_performed, 0);
    assert!(!gc.is_running());
    assert!(!gc.is_collecting());
    assert_eq!(gc.progress(), 0.0);
    assert_eq!(gc.current_phase(), BackgroundCollectionPhase::Idle);
}

#[test]
fn test_background_gc_start_stop() {
    let mut gc = BackgroundGc::new();

    // Start background collection
    gc.start();
    assert!(gc.is_running());

    // Wait a bit for thread to start
    thread::sleep(Duration::from_millis(100));

    // Stop background collection
    gc.stop();
    assert!(!gc.is_running());
}

#[test]
fn test_background_gc_config() {
    let mut gc = BackgroundGc::new();

    // Update configuration
    let mut config = BackgroundGcConfig::default();
    config.enabled = false;
    config.thread_count = 4;
    config.collection_interval_ms = 10000;

    gc.update_config(config);

    let updated_config = gc.config();
    assert_eq!(updated_config.enabled, false);
    assert_eq!(updated_config.thread_count, 4);
    assert_eq!(updated_config.collection_interval_ms, 10000);
}

#[test]
fn test_background_gc_state() {
    let gc = BackgroundGc::new();

    let state = gc.state();
    assert!(!state.is_running);
    assert!(!state.is_collecting);
    assert_eq!(state.collection_phase, BackgroundCollectionPhase::Idle);
    assert_eq!(state.progress, 0.0);
    assert!(!state.should_stop);
    assert!(state.error_message.is_none());
}

#[test]
fn test_background_gc_collection_trigger() {
    let mut gc = BackgroundGc::new();

    // Create mock spaces with high usage
    let mut spaces: HashMap<SpaceType, Box<dyn MemorySpace>> = HashMap::new();

    let space = MockMemorySpace {
        stats: SpaceStats {
            space_type: SpaceType::NewSpace,
            total_size: 1000,
            allocated_size: 850, // 85% usage
            free_size: 150,
            object_count: 100,
            fragmentation_percentage: 0.0,
            allocation_count: 100,
            deallocation_count: 0,
        },
        allocated: 850,
    };

    spaces.insert(SpaceType::NewSpace, Box::new(space));

    // Should trigger collection due to high usage
    assert!(gc.should_trigger_collection(&spaces));
}

#[test]
fn test_background_gc_collection() {
    let mut gc = BackgroundGc::new();

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

    // Perform collection (should not trigger background collection by default)
    let result = gc.collect(&mut spaces);
    assert!(result.is_ok());

    let stats = result.unwrap();
    assert_eq!(stats.objects_collected, 100); // Simulated result
    assert_eq!(stats.bytes_freed, 1024 * 1024); // 1MB simulated
}

#[test]
fn test_background_gc_error_handling() {
    let gc = BackgroundGc::new();

    // Initially no error
    assert!(!gc.has_error());
    assert!(gc.error_message().is_none());

    // Simulate error state
    {
        let mut state_guard = gc.state.lock().unwrap();
        state_guard.error_message = Some("Test error".to_string());
    }

    // Check error state
    assert!(gc.has_error());
    assert_eq!(gc.error_message(), Some("Test error".to_string()));
}
