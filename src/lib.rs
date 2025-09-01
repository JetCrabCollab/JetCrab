//! # JetCrab JavaScript Engine
//!
//! A high-performance JavaScript engine written in Rust, featuring a complete
//! compilation pipeline from source code to bytecode execution.
//!
//! ## Overview
//!
//! JetCrab provides a complete JavaScript runtime environment with:
//!
//! - **Lexical Analysis**: Tokenization of JavaScript source code
//! - **Parsing**: AST generation and syntax validation
//! - **Semantic Analysis**: Type checking and validation
//! - **Bytecode Generation**: Compilation to VM instructions
//! - **Virtual Machine**: High-performance bytecode execution
//! - **Module System**: ES6 module support
//! - **Debugging Tools**: Profiling and inspection capabilities
//!
//! ## Architecture
//!
//! The engine follows a traditional compiler architecture:
//!
//! ```text
//! Source Code → Lexer → Parser → AST → Semantic Analysis → Bytecode → VM Execution
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::Engine;
//!
//! let mut engine = Engine::new();
//! let result = engine.evaluate("2 + 2 * 3").unwrap();
//! println!("Result: {:?}", result);
//! ```
//!
//! ## Features
//!
//! - **ECMAScript 2020+ Support**: Modern JavaScript features
//! - **High Performance**: Optimized bytecode execution
//! - **Memory Safe**: Built with Rust's memory safety guarantees
//! - **Extensible**: Plugin system for custom functionality
//! - **Cross Platform**: Runs on Windows, macOS, and Linux

pub mod api;
pub mod ast;
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod test_utils;
pub mod vm;

// Core API exports
pub use api::compiler::Compiler;
pub use api::engine::Engine;
pub use api::interpreter::Interpreter;

// Configuration and customization
pub use api::config::{EngineConfig, MemoryConfig, ModuleSystem, OptimizationLevel, SecurityLevel};

// Debugging and profiling
pub use api::debug::{Breakpoint, DebugInfo, Debugger, Inspector, Profiler, ProfilingMetrics};

// Event system and callbacks
pub use api::events::{CallbackRegistry, EventChain, EventData, EventEmitter, EventManager};

// Module system
pub use api::modules::{ModuleInfo, ModuleLoader, ModuleProvider, ModuleRegistry};

// Error handling
pub use api::error::ApiError;
