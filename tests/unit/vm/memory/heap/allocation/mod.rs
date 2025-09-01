use jetcrab::vm::memory::heap::allocation::{Allocator, AllocationStrategy, LayoutInfo};
use jetcrab::vm::memory::MemorySize;

// Mock allocator for testing
struct MockAllocator {
    allocated: usize,
    total_size: usize,
}

impl MockAllocator {
    fn new(size: usize) -> Self {
        Self {
            allocated: 0,
            total_size: size,
        }
    }
}

impl Allocator for MockAllocator {
    fn allocate(&mut self, size: MemorySize) -> Option<usize> {
        if self.allocated + size.bytes() <= self.total_size {
            let addr = self.allocated;
            self.allocated += size.bytes();
            Some(addr)
        } else {
            None
        }
    }

    fn deallocate(&mut self, _address: usize, size: MemorySize) -> bool {
        if self.allocated >= size.bytes() {
            self.allocated -= size.bytes();
            true
        } else {
            false
        }
    }

    fn can_allocate(&self, size: MemorySize) -> bool {
        self.allocated + size.bytes() <= self.total_size
    }

    fn total_allocated(&self) -> MemorySize {
        MemorySize::new(self.allocated)
    }

    fn total_free(&self) -> MemorySize {
        MemorySize::new(self.total_size - self.allocated)
    }

    fn fragmentation(&self) -> f64 {
        if self.total_size == 0 {
            0.0
        } else {
            (self.allocated as f64 / self.total_size as f64) * 100.0
        }
    }
}

#[test]
fn test_allocator_trait_implementation() {
    let mut allocator = MockAllocator::new(1024);
    
    let addr = allocator.allocate(MemorySize::new(256));
    assert!(addr.is_some());
    assert_eq!(allocator.total_allocated().bytes(), 256);
    
    assert!(allocator.can_allocate(MemorySize::new(512)));
    assert!(!allocator.can_allocate(MemorySize::new(1024)));
}

#[test]
fn test_allocation_strategy() {
    let strategy = AllocationStrategy::default();
    
    assert!(matches!(strategy, AllocationStrategy::BestFit));
}

#[test]
fn test_allocation_strategy_variants() {
    let best_fit = AllocationStrategy::BestFit;
    let first_fit = AllocationStrategy::FirstFit;
    let worst_fit = AllocationStrategy::WorstFit;
    
    assert!(matches!(best_fit, AllocationStrategy::BestFit));
    assert!(matches!(first_fit, AllocationStrategy::FirstFit));
    assert!(matches!(worst_fit, AllocationStrategy::WorstFit));
}

#[test]
fn test_layout_info() {
    let layout = LayoutInfo::new(64, 8);
    
    assert_eq!(layout.size, 64);
    assert_eq!(layout.alignment, 8);
}

#[test]
fn test_layout_info_padding() {
    let layout = LayoutInfo::new(100, 16);
    
    // Calculate padding needed for alignment
    let padding = layout.padding_needed();
    assert!(padding >= 0);
    assert!(padding < 16);
}

#[test]
fn test_layout_info_aligned_size() {
    let layout = LayoutInfo::new(100, 16);
    
    let aligned_size = layout.aligned_size();
    assert!(aligned_size >= 100);
    assert_eq!(aligned_size % 16, 0);
}

#[test]
fn test_allocator_fragmentation() {
    let mut allocator = MockAllocator::new(1000);
    
    // 0% fragmentation when empty
    assert_eq!(allocator.fragmentation(), 0.0);
    
    // 50% fragmentation when half full
    allocator.allocate(MemorySize::new(500));
    assert_eq!(allocator.fragmentation(), 50.0);
    
    // 100% fragmentation when full
    allocator.allocate(MemorySize::new(500));
    assert_eq!(allocator.fragmentation(), 100.0);
}

#[test]
fn test_allocator_memory_usage() {
    let mut allocator = MockAllocator::new(1024);
    
    assert_eq!(allocator.total_free().bytes(), 1024);
    assert_eq!(allocator.total_allocated().bytes(), 0);
    
    allocator.allocate(MemorySize::new(256));
    
    assert_eq!(allocator.total_free().bytes(), 768);
    assert_eq!(allocator.total_allocated().bytes(), 256);
}

#[test]
fn test_allocator_deallocation() {
    let mut allocator = MockAllocator::new(1024);
    
    let addr = allocator.allocate(MemorySize::new(512)).unwrap();
    assert_eq!(allocator.total_allocated().bytes(), 512);
    
    assert!(allocator.deallocate(addr, MemorySize::new(512)));
    assert_eq!(allocator.total_allocated().bytes(), 0);
    assert_eq!(allocator.total_free().bytes(), 1024);
}
