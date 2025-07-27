use serde::{Deserialize, Serialize};
use std::fmt;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HeapHandleId(usize);

impl HeapHandleId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        self.0 != usize::MAX
    }
}

impl From<usize> for HeapHandleId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<HeapHandleId> for usize {
    fn from(handle: HeapHandleId) -> Self {
        handle.0
    }
}

impl fmt::Display for HeapHandleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handle({})", self.0)
    }
}

pub trait HandleEntry: Send + Sync {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HeapHandle<T: HandleEntry> {
    id: HeapHandleId,
    _phantom: PhantomData<T>,
}

impl<T: HandleEntry> HeapHandle<T> {
    pub fn new(id: HeapHandleId) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }

    pub fn from_usize(id: usize) -> Self {
        Self::new(HeapHandleId::new(id))
    }

    pub fn id(&self) -> HeapHandleId {
        self.id
    }

    pub fn as_usize(&self) -> usize {
        self.id.as_usize()
    }

    pub fn is_valid(&self) -> bool {
        self.id.is_valid()
    }
}

impl<T: HandleEntry> From<usize> for HeapHandle<T> {
    fn from(id: usize) -> Self {
        Self::from_usize(id)
    }
}

impl<T: HandleEntry> From<HeapHandle<T>> for usize {
    fn from(handle: HeapHandle<T>) -> Self {
        handle.as_usize()
    }
}

impl<T: HandleEntry> fmt::Display for HeapHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectEntry;
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayEntry;
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionEntry;

impl HandleEntry for ObjectEntry {}
impl HandleEntry for ArrayEntry {}
impl HandleEntry for FunctionEntry {}

pub type ObjectHandle = HeapHandle<ObjectEntry>;
pub type ArrayHandle = HeapHandle<ArrayEntry>;
pub type FunctionHandle = HeapHandle<FunctionEntry>;

pub const INVALID_HANDLE: HeapHandleId = HeapHandleId(usize::MAX);

pub fn create_object_handle(id: usize) -> ObjectHandle {
    ObjectHandle::from_usize(id)
}

pub fn create_array_handle(id: usize) -> ArrayHandle {
    ArrayHandle::from_usize(id)
}

pub fn create_function_handle(id: usize) -> FunctionHandle {
    FunctionHandle::from_usize(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_creation() {
        let handle_id = HeapHandleId::new(42);
        assert_eq!(handle_id.as_usize(), 42);
        assert!(handle_id.is_valid());

        let obj_handle = ObjectHandle::from_usize(42);
        assert_eq!(obj_handle.as_usize(), 42);
        assert!(obj_handle.is_valid());
    }

    #[test]
    fn test_invalid_handle() {
        assert!(!INVALID_HANDLE.is_valid());

        let invalid_obj = ObjectHandle::new(INVALID_HANDLE);
        assert!(!invalid_obj.is_valid());
    }

    #[test]
    fn test_handle_conversion() {
        let id = 123;
        let handle_id: HeapHandleId = id.into();
        let back_to_usize: usize = handle_id.into();
        assert_eq!(back_to_usize, id);

        let obj_handle: ObjectHandle = id.into();
        let back_to_usize: usize = obj_handle.into();
        assert_eq!(back_to_usize, id);
    }

    #[test]
    fn test_handle_display() {
        let handle_id = HeapHandleId::new(42);
        assert_eq!(handle_id.to_string(), "Handle(42)");

        let obj_handle = ObjectHandle::from_usize(42);
        assert_eq!(obj_handle.to_string(), "Handle(42)");
    }
}
