use jetcrab::vm::memory::heap::spaces::coordinator::SpaceCoordinator;
use jetcrab::vm::memory::heap::spaces::{NewSpace, OldSpace, CellSpace, LargeObjectSpace, CodeSpace};
use jetcrab::vm::memory::MemorySize;
use jetcrab::vm::handle::HeapHandleId;

// Mock implementation for testing
struct MockMemorySpace {
    name: String,
    size: usize,
}

impl MockMemorySpace {
    fn new(name: String, size: usize) -> Self {
        Self { name, size }
    }
}

impl jetcrab::vm::memory::heap::spaces::MemorySpace for MockMemorySpace {
    fn allocate(&mut self, _size: MemorySize) -> Option<HeapHandleId> {
        Some(HeapHandleId::new(1))
    }

    fn deallocate(&mut self, _handle: HeapHandleId) -> bool {
        true
    }

    fn total_allocated(&self) -> MemorySize {
        MemorySize::new(self.size / 2)
    }

    fn total_free(&self) -> MemorySize {
        MemorySize::new(self.size / 2)
    }

    fn is_full(&self) -> bool {
        false
    }

    fn stats(&self) -> &jetcrab::vm::memory::heap::spaces::SpaceStats {
        static STATS: jetcrab::vm::memory::heap::spaces::SpaceStats = jetcrab::vm::memory::heap::spaces::SpaceStats {
            total_size: 0,
            object_count: 0,
            allocation_count: 0,
            deallocation_count: 0,
        };
        &STATS
    }

    fn extract_object(&self, _handle: HeapHandleId) -> Option<jetcrab::vm::value::Value> {
        None
    }

    fn allocate_object(&mut self, _value: jetcrab::vm::value::Value) -> Option<HeapHandleId> {
        Some(HeapHandleId::new(1))
    }
}

#[test]
fn test_coordinator_creation() {
    let coordinator = SpaceCoordinator::new();
    
    assert!(coordinator.spaces.is_empty());
    assert_eq!(coordinator.promotion_policies.len(), 0);
}

#[test]
fn test_coordinator_add_space() {
    let mut coordinator = SpaceCoordinator::new();
    
    let new_space = Box::new(MockMemorySpace::new("new".to_string(), 1024));
    let old_space = Box::new(MockMemorySpace::new("old".to_string(), 2048));
    
    coordinator.add_space("new".to_string(), new_space);
    coordinator.add_space("old".to_string(), old_space);
    
    assert_eq!(coordinator.spaces.len(), 2);
    assert!(coordinator.spaces.contains_key("new"));
    assert!(coordinator.spaces.contains_key("old"));
}

#[test]
fn test_coordinator_get_space() {
    let mut coordinator = SpaceCoordinator::new();
    
    let space = Box::new(MockMemorySpace::new("test".to_string(), 1024));
    coordinator.add_space("test".to_string(), space);
    
    let retrieved_space = coordinator.get_space("test");
    assert!(retrieved_space.is_some());
    
    let non_existent = coordinator.get_space("nonexistent");
    assert!(non_existent.is_none());
}

#[test]
fn test_coordinator_add_promotion_policy() {
    let mut coordinator = SpaceCoordinator::new();
    
    coordinator.add_promotion_policy("new".to_string(), "old".to_string(), 10);
    
    assert_eq!(coordinator.promotion_policies.len(), 1);
    assert!(coordinator.promotion_policies.contains_key("new"));
}

#[test]
fn test_coordinator_allocate() {
    let mut coordinator = SpaceCoordinator::new();
    
    let new_space = Box::new(MockMemorySpace::new("new".to_string(), 1024));
    coordinator.add_space("new".to_string(), new_space);
    
    let handle = coordinator.allocate(MemorySize::new(64));
    assert!(handle.is_some());
}

#[test]
fn test_coordinator_deallocate() {
    let mut coordinator = SpaceCoordinator::new();
    
    let space = Box::new(MockMemorySpace::new("test".to_string(), 1024));
    coordinator.add_space("test".to_string(), space);
    
    let handle = HeapHandleId::new(1);
    let result = coordinator.deallocate(handle);
    assert!(result);
}
