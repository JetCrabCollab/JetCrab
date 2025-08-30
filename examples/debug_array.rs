use jetcrab::Engine;

fn main() {
    println!("=== Debug Array ===");

    let mut engine = Engine::new();

    // Test 1: Check if array is created
    println!("1. Check if array is created:");
    match engine.evaluate("let arr = [1, 2, 3]; typeof arr") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 2: Check array value
    println!("\n2. Check array value:");
    match engine.evaluate("let arr = [1, 2, 3]; arr") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 3: Check if array elements are stored
    println!("\n3. Check if array elements are stored:");
    match engine.evaluate("let arr = [1, 2, 3]; arr[0]") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    // Test 4: Check array length property
    println!("\n4. Check array length property:");
    match engine.evaluate("let arr = [1, 2, 3]; arr.length") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n=== Debug Complete ===");
}
