//! VM Memory Heap Unit Tests - Mirroring src/vm/memory/heap/ structure
//! 
//! This module contains unit tests for heap components:
//! - generational.rs: Generational heap
//! - optimized_arrays.rs: Optimized array representations
//! - object_shapes.rs: Object shapes (hidden classes)
//! - string_interning.rs: String interning system
//! - allocation/: Memory allocation strategies
//! - spaces/: Memory spaces
//! - gc/: Garbage collection

use jetcrab::vm::memory::heap::{Heap, HeapConfig, HeapStats};

#[test]
fn test_heap_creation() {
    let config = HeapConfig::default();
    let heap = Heap::new(config);
    
    assert!(heap.is_initialized());
    assert_eq!(heap.total_size, config.initial_size);
}

#[test]
fn test_heap_with_custom_config() {
    let config = HeapConfig::new()
        .with_initial_size(128 * 1024 * 1024) // 128MB
        .with_max_size(512 * 1024 * 1024); // 512MB
    
    let heap = Heap::new(config);
    
    assert_eq!(heap.total_size, 128 * 1024 * 1024);
    assert_eq!(heap.max_size, 512 * 1024 * 1024);
}

#[test]
fn test_heap_allocation() {
    let mut heap = Heap::new(HeapConfig::default());
    
    let handle = heap.allocate(1024);
    assert!(handle.is_some());
    
    let stats = heap.stats();
    assert_eq!(stats.total_allocated, 1024);
    assert_eq!(stats.allocation_count, 1);
}

#[test]
fn test_heap_deallocation() {
    let mut heap = Heap::new(HeapConfig::default());
    
    let handle = heap.allocate(1024).unwrap();
    
    let result = heap.deallocate(handle);
    assert!(result);
    
    let stats = heap.stats();
    assert_eq!(stats.total_allocated, 0);
    assert_eq!(stats.deallocation_count, 1);
}

#[test]
fn test_heap_stats() {
    let mut heap = Heap::new(HeapConfig::default());
    
    heap.allocate(1024);
    heap.allocate(2048);
    
    let stats = heap.stats();
    
    assert_eq!(stats.total_allocated, 3072);
    assert_eq!(stats.allocation_count, 2);
    assert_eq!(stats.deallocation_count, 0);
    assert!(stats.fragmentation >= 0.0);
}

#[test]
fn test_heap_config_default() {
    let config = HeapConfig::default();
    
    assert_eq!(config.initial_size, 64 * 1024 * 1024); // 64MB
    assert_eq!(config.max_size, 1024 * 1024 * 1024); // 1GB
    assert_eq!(config.gc_threshold, 0.8);
    assert!(config.enable_compaction);
}

#[test]
fn test_heap_config_custom() {
    let config = HeapConfig::new()
        .with_initial_size(32 * 1024 * 1024) // 32MB
        .with_max_size(256 * 1024 * 1024) // 256MB
        .with_gc_threshold(0.7)
        .with_compaction(false);
    
    assert_eq!(config.initial_size, 32 * 1024 * 1024);
    assert_eq!(config.max_size, 256 * 1024 * 1024);
    assert_eq!(config.gc_threshold, 0.7);
    assert!(!config.enable_compaction);
}

#[test]
fn test_heap_gc_trigger() {
    let mut heap = Heap::new(HeapConfig::default());
    
    // Initially no GC should be needed
    assert!(!heap.should_trigger_gc());
    
    // Allocate enough memory to trigger GC
    for _ in 0..100 {
        heap.allocate(1024 * 1024); // 1MB each
    }
    
    // Now GC should be needed
    assert!(heap.should_trigger_gc());
}

#[test]
fn test_heap_compaction() {
    let mut heap = Heap::new(HeapConfig::default());
    
    // Allocate and deallocate to create fragmentation
    let handles: Vec<_> = (0..10)
        .map(|_| heap.allocate(1024).unwrap())
        .collect();
    
    // Deallocate some to create holes
    heap.deallocate(handles[2]);
    heap.deallocate(handles[5]);
    heap.deallocate(handles[8]);
    
    let before_fragmentation = heap.stats().fragmentation;
    
    // Compact the heap
    let compaction_stats = heap.compact();
    
    let after_fragmentation = heap.stats().fragmentation;
    assert!(after_fragmentation < before_fragmentation);
    assert!(compaction_stats.objects_moved > 0);
}
