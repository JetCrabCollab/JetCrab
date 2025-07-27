use crate::runtime::object::Object;
use crate::vm::value::Value;
use std::collections::HashMap;

pub struct Context {
    pub global_object: Object,
    pub variables: HashMap<String, Value>,
    pub this_value: Value,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        let global_object = Object::new();

        Self {
            global_object,
            variables: HashMap::new(),
            this_value: Value::Undefined,
        }
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn has_variable(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    pub fn set_this(&mut self, value: Value) {
        self.this_value = value;
    }

    pub fn get_this(&self) -> &Value {
        &self.this_value
    }
}
