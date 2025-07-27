use crate::runtime::context::Context;
use crate::vm::value::Value;
use std::collections::HashMap;

pub type NativeFunction = fn(&mut Context, &[Value]) -> Result<Value, String>;

pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Option<Vec<u8>>,
    pub native_function: Option<NativeFunction>,
    pub closure: HashMap<String, Value>,
}

impl Function {
    pub fn new(name: String, parameters: Vec<String>) -> Self {
        Self {
            name,
            parameters,
            body: None,
            native_function: None,
            closure: HashMap::new(),
        }
    }

    pub fn native(name: String, parameters: Vec<String>, func: NativeFunction) -> Self {
        Self {
            name,
            parameters,
            body: None,
            native_function: Some(func),
            closure: HashMap::new(),
        }
    }

    pub fn call(&self, context: &mut Context, arguments: &[Value]) -> Result<Value, String> {
        if let Some(native_func) = self.native_function {
            native_func(context, arguments)
        } else {
            Err("Function not implemented".to_string())
        }
    }

    pub fn set_closure_variable(&mut self, name: String, value: Value) {
        self.closure.insert(name, value);
    }

    pub fn get_closure_variable(&self, name: &str) -> Option<&Value> {
        self.closure.get(name)
    }
}
