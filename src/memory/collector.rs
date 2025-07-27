use crate::memory::heap::MemoryHeap;
use crate::vm::types::{AllocationCount, HeapId, MemorySize, ObjectId};
use crate::vm::value::Value;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum GCStrategy {
    MarkAndSweep,
    Generational,
    Incremental,
}

#[derive(Debug, Clone, Default)]
pub struct GCStats {
    pub total_collections: AllocationCount,
    pub total_bytes_freed: MemorySize,
    pub total_collection_time_ms: u64,
    pub last_collection_time_ms: u64,
    pub objects_marked: AllocationCount,
    pub objects_swept: AllocationCount,
}

pub struct GarbageCollector {
    heap: MemoryHeap,

    strategy: GCStrategy,

    stats: GCStats,

    marked_objects: HashSet<ObjectId>,

    root_objects: HashSet<ObjectId>,

    is_collecting: bool,

    collection_threshold: MemorySize,

    allocated_bytes: MemorySize,
}

impl GarbageCollector {
    pub fn new(heap: MemoryHeap) -> Self {
        Self {
            heap,
            strategy: GCStrategy::MarkAndSweep,
            stats: GCStats::default(),
            marked_objects: HashSet::new(),
            root_objects: HashSet::new(),
            is_collecting: false,
            collection_threshold: MemorySize::new(1024 * 1024),
            allocated_bytes: MemorySize::new(0),
        }
    }

    pub fn with_strategy(mut self, strategy: GCStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn with_threshold(mut self, threshold: MemorySize) -> Self {
        self.collection_threshold = threshold;
        self
    }

    pub fn allocate(&mut self, value: Value) -> ObjectId {
        let object_id = self.heap.allocate(value);
        let size = self.heap.total_allocated() - self.allocated_bytes;

        self.allocated_bytes = self.allocated_bytes.add(size);

        if self.allocated_bytes.as_usize() > self.collection_threshold.as_usize() {
            self.collect();
        }

        ObjectId::new(object_id.as_usize())
    }

    pub fn add_root(&mut self, object_id: ObjectId) {
        self.root_objects.insert(object_id);
    }

    pub fn remove_root(&mut self, object_id: ObjectId) {
        self.root_objects.remove(&object_id);
    }

    pub fn get(&self, object_id: ObjectId) -> Option<&Value> {
        self.heap.get(HeapId::from(object_id.as_usize()))
    }

    pub fn get_mut(&mut self, object_id: ObjectId) -> Option<&mut Value> {
        self.heap.get_mut(HeapId::from(object_id.as_usize()))
    }

    pub fn collect(&mut self) -> GCStats {
        if self.is_collecting {
            return self.stats.clone();
        }

        self.is_collecting = true;
        let start_time = std::time::Instant::now();

        match self.strategy {
            GCStrategy::MarkAndSweep => self.mark_and_sweep(),
            GCStrategy::Generational => self.generational_collect(),
            GCStrategy::Incremental => self.incremental_collect(),
        }

        let collection_time = start_time.elapsed().as_millis() as u64;
        self.stats.last_collection_time_ms = collection_time;
        self.stats.total_collection_time_ms += collection_time;
        self.stats.total_collections.increment();

        self.is_collecting = false;
        self.stats.clone()
    }

    fn mark_and_sweep(&mut self) {
        self.mark_phase();

        let freed_bytes = self.sweep_phase();

        self.stats.total_bytes_freed = self.stats.total_bytes_freed.add(freed_bytes);
        self.allocated_bytes = self.allocated_bytes.sub(freed_bytes);
    }

    fn mark_phase(&mut self) {
        self.marked_objects.clear();

        let root_objects: Vec<ObjectId> = self.root_objects.iter().cloned().collect();
        for root_id in root_objects {
            self.mark_object(root_id);
        }

        self.stats.objects_marked = AllocationCount::new(self.marked_objects.len());
    }

    fn mark_object(&mut self, object_id: ObjectId) {
        if self.marked_objects.contains(&object_id) {
            return;
        }

        self.marked_objects.insert(object_id);

        if let Some(value) = self.heap.get(HeapId::from(object_id.as_usize())) {
            match value {
                Value::Object(object_id) => self.mark_object(ObjectId::new(object_id.as_usize())),
                Value::Array(array_id) => self.mark_object(ObjectId::new(array_id.as_usize())),
                Value::Function(function_id) => {
                    self.mark_object(ObjectId::new(function_id.as_usize()))
                }
                _ => {}
            }
        }
    }

    fn sweep_phase(&mut self) -> MemorySize {
        let mut freed_bytes = MemorySize::new(0);
        let mut to_remove = Vec::new();

        for (object_id, _) in self.heap.iter_objects() {
            if !self
                .marked_objects
                .contains(&ObjectId::new(object_id.as_usize()))
            {
                to_remove.push(object_id);
            }
        }

        let objects_to_remove = to_remove.len();

        for object_id in to_remove {
            if let Some(_value) = self.heap.get(object_id) {
                freed_bytes = freed_bytes.add(MemorySize::new(
                    self.heap.total_allocated().as_usize()
                        / self.heap.object_count().as_usize().max(1),
                ));
            }
            self.heap.remove(object_id);
        }

        self.stats.objects_swept = AllocationCount::new(objects_to_remove);
        freed_bytes
    }

    fn generational_collect(&mut self) {
        self.mark_and_sweep();
    }

    fn incremental_collect(&mut self) {
        self.mark_and_sweep();
    }

    pub fn stats(&self) -> &GCStats {
        &self.stats
    }

    pub fn heap(&self) -> &MemoryHeap {
        &self.heap
    }

    pub fn heap_mut(&mut self) -> &mut MemoryHeap {
        &mut self.heap
    }

    pub fn is_marked(&self, object_id: ObjectId) -> bool {
        self.marked_objects.contains(&object_id)
    }

    pub fn allocated_bytes(&self) -> MemorySize {
        self.allocated_bytes
    }

    pub fn set_threshold(&mut self, threshold: MemorySize) {
        self.collection_threshold = threshold;
    }

    pub fn force_collect(&mut self) -> GCStats {
        self.collect()
    }
}

impl Default for GarbageCollector {
    fn default() -> Self {
        Self::new(MemoryHeap::new())
    }
}
