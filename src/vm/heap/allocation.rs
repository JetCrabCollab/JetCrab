use super::entries::HeapEntry;
use super::types::HeapMetrics;
use crate::vm::bytecode::Bytecode;
use crate::vm::handle::HeapHandleId;
use crate::vm::types::{ArgIndex, LocalIndex};
use std::collections::HashMap;

pub struct HeapAllocatorImpl {
    entries: HashMap<HeapHandleId, HeapEntry>,
    next_id: usize,
    metrics: HeapMetrics,
}

impl Clone for HeapAllocatorImpl {
    fn clone(&self) -> Self {
        Self {
            entries: self.entries.clone(),
            next_id: self.next_id,
            metrics: self.metrics.clone(),
        }
    }
}

impl HeapAllocatorImpl {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            next_id: 0,
            metrics: HeapMetrics::new(),
        }
    }

    pub fn with_capacity(_capacity: usize) -> Self {
        Self {
            entries: HashMap::new(),
            next_id: 0,
            metrics: HeapMetrics::new(),
        }
    }

    pub fn get_next_id(&self) -> usize {
        self.next_id
    }

    pub fn set_next_id(&mut self, id: usize) {
        self.next_id = id;
    }

    // Métodos auxiliares para compatibilidade
    pub fn alloc_object(&mut self) -> HeapHandleId {
        let id = HeapHandleId::new(self.next_id);
        self.entries.insert(id, HeapEntry::Object(HashMap::new()));
        self.next_id += 1;
        self.metrics.record_allocation();
        id
    }

    pub fn alloc_array(&mut self) -> HeapHandleId {
        let id = HeapHandleId::new(self.next_id);
        self.entries.insert(id, HeapEntry::Array(Vec::new()));
        self.next_id += 1;
        self.metrics.record_allocation();
        id
    }

    pub fn alloc_function(
        &mut self,
        bytecode: Bytecode,
        arg_count: ArgIndex,
        local_count: LocalIndex,
    ) -> HeapHandleId {
        let id = HeapHandleId::new(self.next_id);
        self.entries.insert(
            id,
            HeapEntry::Function {
                bytecode,
                arg_count,
                local_count,
                closure_vars: HashMap::new(),
            },
        );
        self.next_id += 1;
        self.metrics.record_allocation();
        id
    }

    pub fn alloc_string(&mut self, value: String) -> HeapHandleId {
        let id = HeapHandleId::new(self.next_id);
        self.entries.insert(id, HeapEntry::String(value));
        self.next_id += 1;
        self.metrics.record_allocation();
        id
    }
}

impl super::management::HeapAllocator for HeapAllocatorImpl {
    fn allocate(&mut self, entry: HeapEntry) -> HeapHandleId {
        let id = HeapHandleId::new(self.next_id);
        self.entries.insert(id, entry);
        self.next_id += 1;
        self.metrics.record_allocation();
        id
    }

    fn deallocate(&mut self, handle: HeapHandleId) -> bool {
        if self.entries.remove(&handle).is_some() {
            self.metrics.record_deallocation();
            true
        } else {
            false
        }
    }

    fn get(&self, handle: HeapHandleId) -> Option<&HeapEntry> {
        self.entries.get(&handle)
    }

    fn get_mut(&mut self, handle: HeapHandleId) -> Option<&mut HeapEntry> {
        self.entries.get_mut(&handle)
    }

    fn get_entries(&self) -> &HashMap<HeapHandleId, HeapEntry> {
        &self.entries
    }

    fn get_entries_mut(&mut self) -> &mut HashMap<HeapHandleId, HeapEntry> {
        &mut self.entries
    }

    fn size(&self) -> usize {
        self.entries.len()
    }

    fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn clear(&mut self) {
        self.entries.clear();
    }

    fn get_metrics(&self) -> &HeapMetrics {
        &self.metrics
    }
}

impl Default for HeapAllocatorImpl {
    fn default() -> Self {
        Self::new()
    }
}
