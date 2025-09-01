//! VM Config Unit Tests
//! 
//! Tests for VM configuration and settings

use jetcrab::vm::core::VmConfig;

#[test]
fn test_vm_config_default() {
    let config = VmConfig::default();
    assert_eq!(config.memory.initial_heap_size, 64 * 1024 * 1024);
    assert_eq!(config.memory.max_heap_size, 1024 * 1024 * 1024);
    assert!(config.gc.enable_minor_gc);
    assert!(config.gc.enable_major_gc);
}

#[test]
fn test_vm_config_performance() {
    let config = VmConfig::performance();
    assert_eq!(config.memory.initial_heap_size, 128 * 1024 * 1024);
    assert_eq!(config.memory.max_heap_size, 2048 * 1024 * 1024);
    assert!(config.gc.enable_background_gc);
    assert_eq!(config.gc.gc_threshold, 0.85);
}

#[test]
fn test_vm_config_debug() {
    let config = VmConfig::debug();
    assert!(config.debug.debug_mode);
    assert!(config.debug.verbose_logging);
    assert!(config.debug.memory_tracing);
    assert_eq!(config.debug.log_level, 4);
}

#[test]
fn test_vm_config_memory_efficient() {
    let config = VmConfig::memory_efficient();
    assert_eq!(config.memory.initial_heap_size, 16 * 1024 * 1024);
    assert_eq!(config.memory.max_heap_size, 256 * 1024 * 1024);
    assert_eq!(config.gc.gc_threshold, 0.6);
    assert_eq!(config.gc.minor_gc_frequency, 4);
}

#[test]
fn test_memory_config_default() {
    let config = VmConfig::default();
    assert_eq!(config.memory.new_space_size, 16 * 1024 * 1024);
    assert_eq!(config.memory.old_space_size, 256 * 1024 * 1024);
    assert_eq!(config.memory.large_object_threshold, 1024 * 1024);
}

#[test]
fn test_gc_config_default() {
    let config = VmConfig::default();
    assert!(config.gc.enable_minor_gc);
    assert!(config.gc.enable_major_gc);
    assert!(config.gc.enable_incremental_gc);
    assert!(!config.gc.enable_background_gc);
    assert_eq!(config.gc.gc_threshold, 0.75);
}

#[test]
fn test_performance_config_default() {
    let config = VmConfig::default();
    assert!(!config.performance.enable_jit);
    assert!(config.performance.enable_optimization);
    assert!(config.performance.enable_inline_caching);
    assert!(config.performance.enable_hidden_classes);
    assert!(config.performance.enable_string_interning);
}

#[test]
fn test_debug_config_default() {
    let config = VmConfig::default();
    assert!(!config.debug.debug_mode);
    assert!(!config.debug.verbose_logging);
    assert!(!config.debug.memory_tracing);
    assert!(!config.debug.instruction_tracing);
    assert!(!config.debug.gc_tracing);
    assert_eq!(config.debug.log_level, 1);
}
