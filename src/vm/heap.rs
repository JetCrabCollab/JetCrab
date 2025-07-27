use crate::vm::bytecode::Bytecode;
use crate::vm::handle::HeapHandleId;
use crate::vm::types::{ArgIndex, ArraySize, LocalIndex};
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

pub struct Heap {
    entries: Vec<HeapEntry>,
    next_id: usize,
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}

impl Heap {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 0,
        }
    }

    pub fn alloc_object(&mut self) -> HeapHandleId {
        let id = HeapHandleId::new(self.next_id);
        self.entries.push(HeapEntry::Object(HashMap::new()));
        self.next_id += 1;
        id
    }

    pub fn alloc_array(&mut self) -> HeapHandleId {
        let id = HeapHandleId::new(self.next_id);
        self.entries.push(HeapEntry::Array(Vec::new()));
        self.next_id += 1;
        id
    }

    pub fn alloc_function(
        &mut self,
        bytecode: Bytecode,
        arg_count: ArgIndex,
        local_count: LocalIndex,
    ) -> HeapHandleId {
        let id = HeapHandleId::new(self.next_id);
        self.entries.push(HeapEntry::Function {
            bytecode,
            arg_count,
            local_count,
            closure_vars: HashMap::new(),
        });
        self.next_id += 1;
        id
    }

    pub fn get(&self, handle: HeapHandleId) -> Option<&HeapEntry> {
        self.entries.get(handle.as_usize())
    }

    pub fn get_mut(&mut self, handle: HeapHandleId) -> Option<&mut HeapEntry> {
        self.entries.get_mut(handle.as_usize())
    }

    pub fn set_object_property(&mut self, handle: HeapHandleId, key: String, value: Value) {
        if let Some(HeapEntry::Object(obj)) = self.entries.get_mut(handle.as_usize()) {
            obj.insert(key, value);
        }
    }

    pub fn get_object_property(&self, handle: HeapHandleId, key: &str) -> Option<&Value> {
        if let Some(HeapEntry::Object(obj)) = self.entries.get(handle.as_usize()) {
            obj.get(key)
        } else {
            None
        }
    }

    pub fn push_array_element(&mut self, handle: HeapHandleId, value: Value) {
        if let Some(HeapEntry::Array(arr)) = self.entries.get_mut(handle.as_usize()) {
            arr.push(value);
        }
    }

    pub fn get_array_element(&self, handle: HeapHandleId, idx: ArraySize) -> Option<&Value> {
        if let Some(HeapEntry::Array(arr)) = self.entries.get(handle.as_usize()) {
            arr.get(idx.as_usize())
        } else {
            None
        }
    }

    pub fn set_array_element(&mut self, handle: HeapHandleId, idx: ArraySize, value: Value) {
        if let Some(HeapEntry::Array(arr)) = self.entries.get_mut(handle.as_usize()) {
            let index = idx.as_usize();
            if index < arr.len() {
                arr[index] = value;
            } else {
                arr.resize(index + 1, Value::Undefined);
                arr[index] = value;
            }
        }
    }

    pub fn remove_object_property(&mut self, handle: HeapHandleId, key: &str) {
        if let Some(HeapEntry::Object(obj)) = self.entries.get_mut(handle.as_usize()) {
            obj.remove(key);
        }
    }

    pub fn has_object_property(&self, handle: HeapHandleId, key: &str) -> bool {
        if let Some(HeapEntry::Object(obj)) = self.entries.get(handle.as_usize()) {
            obj.contains_key(key)
        } else {
            false
        }
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.next_id = 0;
    }
}
