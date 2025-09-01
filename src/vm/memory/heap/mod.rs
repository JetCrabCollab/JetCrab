pub mod allocation;
pub mod gc;
pub mod generational;
pub mod object_shapes;
pub mod spaces;
pub mod string_interning;
pub mod types;

pub use allocation::*;
pub use generational::GenerationalHeap;
pub use object_shapes::*;
pub use string_interning::*;
pub use types::*;

use crate::vm::handle::HeapHandleId;
use crate::vm::types::MemorySize;
use std::collections::HashMap;

/// Simple heap implementation for basic memory management
pub struct Heap {
    total_allocated: usize,
    total_freed: usize,
    entries: HashMap<HeapHandleId, HeapEntry>,
    next_handle: HeapHandleId,

    // Alocadores especializados
    bump_allocator: BumpAllocator,
    free_list_allocator: FreeListAllocator,
    cell_allocator: CellAllocator,
}

#[derive(Debug, Clone)]
pub enum HeapEntry {
    Object(HashMap<String, crate::vm::value::Value>),
    Array(Vec<crate::vm::value::Value>),
    Function(String),
    String(String),
    Number(f64),
    Boolean(bool),
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}

impl Heap {
    pub fn new() -> Self {
        Self {
            total_allocated: 0,
            total_freed: 0,
            entries: HashMap::new(),
            next_handle: HeapHandleId::new(0),

            // Inicializar alocadores
            bump_allocator: BumpAllocator::new(MemorySize::new(1024 * 1024)), // 1MB
            free_list_allocator: FreeListAllocator::new(),
            cell_allocator: CellAllocator::new(64, 1024), // 64 bytes, 1024 cells
        }
    }

    pub fn allocate(
        &mut self,
        size: MemorySize,
        object_type: ObjectType,
    ) -> Result<HeapHandleId, String> {
        // Escolher o alocador apropriado baseado no tamanho e tipo
        let address = match object_type {
            ObjectType::String | ObjectType::Number | ObjectType::Boolean => {
                // Objetos pequenos vão para cell allocator
                self.cell_allocator.allocate(size)
            }
            ObjectType::Array => {
                // Arrays vão para free list allocator
                self.free_list_allocator.allocate(size)
            }
            ObjectType::Object | ObjectType::Function => {
                if size.bytes() <= 1024 {
                    // Objetos pequenos vão para bump allocator
                    self.bump_allocator.allocate(size)
                } else {
                    // Objetos grandes vão para free list allocator
                    self.free_list_allocator.allocate(size)
                }
            }
        };

        if let Some(_addr) = address {
            self.total_allocated += size.bytes();
            let handle = self.next_handle;
            self.next_handle = HeapHandleId::new(self.next_handle.as_usize() + 1);

            // Criar entrada baseada no tipo
            let entry = match object_type {
                ObjectType::Object => HeapEntry::Object(HashMap::new()),
                ObjectType::Array => HeapEntry::Array(Vec::new()),
                ObjectType::Function => HeapEntry::Function("anonymous".to_string()),
                ObjectType::String => HeapEntry::String(String::new()),
                ObjectType::Number => HeapEntry::Number(0.0),
                ObjectType::Boolean => HeapEntry::Boolean(false),
            };

            self.entries.insert(handle, entry);
            Ok(handle)
        } else {
            Err("Failed to allocate memory".to_string())
        }
    }

    pub fn deallocate(&mut self, handle: HeapHandleId) -> Result<(), String> {
        if let Some(entry) = self.entries.remove(&handle) {
            let size = match entry {
                HeapEntry::Object(_) => 64,
                HeapEntry::Array(_) => 32,
                HeapEntry::Function(_) => 128,
                HeapEntry::String(_) => 16,
                HeapEntry::Number(_) => 8,
                HeapEntry::Boolean(_) => 1,
            };

            // Tentar deallocar em todos os alocadores (um deles deve funcionar)
            let _ = self
                .cell_allocator
                .deallocate(handle.as_usize(), MemorySize::new(size));
            let _ = self
                .free_list_allocator
                .deallocate(handle.as_usize(), MemorySize::new(size));

            self.total_freed += size;
            Ok(())
        } else {
            Err("Invalid handle".to_string())
        }
    }

