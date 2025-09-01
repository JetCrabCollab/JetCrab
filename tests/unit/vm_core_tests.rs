//! Unit tests for VM Core modules

use jetcrab::vm::core::config::{DebugConfig, GcConfig, MemoryConfig, PerformanceConfig, VmConfig};
use jetcrab::vm::core::engine::VmEngine;
use jetcrab::vm::core::VmCore;

#[test]
fn test_vm_config_default() {
    let config = VmConfig::default();
    assert_eq!(config.memory.initial_heap_size, 64 * 1024 * 1024);
    assert!(config.gc.enable_minor_gc);
    assert!(!config.performance.enable_jit);
    assert!(!config.debug.debug_mode);
}

#[test]
fn test_vm_config_performance() {
    let config = VmConfig::performance();
    assert_eq!(config.memory.initial_heap_size, 128 * 1024 * 1024);
}

#[test]
fn test_vm_config_debug() {
    let config = VmConfig::debug();
    assert!(config.debug.debug_mode);
}

#[test]
fn test_memory_config_default() {
    let config = MemoryConfig::default();
    assert_eq!(config.initial_heap_size, 64 * 1024 * 1024);
}

#[test]
fn test_gc_config_default() {
    let config = GcConfig::default();
    assert!(config.enable_minor_gc);
}

#[test]
fn test_performance_config_default() {
    let config = PerformanceConfig::default();
    assert!(!config.enable_jit);
}

#[test]
fn test_debug_config_default() {
    let config = DebugConfig::default();
    assert!(!config.debug_mode);
}

#[test]
fn test_vm_engine_creation() {
    let engine = VmEngine::new();
    let memory_stats = engine.memory_stats();
    assert!(!memory_stats.is_empty());
}

#[test]
fn test_vm_engine_default() {
    let engine = VmEngine::default();
    let memory_stats = engine.memory_stats();
    assert!(!memory_stats.is_empty());
}

#[test]
fn test_vm_core_creation() {
    let core = VmCore::new();
    let config = core.config();
    assert_eq!(config.memory.initial_heap_size, 64 * 1024 * 1024);
}

#[test]
fn test_vm_core_default() {
    let core = VmCore::default();
    let config = core.config();
    assert_eq!(config.memory.initial_heap_size, 64 * 1024 * 1024);
}

#[test]
fn test_vm_core_with_config() {
    let config = VmConfig::performance();
    let core = VmCore::with_config(config);
    let core_config = core.config();
    assert_eq!(core_config.memory.initial_heap_size, 128 * 1024 * 1024);
}
