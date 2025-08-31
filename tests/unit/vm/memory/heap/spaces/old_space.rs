use crate::vm::memory::heap::spaces::old_space::*;
use crate::vm::memory::heap::spaces::MemorySpace;
use crate::vm::types::MemorySize;

#[test]
fn test_old_space_new() {
    let old_space = OldSpace::new(1024);
    assert_eq!(old_space.total_size, 1024);
    assert_eq!(old_space.total_allocated().as_usize(), 0);
    assert_eq!(old_space.total_free().as_usize(), 1024);
    assert_eq!(old_space.stats.object_count, 0);
}

#[test]
fn test_old_space_allocate() {
    let mut old_space = OldSpace::new(1024);

    let handle = old_space.allocate(MemorySize::new(64));
    assert!(handle.is_some());
    assert_eq!(old_space.total_allocated().as_usize(), 64);
    assert_eq!(old_space.stats.object_count, 1);
    assert_eq!(old_space.stats.allocation_count, 1);

    // Check age tracking
    assert_eq!(old_space.object_ages.get(&handle.unwrap()), Some(&0));
}

#[test]
fn test_old_space_deallocate() {
    let mut old_space = OldSpace::new(1024);

    let handle = old_space.allocate(MemorySize::new(64)).unwrap();
    assert_eq!(old_space.stats.object_count, 1);

    assert!(old_space.deallocate(handle));
    assert_eq!(old_space.stats.object_count, 0);
    assert_eq!(old_space.stats.deallocation_count, 1);

    // Age tracking should be removed
    assert!(old_space.object_ages.is_empty());
}

#[test]
fn test_old_space_age_tracking() {
    let mut old_space = OldSpace::new(1024);

    let handle = old_space.allocate(MemorySize::new(64)).unwrap();

    // Increment ages
    old_space.increment_ages();
    old_space.increment_ages();

    assert_eq!(old_space.object_ages.get(&handle), Some(&2));
    assert_eq!(old_space.age_counter, 2);
}

#[test]
fn test_old_space_promotion() {
    let mut old_space = OldSpace::new(1024);

    let handle = old_space.allocate(MemorySize::new(64)).unwrap();

    // Age the object beyond promotion threshold
    for _ in 0..15 {
        old_space.increment_ages();
    }

    assert!(old_space.should_promote());

    let promotable = old_space.get_promotable_objects();
    assert_eq!(promotable.len(), 1);
    assert_eq!(promotable[0], handle);

    // Remove promoted object
    old_space.remove_promoted_objects(&promotable);
    assert!(old_space.object_ages.is_empty());
}

#[test]
fn test_old_space_collect() {
    let mut old_space = OldSpace::new(1024);

    // Allocate some objects
    old_space.allocate(MemorySize::new(64));
    old_space.allocate(MemorySize::new(128));

    let before_objects = old_space.stats.object_count;

    // Perform collection
    let stats = old_space.collect();

    assert!(stats.collection_time > 0);
    assert_eq!(
        old_space.stats.object_count,
        before_objects - stats.objects_collected
    );
}

#[test]
fn test_old_space_defragment() {
    let mut old_space = OldSpace::new(1024);

    // Allocate and deallocate to create fragmentation
    let handle1 = old_space.allocate(MemorySize::new(64)).unwrap();
    let handle2 = old_space.allocate(MemorySize::new(128)).unwrap();

    old_space.deallocate(handle1);

    let initial_fragmentation = old_space.allocator.fragmentation();

    // Defragment
    let stats = old_space.defragment();
    assert!(stats.free_blocks_after < stats.free_blocks_before);
}
