use jetcrab::api::engine::Engine;

fn main() {
    let mut engine = Engine::new();

    println!("=== Testing Array Methods ===\n");

    // Test 1: Create array and test push method
    println!("1. Testing push method:");
    let result = engine.evaluate("let arr = [1, 2]; arr.push(3); arr.length");
    println!("   arr.push(3); arr.length = {:?}", result);

    // Test 2: Test pop method on existing array
    println!("\n2. Testing pop method:");
    let result = engine.evaluate("arr.pop()");
    println!("   arr.pop() = {:?}", result);

    // Test 3: Test pop on empty array
    println!("\n3. Testing pop on empty array:");
    let result = engine.evaluate("let emptyArr = []; emptyArr.pop()");
    println!("   emptyArr.pop() on empty array = {:?}", result);

    println!("\n=== Array Methods Test Complete ===");
}
