use jetcrab::api::engine::Engine;

fn main() {
    let mut engine = Engine::new();

    println!("=== Error Handling Test ===\n");

    // Test 1: Valid operations
    println!("1. Testing valid operations:");
    let result = engine.evaluate("let x = 42; x");
    println!("   let x = 42; x = {:?}", result);

    // Test 2: Array operations
    println!("\n2. Testing array operations:");
    let result = engine.evaluate("let arr = [1, 2, 3]; arr.length");
    println!("   let arr = [1, 2, 3]; arr.length = {:?}", result);

    // Test 3: Complex expression
    println!("\n3. Testing complex expression:");
    let result = engine.evaluate("let arr = [1, 2]; arr.push(3); arr.length + arr[0]");
    println!("   Complex expression = {:?}", result);

    // Test 4: Object operations
    println!("\n4. Testing object operations:");
    let result = engine.evaluate("let obj = {}; obj.x = 10; obj.x");
    println!("   Object property access = {:?}", result);

    // Test 5: Type operations
    println!("\n5. Testing type operations:");
    let result = engine.evaluate("typeof 42");
    println!("   typeof 42 = {:?}", result);

    let result = engine.evaluate("typeof \"hello\"");
    println!("   typeof \"hello\" = {:?}", result);

    let result = engine.evaluate("typeof [1, 2, 3]");
    println!("   typeof [1, 2, 3] = {:?}", result);

    println!("\n=== Error Handling Test Complete ===");
}
