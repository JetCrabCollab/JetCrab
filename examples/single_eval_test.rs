use jetcrab::Engine;

fn main() {
    println!("=== Single Evaluation Test ===");
    
    let mut engine = Engine::new();
    
    // Test: Create object and access property in single evaluation
    println!("1. Object creation and property access:");
    match engine.evaluate("let person = { name: 'Alice', age: 25 }; person.name") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    
    // Test: Object with multiple properties
    println!("\n2. Object with multiple properties:");
    match engine.evaluate("let config = { timeout: 5000, retries: 3 }; config.timeout + config.retries") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    
    // Test: Empty object
    println!("\n3. Empty object:");
    match engine.evaluate("let empty = {}; empty") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    
    println!("\n=== Test Complete ===");
}
