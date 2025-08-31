//! # Bytecode Generation Module
//!
//! Converts Abstract Syntax Trees (AST) into executable bytecode instructions
//! for the virtual machine, including optimization and scope management.
//!
//! ## Overview
//!
//! The bytecode module handles:
//!
//! - **Code Generation**: AST to bytecode conversion
//! - **Optimization**: Instruction-level optimizations
//! - **Scope Management**: Variable and function scope tracking
//! - **Statement Compilation**: Control flow and declarations
//! - **Expression Compilation**: Operations and function calls
//! - **Literal Handling**: Constants and value compilation
//!
//! ## Architecture
//!
//! The bytecode generator follows a visitor pattern:
//! - Traverses AST nodes
//! - Generates appropriate instructions
//! - Manages constant pool
//! - Tracks scope information
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::bytecode::BytecodeGenerator;
//! use jetcrab::ast::Node;
//!
//! let mut generator = BytecodeGenerator::new();
//! let bytecode = generator.generate(&ast);
//! let constants = generator.get_constants();
//! ```

pub mod bytecode;
pub mod error;
pub mod expressions;
pub mod generator;
pub mod literals;
pub mod optimizer;
pub mod scope;
pub mod statements;

pub use bytecode::Bytecode;
pub use error::BytecodeError;
pub use generator::BytecodeGenerator;
pub use optimizer::BytecodeOptimizer;