    pub fn get(&self, handle: HeapHandleId) -> Option<&HeapEntry> {
        self.entries.get(&handle)
    }

    pub fn get_mut(&mut self, handle: HeapHandleId) -> Option<&mut HeapEntry> {
        self.entries.get_mut(&handle)
    }

    pub fn total_allocated(&self) -> MemorySize {
        MemorySize::new(self.total_allocated)
    }

    pub fn total_free(&self) -> MemorySize {
        let bump_free = self.bump_allocator.total_free();
        let free_list_free = self.free_list_allocator.total_free();
        let cell_free = self.cell_allocator.total_free();

        MemorySize::new(bump_free.bytes() + free_list_free.bytes() + cell_free.bytes())
    }

    /// Get heap statistics
    pub fn stats(&self) -> String {
        format!(
            "Heap Stats:\n\
             Total Allocated: {} bytes\n\
             Total Freed: {} bytes\n\
             Bump Allocator: {} allocated, {} free\n\
             Free List Allocator: {} allocated, {} free\n\
             Cell Allocator: {} allocated, {} free",
            self.total_allocated,
            self.total_freed,
            self.bump_allocator.total_allocated().bytes(),
            self.bump_allocator.total_free().bytes(),
            self.free_list_allocator.total_allocated().bytes(),
            self.free_list_allocator.total_free().bytes(),
            self.cell_allocator.total_allocated().bytes(),
            self.cell_allocator.total_free().bytes()
        )
    }
}

/// Object type for allocation
#[derive(Debug, Clone, Copy)]
pub enum ObjectType {
    Object,
    Array,
    Function,
    String,
    Number,
    Boolean,
}

/// Implementation of HeapOperations trait for Heap
impl crate::vm::executor::traits::HeapOperations for Heap {
    fn alloc_object(&mut self) -> HeapHandleId {
        self.total_allocated += 64;
        let handle = self.next_handle;
        self.next_handle = HeapHandleId::new(self.next_handle.as_usize() + 1);
        self.entries
            .insert(handle, HeapEntry::Object(HashMap::new()));
        handle
    }

    fn alloc_array(&mut self) -> HeapHandleId {
        self.total_allocated += 32;
        let handle = self.next_handle;
        self.next_handle = HeapHandleId::new(self.next_handle.as_usize() + 1);
        self.entries.insert(handle, HeapEntry::Array(Vec::new()));
        handle
    }

    fn alloc_function(
        &mut self,
        _bytecode: crate::vm::compiler::Bytecode,
        _arg_count: crate::vm::types::ArgIndex,
        _local_count: crate::vm::types::LocalIndex,
    ) -> HeapHandleId {
        self.total_allocated += 128;
        let handle = self.next_handle;
        self.next_handle = HeapHandleId::new(self.next_handle.as_usize() + 1);
        self.entries
            .insert(handle, HeapEntry::Function("anonymous".to_string()));
        handle
    }

    fn get_object_property(
        &self,
        handle: HeapHandleId,
        key: &str,
    ) -> Option<&crate::vm::value::Value> {
        if let Some(HeapEntry::Object(obj)) = self.entries.get(&handle) {
            obj.get(key)
        } else {
            None
        }
    }

    fn set_object_property(
        &mut self,
        handle: HeapHandleId,
        key: String,
        value: crate::vm::value::Value,
    ) {
        if let Some(HeapEntry::Object(obj)) = self.entries.get_mut(&handle) {
            obj.insert(key, value);
        }
    }

    fn set_array_element(
        &mut self,
        handle: HeapHandleId,
        index: crate::vm::types::ArraySize,
        value: crate::vm::value::Value,
    ) {
        if let Some(HeapEntry::Array(arr)) = self.entries.get_mut(&handle) {
            let idx = index.as_usize();
            if idx >= arr.len() {
                arr.resize(idx + 1, crate::vm::value::Value::Undefined);
            }
            arr[idx] = value;
        }
    }

    fn get_heap(&self) -> &crate::vm::memory::heap::Heap {
        self
    }
}
