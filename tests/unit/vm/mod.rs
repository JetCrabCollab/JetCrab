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

// Core VM components
pub mod core;

// Compiler components
pub mod compiler;

// Runtime components
pub mod runtime;

// Memory management
pub mod memory;

// Executor components
pub mod executor;

// Types and utilities
pub mod types;

// Individual VM files
pub mod error;
pub mod value;
pub mod handle;
pub mod frame;
pub mod registers;
pub mod instructions;
