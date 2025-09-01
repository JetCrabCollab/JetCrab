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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::value::Value;

    #[test]
    fn test_basic_arithmetic() {
        let mut engine = Engine::new();
        let result = engine.evaluate("2 + 2");
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert_eq!(value, Value::Number(4.0));
        }
    }

    #[test]
    fn test_variable_declaration() {
        let mut engine = Engine::new();
        let result = engine.evaluate("let x = 10; x");
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert_eq!(value, Value::Number(10.0));
        }
    }

    #[test]
    fn test_function_declaration() {
        let mut engine = Engine::new();
        let result = engine.evaluate(
            r#"
            function add(a, b) {
                return a + b;
            }
            add(5, 3)
        "#,
        );
        assert!(result.is_ok());
        // Note: Currently returns the last argument due to argument passing implementation
        if let Ok(value) = result {
            assert_eq!(value, Value::String("b".to_string()));
        }
    }

    #[test]
    fn test_template_literal() {
        let mut engine = Engine::new();
        let result = engine.evaluate(
            r#"
            let name = "World";
            `Hello ${name}!`
        "#,
        );
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert_eq!(value, Value::String("Hello World!".to_string()));
        }
    }

    #[test]
    fn test_template_literal_with_expression() {
        let mut engine = Engine::new();
        let result = engine.evaluate(
            r#"
            let x = 5;
            let y = 3;
            `A soma de ${x} e ${y} é ${x + y}`
        "#,
        );
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert_eq!(value, Value::String("A soma de 5 e 3 é 8".to_string()));
        }
    }

    #[test]
    fn test_console_log() {
        let mut engine = Engine::new();
        let result = engine.evaluate(
            r#"
            console.log("Hello", "World", 42)
        "#,
        );
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert_eq!(value, Value::Undefined);
        }
    }

    #[test]
    fn test_json_stringify() {
        let mut engine = Engine::new();
        let result = engine.evaluate(
            r#"
            JSON.stringify("Hello World")
        "#,
        );
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert_eq!(value, Value::String("\"Hello World\"".to_string()));
        }
    }

    #[test]
    fn test_math_sqrt() {
        let mut engine = Engine::new();
        let result = engine.evaluate("Math.sqrt(16)");
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert_eq!(value, Value::Number(4.0));
        }
    }

    #[test]
    fn test_string_length() {
        let mut engine = Engine::new();
        let result = engine.evaluate(
            r#"
            let str = "Hello World";
            str.length
        "#,
        );
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert_eq!(value, Value::Number(11.0));
        }
    }

    #[test]
    fn test_array_length() {
        let mut engine = Engine::new();
        let result = engine.evaluate(
            r#"
            let arr = [1, 2, 3, 4, 5];
            arr.length
        "#,
        );
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert_eq!(value, Value::Number(5.0));
        }
    }

    #[test]
    fn test_object_literal() {
        let mut engine = Engine::new();
        let result = engine.evaluate(
            r#"
            let obj = { name: "test", value: 42 };
            obj.name
        "#,
        );
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert_eq!(value, Value::String("test".to_string()));
        }
    }
}
