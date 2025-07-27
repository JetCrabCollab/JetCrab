use crate::vm::instructions::Instruction;
use crate::vm::{Bytecode, Executor, Value};

pub struct Interpreter {
    instructions: Vec<Instruction>,
    constants: Vec<String>,
}

impl Interpreter {
    pub fn new(instructions: Vec<Instruction>, constants: Vec<String>) -> Self {
        Self {
            instructions,
            constants,
        }
    }

    pub fn execute(&self) -> Result<Value, String> {
        let mut executor = Executor::new();

        let values: Vec<Value> = self
            .constants
            .iter()
            .map(|s| {
                if let Ok(num) = s.parse::<f64>() {
                    Value::Number(num)
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

        let bytecode = Bytecode::new(self.instructions.clone());
        executor.execute(&bytecode, &values);

        Ok(executor.stack.pop().unwrap_or(Value::Undefined))
    }

    pub fn execute_with_context(
        &self,
        _context: &mut crate::runtime::Context,
    ) -> Result<Value, String> {
        let mut executor = Executor::new();

        let values: Vec<Value> = self
            .constants
            .iter()
            .map(|s| {
                if let Ok(num) = s.parse::<f64>() {
                    Value::Number(num)
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

        let bytecode = Bytecode::new(self.instructions.clone());
        executor.execute(&bytecode, &values);

        Ok(executor.stack.pop().unwrap_or(Value::Undefined))
    }

    pub fn get_instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn get_constants(&self) -> &[String] {
        &self.constants
    }
}
