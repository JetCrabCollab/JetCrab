use jetcrab::api::engine::Engine;

fn main() {
    let mut engine = Engine::new();

    println!("=== Stress Test ===\n");

    // Test 1: Memory stress with large arrays
    println!("1. Testing memory stress with large arrays:");
    let result = engine.evaluate("let bigArr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; bigArr.length");
    println!("   Large array length: {:?}", result);

    // Test 2: Function call stress
    println!("\n2. Testing function call stress:");
    let result = engine.evaluate("let arr = [1, 2]; arr.push(3); arr.push(4); arr.push(5); arr.length");
    println!("   Multiple push operations: {:?}", result);

    // Test 3: Variable scope stress
    println!("\n3. Testing variable scope stress:");
    let result = engine.evaluate("let a = 1; let b = 2; let c = 3; let d = 4; let e = 5; a + b + c + d + e");
    println!("   Multiple variable operations: {:?}", result);

    // Test 4: Array access stress
    println!("\n4. Testing array access stress:");
    let result = engine.evaluate("let arr = [10, 20, 30, 40, 50]; arr[0] + arr[1] + arr[2] + arr[3] + arr[4]");
    println!("   Multiple array accesses: {:?}", result);

    // Test 5: Mixed operations stress
    println!("\n5. Testing mixed operations stress:");
    let result = engine.evaluate("let x = 100; let arr = [x, x*2, x*3]; arr.push(x*4); arr.length + x");
    println!("   Mixed operations: {:?}", result);

    println!("\n=== Stress Test Complete ===");
}
