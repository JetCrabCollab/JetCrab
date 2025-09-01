use jetcrab::vm::memory::heap::spaces::CodeSpace;
use jetcrab::vm::memory::MemorySize;

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
    let allocated_blocks = code_space.code_blocks.iter().filter(|b| b.is_allocated).count();
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

#[test]
fn test_code_space_compact() {
    let mut code_space = CodeSpace::new(1024 * 1024);

    // Allocate some code blocks
    let handles: Vec<_> = (0..5)
        .map(|_| code_space.allocate(MemorySize::new(64 * 1024)).unwrap())
        .collect();

    // Deallocate some blocks to create fragmentation
    code_space.deallocate(handles[1]);
    code_space.deallocate(handles[3]);

    let initial_fragmentation = code_space.calculate_fragmentation();

    // Compact
    let stats = code_space.compact();
    assert!(stats.final_fragmentation < stats.initial_fragmentation);
    assert!(stats.cells_moved > 0);
}

#[test]
fn test_code_space_optimization_tracking() {
    let mut code_space = CodeSpace::new(1024 * 1024);

    // Allocate some code
    let handle = code_space.allocate(MemorySize::new(64 * 1024)).unwrap();

    // Record execution multiple times to reach threshold
    for _ in 0..1000 {
        code_space.record_execution(handle);
    }
    
    // Now it should be marked as hot
    assert!(code_space.code_objects.get(&handle).unwrap().is_hot);

    // Check hot code info
    let hot_info = code_space.hot_code_info();
    assert!(hot_info.optimization_opportunities > 0);
}
