use jetcrab::vm::{
    heap::Heap, ArrayHandle, FunctionHandle, HeapHandleId, ObjectHandle, Value, INVALID_HANDLE,
};

fn main() {
    println!("=== Type-Safe Handles Example ===\n");

    let object_handle = ObjectHandle::from_usize(42);
    let array_handle = ArrayHandle::from_usize(43);
    let function_handle = FunctionHandle::from_usize(44);

    println!("Created handles:");
    println!("  Object: {}", object_handle);
    println!("  Array:  {}", array_handle);
    println!("  Function: {}", function_handle);

    let object_value = Value::Object(object_handle.clone());
    let array_value = Value::Array(array_handle.clone());
    let function_value = Value::Function(function_handle.clone());

    println!("\nValues with type-safe handles:");
    println!("  Object value: {:?}", object_value);
    println!("  Array value:  {:?}", array_value);
    println!("  Function value: {:?}", function_value);

    println!("\nHandle validation:");
    println!("  Object handle valid: {}", object_handle.is_valid());
    println!("  Array handle valid:  {}", array_handle.is_valid());
    println!("  Function handle valid: {}", function_handle.is_valid());

    let invalid_handle = ObjectHandle::new(INVALID_HANDLE);
    println!("  Invalid handle valid: {}", invalid_handle.is_valid());

    println!("\nHandle conversions:");
    let handle_id: HeapHandleId = object_handle.id();
    let back_to_usize: usize = handle_id.into();
    println!(
        "  Object handle ID: {} -> usize: {}",
        handle_id, back_to_usize
    );

    println!("\nHelper functions:");
    let obj_handle_2 = jetcrab::vm::handle::create_object_handle(100);
    let arr_handle_2 = jetcrab::vm::handle::create_array_handle(101);
    let func_handle_2 = jetcrab::vm::handle::create_function_handle(102);

    println!("  Created object handle: {}", obj_handle_2);
    println!("  Created array handle:  {}", arr_handle_2);
    println!("  Created function handle: {}", func_handle_2);

    println!("\n=== Benefits of Type-Safe Handles ===");
    println!("✅ Type safety: Prevents mixing different handle types");
    println!("✅ Better semantics: Clear distinction between handle types");
    println!("✅ Compile-time errors: Catches handle type mismatches");
    println!("✅ Self-documenting: No need for comments explaining handle types");
    println!("✅ Validation: Built-in handle validity checking");
    println!("✅ Extensibility: Easy to add new handle types");

    println!("\n=== Before vs After ===");
    println!("Before (usize):");
    println!("  Value::Object(42)  // What does 42 represent?");
    println!("  Value::Array(42)   // Same number, different meaning");
    println!("  Value::Function(42) // Confusing!");

    println!("\nAfter (type-safe):");
    println!("  Value::Object(ObjectHandle::from_usize(42))");
    println!("  Value::Array(ArrayHandle::from_usize(42))");
    println!("  Value::Function(FunctionHandle::from_usize(42))");
    println!("  // Clear distinction, type safety, no confusion!");
}
