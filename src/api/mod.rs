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
