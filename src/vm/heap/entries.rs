use crate::vm::bytecode::Bytecode;
use crate::vm::types::{ArgIndex, LocalIndex};
use crate::vm::value::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum HeapEntry {
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    Function {
        bytecode: Bytecode,
        arg_count: ArgIndex,
        local_count: LocalIndex,
        closure_vars: HashMap<String, Value>,
    },
    String(String),
}

impl HeapEntry {
    pub fn size(&self) -> usize {
        match self {
            HeapEntry::Object(obj) => obj.len(),
            HeapEntry::Array(arr) => arr.len(),
            HeapEntry::Function { .. } => 1,
            HeapEntry::String(s) => s.len(),
        }
    }

    pub fn memory_usage(&self) -> usize {
        match self {
            HeapEntry::Object(obj) => {
                let mut size = 0;
                for (key, value) in obj {
                    size += key.len() + std::mem::size_of_val(value);
                }
                size
            }
            HeapEntry::Array(arr) => {
                let mut size = 0;
                for value in arr {
                    size += std::mem::size_of_val(value);
                }
                size
            }
            HeapEntry::Function { bytecode, closure_vars, .. } => {
                let mut size = bytecode.instructions.len() * std::mem::size_of::<crate::vm::instructions::Instruction>();
                for (key, value) in closure_vars {
                    size += key.len() + std::mem::size_of_val(value);
                }
                size
            }
            HeapEntry::String(s) => s.len(),
        }
    }

    pub fn is_object(&self) -> bool {
        matches!(self, HeapEntry::Object(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, HeapEntry::Array(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(self, HeapEntry::Function { .. })
    }

    pub fn is_string(&self) -> bool {
        matches!(self, HeapEntry::String(_))
    }

    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        match self {
            HeapEntry::Object(obj) => Some(obj),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<String, Value>> {
        match self {
            HeapEntry::Object(obj) => Some(obj),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            HeapEntry::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            HeapEntry::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_function(&self) -> Option<(&Bytecode, &ArgIndex, &LocalIndex, &HashMap<String, Value>)> {
        match self {
            HeapEntry::Function { bytecode, arg_count, local_count, closure_vars } => {
                Some((bytecode, arg_count, local_count, closure_vars))
            }
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            HeapEntry::String(s) => Some(s),
            _ => None,
        }
    }
}

impl Default for HeapEntry {
    fn default() -> Self {
        HeapEntry::Object(HashMap::new())
    }
}
