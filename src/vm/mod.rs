//! JetCrab Virtual Machine - High-performance JavaScript engine
//! 
//! This module provides a complete JavaScript execution environment with:
//! - Advanced memory management with generational heap
//! - Optimized bytecode execution
//! - Comprehensive runtime system
//! - High-performance garbage collection

pub mod core;
pub mod compiler;
pub mod runtime;
pub mod memory;
pub mod executor;
pub mod types;
pub mod error;
pub mod value;
pub mod handle;
pub mod frame;
pub mod registers;
pub mod instructions;

// Core VM components
pub use core::{VmCore, VmEngine, VmConfig};

// Compiler components
pub use compiler::{Bytecode, BytecodeGenerator, BytecodeOptimizer};

// Runtime components
pub use runtime::{Builtins, Context, Function, Object};

// Memory management
pub use memory::{Heap, Stack, MemoryManager};

// Executor components
pub use executor::{
    InstructionExecutorImpl,
    instruction_handlers::{ObjectHandler, ArithmeticHandler, ComparisonHandler, ControlFlowHandler},
};

// Types and utilities
pub use types::{MemorySize, HeapHandleId};
pub use error::VmError;
pub use value::Value;
