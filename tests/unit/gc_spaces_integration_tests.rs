use jetcrab::vm::memory::{MemoryManager, MemorySize};
use jetcrab::vm::memory::heap::{ObjectType, GarbageCollector};
use jetcrab::vm::memory::heap::spaces::SpaceCoordinator;
use jetcrab::vm::memory::heap::gc::write_barrier::{WriteBarrier, BarrierType};

#[test]
fn test_memory_manager_integration() {
    let mut memory_manager = MemoryManager::new();
    
    // Test allocation through space coordinator
    let size = MemorySize::new(64);
    let result = memory_manager.allocate(size, ObjectType::Object);
    assert!(result.is_ok());
    
    let handle = result.unwrap();
    assert!(handle.is_valid());
    
    // Test deallocation
    let dealloc_result = memory_manager.deallocate(handle);
    assert!(dealloc_result);
}

#[test]
fn test_gc_spaces_integration() {
    let mut gc = GarbageCollector::new();
    let mut space_coordinator = SpaceCoordinator::new();
    
    // Register spaces
    space_coordinator.register_space(
        jetcrab::vm::memory::heap::spaces::SpaceType::NewSpace,
        Box::new(jetcrab::vm::memory::heap::spaces::NewSpace::new())
    );
    
    space_coordinator.register_space(
        jetcrab::vm::memory::heap::spaces::SpaceType::OldSpace,
        Box::new(jetcrab::vm::memory::heap::spaces::OldSpace::new())
    );
    
    // Test allocation
    let size = MemorySize::new(128);
    let handle = space_coordinator.allocate(size, ObjectType::String);
    assert!(handle.is_some());
    
    // Test promotion
    let handle = handle.unwrap();
    space_coordinator.increment_tenure(handle);
    space_coordinator.increment_tenure(handle);
    space_coordinator.increment_tenure(handle);
    
    assert!(space_coordinator.should_promote(handle));
}

#[test]
fn test_write_barrier_integration() {
    let mut write_barrier = WriteBarrier::new(1024 * 1024, BarrierType::Hybrid);
    
    // Test write barrier recording
    let object_id = jetcrab::vm::handle::HeapHandleId::new(1);
    let field_address = 0x1000;
    
    write_barrier.record_write(object_id, field_address);
    
    let stats = write_barrier.get_stats();
    assert_eq!(stats.total_dirty_objects, 1);
    assert_eq!(stats.total_dirty_cards, 1);
    
    // Test reference write recording
    let source_object = jetcrab::vm::handle::HeapHandleId::new(2);
    let target_object = jetcrab::vm::handle::HeapHandleId::new(3);
    
    write_barrier.record_reference_write(source_object, target_object);
    
    let stats = write_barrier.get_stats();
    assert_eq!(stats.total_dirty_objects, 3);
    assert_eq!(stats.total_dirty_cards, 2);
}

#[test]
fn test_space_coordinator_allocation_strategies() {
    let mut coordinator = SpaceCoordinator::new();
    
    // Register all spaces
    coordinator.register_space(
        jetcrab::vm::memory::heap::spaces::SpaceType::NewSpace,
        Box::new(jetcrab::vm::memory::heap::spaces::NewSpace::new())
    );
    coordinator.register_space(
        jetcrab::vm::memory::heap::spaces::SpaceType::OldSpace,
        Box::new(jetcrab::vm::memory::heap::spaces::OldSpace::new())
    );
    coordinator.register_space(
        jetcrab::vm::memory::heap::spaces::SpaceType::LargeObjectSpace,
        Box::new(jetcrab::vm::memory::heap::spaces::LargeObjectSpace::new())
    );
    coordinator.register_space(
        jetcrab::vm::memory::heap::spaces::SpaceType::CodeSpace,
        Box::new(jetcrab::vm::memory::heap::spaces::CodeSpace::new())
    );
    coordinator.register_space(
        jetcrab::vm::memory::heap::spaces::SpaceType::CellSpace,
        Box::new(jetcrab::vm::memory::heap::spaces::CellSpace::new())
    );
    
    // Test small object allocation (should go to CellSpace)
    let small_size = MemorySize::new(32);
    let small_handle = coordinator.allocate(small_size, ObjectType::String);
    assert!(small_handle.is_some());
    
    // Test medium object allocation (should go to NewSpace)
    let medium_size = MemorySize::new(512);
    let medium_handle = coordinator.allocate(medium_size, ObjectType::Object);
    assert!(medium_handle.is_some());
    
    // Test large object allocation (should go to LargeObjectSpace)
    let large_size = MemorySize::new(2 * 1024 * 1024);
    let large_handle = coordinator.allocate(large_size, ObjectType::Array);
    assert!(large_handle.is_some());
}

#[test]
fn test_memory_pressure_detection() {
    let mut memory_manager = MemoryManager::new();
    
    // Allocate memory to create pressure
    for i in 0..100 {
        let size = MemorySize::new(1024);
        let result = memory_manager.allocate(size, ObjectType::Object);
        assert!(result.is_ok());
    }
    
    // Update stats to reflect current state
    memory_manager.update_stats();
    
    let stats = memory_manager.get_stats();
    assert!(stats.allocated_memory.bytes() > 0);
    assert!(stats.heap_efficiency > 0.0);
    
    // Test garbage collection trigger
    let gc_result = memory_manager.collect();
    assert!(gc_result.is_ok());
}
