//! JetCrab Virtual Machine - High-performance JavaScript engine
//!
//! This module provides a complete JavaScript execution environment with:
//! - Advanced memory management with generational heap
//! - Optimized bytecode execution
//! - Comprehensive runtime system
//! - High-performance garbage collection

pub mod compiler;
pub mod core;
pub mod error;
pub mod executor;
pub mod frame;
pub mod function;
pub mod function_manager;
pub mod handle;
pub mod instructions;
pub mod memory;
pub mod registers;
pub mod runtime;
pub mod types;
pub mod value;

// Core VM components
pub use core::{VmConfig, VmCore, VmEngine};

// Compiler components
pub use compiler::{Bytecode, BytecodeGenerator, BytecodeOptimizer};

// Runtime components
pub use runtime::{Builtins, Context, Function, Object};

// Memory management
pub use memory::{Heap, MemoryManager, Stack};

// Executor components
pub use executor::{
    instruction_handlers::{
        ArithmeticHandler, ComparisonHandler, ControlFlowHandler, ObjectHandler,
    },
    InstructionExecutorImpl,
};

// Types and utilities
pub use error::VmError;
pub use types::{HeapHandleId, MemorySize};
pub use value::Value;
