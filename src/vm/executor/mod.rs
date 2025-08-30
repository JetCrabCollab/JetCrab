pub mod core;
pub mod error_handler;
pub mod heap_manager;
pub mod instruction_executor;
pub mod performance_monitor;
pub mod stack_manager;
pub mod variable_manager;

use crate::vm::bytecode::Bytecode;
use crate::vm::executor::error_handler::ExecutionError;
use crate::vm::value::Value;

pub trait InstructionExecutor {
    fn execute(&mut self, bytecode: &Bytecode, constants: &[Value]) -> Result<(), ExecutionError>;
}

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
    fn alloc_object(&mut self) -> crate::vm::handle::HeapHandleId;
    fn alloc_array(&mut self) -> crate::vm::handle::HeapHandleId;
    fn alloc_function(
        &mut self,
        bytecode: Bytecode,
        arg_count: crate::vm::types::ArgIndex,
        local_count: crate::vm::types::LocalIndex,
    ) -> crate::vm::handle::HeapHandleId;
    fn get_object_property(
        &self,
        handle: crate::vm::handle::HeapHandleId,
        key: &str,
    ) -> Option<&Value>;
    fn set_object_property(
        &mut self,
        handle: crate::vm::handle::HeapHandleId,
        key: String,
        value: Value,
    );
    fn set_array_element(
        &mut self,
        handle: crate::vm::handle::HeapHandleId,
        index: crate::vm::types::ArraySize,
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

pub use core::Executor;
