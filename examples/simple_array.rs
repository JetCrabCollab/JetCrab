use jetcrab::Engine;

fn main() {
    println!("=== Simple Array Test ===");
    
    let mut engine = Engine::new();
    
    // Test 1: Just create array
    println!("1. Just create array:");
    match engine.evaluate("[1, 2, 3]") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    
    // Test 2: Create array and store in variable
    println!("\n2. Create array and store in variable:");
    match engine.evaluate("let arr = [1, 2, 3]; arr") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    
    // Test 3: Check array length
    println!("\n3. Check array length:");
    match engine.evaluate("let arr = [1, 2, 3]; arr.length") {
        Ok(result) => println!("   Result: {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    
    println!("\n=== Test Complete ===");
}
