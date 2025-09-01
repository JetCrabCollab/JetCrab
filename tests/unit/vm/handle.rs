use jetcrab::vm::handle::{HeapHandle, HeapHandleId, ObjectHandle, INVALID_HANDLE};

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
