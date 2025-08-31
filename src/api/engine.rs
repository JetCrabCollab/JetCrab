//! # JavaScript Engine API
//!
//! Provides the main execution engine for JavaScript code, combining parsing,
//! semantic analysis, bytecode generation, and VM execution.
//!
//! ## Overview
//!
//! The Engine is the primary entry point for JavaScript execution:
//!
//! - **Source Code Input**: Accepts JavaScript source as strings
//! - **Complete Pipeline**: Parsing → Analysis → Compilation → Execution
//! - **Result Output**: Returns computed values or error messages
//! - **Context Management**: Maintains execution state and context
//!
//! ## Execution Flow
//!
//! ```text
//! Source Code → Parser → AST → Semantic Analysis → Bytecode → VM → Result
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::api::Engine;
//!
//! let mut engine = Engine::new();
//! let result = engine.evaluate("2 + 2 * 3")?;
//! println!("Result: {:?}", result);
//! ```

use crate::vm::compiler::generator::BytecodeGenerator;
use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;
use crate::vm::runtime::Context;
use crate::vm::executor::Executor;
use crate::vm::{Bytecode, Value};

/// Main JavaScript execution engine
///
/// Combines all components needed for JavaScript execution:
/// parsing, semantic analysis, bytecode generation, and VM execution.
pub struct Engine {
    context: Context,
    executor: Executor,
    generator: BytecodeGenerator,
    analyzer: SemanticAnalyzer,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            context: Context::new(),
            executor: Executor::new(),
            generator: BytecodeGenerator::new(),
            analyzer: SemanticAnalyzer::new(),
        }
    }

    pub fn evaluate(&mut self, source: &str) -> Result<Value, String> {
        let mut parser = Parser::new(source);
        let ast = parser.parse().map_err(|e| format!("Parser error: {e}"))?;

        self.analyzer
            .analyze(&ast)
            .map_err(|e| format!("Semantic error: {e}"))?;

        let instructions = self.generator.generate(&ast);
        let constants = self.generator.get_constants().clone();

        let values: Vec<Value> = constants
            .iter()
            .map(|s| {
                // Check if it's a string that starts and ends with quotes (string literal)
                if s.starts_with('"') && s.ends_with('"') {
                    // Remove quotes and treat as string literal
                    let content = &s[1..s.len() - 1];
                    Value::String(content.to_string())
                } else if s.starts_with("'") && s.ends_with("'") {
                    // Remove quotes and treat as string literal
                    let content = &s[1..s.len() - 1];
                    Value::String(content.to_string())
                } else if let Ok(num) = s.parse::<f64>() {
                    // Check if it's actually a number (not a string that happens to be numeric)
                    if s.contains('.') || s.parse::<i64>().is_ok() {
                        Value::Number(num)
                    } else {
                        // If it's a string that looks like a number, keep it as string
                        Value::String(s.clone())
                    }
                } else if s == "true" {
                    Value::Boolean(true)
                } else if s == "false" {
                    Value::Boolean(false)
                } else if s == "null" {
                    Value::Null
                } else if s == "undefined" {
                    Value::Undefined
                } else {
                    Value::String(s.clone())
                }
            })
            .collect();

        let bytecode = Bytecode::new(instructions);
        self.executor
            .execute(&bytecode, &values)
            .map_err(|e| format!("Execution error: {}", e))?;

        Ok(self.executor.stack_mut().pop().unwrap_or(Value::Undefined))
    }

    pub fn get_context(&self) -> &Context {
        &self.context
    }

    pub fn get_context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}
