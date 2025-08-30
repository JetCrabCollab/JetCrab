use jetcrab::Engine;

fn main() {
    println!("=== Object Creation Test ===");
    
    let mut engine = Engine::new();
    
    // Test 1: Empty object
    println!("1. Empty object:");
    match engine.evaluate("{}") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    
    // Test 2: Object with properties
    println!("\n2. Object with properties:");
    match engine.evaluate("{ name: 'test', age: 25 }") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    
    // Test 3: Object assignment
    println!("\n3. Object assignment:");
    match engine.evaluate("let obj = { name: 'test' }") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    
    println!("\n=== Test Complete ===");
}
