//! VM Unit Tests - Mirroring src/vm/ structure
//! 
//! This module contains unit tests for all VM components:
//! - core/: Core VM engine and configuration
//! - compiler/: Bytecode compilation
//! - runtime/: JavaScript runtime
//! - memory/: Memory management
//! - executor/: Bytecode execution
//! - types/: Specialized types
//! - Individual files: error, value, handle, frame, registers, instructions

use jetcrab::vm::{VirtualMachine, VmConfig, VmStats};

#[test]
fn test_vm_creation() {
    let config = VmConfig::default();
    let vm = VirtualMachine::new(config);
    
    assert!(vm.is_initialized());
    assert!(vm.is_ready());
}

#[test]
fn test_vm_with_custom_config() {
    let config = VmConfig::new()
        .with_memory_size(128 * 1024 * 1024) // 128MB
        .with_stack_size(1024 * 1024); // 1MB
    
    let vm = VirtualMachine::new(config);
    
    assert_eq!(vm.memory_size(), 128 * 1024 * 1024);
    assert_eq!(vm.stack_size(), 1024 * 1024);
}

#[test]
fn test_vm_initialization() {
    let vm = VirtualMachine::default();
    
    assert!(vm.is_initialized());
    assert!(vm.is_ready());
    assert!(!vm.is_shutdown());
}

#[test]
fn test_vm_config_access() {
    let config = VmConfig::new()
        .with_timeout(std::time::Duration::from_secs(30));
    
    let vm = VirtualMachine::new(config);
    
    let vm_config = vm.config();
    assert_eq!(vm_config.timeout, Some(std::time::Duration::from_secs(30)));
}

#[test]
fn test_vm_status() {
    let vm = VirtualMachine::default();
    
    assert!(vm.is_initialized());
    assert!(vm.is_ready());
    assert!(!vm.is_shutdown());
}

#[test]
fn test_vm_shutdown() {
    let mut vm = VirtualMachine::default();
    
    assert!(vm.is_ready());
    
    vm.shutdown();
    
    assert!(vm.is_shutdown());
    assert!(!vm.is_ready());
}

#[test]
fn test_vm_stats() {
    let vm = VirtualMachine::default();
    
    let stats = vm.stats();
    
    assert_eq!(stats.uptime, 0);
    assert_eq!(stats.instruction_count, 0);
    assert_eq!(stats.memory_usage, 0);
}

#[test]
fn test_vm_config_default() {
    let config = VmConfig::default();
    
    assert_eq!(config.memory_size, 64 * 1024 * 1024); // 64MB
    assert_eq!(config.stack_size, 1024 * 1024); // 1MB
    assert_eq!(config.registers, 16);
    assert!(config.enable_gc);
}

#[test]
fn test_vm_config_custom() {
    let config = VmConfig::new()
        .with_memory_size(32 * 1024 * 1024) // 32MB
        .with_stack_size(512 * 1024) // 512KB
        .with_registers(32)
        .with_gc(false);
    
    assert_eq!(config.memory_size, 32 * 1024 * 1024);
    assert_eq!(config.stack_size, 512 * 1024);
    assert_eq!(config.registers, 32);
    assert!(!config.enable_gc);
}

#[test]
fn test_vm_memory_management() {
    let mut vm = VirtualMachine::default();
    
    // Allocate memory
    let addr = vm.allocate_memory(1024);
    assert!(addr.is_some());
    
    // Check memory usage
    let stats = vm.stats();
    assert!(stats.memory_usage > 0);
    
    // Deallocate memory
    let result = vm.deallocate_memory(addr.unwrap());
    assert!(result);
}

#[test]
fn test_vm_stack_operations() {
    let mut vm = VirtualMachine::default();
    
    // Push values to stack
    vm.push_value(jetcrab::vm::value::Value::Number(42.0));
    vm.push_value(jetcrab::vm::value::Value::String("hello".to_string()));
    
    // Check stack size
    assert_eq!(vm.stack_size(), 2);
    
    // Pop values from stack
    let string_val = vm.pop_value();
    let number_val = vm.pop_value();
    
    assert!(matches!(string_val, Some(jetcrab::vm::value::Value::String(ref s)) if s == "hello"));
    assert!(matches!(number_val, Some(jetcrab::vm::value::Value::Number(42.0)));
}
