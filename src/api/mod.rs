//! # JetCrab API Module
//!
//! Provides the public API for the JetCrab JavaScript engine, including
//! high-level interfaces for code execution, compilation, debugging,
//! and module management.
//!
//! ## Overview
//!
//! The API module exposes the following main components:
//!
//! - **Engine**: Main execution engine for JavaScript code
//! - **Interpreter**: Bytecode interpreter with execution context
//! - **Compiler**: Source code to bytecode compilation
//! - **Debug**: Debugging and profiling tools
//! - **Events**: Event system for execution monitoring
//! - **Modules**: Module loading and management system
//! - **Config**: Engine configuration and settings
//! - **Error**: API error handling and types
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::api::{Engine, Interpreter, Compiler};
//!
//! // Create and use the main engine
//! let mut engine = Engine::new();
//! let result = engine.evaluate("2 + 2")?;
//!
//! // Use the interpreter directly
//! let interpreter = Interpreter::new(vec![], vec![]);
//! let result = interpreter.execute()?;
//! ```

pub mod compiler;
pub mod config;
pub mod debug;
pub mod engine;
pub mod error;
pub mod events;
pub mod interpreter;
pub mod modules;

pub use compiler::Compiler;
pub use config::{EngineConfig, MemoryConfig, ModuleSystem, OptimizationLevel, SecurityLevel};
pub use debug::{Breakpoint, CallFrame, DebugInfo, Debugger, Inspector, Profiler, ProfilingMetrics};
pub use engine::Engine;
pub use error::ApiError;
pub use events::{CallbackRegistry, EventData, EventEmitter, EventManager, EventChain};
pub use interpreter::Interpreter;
pub use modules::{ModuleInfo, ModuleLoader, ModuleProvider, ModuleRegistry, FileSystemModuleProvider};
