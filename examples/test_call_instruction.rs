use jetcrab::api::Engine;

fn main() {
    println!("=== Test Call Instruction ===");

    let mut engine = Engine::new();

    println!("\n1. Testing function call:");
    let result1 = engine.evaluate("function add(a, b) { return a + b; } add(5, 3)");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing built-in function:");
    let result2 = engine.evaluate("add(10, 20)");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing string method call:");
    let result3 = engine.evaluate("'Hello'.repeat(3)");
    println!("   Result: {:?}", result3);

    println!("\n=== Test Complete ===");
}
