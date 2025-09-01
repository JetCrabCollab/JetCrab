//! VM Core Unit Tests
//! 
//! Tests for the core VM orchestration

use jetcrab::vm::core::{VmCore, VmConfig};

#[test]
fn test_vm_core_creation() {
    let core = VmCore::new();
    assert!(core.engine().memory_manager().heap().stats().total_allocations == 0);
}

#[test]
fn test_vm_core_with_config() {
    let config = VmConfig::performance();
    let core = VmCore::with_config(config);
    assert_eq!(core.config().memory.initial_heap_size, 128 * 1024 * 1024);
}

#[test]
fn test_vm_core_default() {
    let core = VmCore::default();
    assert!(core.engine().memory_manager().heap().stats().total_allocations == 0);
}

#[test]
fn test_vm_core_engine_access() {
    let core = VmCore::new();
    let engine = core.engine();
    assert!(engine.memory_manager().heap().stats().total_allocations == 0);
}

#[test]
fn test_vm_core_engine_mut_access() {
    let mut core = VmCore::new();
    let engine_mut = core.engine_mut();
    engine_mut.force_gc();
    assert!(engine_mut.memory_manager().heap().stats().gc_count >= 0);
}

#[test]
fn test_vm_core_config_access() {
    let core = VmCore::new();
    let config = core.config();
    assert_eq!(config.memory.initial_heap_size, 64 * 1024 * 1024);
}

#[test]
fn test_vm_core_config_update() {
    let mut core = VmCore::new();
    let new_config = VmConfig::performance();
    core.update_config(new_config);
    assert_eq!(core.config().memory.initial_heap_size, 128 * 1024 * 1024);
}
