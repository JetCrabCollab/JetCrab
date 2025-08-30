//! # Runtime Environment Module
//!
//! Provides the runtime environment for JavaScript execution, including
//! built-in functions, context management, and object system.
//!
//! ## Overview
//!
//! The runtime module includes:
//!
//! - **Built-in Functions**: Standard library implementations
//! - **Context Management**: Execution context and state
//! - **Object System**: Object creation and manipulation
//! - **Function System**: Function execution and management
//! - **Error Handling**: Runtime error types and handling
//!
//! ## Components
//!
//! - **Context**: Execution environment and state
//! - **Builtins**: Standard JavaScript functions
//! - **Objects**: Dynamic object creation and access
//! - **Functions**: Callable function objects
//! - **Errors**: Runtime error types
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::runtime::{Context, Builtins, Object};
//!
//! let context = Context::new();
//! let builtins = Builtins::new();
//! let object = Object::new();
//! ```

pub mod builtins;
pub mod context;
pub mod error;
pub mod function;
pub mod object;

pub use context::Context;
pub use error::RuntimeError;
pub use function::Function;
pub use object::Object;
