use crate::vm::memory::heap::allocation::cell::*;
use crate::vm::memory::heap::allocation::Allocator;
use crate::vm::types::MemorySize;

#[test]
fn test_cell_allocator_creation() {
    let allocator = CellAllocator::new(64, 100);
    assert_eq!(allocator.total_cells, 100);
    assert_eq!(allocator.cell_size, 64);
    assert_eq!(allocator.free_cell_count(), 100);
    assert_eq!(allocator.allocated_cell_count(), 0);
}

#[test]
fn test_cell_allocator_allocation() {
    let mut allocator = CellAllocator::new(64, 100);

    let addr = allocator.allocate(MemorySize::new(32));
    assert!(addr.is_some());
    assert_eq!(allocator.allocated_cell_count(), 1);
    assert_eq!(allocator.free_cell_count(), 99);
}

#[test]
fn test_cell_allocator_deallocation() {
    let mut allocator = CellAllocator::new(64, 100);

    let addr = allocator.allocate(MemorySize::new(32)).unwrap();
    assert_eq!(allocator.allocated_cell_count(), 1);

    assert!(allocator.deallocate(addr, MemorySize::new(32)));
    assert_eq!(allocator.allocated_cell_count(), 0);
    assert_eq!(allocator.free_cell_count(), 100);
}

#[test]
fn test_cell_allocator_size_limit() {
    let mut allocator = CellAllocator::new(64, 100);

    // Try to allocate more than cell size
    let addr = allocator.allocate(MemorySize::new(128));
    assert!(addr.is_none());
}

#[test]
fn test_cell_allocator_usage_percentage() {
    let mut allocator = CellAllocator::new(64, 100);
    assert_eq!(allocator.usage_percentage(), 0.0);

    allocator.allocate(MemorySize::new(32));
    assert_eq!(allocator.usage_percentage(), 1.0);

    allocator.allocate(MemorySize::new(32));
    assert_eq!(allocator.usage_percentage(), 2.0);
}
