use jetcrab::api::engine::Engine;
use std::time::Instant;

fn main() {
    let mut engine = Engine::new();

    println!("=== Performance Test ===\n");

    // Test 1: Multiple array operations
    println!("1. Testing multiple array operations:");
    let start = Instant::now();

    for i in 0..1000 {
        let _result = engine.evaluate(&format!("let arr{} = [1, 2, 3, 4, 5]; arr{}.length", i, i));
    }

    let duration = start.elapsed();
    println!("   1000 array operations: {:?}", duration);

    // Test 2: Multiple variable operations
    println!("\n2. Testing multiple variable operations:");
    let start = Instant::now();

    for i in 0..1000 {
        let _result = engine.evaluate(&format!("let x{} = {}; x{}", i, i, i));
    }

    let duration = start.elapsed();
    println!("   1000 variable operations: {:?}", duration);

    // Test 3: Complex expression
    println!("\n3. Testing complex expression:");
    let start = Instant::now();

    let _result =
        engine.evaluate("let arr = [1, 2, 3, 4, 5]; arr.push(6); arr.push(7); arr.length");

    let duration = start.elapsed();
    println!("   Complex array expression: {:?}", duration);

    println!("\n=== Performance Test Complete ===");
}
