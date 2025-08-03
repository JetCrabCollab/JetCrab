use crate::bytecode::BytecodeGenerator;
use crate::parser::Parser;
use crate::runtime::Context;
use crate::semantic::SemanticAnalyzer;
use crate::vm::{Bytecode, Executor, Value};

pub struct Engine {
    context: Context,
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
        }
    }

    pub fn evaluate(&mut self, source: &str) -> Result<Value, String> {
        let mut parser = Parser::new(source);
        let ast = parser.parse().map_err(|e| format!("Parser error: {e}"))?;

        let mut analyzer = SemanticAnalyzer::new();
        analyzer
            .analyze(&ast)
            .map_err(|e| format!("Semantic error: {e}"))?;

        let mut generator = BytecodeGenerator::new();
        let instructions = generator.generate(&ast);
        let constants = generator.get_constants().clone();

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
        let mut executor = Executor::new();
        executor.execute(&bytecode, &values);

        Ok(executor.stack.pop().unwrap_or(Value::Undefined))
    }

    pub fn get_context(&self) -> &Context {
        &self.context
    }

    pub fn get_context_mut(&mut self) -> &mut Context {
        &mut self.context
    }
}
