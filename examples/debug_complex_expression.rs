use jetcrab::api::engine::Engine;

fn main() {
    let mut engine = Engine::new();

    println!("=== Debug Complex Expression ===\n");

    // Test 1: Simple arithmetic
    println!("1. Testing simple arithmetic:");
    let result = engine.evaluate("1 + 2");
    println!("   1 + 2 = {:?}", result);

    // Test 2: Array creation
    println!("\n2. Testing array creation:");
    let result = engine.evaluate("let arr = [1, 2, 3]");
    println!("   let arr = [1, 2, 3] = {:?}", result);

    // Test 3: Array push
    println!("\n3. Testing array push:");
    let result = engine.evaluate("arr.push(4)");
    println!("   arr.push(4) = {:?}", result);

    // Test 4: Array length
    println!("\n4. Testing array length:");
    let result = engine.evaluate("arr.length");
    println!("   arr.length = {:?}", result);

    // Test 5: Complex expression (this is where it was hanging)
    println!("\n5. Testing complex expression:");
    let result = engine.evaluate("arr.length + arr[0]");
    println!("   arr.length + arr[0] = {:?}", result);

    println!("\n=== Debug Complete ===");
}
