//! Unit tests for VM Generational Heap

use jetcrab::vm::memory::heap::generational::{
    CellSpace, CodeSpace, GarbageCollectionStats, GenerationalHeap, HeapStats, LargeObjectSpace,
    MapSpace, NewSpace, ObjectType, OldSpace, PromotionStats, PropertyCellSpace,
};
use jetcrab::vm::types::MemorySize;

#[test]
fn test_generational_heap_creation() {
    let heap = GenerationalHeap::new();
    let stats = heap.stats();
    assert_eq!(stats.total_allocations, 0);
    assert_eq!(stats.total_deallocations, 0);
    assert_eq!(heap.total_allocated().bytes(), 0);
}

#[test]
fn test_generational_heap_default() {
    let heap = GenerationalHeap::default();
    let stats = heap.stats();
    assert_eq!(stats.total_allocations, 0);
    assert_eq!(stats.total_deallocations, 0);
}

#[test]
fn test_new_space_creation() {
    let new_space = NewSpace::new(MemorySize::new(1024));
    assert_eq!(new_space.total_allocated().bytes(), 0);
    assert!(new_space.total_free().bytes() > 0);
}

#[test]
fn test_new_space_allocate() {
    let mut new_space = NewSpace::new(MemorySize::new(1024));
    let result = new_space.allocate(MemorySize::new(64));
    assert!(result.is_some());
    assert_eq!(new_space.total_allocated().bytes(), 64);
}

#[test]
fn test_new_space_switch_spaces() {
    let mut new_space = NewSpace::new(MemorySize::new(1024));
    new_space.allocate(MemorySize::new(64));
    assert_eq!(new_space.total_allocated().bytes(), 64);

    new_space.switch_spaces();
    assert_eq!(new_space.total_allocated().bytes(), 0);
}

#[test]
fn test_new_space_is_nearly_full() {
    let new_space = NewSpace::new(MemorySize::new(1024));
    assert!(!new_space.is_nearly_full());
}

#[test]
fn test_old_space_creation() {
    let old_space = OldSpace::new(MemorySize::new(1024));
    assert_eq!(old_space.total_allocated().bytes(), 0);
    assert!(old_space.total_free().bytes() > 0);
}

#[test]
fn test_old_space_allocate() {
    let mut old_space = OldSpace::new(MemorySize::new(1024));
    let result = old_space.allocate(MemorySize::new(64));
    assert!(result.is_some());
    assert_eq!(old_space.total_allocated().bytes(), 64);
}

#[test]
fn test_large_object_space_creation() {
    let large_space = LargeObjectSpace::new(MemorySize::new(1024));
    assert_eq!(large_space.total_allocated().bytes(), 0);
    assert!(large_space.total_free().bytes() > 0);
}

#[test]
fn test_large_object_space_allocate() {
    let mut large_space = LargeObjectSpace::new(MemorySize::new(1024));
    let result = large_space.allocate(MemorySize::new(64));
    assert!(result.is_some());
    assert_eq!(large_space.total_allocated().bytes(), 64);
}

#[test]
fn test_code_space_creation() {
    let code_space = CodeSpace::new(MemorySize::new(1024));
    assert_eq!(code_space.total_allocated().bytes(), 0);
    assert!(code_space.total_free().bytes() > 0);
}

#[test]
fn test_code_space_allocate() {
    let mut code_space = CodeSpace::new(MemorySize::new(1024));
    let result = code_space.allocate(MemorySize::new(64));
    assert!(result.is_some());
    assert_eq!(code_space.total_allocated().bytes(), 64);
}

#[test]
fn test_cell_space_creation() {
    let cell_space = CellSpace::new(64, 100);
    assert_eq!(cell_space.total_allocated().bytes(), 0);
    assert!(cell_space.total_free().bytes() > 0);
}

#[test]
fn test_cell_space_allocate() {
    let mut cell_space = CellSpace::new(64, 100);
    let result = cell_space.allocate(MemorySize::new(64));
    assert!(result.is_some());
    assert_eq!(cell_space.total_allocated().bytes(), 64);
}

#[test]
fn test_property_cell_space_creation() {
    let prop_space = PropertyCellSpace::new(MemorySize::new(1024));
    assert_eq!(prop_space.total_allocated().bytes(), 0);
    assert!(prop_space.total_free().bytes() > 0);
}

#[test]
fn test_property_cell_space_allocate() {
    let mut prop_space = PropertyCellSpace::new(MemorySize::new(1024));
    let result = prop_space.allocate(MemorySize::new(64));
    assert!(result.is_some());
    assert_eq!(prop_space.total_allocated().bytes(), 64);
}

