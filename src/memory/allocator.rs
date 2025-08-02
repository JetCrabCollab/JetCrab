use crate::vm::types::{AllocationCount, MemorySize};
use std::alloc::{alloc, dealloc, Layout};

pub struct MemoryAllocator {
    total_allocated: MemorySize,
    allocations: AllocationCount,
    deallocations: AllocationCount,
}

impl Default for MemoryAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryAllocator {
    pub fn new() -> Self {
        Self {
            total_allocated: MemorySize::new(0),
            allocations: AllocationCount::new(0),
            deallocations: AllocationCount::new(0),
        }
    }

    pub fn allocate(&mut self, size: MemorySize) -> *mut u8 {
        unsafe {
            let layout = Layout::from_size_align(size.as_usize(), 8).unwrap();
            let ptr = alloc(layout);

            if !ptr.is_null() {
                self.total_allocated = self.total_allocated.add(size);
                self.allocations.increment();
            }

            ptr
        }
    }

    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `ptr` is a valid pointer that was previously allocated by this allocator
    /// - `size` matches the size that was used when allocating `ptr`
    /// - `ptr` is not null
    pub unsafe fn deallocate(&mut self, ptr: *mut u8, size: MemorySize) {
        if !ptr.is_null() {
            let layout = Layout::from_size_align(size.as_usize(), 8).unwrap();
            dealloc(ptr, layout);

            self.total_allocated = self.total_allocated.sub(size);
            self.deallocations.increment();
        }
    }

    pub fn get_stats(&self) -> AllocatorStats {
        AllocatorStats {
            total_allocated: self.total_allocated,
            allocations: self.allocations,
            deallocations: self.deallocations,
        }
    }

    pub fn reset_stats(&mut self) {
        self.total_allocated = MemorySize::new(0);
        self.allocations.reset();
        self.deallocations.reset();
    }
}

pub struct AllocatorStats {
    pub total_allocated: MemorySize,
    pub allocations: AllocationCount,
    pub deallocations: AllocationCount,
}
