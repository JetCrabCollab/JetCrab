use jetcrab::api::engine::Engine;

fn main() {
    let mut engine = Engine::new();

    println!("=== Architecture Test ===\n");

    // Test 1: Stack Operations
    println!("1. Testing Stack Operations:");
    let result = engine.evaluate("let x = 42; x");
    println!("   let x = 42; x = {:?}", result);

    // Test 2: Heap Operations
    println!("\n2. Testing Heap Operations:");
    let result = engine.evaluate("let arr = [1, 2, 3]; arr.length");
    println!("   let arr = [1, 2, 3]; arr.length = {:?}", result);

    // Test 3: Variable Management
    println!("\n3. Testing Variable Management:");
    let result = engine.evaluate("let a = 10; let b = 20; a + b");
    println!("   let a = 10; let b = 20; a + b = {:?}", result);

    // Test 4: Complex Operations
    println!("\n4. Testing Complex Operations:");
    let result = engine.evaluate("let arr = [1, 2]; arr.push(3); arr.length + arr[0]");
    println!("   Complex expression = {:?}", result);

    // Test 5: State Persistence
    println!("\n5. Testing State Persistence:");
    let result1 = engine.evaluate("let counter = 0");
    println!("   let counter = 0 = {:?}", result1);
    let result2 = engine.evaluate("counter = counter + 1");
    println!("   counter = counter + 1 = {:?}", result2);
    let result3 = engine.evaluate("counter");
    println!("   counter = {:?}", result3);

    println!("\n=== Architecture Test Complete ===");
}
