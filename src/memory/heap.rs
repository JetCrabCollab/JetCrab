use crate::vm::types::{HeapId, MemorySize, ObjectCount, ObjectSize};
use crate::vm::value::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HeapObject {
    pub value: Value,
    pub marked: bool,
}

impl HeapObject {
    pub fn new(value: Value) -> Self {
        Self {
            value,
            marked: false,
        }
    }
}

pub struct MemoryHeap {
    objects: HashMap<HeapId, HeapObject>,
    next_id: HeapId,
    total_allocated: MemorySize,
}

impl Default for MemoryHeap {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryHeap {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            next_id: HeapId::new(0),
            total_allocated: MemorySize::new(0),
        }
    }

    pub fn allocate(&mut self, value: Value) -> HeapId {
        let id = self.next_id;
        self.next_id = self.next_id.next();

        let size = MemorySize::new(self.calculate_size(&value).as_usize());
        let object = HeapObject::new(value);
        self.objects.insert(id, object);

        self.total_allocated = self.total_allocated.add(size);
        id
    }

    pub fn get(&self, id: HeapId) -> Option<&Value> {
        self.objects.get(&id).map(|obj| &obj.value)
    }

    pub fn get_mut(&mut self, id: HeapId) -> Option<&mut Value> {
        self.objects.get_mut(&id).map(|obj| &mut obj.value)
    }

    pub fn mark(&mut self, id: HeapId) {
        if let Some(obj) = self.objects.get_mut(&id) {
            obj.marked = true;
        }
    }

    pub fn sweep(&mut self) -> MemorySize {
        let mut freed = MemorySize::new(0);
        let mut to_remove = Vec::new();

        for (id, obj) in &self.objects {
            if !obj.marked {
                to_remove.push(*id);
                freed = freed.add(MemorySize::new(self.calculate_size(&obj.value).as_usize()));
            }
        }

        for id in to_remove {
            self.objects.remove(&id);
        }

        self.total_allocated = self.total_allocated.sub(freed);
        freed
    }

    pub fn remove(&mut self, id: HeapId) -> Option<Value> {
        if let Some(obj) = self.objects.remove(&id) {
            let size = MemorySize::new(self.calculate_size(&obj.value).as_usize());
            self.total_allocated = self.total_allocated.sub(size);
            Some(obj.value)
        } else {
            None
        }
    }

    pub fn total_allocated(&self) -> MemorySize {
        self.total_allocated
    }

    pub fn object_count(&self) -> ObjectCount {
        ObjectCount::new(self.objects.len())
    }

    pub fn iter_objects(&self) -> impl Iterator<Item = (HeapId, &HeapObject)> {
        self.objects.iter().map(|(&id, obj)| (id, obj))
    }

    pub fn iter_objects_mut(&mut self) -> impl Iterator<Item = (HeapId, &mut HeapObject)> {
        self.objects.iter_mut().map(|(&id, obj)| (id, obj))
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.total_allocated = MemorySize::new(0);
    }

    pub fn contains(&self, id: HeapId) -> bool {
        self.objects.contains_key(&id)
    }

    pub fn is_marked(&self, id: HeapId) -> bool {
        self.objects.get(&id).map(|obj| obj.marked).unwrap_or(false)
    }

    pub fn unmark_all(&mut self) {
        for obj in self.objects.values_mut() {
            obj.marked = false;
        }
    }

    pub fn mark_all(&mut self) {
        for obj in self.objects.values_mut() {
            obj.marked = true;
        }
    }

    pub fn get_marked_objects(&self) -> Vec<HeapId> {
        self.objects
            .iter()
            .filter(|(_, obj)| obj.marked)
            .map(|(&id, _)| id)
            .collect()
    }

    pub fn get_unmarked_objects(&self) -> Vec<HeapId> {
        self.objects
            .iter()
            .filter(|(_, obj)| !obj.marked)
            .map(|(&id, _)| id)
            .collect()
    }

    fn calculate_size(&self, value: &Value) -> ObjectSize {
        match value {
            Value::Number(_) => ObjectSize::new(8),
            Value::String(s) => ObjectSize::new(s.len()),
            Value::Boolean(_) => ObjectSize::new(1),
            Value::Undefined | Value::Null => ObjectSize::new(0),
            Value::Object(_) | Value::Array(_) | Value::Function(_) => ObjectSize::new(8),
        }
    }
}
