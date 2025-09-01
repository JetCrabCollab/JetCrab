//! JavaScript Runtime - Core runtime environment for JavaScript execution
//!
//! This module provides:
//! - Built-in functions and objects
//! - Execution context management
//! - Function and object systems
//! - Runtime error handling

pub mod builtins;
pub mod context;
pub mod errors;
pub mod function;
pub mod object;

pub use builtins::Builtins;
pub use context::Context;
pub use errors::RuntimeError;
pub use function::Function;
pub use object::Object;
