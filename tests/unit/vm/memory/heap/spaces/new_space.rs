use crate::vm::memory::heap::spaces::new_space::*;
use crate::vm::memory::heap::spaces::MemorySpace;
use crate::vm::types::MemorySize;

#[test]
fn test_new_space_new() {
    let new_space = NewSpace::new(1024);
    assert_eq!(new_space.space_size, 1024);
    assert_eq!(new_space.total_allocated().bytes(), 0);
    assert_eq!(new_space.total_free().bytes(), 2048); // 2 * 1024
    assert!(!new_space.is_full());
}

#[test]
fn test_new_space_allocate() {
    let mut new_space = NewSpace::new(1024);

    let handle = new_space.allocate(MemorySize::new(64));
    assert!(handle.is_some());
    assert_eq!(new_space.total_allocated().bytes(), 64);
    assert_eq!(new_space.stats.object_count, 1);
    assert_eq!(new_space.stats.allocation_count, 1);
}

#[test]
fn test_new_space_reset() {
    let mut new_space = NewSpace::new(1024);

    // Allocate some memory
    new_space.allocate(MemorySize::new(64));
    new_space.allocate(MemorySize::new(128));

    assert_eq!(new_space.total_allocated().bytes(), 64 + 128);

    // Reset
    new_space.reset();

    assert_eq!(new_space.total_allocated().bytes(), 0);
    assert_eq!(new_space.stats.object_count, 0);
    assert_eq!(new_space.stats.allocation_count, 0);
}

#[test]
fn test_new_space_collect() {
    let mut new_space = NewSpace::new(1024);

    // Allocate some memory
    new_space.allocate(MemorySize::new(64));
    new_space.allocate(MemorySize::new(128));

    let before_usage = new_space.total_allocated().bytes();

    // Perform collection
    let stats = new_space.collect();

    assert!(stats.objects_collected > 0);
    assert_eq!(stats.bytes_freed, before_usage);
    assert!(stats.collection_time >= 0);

    // Space should be reset
    assert_eq!(new_space.total_allocated().bytes(), 0);
}

#[test]
fn test_new_space_promotion() {
    let mut new_space = NewSpace::new(1024);

    assert!(!new_space.should_promote());
    assert_eq!(new_space.promotion_info().current_count, 0);

    // Increment promotion count
    new_space.increment_promotion_count();
    new_space.increment_promotion_count();
    new_space.increment_promotion_count();

    assert!(new_space.should_promote());
    assert_eq!(new_space.promotion_info().current_count, 3);

    // Reset promotion count
    new_space.reset_promotion_count();

    assert!(!new_space.should_promote());
    assert_eq!(new_space.promotion_info().current_count, 0);
}
