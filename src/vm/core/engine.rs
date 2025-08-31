//! VM Engine - Main execution engine that orchestrates all VM components
//!
//! The engine coordinates:
//! - Compilation pipeline
//! - Memory management
//! - Execution engine
//! - Runtime environment

use crate::vm::{
    compiler::BytecodeGenerator,
    error::VmError,
    executor::core::InstructionExecutorImpl,
    memory::{Heap, MemoryManager, Stack},
    runtime::{Builtins, Context},
    value::Value,
};

/// Main VM execution engine
pub struct VmEngine {
    /// Memory manager for heap and stack
    memory_manager: MemoryManager,
    /// Built-in functions and objects
    builtins: Builtins,
    /// Execution context
    context: Context,
    /// Bytecode generator
    bytecode_generator: BytecodeGenerator,
}

impl VmEngine {
    /// Create a new VM engine
    pub fn new() -> Self {
        Self {
            memory_manager: MemoryManager::new(),
            builtins: Builtins::new(),
            context: Context::new(),
            bytecode_generator: BytecodeGenerator::new(),
        }
    }

    /// Execute JavaScript source code
    pub fn execute(&mut self, source: &str) -> Result<Value, VmError> {
        // Parse and compile to bytecode
        let bytecode = self.bytecode_generator.generate(source)?;

        // Create execution context
        let mut executor = InstructionExecutorImpl::new(
            self.memory_manager.heap_mut(),
            self.memory_manager.stack_mut(),
            &mut self.builtins,
            &mut self.context,
        );

        // Execute bytecode
        executor.execute(&bytecode)
    }

    /// Execute pre-compiled bytecode
    pub fn execute_bytecode(
        &mut self,
        bytecode: &crate::vm::compiler::Bytecode,
    ) -> Result<Value, VmError> {
        let mut executor = InstructionExecutorImpl::new(
            self.memory_manager.heap_mut(),
            self.memory_manager.stack_mut(),
            &mut self.builtins,
            &mut self.context,
        );

        executor.execute(bytecode)
    }

    /// Get a reference to the memory manager
    pub fn memory_manager(&self) -> &MemoryManager {
        &self.memory_manager
    }

    /// Get a mutable reference to the memory manager
    pub fn memory_manager_mut(&mut self) -> &mut MemoryManager {
        &mut self.memory_manager
    }

    /// Get a reference to built-ins
    pub fn builtins(&self) -> &Builtins {
        &self.builtins
    }

    /// Get a mutable reference to built-ins
    pub fn builtins_mut(&mut self) -> &mut Builtins {
        &mut self.builtins
    }

    /// Get the current execution context
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// Get a mutable reference to the execution context
    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    /// Reset the VM engine to initial state
    pub fn reset(&mut self) {
        self.memory_manager = MemoryManager::new();
        self.context = Context::new();
    }

    /// Get memory statistics
    pub fn memory_stats(&self) -> crate::vm::memory::heap::HeapStats {
        self.memory_manager.heap().stats()
    }

    /// Force garbage collection
    pub fn force_gc(&mut self) {
        self.memory_manager.heap_mut().force_gc();
    }
}

impl Default for VmEngine {
    fn default() -> Self {
        Self::new()
    }
}
