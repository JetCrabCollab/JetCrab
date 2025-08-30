use jetcrab::api::Engine;

fn main() {
    println!("=== Test Simple Repeat ===");

    let mut engine = Engine::new();

    println!("\n1. Testing simple repeat:");
    let result1 = engine.evaluate("'Hi'.repeat(2)");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing repeat with number:");
    let result2 = engine.evaluate("'*'.repeat(5)");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing repeat with variable:");
    let result3 = engine.evaluate("let str = 'Hello'; str.repeat(3)");
    println!("   Result: {:?}", result3);

    println!("\n=== Test Complete ===");
}
