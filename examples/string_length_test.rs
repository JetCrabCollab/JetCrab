use jetcrab::Engine;

fn main() {
    println!("=== String Length Test ===");

    let mut engine = Engine::new();

    // Test 1: Simple string length
    println!("1. Simple string length:");
    match engine.evaluate("'hello'.length") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 2: Variable string length
    println!("\n2. Variable string length:");
    match engine.evaluate("let str = 'JavaScript'; str.length") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 3: Empty string length
    println!("\n3. Empty string length:");
    match engine.evaluate("''.length") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 4: Math.pow test
    println!("\n4. Math.pow test:");
    match engine.evaluate("Math.pow(2, 3)") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n=== Test Complete ===");
}
