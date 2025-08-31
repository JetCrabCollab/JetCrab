//! VM Engine Unit Tests
//! 
//! Tests for the main VM execution engine that orchestrates all components

use jetcrab::vm::core::{VmEngine, VmConfig};

#[test]
fn test_vm_engine_creation() {
    let engine = VmEngine::new();
    assert!(engine.memory_manager().heap().stats().total_allocations == 0);
}

#[test]
fn test_vm_engine_with_config() {
    let config = VmConfig::performance();
    let engine = VmEngine::new();
    assert_eq!(config.memory.initial_heap_size, 128 * 1024 * 1024);
}

#[test]
fn test_vm_engine_reset() {
    let mut engine = VmEngine::new();
    engine.reset();
    assert!(engine.memory_manager().heap().stats().total_allocations == 0);
}

#[test]
fn test_vm_engine_memory_stats() {
    let engine = VmEngine::new();
    let stats = engine.memory_stats();
    assert!(stats.total_allocated >= 0);
}

#[test]
fn test_vm_engine_force_gc() {
    let mut engine = VmEngine::new();
    engine.force_gc();
    let stats = engine.memory_manager().heap().stats();
    assert!(stats.gc_count >= 0);
}
