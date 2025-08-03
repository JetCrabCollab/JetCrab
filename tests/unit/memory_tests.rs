use jetcrab::memory::allocator::MemoryAllocator;
use jetcrab::memory::*;
use jetcrab::vm::types::*;
use jetcrab::vm::value::Value;
use jetcrab::vm::HeapHandleId;

#[test]
fn test_memory_size_new() {
    let size = MemorySize::new(1024);
    assert_eq!(size.as_usize(), 1024);
}

#[test]
fn test_memory_size_add() {
    let size1 = MemorySize::new(512);
    let size2 = MemorySize::new(256);
    let result = size1.add(size2);
    assert_eq!(result.as_usize(), 768);
}

#[test]
fn test_memory_size_sub() {
    let size1 = MemorySize::new(1024);
    let size2 = MemorySize::new(256);
    let result = size1.sub(size2);
    assert_eq!(result.as_usize(), 768);
}

#[test]
fn test_allocation_count_new() {
    let count = AllocationCount::new(42);
    assert_eq!(count.as_usize(), 42);
}

#[test]
fn test_allocation_count_increment() {
    let mut count = AllocationCount::new(10);
    count.increment();
    assert_eq!(count.as_usize(), 11);
}

#[test]
fn test_object_id_new() {
    let id = ObjectId::new(123);
    assert_eq!(id.as_usize(), 123);
}

#[test]
fn test_object_id_equality() {
    let id1 = ObjectId::new(42);
    let id2 = ObjectId::new(42);
    let id3 = ObjectId::new(43);
    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_heap_handle_id_new() {
    let handle_id = HeapHandleId::new(456);
    assert_eq!(handle_id.as_usize(), 456);
}

#[test]
fn test_heap_handle_id_equality() {
    let handle1 = HeapHandleId::new(100);
    let handle2 = HeapHandleId::new(100);
    let handle3 = HeapHandleId::new(101);
    assert_eq!(handle1, handle2);
    assert_ne!(handle1, handle3);
}

#[test]
fn test_memory_heap_new() {
    let heap = MemoryHeap::new();
    assert_eq!(heap.total_allocated().as_usize(), 0);
    assert_eq!(heap.object_count().as_usize(), 0);
}

#[test]
fn test_memory_heap_allocate() {
    let mut heap = MemoryHeap::new();
    let value = Value::Number(42.0);
    let _id = heap.allocate(value);
    assert_eq!(heap.object_count().as_usize(), 1);
    assert!(heap.total_allocated().as_usize() > 0);
}

#[test]
fn test_memory_heap_get() {
    let mut heap = MemoryHeap::new();
    let value = Value::String("test".to_string());
    let id = heap.allocate(value.clone());

    let retrieved = heap.get(id);
    assert!(retrieved.is_some());
    assert_eq!(*retrieved.unwrap(), value);
}

#[test]
fn test_memory_heap_remove() {
    let mut heap = MemoryHeap::new();
    let value = Value::Boolean(true);
    let id = heap.allocate(value.clone());

    let removed = heap.remove(id);
    assert!(removed.is_some());
    assert_eq!(removed.unwrap(), value);
    assert_eq!(heap.object_count().as_usize(), 0);
}

#[test]
fn test_memory_heap_mark_and_sweep() {
    let mut heap = MemoryHeap::new();
    let value1 = Value::Number(1.0);
    let value2 = Value::Number(2.0);

    let id1 = heap.allocate(value1);
    let id2 = heap.allocate(value2);

    heap.mark(id1);
    let freed = heap.sweep();

    assert!(heap.get(id1).is_some());
    assert!(heap.get(id2).is_none());
    assert!(freed.as_usize() > 0);
}

#[test]
fn test_memory_allocator_new() {
    let allocator = MemoryAllocator::new();
    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), 0);
}

#[test]
fn test_memory_allocator_allocate() {
    let mut allocator = MemoryAllocator::new();
    let size = MemorySize::new(1024);
    let ptr = allocator.allocate(size);
    assert!(!ptr.is_null());

    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), 1024);
}

#[test]
fn test_memory_allocator_deallocate() {
    let mut allocator = MemoryAllocator::new();
    let size = MemorySize::new(512);
    let ptr = allocator.allocate(size);

    unsafe {
        allocator.deallocate(ptr, size);
    }

    let stats = allocator.get_stats();
    assert_eq!(stats.total_allocated.as_usize(), 0);
}

#[test]
fn test_garbage_collector_new() {
    let heap = MemoryHeap::new();
    let gc = GarbageCollector::new(heap);
    assert_eq!(gc.allocated_bytes().as_usize(), 0);
}

#[test]
fn test_garbage_collector_allocate() {
    let heap = MemoryHeap::new();
    let mut gc = GarbageCollector::new(heap);
    let value = Value::String("test".to_string());
    let object_id = gc.allocate(value);
    assert!(gc.get(object_id).is_some());
}

#[test]
fn test_garbage_collector_add_remove_root() {
    let heap = MemoryHeap::new();
    let mut gc = GarbageCollector::new(heap);
    let value = Value::Number(42.0);
    let object_id = gc.allocate(value);

    gc.add_root(object_id);
    assert!(gc.heap().contains(HeapId::from(object_id.as_usize())));

    gc.remove_root(object_id);
    assert!(gc.heap().contains(HeapId::from(object_id.as_usize())));
}

#[test]
fn test_garbage_collector_collect() {
    let heap = MemoryHeap::new();
    let mut gc = GarbageCollector::new(heap);
    let value = Value::Number(42.0);
    let object_id = gc.allocate(value);

    gc.add_root(object_id);
    let stats = gc.collect();
    assert_eq!(stats.total_collections.as_usize(), 1);
}
