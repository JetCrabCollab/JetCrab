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
    fn get_object_property(
        &self,
        handle: HeapHandleId,
        key: &str,
    ) -> Option<&Value>;
    fn set_object_property(
        &mut self,
        handle: HeapHandleId,
        key: String,
        value: Value,
    );
    fn set_array_element(
        &mut self,
        handle: HeapHandleId,
        index: ArraySize,
        value: Value,
    );
    fn get_heap(&self) -> &crate::vm::heap::Heap;
}

pub trait VariableManager {
    fn get_local(&self, idx: usize) -> Option<&Value>;
    fn set_local(&mut self, idx: usize, value: Value);
    fn get_global(&self, idx: usize) -> Option<&Value>;
    fn set_global(&mut self, idx: usize, value: Value);
    fn get_local_mut(&mut self, idx: usize) -> Option<&mut Value>;
    fn get_global_mut(&mut self, idx: usize) -> Option<&mut Value>;
}

pub trait InstructionExecutor {
    fn execute(&mut self, bytecode: &Bytecode, constants: &[Value]) -> Result<(), crate::vm::executor::error_handler::ExecutionError>;
}
