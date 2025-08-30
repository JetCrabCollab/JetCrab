use jetcrab::Engine;

fn main() {
    println!("=== Simple Test ===");

    let mut engine = Engine::new();

    // Test 1: Just a number
    println!("1. Number literal:");
    match engine.evaluate("42") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 2: Simple variable declaration
    println!("\n2. Variable declaration:");
    match engine.evaluate("let x = 42") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 3: Variable retrieval
    println!("\n3. Variable retrieval:");
    match engine.evaluate("x") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n=== Test Complete ===");
}
