//! # JetCrab Runtime
//!
//! A modern JavaScript runtime in Rust, powered by Boa engine.
//!
//! ## Overview
//!
//! JetCrab provides a complete JavaScript runtime environment with:
//!
//! - **Modern JavaScript Engine**: Powered by Boa (85%+ ECMAScript compliance)
//! - **CLI Interface**: Command-line tools for development
//! - **Interactive REPL**: Read-Eval-Print Loop for testing
//! - **Built-in Tools**: Test runner, formatter, linter, bundler
//! - **Module System**: ES6 module support with bundling
//! - **Rust Integration**: Native Rust APIs and performance
//!
//! ## Architecture
//!
//! JetCrab Runtime architecture:
//!
//! ```text
//! CLI → Runtime Core → JetCrab Engine → Boa Backend → JavaScript Execution
//! ```
//!
//! ## Usage
//!
//! ### Command Line
//!
//! ```bash
//! # Run a JavaScript file
//! jetcrab run script.js
//!
//! # Start interactive REPL
//! jetcrab repl
//!
//! # Evaluate code directly
//! jetcrab eval "console.log('Hello World!')"
//!
//! # Run tests
//! jetcrab test
//!
//! # Format code
//! jetcrab fmt script.js
//! ```
//!
//! ### Library Usage
//!
//! ```rust
//! use jetcrab::runtime::JetCrabRuntime;
//!
//! let mut runtime = JetCrabRuntime::new();
//! runtime.evaluate_code("2 + 2 * 3").await?;
//! ```
//!
//! ## Features
//!
//! - **ECMAScript 2020+ Support**: Modern JavaScript features via Boa
//! - **High Performance**: Rust-native performance with Boa backend
//! - **Memory Safe**: Built with Rust's memory safety guarantees
//! - **Built-in Tools**: Test, format, lint, bundle out of the box
//! - **Cross Platform**: Runs on Windows, macOS, and Linux

pub mod cli;
pub mod easter_egg;
pub mod runtime;
pub mod tools;

pub use runtime::apis::BuiltinAPIs;
pub use runtime::engine::JetCrabEngine;
pub use runtime::module_loader::{ModuleInfo, ModuleLoader};
pub use runtime::repl::Repl;
pub use runtime::wasm_runtime::WasmRuntime;
pub use runtime::JetCrabRuntime;

pub use cli::Cli;

pub use tools::{Claw, DevConfig, DevTools};
