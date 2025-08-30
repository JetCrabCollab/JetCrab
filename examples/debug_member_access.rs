use jetcrab::Engine;

fn main() {
    println!("=== Debug Member Access ===");

    let mut engine = Engine::new();

    // Test 1: String literal property access
    println!("1. String literal property access:");
    match engine.evaluate("'hello'.length") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 2: Number property access
    println!("\n2. Number property access:");
    match engine.evaluate("(42).toString") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 3: Check if the issue is with literals vs variables
    println!("\n3. Variable string property access:");
    match engine.evaluate("let s = 'hello'; s.length") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 4: Array length property
    println!("\n4. Array length property:");
    match engine.evaluate("let arr = [1, 2, 3]; arr.length") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n=== Debug Complete ===");
}
