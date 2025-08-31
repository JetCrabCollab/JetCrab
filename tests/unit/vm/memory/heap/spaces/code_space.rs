use crate::vm::memory::heap::spaces::code_space::*;
use crate::vm::memory::heap::spaces::MemorySpace;
use crate::vm::types::MemorySize;

#[test]
fn test_code_space_new() {
    let code_space = CodeSpace::new(1024 * 1024);
    assert_eq!(code_space.total_size, 1024 * 1024);
    assert_eq!(code_space.total_allocated().as_usize(), 0);
    assert_eq!(code_space.total_free().as_usize(), 1024 * 1024);
    assert_eq!(code_space.stats.object_count, 0);
    assert!(!code_space.code_blocks.is_empty());
}

#[test]
fn test_code_space_allocate() {
    let mut code_space = CodeSpace::new(1024 * 1024);

    let handle = code_space.allocate(MemorySize::new(32 * 1024));
    assert!(handle.is_some());
    assert_eq!(code_space.total_allocated().as_usize(), 32 * 1024);
    assert_eq!(code_space.stats.object_count, 1);
    assert_eq!(code_space.stats.allocation_count, 1);

    // Check that a block was allocated
    let allocated_blocks = code_space
        .code_blocks
        .iter()
        .filter(|b| b.is_allocated)
        .count();
    assert_eq!(allocated_blocks, 1);
}

#[test]
fn test_code_space_deallocate() {
    let mut code_space = CodeSpace::new(1024 * 1024);

    let handle = code_space.allocate(MemorySize::new(32 * 1024)).unwrap();
    assert_eq!(code_space.stats.object_count, 1);

    assert!(code_space.deallocate(handle));
    assert_eq!(code_space.stats.object_count, 0);
    assert_eq!(code_space.stats.deallocation_count, 1);

    // Check that the block was freed
    let allocated_blocks = code_space
        .code_blocks
        .iter()
        .filter(|b| b.is_allocated)
        .count();
    assert_eq!(allocated_blocks, 0);
}

#[test]
fn test_code_space_hot_code_detection() {
    let mut code_space = CodeSpace::new(1024 * 1024);

    let handle = code_space.allocate(MemorySize::new(16 * 1024)).unwrap();

    // Record executions to make it hot
    for _ in 0..1000 {
        code_space.record_execution(handle);
    }

    let hot_info = code_space.hot_code_info();
    assert_eq!(hot_info.hot_code_count, 1);
    assert!(hot_info.optimization_opportunities > 0);
}

#[test]
fn test_code_space_optimization() {
    let mut code_space = CodeSpace::new(1024 * 1024);

    let handle = code_space.allocate(MemorySize::new(16 * 1024)).unwrap();

    // Make it hot
    for _ in 0..1000 {
        code_space.record_execution(handle);
    }

    // Apply optimization
    let stats = code_space.optimize_hot_code();
    assert!(stats.optimizations_applied > 0);

    // Check optimization level was upgraded
    let code_obj = code_space.code_objects.get(&handle).unwrap();
    assert!(code_obj.optimization_level > OptimizationLevel::None);
}

#[test]
fn test_code_space_compact() {
    let mut code_space = CodeSpace::new(1024 * 1024);

    // Allocate some code objects
    let handle1 = code_space.allocate(MemorySize::new(32 * 1024)).unwrap();
    let handle2 = code_space.allocate(MemorySize::new(32 * 1024)).unwrap();

    // Deallocate first object to create fragmentation
    code_space.deallocate(handle1);

    let initial_fragmentation = code_space.calculate_fragmentation();

    // Compact
    let stats = code_space.compact();
    assert!(stats.final_fragmentation < stats.initial_fragmentation);
    assert!(stats.cells_moved > 0);
}

#[test]
fn test_code_space_collect() {
    let mut code_space = CodeSpace::new(1024 * 1024);

    // Allocate some code objects
    code_space.allocate(MemorySize::new(32 * 1024));
    code_space.allocate(MemorySize::new(32 * 1024));

    let before_objects = code_space.stats.object_count;

    // Perform collection
    let stats = code_space.collect();

    assert!(stats.collection_time > 0);
    assert_eq!(
        code_space.stats.object_count,
        before_objects - stats.objects_collected
    );
}

#[test]
fn test_code_space_efficiency() {
    let mut code_space = CodeSpace::new(1024 * 1024);

    // Allocate 25% of space
    code_space.allocate(MemorySize::new(256 * 1024));

    let efficiency = code_space.efficiency();
    assert_eq!(efficiency, 25.0);

    let health = code_space.health_score();
    assert!(health > 30.0);
}
