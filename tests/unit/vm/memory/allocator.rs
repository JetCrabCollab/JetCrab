use jetcrab::vm::memory::allocator::Allocator;
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
fn test_allocator_allocate() {
    let mut allocator = MockAllocator::new(1024);
    
    let addr1 = allocator.allocate(MemorySize::new(256));
    assert!(addr1.is_some());
    assert_eq!(allocator.total_allocated().bytes(), 256);
    
    let addr2 = allocator.allocate(MemorySize::new(512));
    assert!(addr2.is_some());
    assert_eq!(allocator.total_allocated().bytes(), 768);
}

#[test]
fn test_allocator_deallocate() {
    let mut allocator = MockAllocator::new(1024);
    
    let addr = allocator.allocate(MemorySize::new(256)).unwrap();
    assert_eq!(allocator.total_allocated().bytes(), 256);
    
    assert!(allocator.deallocate(addr, MemorySize::new(256)));
    assert_eq!(allocator.total_allocated().bytes(), 0);
}

#[test]
fn test_allocator_can_allocate() {
    let allocator = MockAllocator::new(1024);
    
    assert!(allocator.can_allocate(MemorySize::new(512)));
    assert!(allocator.can_allocate(MemorySize::new(1024)));
    assert!(!allocator.can_allocate(MemorySize::new(2048)));
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
