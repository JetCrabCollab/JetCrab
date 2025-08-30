use crate::vm::bytecode::Bytecode;
use crate::vm::handle::HeapHandleId;
use crate::vm::types::{ArgIndex, ArraySize, LocalIndex};
use crate::vm::value::Value;

pub trait StackOperations {
    fn push(&mut self, value: Value);
    fn pop(&mut self) -> Option<Value>;
    fn peek(&self) -> Option<&Value>;
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn stack_mut(&mut self) -> &mut crate::vm::stack::Stack;

    // Default implementations for convenience
    fn size(&self) -> usize {
        self.len()
    }
}

pub trait HeapOperations {
    fn alloc_object(&mut self) -> HeapHandleId;
    fn alloc_array(&mut self) -> HeapHandleId;
    fn alloc_function(
        &mut self,
        bytecode: Bytecode,
        arg_count: ArgIndex,
        local_count: LocalIndex,
    ) -> HeapHandleId;
    fn get_object_property(&self, handle: HeapHandleId, key: &str) -> Option<&Value>;
    fn set_object_property(&mut self, handle: HeapHandleId, key: String, value: Value);
    fn set_array_element(&mut self, handle: HeapHandleId, index: ArraySize, value: Value);
    fn get_heap(&self) -> &crate::vm::heap::Heap;

    // Additional methods needed by instruction handlers
    fn get_array_element(&self, _handle: HeapHandleId, _index: ArraySize) -> Option<&Value> {
        // Default implementation - can be overridden
        None
    }

    fn has_object_property(&self, handle: HeapHandleId, key: &str) -> bool {
        self.get_object_property(handle, key).is_some()
    }

    fn get_array_length(&self, _handle: HeapHandleId) -> usize {
        // Default implementation - can be overridden
        0
    }
}

pub trait VariableManager {
    fn get_local(&self, idx: usize) -> Option<&Value>;
    fn set_local(&mut self, idx: usize, value: Value);
    fn get_global(&self, idx: usize) -> Option<&Value>;
    fn set_global(&mut self, idx: usize, value: Value);
    fn get_local_mut(&mut self, idx: usize) -> Option<&mut Value>;
    fn get_global_mut(&mut self, idx: usize) -> Option<&mut Value>;

    // Additional methods for arguments
    fn get_argument(&self, idx: usize) -> Option<&Value> {
        self.get_local(idx)
    }

    fn set_argument(&mut self, idx: usize, value: Value) {
        self.set_local(idx, value);
    }
}

pub trait InstructionExecutor {
    fn execute(
        &mut self,
        bytecode: &Bytecode,
        constants: &[Value],
    ) -> Result<(), crate::vm::executor::error_handler::ExecutionError>;
}
