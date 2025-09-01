use crate::vm::function::Function;
use std::collections::HashMap;

/// Manages functions in the VM
#[derive(Debug, Default)]
pub struct FunctionManager {
    /// Global functions
    functions: HashMap<String, Function>,
}

impl FunctionManager {
    /// Creates a new function manager
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    /// Registers a function
    pub fn register_function(&mut self, function: Function) {
        self.functions.insert(function.name.clone(), function);
    }

    /// Gets a function by name
    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.get(name)
    }

    /// Checks if a function exists
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Gets all function names
    pub fn get_function_names(&self) -> Vec<String> {
        self.functions.keys().cloned().collect()
    }

    /// Removes a function
    pub fn remove_function(&mut self, name: &str) -> Option<Function> {
        self.functions.remove(name)
    }

    /// Clears all functions
    pub fn clear(&mut self) {
        self.functions.clear();
    }
}
