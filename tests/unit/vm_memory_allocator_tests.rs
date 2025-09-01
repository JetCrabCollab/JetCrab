//! Unit tests for VM Memory Allocator

use jetcrab::vm::memory::allocator::{AllocatorStats, MemoryAllocator};
use jetcrab::vm::types::{AllocationCount, MemorySize};

#[test]
fn test_memory_allocator_new() {
    let allocator = MemoryAllocator::new();
    let stats = allocator.get_stats();

    assert_eq!(stats.total_allocated.as_usize(), 0);
    assert_eq!(stats.allocations.as_usize(), 0);
    assert_eq!(stats.deallocations.as_usize(), 0);
}

#[test]
fn test_memory_allocator_default() {
    let allocator = MemoryAllocator::default();
    let stats = allocator.get_stats();

    assert_eq!(stats.total_allocated.as_usize(), 0);
    assert_eq!(stats.allocations.as_usize(), 0);
    assert_eq!(stats.deallocations.as_usize(), 0);
}

#[test]
fn test_memory_allocator_allocate() {
    let mut allocator = MemoryAllocator::new();
    let size = MemorySize::new(1024);

    let ptr = allocator.allocate(size);
    assert!(!ptr.is_null());

    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), 1024);
    assert_eq!(stats.allocations.as_usize(), 1);
    assert_eq!(stats.deallocations.as_usize(), 0);

    unsafe {
        allocator.deallocate(ptr, size);
    }
}

#[test]
fn test_memory_allocator_allocate_multiple() {
    let mut allocator = MemoryAllocator::new();
    let size1 = MemorySize::new(512);
    let size2 = MemorySize::new(256);

    let ptr1 = allocator.allocate(size1);
    let ptr2 = allocator.allocate(size2);

    assert!(!ptr1.is_null());
    assert!(!ptr2.is_null());

    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), 768);
    assert_eq!(stats.allocations.as_usize(), 2);
    assert_eq!(stats.deallocations.as_usize(), 0);

    unsafe {
        allocator.deallocate(ptr1, size1);
        allocator.deallocate(ptr2, size2);
    }
}

#[test]
fn test_memory_allocator_allocate_zero_size() {
    let mut allocator = MemoryAllocator::new();
    let size = MemorySize::new(0);

    let ptr = allocator.allocate(size);
    assert!(!ptr.is_null());

    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), 0);
    assert_eq!(stats.allocations.as_usize(), 1);
}

#[test]
fn test_memory_allocator_deallocate() {
    let mut allocator = MemoryAllocator::new();
    let size = MemorySize::new(1024);

    let ptr = allocator.allocate(size);
    assert!(!ptr.is_null());

    unsafe {
        allocator.deallocate(ptr, size);
    }

    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), 0);
    assert_eq!(stats.allocations.as_usize(), 1);
    assert_eq!(stats.deallocations.as_usize(), 1);
}

#[test]
fn test_memory_allocator_deallocate_null_ptr() {
    let mut allocator = MemoryAllocator::new();
    let size = MemorySize::new(1024);

    unsafe {
        allocator.deallocate(std::ptr::null_mut(), size);
    }

    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), 0);
    assert_eq!(stats.allocations.as_usize(), 0);
    assert_eq!(stats.deallocations.as_usize(), 0);
}

#[test]
fn test_memory_allocator_deallocate_multiple() {
    let mut allocator = MemoryAllocator::new();
    let size1 = MemorySize::new(512);
    let size2 = MemorySize::new(256);

    let ptr1 = allocator.allocate(size1);
    let ptr2 = allocator.allocate(size2);

    unsafe {
        allocator.deallocate(ptr1, size1);
    }

    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), 256);
    assert_eq!(stats.allocations.as_usize(), 2);
    assert_eq!(stats.deallocations.as_usize(), 1);

    unsafe {
        allocator.deallocate(ptr2, size2);
    }

    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), 0);
    assert_eq!(stats.allocations.as_usize(), 2);
    assert_eq!(stats.deallocations.as_usize(), 2);
}

