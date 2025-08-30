use jetcrab::vm::{ArrayHandle, FunctionHandle, ObjectHandle, INVALID_HANDLE};

fn main() {
    println!("=== Type-Safe Handles - Simple Example ===\n");

    // Example 1: Creating and using type-safe handles
    println!("1. Handle Creation and Validation:");
    let object_handle = ObjectHandle::from_usize(42);
    let array_handle = ArrayHandle::from_usize(43);
    let function_handle = FunctionHandle::from_usize(44);

    println!("  Created handles:");
    println!(
        "    Object: {} (valid: {})",
        object_handle,
        object_handle.is_valid()
    );
    println!(
        "    Array:  {} (valid: {})",
        array_handle,
        array_handle.is_valid()
    );
    println!(
        "    Function: {} (valid: {})",
        function_handle,
        function_handle.is_valid()
    );
    println!();

    // Example 2: Handle validation
    println!("2. Handle Validation:");
    let invalid_handle = ObjectHandle::new(INVALID_HANDLE);
    println!(
        "  Invalid handle: {} (valid: {})",
        invalid_handle,
        invalid_handle.is_valid()
    );

    let zero_handle = ObjectHandle::from_usize(0);
    println!(
        "  Zero handle: {} (valid: {})",
        zero_handle,
        zero_handle.is_valid()
    );
    println!();

    // Example 3: Handle conversions
    println!("3. Handle Conversions:");
    let handle_id: usize = object_handle.id().into();
    let back_to_handle = ObjectHandle::from_usize(handle_id);
    println!(
        "  Object handle ID: {} -> usize: {} -> handle: {}",
        object_handle.id(),
        handle_id,
        back_to_handle
    );
    println!();

    // Example 4: Helper functions
    println!("4. Helper Functions:");
    let obj_handle_2 = jetcrab::vm::handle::create_object_handle(100);
    let arr_handle_2 = jetcrab::vm::handle::create_array_handle(101);
    let func_handle_2 = jetcrab::vm::handle::create_function_handle(102);

    println!("  Created handles using helpers:");
    println!(
        "    Object handle: {} (valid: {})",
        obj_handle_2,
        obj_handle_2.is_valid()
    );
    println!(
        "    Array handle:  {} (valid: {})",
        arr_handle_2,
        arr_handle_2.is_valid()
    );
    println!(
        "    Function handle: {} (valid: {})",
        func_handle_2,
        func_handle_2.is_valid()
    );
    println!();

    // Example 5: Type safety demonstration
    println!("5. Type Safety Demonstration:");
    demonstrate_type_safety();
    println!();

    // Example 6: Practical usage simulation
    println!("6. Practical Usage Simulation:");
    simulate_practical_usage();
    println!();

    println!("=== Type-Safe Handles Example Complete ===");
    println!("This demonstrates how type-safe handles provide:");
    println!("✅ Type safety: Prevents mixing different handle types");
    println!("✅ Better semantics: Clear distinction between handle types");
    println!("✅ Compile-time errors: Catches handle type mismatches");
    println!("✅ Self-documenting: No need for comments explaining handle types");
    println!("✅ Validation: Built-in handle validity checking");
    println!("✅ Extensibility: Easy to add new handle types");
}

fn demonstrate_type_safety() {
    let object_handle = ObjectHandle::from_usize(1);
    let array_handle = ArrayHandle::from_usize(1);
    let function_handle = FunctionHandle::from_usize(1);

    // This would cause a compile-time error if we tried to mix types:
    // let mixed_handles = vec![object_handle, array_handle]; // Won't compile!

    // Instead, we need separate collections for each type:
    let object_handles = vec![object_handle];
    let array_handles = vec![array_handle];
    let function_handles = vec![function_handle];

    println!("  Separate collections for different handle types:");
    println!("    Objects: {:?}", object_handles);
    println!("    Arrays: {:?}", array_handles);
    println!("    Functions: {:?}", function_handles);
}

fn simulate_practical_usage() {
    // Simulate a simple object system
    let mut object_counter = 0;
    let mut array_counter = 0;
    let mut function_counter = 0;

    // Create some handles
    let user_handle = ObjectHandle::from_usize(object_counter);
    object_counter += 1;

    let scores_handle = ArrayHandle::from_usize(array_counter);
    array_counter += 1;

    let callback_handle = FunctionHandle::from_usize(function_counter);
    function_counter += 1;

    println!("  Simulated object system:");
    println!("    User object: {}", user_handle);
    println!("    Scores array: {}", scores_handle);
    println!("    Callback function: {}", callback_handle);

    // Demonstrate handle validation in practice
    if user_handle.is_valid() {
        println!("    User handle is valid and ready to use");
    }

    if scores_handle.is_valid() {
        println!("    Scores array handle is valid and ready to use");
    }

    if callback_handle.is_valid() {
        println!("    Callback function handle is valid and ready to use");
    }
}
