use crate::vm::value::Value;
use super::VariableManager;

pub struct VariableManagerImpl {
    locals: Vec<Value>,
    globals: Vec<Value>,
}

impl VariableManagerImpl {
    pub fn new() -> Self {
        Self {
            locals: vec![Value::Undefined; 32],
            globals: vec![Value::Undefined; 32],
        }
    }

    pub fn locals(&self) -> &[Value] {
        &self.locals
    }

    pub fn locals_mut(&mut self) -> &mut [Value] {
        &mut self.locals
    }

    pub fn globals(&self) -> &[Value] {
        &self.globals
    }

    pub fn globals_mut(&mut self) -> &mut [Value] {
        &mut self.globals
    }
}

impl VariableManager for VariableManagerImpl {
    fn get_local(&self, idx: usize) -> Option<&Value> {
        self.locals.get(idx)
    }

    fn set_local(&mut self, idx: usize, value: Value) {
        if let Some(slot) = self.locals.get_mut(idx) {
            *slot = value;
        }
    }

    fn get_global(&self, idx: usize) -> Option<&Value> {
        self.globals.get(idx)
    }

    fn set_global(&mut self, idx: usize, value: Value) {
        if let Some(slot) = self.globals.get_mut(idx) {
            *slot = value;
        }
    }

    fn get_local_mut(&mut self, idx: usize) -> Option<&mut Value> {
        self.locals.get_mut(idx)
    }

    fn get_global_mut(&mut self, idx: usize) -> Option<&mut Value> {
        self.globals.get_mut(idx)
    }
}
