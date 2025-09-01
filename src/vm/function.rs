use crate::vm::instructions::Instruction;
use crate::vm::types::LocalIndex;
use std::collections::HashMap;

/// Represents a function in the VM
#[derive(Debug, Clone)]
pub struct Function {
    /// Function name
    pub name: String,
    /// Function parameters
    pub params: Vec<String>,
    /// Function bytecode
    pub bytecode: Vec<Instruction>,
    /// Local variables mapping
    pub locals: HashMap<String, LocalIndex>,
    /// Number of local variables
    pub local_count: usize,
}

impl Function {
    /// Creates a new function
    pub fn new(name: String, params: Vec<String>) -> Self {
        Self {
            name,
            params,
            bytecode: Vec::new(),
            locals: HashMap::new(),
            local_count: 0,
        }
    }

    /// Adds a local variable to the function
    pub fn add_local(&mut self, name: String) -> LocalIndex {
        if let Some(&index) = self.locals.get(&name) {
            index
        } else {
            let index = LocalIndex::new(self.local_count);
            self.locals.insert(name, index);
            self.local_count += 1;
            index
        }
    }

    /// Gets the local variable index
    pub fn get_local(&self, name: &str) -> Option<LocalIndex> {
        self.locals.get(name).copied()
    }

    /// Adds an instruction to the function
    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.bytecode.push(instruction);
    }

    /// Gets the number of parameters
    pub fn param_count(&self) -> usize {
        self.params.len()
    }
}
