use jetcrab::api::engine::Engine;

fn main() {
    let mut engine = Engine::new();

    println!("=== Debug Complex Expression Step by Step ===\n");

    // Test 1: Create array
    println!("1. Creating array:");
    let result = engine.evaluate("let arr = [1, 2, 3]");
    println!("   let arr = [1, 2, 3] = {:?}", result);

    // Test 2: First push
    println!("\n2. First push:");
    let result = engine.evaluate("arr.push(4)");
    println!("   arr.push(4) = {:?}", result);

    // Test 3: Second push
    println!("\n3. Second push:");
    let result = engine.evaluate("arr.push(5)");
    println!("   arr.push(5) = {:?}", result);

    // Test 4: Get length
    println!("\n4. Get length:");
    let result = engine.evaluate("arr.length");
    println!("   arr.length = {:?}", result);

    // Test 5: Get first element
    println!("\n5. Get first element:");
    let result = engine.evaluate("arr[0]");
    println!("   arr[0] = {:?}", result);

    // Test 6: Get second element
    println!("\n6. Get second element:");
    let result = engine.evaluate("arr[1]");
    println!("   arr[1] = {:?}", result);

    // Test 7: Add length + first element
    println!("\n7. Add length + first element:");
    let result = engine.evaluate("arr.length + arr[0]");
    println!("   arr.length + arr[0] = {:?}", result);

    // Test 8: Add length + first + second element
    println!("\n8. Add length + first + second element:");
    let result = engine.evaluate("arr.length + arr[0] + arr[1]");
    println!("   arr.length + arr[0] + arr[1] = {:?}", result);

    println!("\n=== Debug Complete ===");
}
