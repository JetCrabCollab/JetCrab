use super::entries::HeapEntry;
use super::types::{HeapMetrics, HeapStats};
use crate::vm::bytecode::Bytecode;
use crate::vm::handle::HeapHandleId;
use crate::vm::types::{ArgIndex, ArraySize, LocalIndex};
use crate::vm::value::Value;
use std::collections::HashMap;

pub trait HeapAllocator {
    fn allocate(&mut self, entry: HeapEntry) -> HeapHandleId;
    fn deallocate(&mut self, handle: HeapHandleId) -> bool;
    fn get(&self, handle: HeapHandleId) -> Option<&HeapEntry>;
    fn get_mut(&mut self, handle: HeapHandleId) -> Option<&mut HeapEntry>;
    fn get_entries(&self) -> &HashMap<HeapHandleId, HeapEntry>;
    fn get_entries_mut(&mut self) -> &mut HashMap<HeapHandleId, HeapEntry>;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn clear(&mut self);
    fn get_metrics(&self) -> &HeapMetrics;
}

pub trait GarbageCollector {
    fn collect_garbage(
        &mut self,
        entries: &mut HashMap<HeapHandleId, HeapEntry>,
        roots: &[HeapHandleId],
    ) -> usize;
    fn get_collection_stats(&self) -> (usize, u32, u32);
}

pub trait HeapManager {
    fn get(&self, handle: HeapHandleId) -> Option<&HeapEntry>;
    fn get_mut(&mut self, handle: HeapHandleId) -> Option<&mut HeapEntry>;
    fn set_object_property(&mut self, handle: HeapHandleId, key: String, value: Value);
    fn get_object_property(&self, handle: HeapHandleId, key: &str) -> Option<&Value>;
    fn push_array_element(&mut self, handle: HeapHandleId, value: Value);
    fn get_array_element(&self, handle: HeapHandleId, idx: ArraySize) -> Option<&Value>;
    fn set_array_element(&mut self, handle: HeapHandleId, idx: ArraySize, value: Value);
    fn remove_object_property(&mut self, handle: HeapHandleId, key: &str);
    fn has_object_property(&self, handle: HeapHandleId, key: &str) -> bool;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn clear(&mut self);
    fn get_stats(&self) -> HeapStats;
    fn get_metrics(&self) -> &HeapMetrics;
    fn collect_garbage(&mut self, roots: &[HeapHandleId]) -> usize;
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
    fn alloc_string(&mut self, value: String) -> HeapHandleId;
}

pub struct Heap {
    allocator: Box<dyn HeapAllocator>,
    gc: Box<dyn GarbageCollector>,
    stats: HeapStats,
    metrics: HeapMetrics,
}

impl Clone for Heap {
    fn clone(&self) -> Self {
        Self {
            allocator: Box::new(super::allocation::HeapAllocatorImpl::new()),
            gc: Box::new(super::garbage_collection::GarbageCollectorImpl::new()),
            stats: self.stats.clone(),
            metrics: self.metrics.clone(),
        }
    }
}

impl Heap {
    pub fn new() -> Self {
        Self {
            allocator: Box::new(super::allocation::HeapAllocatorImpl::new()),
            gc: Box::new(super::garbage_collection::GarbageCollectorImpl::new()),
            stats: HeapStats::new(),
            metrics: HeapMetrics::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            allocator: Box::new(super::allocation::HeapAllocatorImpl::with_capacity(
                capacity,
            )),
            gc: Box::new(super::garbage_collection::GarbageCollectorImpl::new()),
            stats: HeapStats::new(),
            metrics: HeapMetrics::new(),
        }
    }

    // Métodos auxiliares para compatibilidade
    pub fn alloc_object(&mut self) -> HeapHandleId {
        let handle = self.allocator.allocate(HeapEntry::Object(HashMap::new()));
        self.metrics.record_allocation();
        self.update_stats();
        handle
    }

    pub fn alloc_array(&mut self) -> HeapHandleId {
        let handle = self.allocator.allocate(HeapEntry::Array(Vec::new()));
        self.metrics.record_allocation();
        self.update_stats();
        handle
    }

    pub fn alloc_function(
        &mut self,
        bytecode: Bytecode,
        arg_count: ArgIndex,
        local_count: LocalIndex,
    ) -> HeapHandleId {
        let handle = self.allocator.allocate(HeapEntry::Function {
            bytecode,
            arg_count,
            local_count,
            closure_vars: HashMap::new(),
        });
        self.metrics.record_allocation();
        self.update_stats();
        handle
    }

    pub fn alloc_string(&mut self, value: String) -> HeapHandleId {
        let handle = self.allocator.allocate(HeapEntry::String(value));
        self.metrics.record_allocation();
        self.update_stats();
        handle
    }

    // Métodos de compatibilidade para código existente
    pub fn get(&self, handle: HeapHandleId) -> Option<&HeapEntry> {
        self.allocator.get(handle)
    }

    pub fn get_mut(&mut self, handle: HeapHandleId) -> Option<&mut HeapEntry> {
        self.allocator.get_mut(handle)
    }