#[test]
fn test_map_space_creation() {
    let map_space = MapSpace::new(MemorySize::new(1024));
    assert_eq!(map_space.total_allocated().bytes(), 0);
    assert!(map_space.total_free().bytes() > 0);
}

#[test]
fn test_map_space_allocate() {
    let mut map_space = MapSpace::new(MemorySize::new(1024));
    let result = map_space.allocate(MemorySize::new(64));
    assert!(result.is_some());
    assert_eq!(map_space.total_allocated().bytes(), 64);
}

#[test]
fn test_object_type_enum() {
    let object_type = ObjectType::Object;
    assert!(matches!(object_type, ObjectType::Object));

    let array_type = ObjectType::Array;
    assert!(matches!(array_type, ObjectType::Array));

    let function_type = ObjectType::Function;
    assert!(matches!(function_type, ObjectType::Function));

    let string_type = ObjectType::String;
    assert!(matches!(string_type, ObjectType::String));

    let number_type = ObjectType::Number;
    assert!(matches!(number_type, ObjectType::Number));

    let boolean_type = ObjectType::Boolean;
    assert!(matches!(boolean_type, ObjectType::Boolean));
}

#[test]
fn test_heap_stats_creation() {
    let stats = HeapStats::new();
    assert_eq!(stats.total_allocations, 0);
    assert_eq!(stats.total_deallocations, 0);
    assert_eq!(stats.total_allocated.bytes(), 0);
    assert_eq!(stats.total_freed.bytes(), 0);
    assert_eq!(stats.peak_usage.bytes(), 0);
    assert_eq!(stats.current_usage.bytes(), 0);
}

#[test]
fn test_heap_stats_default() {
    let stats = HeapStats::default();
    assert_eq!(stats.total_allocations, 0);
    assert_eq!(stats.total_deallocations, 0);
}

#[test]
fn test_garbage_collection_stats() {
    let gc_stats = GarbageCollectionStats {
        duration_micros: 1000,
        objects_collected: 10,
        memory_freed: MemorySize::new(1024),
        new_space_collections: 1,
        old_space_collections: 0,
    };

    assert_eq!(gc_stats.duration_micros, 1000);
    assert_eq!(gc_stats.objects_collected, 10);
    assert_eq!(gc_stats.memory_freed.bytes(), 1024);
    assert_eq!(gc_stats.new_space_collections, 1);
    assert_eq!(gc_stats.old_space_collections, 0);
}

#[test]
fn test_promotion_stats() {
    let promo_stats = PromotionStats {
        objects_promoted: 5,
        memory_promoted: MemorySize::new(512),
        promotion_duration_micros: 500,
    };

    assert_eq!(promo_stats.objects_promoted, 5);
    assert_eq!(promo_stats.memory_promoted.bytes(), 512);
    assert_eq!(promo_stats.promotion_duration_micros, 500);
}

#[test]
fn test_generational_heap_alloc_object() {
    let mut heap = GenerationalHeap::new();
    let handle = heap.alloc_object(MemorySize::new(64), ObjectType::Object);
    assert!(handle.is_some());
    assert_eq!(heap.stats().total_allocations, 1);
}

#[test]
fn test_generational_heap_alloc_code() {
    let mut heap = GenerationalHeap::new();
    let handle = heap.alloc_code(MemorySize::new(64));
    assert!(handle.is_some());
}

#[test]
fn test_generational_heap_alloc_property_cell() {
    let mut heap = GenerationalHeap::new();
    let handle = heap.alloc_property_cell(MemorySize::new(64));
    assert!(handle.is_some());
}

#[test]
fn test_generational_heap_alloc_map() {
    let mut heap = GenerationalHeap::new();
    let handle = heap.alloc_map(MemorySize::new(64));
    assert!(handle.is_some());
}

#[test]
fn test_generational_heap_collect_garbage() {
    let mut heap = GenerationalHeap::new();
    let gc_stats = heap.collect_garbage();
    assert!(gc_stats.duration_micros >= 0);
    assert_eq!(gc_stats.objects_collected, 0);
    assert_eq!(gc_stats.memory_freed.bytes(), 0);
}

#[test]
fn test_generational_heap_promote_objects() {
    let mut heap = GenerationalHeap::new();
    let promo_stats = heap.promote_objects();
    assert!(promo_stats.promotion_duration_micros >= 0);
    assert_eq!(promo_stats.objects_promoted, 0);
    assert_eq!(promo_stats.memory_promoted.bytes(), 0);
}
