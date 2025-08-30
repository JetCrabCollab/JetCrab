use super::{HeapOperations, StackOperations, VariableManager};
use crate::vm::handle::ArrayHandle;
use crate::vm::heap::HeapEntry;
use crate::vm::types::ArraySize;
use crate::vm::value::Value;

pub trait ControlFlowOperations<S, H, V>
where
    S: StackOperations,
    H: HeapOperations,
    V: VariableManager,
{
    fn execute_new_array(&mut self, stack: &mut S, heap: &mut H, size: usize)
        -> Result<(), String>;
    fn execute_get_property(&mut self, stack: &mut S, heap: &H) -> Result<(), String>;
    fn execute_set_property(&mut self, stack: &mut S, heap: &mut H) -> Result<(), String>;
}

pub trait VariableOperations<S, V>
where
    S: StackOperations,
    V: VariableManager,
{
    fn execute_load_local(&mut self, stack: &mut S, vars: &V, idx: usize) -> Result<(), String>;
    fn execute_store_local(
        &mut self,
        stack: &mut S,
        vars: &mut V,
        idx: usize,
    ) -> Result<(), String>;
    fn execute_load_global(&mut self, stack: &mut S, vars: &V, idx: usize) -> Result<(), String>;
    fn execute_store_global(
        &mut self,
        stack: &mut S,
        vars: &mut V,
        idx: usize,
    ) -> Result<(), String>;
}

pub trait JumpOperations {
    fn execute_jump(&mut self, ip: &mut usize, target: usize) -> Result<(), String>;
    fn execute_jump_if_false(
        &mut self,
        stack: &mut impl StackOperations,
        ip: &mut usize,
        target: usize,
    ) -> Result<(), String>;
    fn execute_jump_if_true(
        &mut self,
        stack: &mut impl StackOperations,
        ip: &mut usize,
        target: usize,
    ) -> Result<(), String>;
}

pub struct ControlFlowExecutor;

impl ControlFlowExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl<S, H, V> ControlFlowOperations<S, H, V> for ControlFlowExecutor
where
    S: StackOperations,
    H: HeapOperations,
    V: VariableManager,
{
    fn execute_new_array(
        &mut self,
        stack: &mut S,
        heap: &mut H,
        size: usize,
    ) -> Result<(), String> {
        let handle = heap.alloc_array();
        let size_usize = size;

        let mut elements = Vec::with_capacity(size_usize);
        for _ in 0..size_usize {
            if let Some(element) = stack.pop() {
                elements.push(element);
            }
        }
        elements.reverse();

        for (index, element) in elements.into_iter().enumerate() {
            heap.set_array_element(handle, ArraySize::new(index), element);
        }

        stack.push(Value::Array(ArrayHandle::from(handle.as_usize())));
        Ok(())
    }

    fn execute_get_property(&mut self, stack: &mut S, heap: &H) -> Result<(), String> {
        let key = stack.pop().ok_or("Stack underflow")?;
        let obj = stack.pop().ok_or("Stack underflow")?;

        let result = match (&obj, &key) {
            (Value::String(str_val), Value::String(key_str)) => {
                if key_str == "length" {
                    Value::Number(str_val.len() as f64)
                } else {
                    Value::Undefined
                }
            }
            (Value::Array(handle), Value::String(key_str)) => {
                if key_str == "length" {
                    if let Some(HeapEntry::Array(arr)) = heap.get_heap().get(handle.id()) {
                        Value::Number(arr.len() as f64)
                    } else {
                        Value::Undefined
                    }
                } else if key_str == "push" || key_str == "pop" {
                    Value::String(format!("Array.prototype.{}", key_str))
                } else if let Ok(index) = key_str.parse::<usize>() {
                    if let Some(HeapEntry::Array(arr)) = heap.get_heap().get(handle.id()) {
                        arr.get(index).cloned().unwrap_or(Value::Undefined)
                    } else {
                        Value::Undefined
                    }
                } else {
                    Value::Undefined
                }
            }
            (Value::Array(handle), Value::Number(num)) => {
                let index = *num as usize;
                if let Some(HeapEntry::Array(arr)) = heap.get_heap().get(handle.id()) {
                    arr.get(index).cloned().unwrap_or(Value::Undefined)
                } else {
                    Value::Undefined
                }
            }
            (Value::Object(handle), Value::String(key_str)) => heap
                .get_object_property(handle.id(), key_str)
                .cloned()
                .unwrap_or(Value::Undefined),
            _ => Value::Undefined,
        };

        stack.push(result);
        Ok(())
    }

    fn execute_set_property(&mut self, stack: &mut S, heap: &mut H) -> Result<(), String> {
        let value = stack.pop().ok_or("Stack underflow")?;
        let key = stack.pop().ok_or("Stack underflow")?;
        let obj = stack.pop().ok_or("Stack underflow")?;

        match (obj, key) {
            (Value::Object(handle), Value::String(key_str)) => {
                heap.set_object_property(handle.id(), key_str, value);
            }
            _ => {
                return Err("Cannot set property on non-object value".to_string());
            }
        }

        Ok(())
    }
}

impl<S, V> VariableOperations<S, V> for ControlFlowExecutor
where
    S: StackOperations,
    V: VariableManager,
{
    fn execute_load_local(&mut self, stack: &mut S, vars: &V, idx: usize) -> Result<(), String> {
        let value = vars.get_local(idx).cloned().unwrap_or(Value::Undefined);
        stack.push(value);
        Ok(())
    }

    fn execute_store_local(
        &mut self,
        stack: &mut S,
        vars: &mut V,
        idx: usize,
    ) -> Result<(), String> {
        let value = stack.pop().ok_or("Stack underflow")?;
        vars.set_local(idx, value);
        Ok(())
    }

    fn execute_load_global(&mut self, stack: &mut S, vars: &V, idx: usize) -> Result<(), String> {
        let value = vars.get_global(idx).cloned().unwrap_or(Value::Undefined);
        stack.push(value);
        Ok(())
    }

    fn execute_store_global(
        &mut self,
        stack: &mut S,
        vars: &mut V,
        idx: usize,
    ) -> Result<(), String> {
        let value = stack.pop().ok_or("Stack underflow")?;
        vars.set_global(idx, value);
        Ok(())
    }
}

impl JumpOperations for ControlFlowExecutor {
    fn execute_jump(&mut self, ip: &mut usize, target: usize) -> Result<(), String> {
        *ip = target;
        Ok(())
    }

    fn execute_jump_if_false(
        &mut self,
        stack: &mut impl StackOperations,
        ip: &mut usize,
        target: usize,
    ) -> Result<(), String> {
        let condition = stack.pop().ok_or("Stack underflow")?;
        if !condition.is_truthy() {
            *ip = target;
        }
        Ok(())
    }

    fn execute_jump_if_true(
        &mut self,
        stack: &mut impl StackOperations,
        ip: &mut usize,
        target: usize,
    ) -> Result<(), String> {
        let condition = stack.pop().ok_or("Stack underflow")?;
        if condition.is_truthy() {
            *ip = target;
        }
        Ok(())
    }
}