    pub fn get_object_property(&self, handle: HeapHandleId, key: &str) -> Option<&Value> {
        if let Some(HeapEntry::Object(obj)) = self.allocator.get(handle) {
            obj.get(key)
        } else {
            None
        }
    }

    pub fn set_object_property(&mut self, handle: HeapHandleId, key: String, value: Value) {
        if let Some(HeapEntry::Object(obj)) = self.allocator.get_mut(handle) {
            obj.insert(key, value);
        }
    }

    pub fn set_array_element(&mut self, handle: HeapHandleId, idx: ArraySize, value: Value) {
        if let Some(HeapEntry::Array(arr)) = self.allocator.get_mut(handle) {
            let index = idx.as_usize();
            if index < arr.len() {
                arr[index] = value;
            } else {
                arr.resize(index + 1, Value::Undefined);
                arr[index] = value;
            }
        }
    }

    fn update_stats(&mut self) {
        let entries = self.allocator.get_entries();
        let mut object_count = 0;
        let mut array_count = 0;
        let mut function_count = 0;
        let mut string_count = 0;
        let mut memory_usage = 0;

        for entry in entries.values() {
            match entry {
                HeapEntry::Object(_) => object_count += 1,
                HeapEntry::Array(_) => array_count += 1,
                HeapEntry::Function { .. } => function_count += 1,
                HeapEntry::String(_) => string_count += 1,
            }
            memory_usage += entry.memory_usage();
        }

        self.stats
            .update_counts(object_count, array_count, function_count, string_count);
        self.stats.set_memory_usage(memory_usage);
    }
}

impl HeapManager for Heap {
    fn get(&self, handle: HeapHandleId) -> Option<&HeapEntry> {
        self.allocator.get(handle)
    }

    fn get_mut(&mut self, handle: HeapHandleId) -> Option<&mut HeapEntry> {
        self.allocator.get_mut(handle)
    }

    fn set_object_property(&mut self, handle: HeapHandleId, key: String, value: Value) {
        if let Some(HeapEntry::Object(obj)) = self.allocator.get_mut(handle) {
            obj.insert(key, value);
        }
    }

    fn get_object_property(&self, handle: HeapHandleId, key: &str) -> Option<&Value> {
        if let Some(HeapEntry::Object(obj)) = self.allocator.get(handle) {
            obj.get(key)
        } else {
            None
        }
    }

    fn push_array_element(&mut self, handle: HeapHandleId, value: Value) {
        if let Some(HeapEntry::Array(arr)) = self.allocator.get_mut(handle) {
            arr.push(value);
        }
    }

    fn get_array_element(&self, handle: HeapHandleId, idx: ArraySize) -> Option<&Value> {
        if let Some(HeapEntry::Array(arr)) = self.allocator.get(handle) {
            arr.get(idx.as_usize())
        } else {
            None
        }
    }

    fn set_array_element(&mut self, handle: HeapHandleId, idx: ArraySize, value: Value) {
        if let Some(HeapEntry::Array(arr)) = self.allocator.get_mut(handle) {
            let index = idx.as_usize();
            if index < arr.len() {
                arr[index] = value;
            } else {
                arr.resize(index + 1, Value::Undefined);
                arr[index] = value;
            }
        }
    }

    fn remove_object_property(&mut self, handle: HeapHandleId, key: &str) {
        if let Some(HeapEntry::Object(obj)) = self.allocator.get_mut(handle) {
            obj.remove(key);
        }
    }

    fn has_object_property(&self, handle: HeapHandleId, key: &str) -> bool {
        if let Some(HeapEntry::Object(obj)) = self.allocator.get(handle) {
            obj.contains_key(key)
        } else {
            false
        }
    }

    fn size(&self) -> usize {
        self.allocator.size()
    }

    fn is_empty(&self) -> bool {
        self.allocator.is_empty()
    }

    fn clear(&mut self) {
        self.allocator.clear();
    }

    fn get_stats(&self) -> HeapStats {
        self.stats.clone()
    }

    fn get_metrics(&self) -> &HeapMetrics {
        &self.metrics
    }

    fn collect_garbage(&mut self, roots: &[HeapHandleId]) -> usize {
        let entries = self.allocator.get_entries_mut();
        let collected = self.gc.collect_garbage(entries, roots);
        self.metrics.record_gc_cycle(std::time::Duration::ZERO);
        collected
    }
}

impl HeapOperations for Heap {
    fn alloc_object(&mut self) -> HeapHandleId {
        self.alloc_object()
    }

    fn alloc_array(&mut self) -> HeapHandleId {
        self.alloc_array()
    }

    fn alloc_function(
        &mut self,
        bytecode: Bytecode,
        arg_count: ArgIndex,
        local_count: LocalIndex,
    ) -> HeapHandleId {
        self.alloc_function(bytecode, arg_count, local_count)
    }

    fn alloc_string(&mut self, value: String) -> HeapHandleId {
        self.alloc_string(value)
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}
