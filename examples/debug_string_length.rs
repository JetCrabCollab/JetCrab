use jetcrab::Engine;

fn main() {
    println!("=== Debug String Length ===");

    let mut engine = Engine::new();

    // Test 1: Just the string
    println!("1. Just the string:");
    match engine.evaluate("'hello'") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 2: String stored in variable
    println!("\n2. String stored in variable:");
    match engine.evaluate("let str = 'hello'; str") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 3: Test typeof string
    println!("\n3. Test typeof string:");
    match engine.evaluate("typeof 'hello'") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 4: Test with simple property access
    println!("\n4. Test with simple property access:");
    match engine.evaluate("let obj = { test: 5 }; obj.test") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n=== Debug Complete ===");
}
