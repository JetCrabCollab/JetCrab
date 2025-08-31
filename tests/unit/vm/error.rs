//! VM Error Unit Tests
//!
//! Tests for VM error handling

use jetcrab::vm::error::VmError;

#[test]
fn test_vm_error_creation() {
    let error = VmError::CompilationError("Test error".to_string());
    assert!(error.to_string().contains("Test error"));
}

#[test]
fn test_vm_error_display() {
    let error = VmError::RuntimeError(jetcrab::vm::runtime::errors::RuntimeError::type_error(
        "test", "number", "string",
    ));
    assert!(error.to_string().contains("TypeError"));
}

#[test]
fn test_vm_error_variants() {
    let compilation_error = VmError::CompilationError("compilation".to_string());
    let runtime_error = VmError::RuntimeError(
        jetcrab::vm::runtime::errors::RuntimeError::type_error("test", "number", "string"),
    );
    let memory_error = VmError::MemoryError("memory".to_string());
    let execution_error = VmError::ExecutionError("execution".to_string());

    assert!(compilation_error.to_string().contains("compilation"));
    assert!(runtime_error.to_string().contains("TypeError"));
    assert!(memory_error.to_string().contains("memory"));
    assert!(execution_error.to_string().contains("execution"));
}
