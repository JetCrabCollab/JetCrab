use jetcrab::vm::memory::heap::allocation::{BumpAllocator, Allocator};
use jetcrab::vm::memory::MemorySize;

#[test]
fn test_bump_allocator_creation() {
    let allocator = BumpAllocator::new(MemorySize::new(1024));
    assert_eq!(allocator.remaining().bytes(), 1024);
}

#[test]
fn test_bump_allocator_allocation() {
    let mut allocator = BumpAllocator::new(MemorySize::new(1024));

    let addr1 = allocator.allocate(MemorySize::new(64));
    assert!(addr1.is_some());
    assert_eq!(allocator.total_allocated().bytes(), 64);

    let addr2 = allocator.allocate(MemorySize::new(128));
    assert!(addr2.is_some());
    assert_eq!(allocator.total_allocated().bytes(), 192);
}

#[test]
fn test_bump_allocator_reset() {
    let mut allocator = BumpAllocator::new(MemorySize::new(1024));

    allocator.allocate(MemorySize::new(256));
    assert_eq!(allocator.total_allocated().bytes(), 256);

    allocator.reset();
    assert_eq!(allocator.total_allocated().bytes(), 0);
    assert_eq!(allocator.remaining().bytes(), 1024);
}
