//! VM Engine - Main execution engine that orchestrates all VM components

use crate::vm::{error::VmError, value::Value};

/// Main VM execution engine
pub struct VmEngine {
    /// Memory manager for heap and stack
    memory_manager: crate::vm::memory::MemoryManager,
}

impl VmEngine {
    /// Create a new VM engine
    pub fn new() -> Self {
        Self {
            memory_manager: crate::vm::memory::MemoryManager::new(),
        }
    }

    /// Execute JavaScript source code
    pub fn execute(&mut self, _source: &str) -> Result<Value, VmError> {
        // For now, just return a simple value
        Ok(Value::Number(42.0))
    }

    /// Execute pre-compiled bytecode
    pub fn execute_bytecode(
        &mut self,
        _bytecode: &crate::vm::compiler::Bytecode,
    ) -> Result<Value, VmError> {
        // For now, just return a simple value
        Ok(Value::Number(42.0))
    }

    /// Get a reference to the memory manager
    pub fn memory_manager(&self) -> &crate::vm::memory::MemoryManager {
        &self.memory_manager
    }

    /// Get a mutable reference to the memory manager
    pub fn memory_manager_mut(&mut self) -> &mut crate::vm::memory::MemoryManager {
        &mut self.memory_manager
    }

    /// Reset the VM engine to initial state
    pub fn reset(&mut self) {
        self.memory_manager = crate::vm::memory::MemoryManager::new();
    }

    /// Force garbage collection
    pub fn force_gc(&mut self) {
        // For now, do nothing
    }

    /// Get memory statistics
    pub fn memory_stats(&self) -> String {
        self.memory_manager.heap_info()
    }
}

impl Default for VmEngine {
    fn default() -> Self {
        Self::new()
    }
}
