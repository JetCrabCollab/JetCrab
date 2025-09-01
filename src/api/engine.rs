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
//! let result = engine.evaluate("2 + 2 * 3").unwrap();
//! println!("Result: {:?}", result);
//! ```

use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;
use crate::vm::compiler::generator::BytecodeGenerator;
use crate::vm::executor::Executor;
use crate::vm::runtime::Context;
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

        self.analyzer.analyze(&ast).map_err(|errors| {
            let error_messages: Vec<String> = errors.iter().map(|e| e.message.clone()).collect();
            format!("Semantic errors: {}", error_messages.join("; "))
        })?;

        let instructions = self.generator.generate(&ast);
        let constants = self.generator.get_constants().clone();

        // Copy functions from generator to executor
        let function_manager = self.generator.get_function_manager();
        for function_name in function_manager.get_function_names() {
            if let Some(function) = function_manager.get_function(&function_name) {
                self.executor
                    .get_function_manager_mut()
                    .register_function(function.clone());
            }
        }

        let values: Vec<Value> = constants
            .iter()
            .map(|s| {
                if (s.starts_with('"') && s.ends_with('"'))
                    || (s.starts_with("'") && s.ends_with("'"))
                {
                    let content = &s[1..s.len() - 1];
                    Value::String(content.to_string())
                } else if let Ok(num) = s.parse::<f64>() {
                    if s.contains('.') || s.parse::<i64>().is_ok() {
                        Value::Number(num)
                    } else {
                        Value::String(s.clone())
                    }
                } else if s == "true" {
                    Value::Boolean(true)
                } else if s == "false" {
                    Value::Boolean(false)
                } else if s == "null" {
                    Value::Null
                } else {
                    Value::String(s.clone())
                }
            })
            .collect();

        let bytecode = Bytecode::new(instructions);

        self.executor
            .execute(&bytecode, &values)
            .map_err(|e| format!("Execution error: {e:?}"))?;

        let result = self
            .executor
            .stack()
            .peek()
            .cloned()
            .unwrap_or(Value::Undefined);
        Ok(result)
    }

    pub fn get_context(&self) -> &Context {
        &self.context
    }

    pub fn get_context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}
