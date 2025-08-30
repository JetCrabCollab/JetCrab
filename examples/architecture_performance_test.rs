use jetcrab::api::engine::Engine;
use std::time::Instant;

fn main() {
    let mut engine = Engine::new();

    println!("=== Architecture Performance Test ===\n");

    // Test 1: Basic operations performance
    println!("1. Testing basic operations performance:");
    let start = Instant::now();
    
    for i in 0..100 {
        let _result = engine.evaluate(&format!("let x{} = {}; x{}", i, i, i));
    }
    
    let duration = start.elapsed();
    println!("   100 variable operations: {:?}", duration);

    // Test 2: Array operations performance
    println!("\n2. Testing array operations performance:");
    let start = Instant::now();
    
    for i in 0..100 {
        let _result = engine.evaluate(&format!("let arr{} = [1, 2, 3, 4, 5]; arr{}.length", i, i));
    }
    
    let duration = start.elapsed();
    println!("   100 array operations: {:?}", duration);

    // Test 3: Complex expressions performance
    println!("\n3. Testing complex expressions performance:");
    let start = Instant::now();
    
    for i in 0..10 {
        let _result = engine.evaluate(&format!(
            "let arr{} = [1, 2, 3]; arr{}.push(4); arr{}.push(5); arr{}.length + arr{}[0] + arr{}[1]",
            i, i, i, i, i, i
        ));
    }
    
    let duration = start.elapsed();
    println!("   10 complex expressions: {:?}", duration);

    // Test 4: Memory stress test
    println!("\n4. Testing memory stress:");
    let start = Instant::now();
    
    let _result = engine.evaluate("let bigArr = []; for (let i = 0; i < 100; i++) { bigArr.push(i); } bigArr.length");
    
    let duration = start.elapsed();
    println!("   Large array creation: {:?}", duration);

    // Test 5: Mixed operations stress
    println!("\n5. Testing mixed operations stress:");
    let start = Instant::now();
    
    let _result = engine.evaluate("let obj = {}; obj.x = 10; obj.y = 20; obj.z = 30; obj.x + obj.y + obj.z");
    
    let duration = start.elapsed();
    println!("   Object operations: {:?}", duration);

    println!("\n=== Architecture Performance Test Complete ===");
}
