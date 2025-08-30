use jetcrab::Engine;

fn main() {
    println!("=== Array Test ===");

    let mut engine = Engine::new();

    // Test 1: Simple array creation
    println!("1. Simple array creation:");
    match engine.evaluate("[1, 2, 3]") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 2: Array length
    println!("\n2. Array length:");
    match engine.evaluate("let arr = [1, 2, 3, 4, 5]; arr.length") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 3: Empty array length
    println!("\n3. Empty array length:");
    match engine.evaluate("let empty = []; empty.length") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 4: Array with mixed types
    println!("\n4. Array with mixed types:");
    match engine.evaluate("let mixed = [1, 'hello', true]; mixed.length") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n=== Test Complete ===");
}