#[test]
fn test_memory_allocator_reset_stats() {
    let mut allocator = MemoryAllocator::new();
    let size = MemorySize::new(1024);

    let ptr = allocator.allocate(size);
    unsafe {
        allocator.deallocate(ptr, size);
    }

    let stats = allocator.get_stats();
    assert_eq!(stats.allocations.as_usize(), 1);
    assert_eq!(stats.deallocations.as_usize(), 1);

    allocator.reset_stats();

    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), 0);
    assert_eq!(stats.allocations.as_usize(), 0);
    assert_eq!(stats.deallocations.as_usize(), 0);
}

#[test]
fn test_memory_allocator_get_stats() {
    let mut allocator = MemoryAllocator::new();
    let size = MemorySize::new(2048);

    let ptr = allocator.allocate(size);
    let stats = allocator.get_stats();

    assert_eq!(stats.total_allocated.as_usize(), 2048);
    assert_eq!(stats.allocations.as_usize(), 1);
    assert_eq!(stats.deallocations.as_usize(), 0);

    unsafe {
        allocator.deallocate(ptr, size);
    }

    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), 0);
    assert_eq!(stats.allocations.as_usize(), 1);
    assert_eq!(stats.deallocations.as_usize(), 1);
}

#[test]
fn test_allocator_stats() {
    let stats = AllocatorStats {
        total_allocated: MemorySize::new(1024),
        allocations: AllocationCount::new(5),
        deallocations: AllocationCount::new(3),
    };

    assert_eq!(stats.total_allocated.as_usize(), 1024);
    assert_eq!(stats.allocations.as_usize(), 5);
    assert_eq!(stats.deallocations.as_usize(), 3);
}

#[test]
fn test_memory_allocator_large_allocation() {
    let mut allocator = MemoryAllocator::new();
    let size = MemorySize::new(1024 * 1024);

    let ptr = allocator.allocate(size);
    assert!(!ptr.is_null());

    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), 1024 * 1024);
    assert_eq!(stats.allocations.as_usize(), 1);

    unsafe {
        allocator.deallocate(ptr, size);
    }
}

#[test]
fn test_memory_allocator_alignment() {
    let mut allocator = MemoryAllocator::new();
    let size = MemorySize::new(1);

    let ptr = allocator.allocate(size);
    assert!(!ptr.is_null());

    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), 1);

    unsafe {
        allocator.deallocate(ptr, size);
    }
}

#[test]
fn test_memory_allocator_allocate_after_reset() {
    let mut allocator = MemoryAllocator::new();
    let size = MemorySize::new(1024);

    let ptr1 = allocator.allocate(size);
    unsafe {
        allocator.deallocate(ptr1, size);
    }

    allocator.reset_stats();

    let ptr2 = allocator.allocate(size);
    assert!(!ptr2.is_null());

    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), 1024);
    assert_eq!(stats.allocations.as_usize(), 1);
    assert_eq!(stats.deallocations.as_usize(), 0);

    unsafe {
        allocator.deallocate(ptr2, size);
    }
}

#[test]
fn test_memory_allocator_multiple_allocations_different_sizes() {
    let mut allocator = MemoryAllocator::new();
    let sizes = vec![
        MemorySize::new(1),
        MemorySize::new(8),
        MemorySize::new(16),
        MemorySize::new(32),
        MemorySize::new(64),
        MemorySize::new(128),
        MemorySize::new(256),
        MemorySize::new(512),
        MemorySize::new(1024),
    ];

    let mut ptrs = Vec::new();
    let mut total_size = 0;

    for size in &sizes {
        let ptr = allocator.allocate(*size);
        assert!(!ptr.is_null());
        ptrs.push((ptr, *size));
        total_size += size.as_usize();
    }

    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), total_size);
    assert_eq!(stats.allocations.as_usize(), sizes.len());

    for (ptr, size) in ptrs {
        unsafe {
            allocator.deallocate(ptr, size);
        }
    }

    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), 0);
    assert_eq!(stats.deallocations.as_usize(), sizes.len());
}
