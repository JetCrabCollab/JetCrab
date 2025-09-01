use jetcrab::vm::memory::heap::spaces::LargeObjectSpace;
use jetcrab::vm::memory::MemorySize;

#[test]
fn test_large_object_space_new() {
    let large_space = LargeObjectSpace::new(1024 * 1024);
    assert_eq!(large_space.total_size, 1024 * 1024);
    assert_eq!(large_space.total_allocated().as_usize(), 0);
    assert_eq!(large_space.total_free().as_usize(), 1024 * 1024);
    assert_eq!(large_space.stats.object_count, 0);
    assert!(!large_space.memory_regions.is_empty());
}

#[test]
fn test_large_object_space_allocate() {
    let mut large_space = LargeObjectSpace::new(1024 * 1024);

    let handle = large_space.allocate(MemorySize::new(512 * 1024));
    assert!(handle.is_some());
    assert_eq!(large_space.total_allocated().as_usize(), 512 * 1024);
    assert_eq!(large_space.stats.object_count, 1);
    assert_eq!(large_space.stats.allocation_count, 1);

    // Check that a region was allocated
    let allocated_regions = large_space.memory_regions.iter().filter(|r| r.is_allocated).count();
    assert_eq!(allocated_regions, 1);
}

#[test]
fn test_large_object_space_deallocate() {
    let mut large_space = LargeObjectSpace::new(1024 * 1024);

    let handle = large_space.allocate(MemorySize::new(512 * 1024)).unwrap();
    assert_eq!(large_space.stats.object_count, 1);

    assert!(large_space.deallocate(handle));
    assert_eq!(large_space.stats.object_count, 0);
    assert_eq!(large_space.stats.deallocation_count, 1);
}

#[test]
fn test_large_object_space_compact() {
    let mut large_space = LargeObjectSpace::new(128 * 1024 * 1024);

    // Allocate some objects
    let handle1 = large_space.allocate(MemorySize::new(256 * 1024)).unwrap();
    let handle2 = large_space.allocate(MemorySize::new(256 * 1024)).unwrap();

    // Deallocate first object to create fragmentation
    large_space.deallocate(handle1);

    let initial_fragmentation = large_space.calculate_fragmentation();

    // Compact
    let stats = large_space.compact();
    assert!(stats.final_fragmentation < stats.initial_fragmentation);
    assert!(stats.cells_moved > 0);
}

#[test]
fn test_large_object_space_collect() {
    let mut large_space = LargeObjectSpace::new(1024 * 1024);

    // Allocate some objects
    large_space.allocate(MemorySize::new(256 * 1024));
    large_space.allocate(MemorySize::new(256 * 1024));

    let before_objects = large_space.stats.object_count;

    // Perform collection
    let stats = large_space.collect();

    assert!(stats.collection_time > 0);
    assert_eq!(
        large_space.stats.object_count,
        before_objects - stats.objects_collected
    );
}

#[test]
fn test_large_object_space_efficiency() {
    let mut large_space = LargeObjectSpace::new(1024 * 1024);

    // Allocate 50% of space
    large_space.allocate(MemorySize::new(512 * 1024));

    let efficiency = large_space.efficiency();
    assert_eq!(efficiency, 50.0);

    let health = large_space.health_score();
    assert!(health > 60.0);
}
