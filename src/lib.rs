pub mod api;
pub mod ast;
pub mod bytecode;
pub mod lexer;
pub mod memory;
pub mod parser;
pub mod runtime;
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
pub use api::debug::{Debugger, Inspector, Profiler, Breakpoint, DebugInfo, ProfilingMetrics};

// Event system and callbacks
pub use api::events::{EventEmitter, EventManager, CallbackRegistry, EventData, EventChain};

// Module system
pub use api::modules::{ModuleLoader, ModuleRegistry, ModuleProvider, ModuleInfo};

// Error handling
pub use api::error::ApiError;
