use jetcrab::api::Engine;

fn main() {
    println!("=== Test Simple Object ===");

    let mut engine = Engine::new();

    println!("\n1. Testing object creation:");
    let result1 = engine.evaluate("let obj = {}; obj");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing object with property:");
    let result2 = engine.evaluate("let obj = { a: 10 }; obj");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing simple property access:");
    let result3 = engine.evaluate("let obj = { value: 42 }; obj.value");
    println!("   Result: {:?}", result3);

    println!("\n=== Test Complete ===");
}
