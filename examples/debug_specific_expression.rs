use jetcrab::api::engine::Engine;

fn main() {
    let mut engine = Engine::new();

    println!("=== Debug Specific Expression ===\n");

    // Test the exact expression that was hanging
    println!("Testing the exact expression that was hanging:");
    let result = engine.evaluate("let arr = [1, 2, 3]; arr.push(4); arr.push(5); arr.length + arr[0] + arr[1]");
    println!("   Result: {:?}", result);

    println!("\n=== Debug Complete ===");
}
