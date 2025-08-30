use jetcrab::Engine;

fn main() {
    println!("=== Debug Object Creation ===");

    let mut engine = Engine::new();

    // Test 1: Simple object creation
    println!("1. Creating empty object:");
    match engine.evaluate("let obj = {}; obj") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 2: Object with one property
    println!("\n2. Object with one property:");
    match engine.evaluate("let obj = { name: 'test' }; obj") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 3: Property access
    println!("\n3. Property access:");
    match engine.evaluate("let obj = { name: 'test' }; obj.name") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n=== Debug Complete ===");
}
