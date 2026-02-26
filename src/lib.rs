//! # JetCrab Runtime
//!
//! A modern JavaScript runtime in Rust, powered by Chitin (WASM) engine.
//!
//! ## Overview
//!
//! JetCrab provides a complete JavaScript runtime environment with:
//!
//! - **Modern JavaScript Engine**: Chitin (WASM)
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
//! CLI → Runtime Core → Chitin Engine (WASM) → JavaScript Execution
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
//! - **ECMAScript 2020+ Support**: Via Chitin (WASM) engine
//! - **High Performance**: Rust-native with Chitin backend
//! - **Memory Safe**: Built with Rust's memory safety guarantees
//! - **Built-in Tools**: Test, format, lint, bundle out of the box
//! - **Cross Platform**: Runs on Windows, macOS, and Linux

pub mod cli;
pub mod easter_egg;
pub mod runtime;


// pub use runtime::apis::BuiltinAPIs;
pub use runtime::JetCrabEngine;
// pub use runtime::module_loader::{ModuleInfo, ModuleLoader};
// pub use runtime::repl::Repl;
// pub use runtime::wasm_runtime::WasmRuntime;
pub use runtime::JetCrabRuntime;

pub use cli::Cli;


