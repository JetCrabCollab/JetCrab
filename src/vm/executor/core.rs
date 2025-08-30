//! # VM Executor Core
//!
//! Provides the main executor interface that combines all VM components
//! into a single, easy-to-use execution engine. This is the primary entry
//! point for executing bytecode in the JetCrab VM.
//!
//! ## Overview
//!
//! The `Executor` struct integrates:
//! - Stack management for value operations
//! - Heap management for object allocation
//! - Variable management for local/global variables
//! - Instruction execution engine
//!
//! ## Architecture
//!
//! The executor uses concrete implementations of the execution traits,
//! providing a complete VM runtime that can execute JavaScript-like
//! bytecode with proper memory management and error handling.
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::vm::executor::Executor;
//! use jetcrab::vm::bytecode::Bytecode;
//! use jetcrab::vm::value::Value;
//!
//! let mut executor = Executor::new();
//! let bytecode = Bytecode::new();
//! let constants = vec![Value::Number(42.0)];
//!
//! match executor.execute(&bytecode, &constants) {
//!     Ok(()) => println!("Execution successful"),
//!     Err(e) => eprintln!("Execution failed: {:?}", e),
//! }
//! ```

use super::{
    heap_manager::HeapManager, instruction_executor::InstructionExecutorImpl,
    stack_manager::StackManager, variable_manager::VariableManagerImpl, InstructionExecutor,
};
use crate::vm::bytecode::Bytecode;
use crate::vm::value::Value;

/// Main VM executor that combines all execution components
///
/// Provides a high-level interface for executing bytecode by integrating
/// stack management, heap management, variable management, and instruction
/// execution into a single, cohesive system.
pub struct Executor {
    instruction_executor: InstructionExecutorImpl<StackManager, HeapManager, VariableManagerImpl>,
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor {
    /// Creates a new executor with default components
    ///
    /// Initializes the executor with new instances of all required
    /// components: stack manager, heap manager, variable manager,
    /// and instruction executor.
    ///
    /// # Returns
    /// A new executor ready for bytecode execution
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::Executor;
    ///
    /// let mut executor = Executor::new();
    /// ```
    pub fn new() -> Self {
        let stack_manager = StackManager::new();
        let heap_manager = HeapManager::new();
        let variable_manager = VariableManagerImpl::new();

        let instruction_executor =
            InstructionExecutorImpl::new(stack_manager, heap_manager, variable_manager);

        Self {
            instruction_executor,
        }
    }

    /// Executes bytecode with the provided constants
    ///
    /// Runs the complete execution cycle for the given bytecode,
    /// using the provided constants array for constant lookups.
    ///
    /// # Arguments
    /// * `bytecode` - The bytecode to execute
    /// * `constants` - Array of constant values
    ///
    /// # Returns
    /// * `Ok(())` - Execution completed successfully
    /// * `Err(ExecutionError)` - Execution failed
    ///
    /// # Examples
    ///
    /// ```rust
    /// let bytecode = Bytecode::new();
    /// let constants = vec![Value::Number(42.0)];
    /// executor.execute(&bytecode, &constants)?;
    /// ```
    pub fn execute(
        &mut self,
        bytecode: &Bytecode,
        constants: &[Value],
    ) -> Result<(), crate::vm::executor::error_handler::ExecutionError> {
        self.instruction_executor.execute(bytecode, constants)
    }

    /// Gets read-only access to the VM stack
    ///
    /// Provides access to the current state of the execution stack
    /// for inspection and debugging purposes.
    pub fn stack(&self) -> &crate::vm::stack::Stack {
        self.instruction_executor.stack_manager().stack()
    }

    /// Gets mutable access to the VM stack
    ///
    /// Provides write access to the execution stack for direct
    /// manipulation and testing purposes.
    pub fn stack_mut(&mut self) -> &mut crate::vm::stack::Stack {
        self.instruction_executor.stack_manager_mut().stack_mut()
    }

    /// Gets read-only access to the VM heap
    ///
    /// Provides access to the current state of the execution heap
    /// for inspection and debugging purposes.
    pub fn heap(&self) -> &crate::vm::heap::Heap {
        self.instruction_executor.heap_manager().heap()
    }

    /// Gets mutable access to the VM heap
    ///
    /// Provides write access to the execution heap for direct
    /// manipulation and testing purposes.
    pub fn heap_mut(&mut self) -> &mut crate::vm::heap::Heap {
        self.instruction_executor.heap_manager_mut().heap_mut()
    }
}
