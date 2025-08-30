use super::HeapOperations;
use crate::vm::bytecode::Bytecode;
use crate::vm::handle::HeapHandleId;
use crate::vm::heap::Heap;
use crate::vm::types::ArraySize;
use crate::vm::value::Value;

pub struct HeapManager {
    heap: Heap,
}

impl HeapManager {
    pub fn new() -> Self {
        Self { heap: Heap::new() }
    }

    pub fn heap(&self) -> &Heap {
        &self.heap
    }

    pub fn heap_mut(&mut self) -> &mut Heap {
        &mut self.heap
    }
}

impl HeapOperations for HeapManager {
    fn alloc_object(&mut self) -> HeapHandleId {
        self.heap.alloc_object()
    }

    fn alloc_array(&mut self) -> HeapHandleId {
        self.heap.alloc_array()
    }

    fn alloc_function(
        &mut self,
        bytecode: Bytecode,
        arg_count: crate::vm::types::ArgIndex,
        local_count: crate::vm::types::LocalIndex,
    ) -> HeapHandleId {
        self.heap.alloc_function(bytecode, arg_count, local_count)
    }

    fn get_object_property(&self, handle: HeapHandleId, key: &str) -> Option<&Value> {
        self.heap.get_object_property(handle, key)
    }

    fn set_object_property(&mut self, handle: HeapHandleId, key: String, value: Value) {
        self.heap.set_object_property(handle, key, value);
    }

    fn set_array_element(&mut self, handle: HeapHandleId, index: ArraySize, value: Value) {
        self.heap.set_array_element(handle, index, value);
    }

    fn get_heap(&self) -> &crate::vm::heap::Heap {
        &self.heap
    }
}
