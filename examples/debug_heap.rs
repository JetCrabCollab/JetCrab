use jetcrab::Engine;

fn main() {
    println!("=== Debug Heap and Objects ===");

    let mut engine = Engine::new();

    // Test 1: Create object and check if it's in memory
    println!("1. Creating object:");
    match engine.evaluate("let obj = { name: 'test' }") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 2: Try to access the object
    println!("\n2. Accessing object:");
    match engine.evaluate("obj") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 3: Try to access object property
    println!("\n3. Accessing object property:");
    match engine.evaluate("obj.name") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 4: Check if object exists in scope
    println!("\n4. Checking object existence:");
    match engine.evaluate("typeof obj") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n=== Debug Complete ===");
}
