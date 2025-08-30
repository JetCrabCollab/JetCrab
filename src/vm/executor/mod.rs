//! # VM Executor Module
//!
//! This module provides the execution engine for the JetCrab virtual machine.
//! It handles instruction execution, control flow, and memory management through
//! a modular architecture of specialized handlers.
//!
//! ## Architecture
//!
//! The executor is built around several key components:
//!
//! - **Instruction Handlers**: Specialized modules for different types of operations
//! - **Instruction Dispatcher**: Routes instructions to appropriate handlers
//! - **Traits**: Define interfaces for stack, heap, and variable operations
//! - **Error Handling**: Comprehensive error handling and recovery
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::vm::executor::{InstructionExecutor, InstructionExecutorImpl};
//! use jetcrab::vm::bytecode::Bytecode;
//! use jetcrab::vm::value::Value;
//!
//! let mut executor = InstructionExecutorImpl::new();
//! let bytecode = Bytecode::new();
//! let constants = vec![Value::Number(42.0)];
//!
//! match executor.execute(&bytecode, &constants) {
//!     Ok(()) => println!("Execution completed successfully"),
//!     Err(e) => eprintln!("Execution failed: {:?}", e),
//! }
//! ```

pub mod error_handler;
pub mod instruction_dispatcher;
pub mod instruction_handlers;
pub mod instruction_executor;
pub mod traits;
pub mod stack_manager;
pub mod heap_manager;
pub mod variable_manager;

pub use error_handler::ExecutionError;
pub use instruction_dispatcher::InstructionDispatcher;
pub use instruction_executor::InstructionExecutorImpl;
pub use traits::{HeapOperations, InstructionExecutor, StackOperations, VariableManager};

pub mod core;
pub use core::Executor;
