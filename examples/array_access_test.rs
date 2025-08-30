use jetcrab::Engine;

fn main() {
    println!("=== Array Access Test ===");

    let mut engine = Engine::new();

    // Test 1: Array with bracket notation
    println!("1. Array with bracket notation:");
    match engine.evaluate("let arr = [1, 2, 3]; arr[0]") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 2: Different indices
    println!("\n2. Different indices:");
    match engine.evaluate("let arr = [10, 20, 30]; arr[1]") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 3: Last element
    println!("\n3. Last element:");
    match engine.evaluate("let arr = [100, 200, 300]; arr[2]") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 4: Out of bounds
    println!("\n4. Out of bounds:");
    match engine.evaluate("let arr = [1, 2, 3]; arr[5]") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n=== Test Complete ===");
}
