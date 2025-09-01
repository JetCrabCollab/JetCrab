use jetcrab::vm::memory::heap::allocation::{FreeListAllocator, Allocator};
use jetcrab::vm::memory::MemorySize;

#[test]
fn test_free_list_allocator_creation() {
    let allocator = FreeListAllocator::new();
    assert_eq!(allocator.total_allocated().bytes(), 0);
    assert_eq!(allocator.total_free().bytes(), 0);
}

#[test]
fn test_free_list_allocator_add_free_block() {
    let mut allocator = FreeListAllocator::new();
    allocator.add_free_block(0, 1024);

    assert!(allocator.can_allocate(MemorySize::new(1024)));
    assert!(allocator.can_allocate(MemorySize::new(512)));
    assert!(!allocator.can_allocate(MemorySize::new(2048)));
}

#[test]
fn test_free_list_allocator_allocate() {
    let mut allocator = FreeListAllocator::new();
    allocator.add_free_block(0, 1024);

    let addr = allocator.allocate(MemorySize::new(512));
    assert!(addr.is_some());
    assert_eq!(allocator.total_allocated().bytes(), 512);
}

#[test]
fn test_free_list_allocator_deallocate() {
    let mut allocator = FreeListAllocator::new();
    allocator.add_free_block(0, 1024);

    let addr = allocator.allocate(MemorySize::new(512));
    assert!(addr.is_some());

    allocator.deallocate(addr.unwrap(), MemorySize::new(512));
    assert_eq!(allocator.total_free().bytes(), 1024);
}
