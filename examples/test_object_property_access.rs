use jetcrab::api::Engine;

fn main() {
    println!("=== Test Object Property Access ===");

    let mut engine = Engine::new();

    println!("\n1. Testing simple object property access:");
    let result1 = engine.evaluate("let obj = { a: 10, b: 20 }; obj.a");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing object property addition:");
    let result2 = engine.evaluate("let obj = { x: 5, y: 15 }; obj.x + obj.y");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing object property assignment:");
    let result3 = engine.evaluate("let obj = { value: 100 }; obj.newValue = 200; obj.newValue");
    println!("   Result: {:?}", result3);

    println!("\n4. Testing nested object property access:");
    let result4 = engine.evaluate("let obj = { nested: { deep: 42 } }; obj.nested.deep");
    println!("   Result: {:?}", result4);

    println!("\n5. Testing object property with string:");
    let result5 = engine.evaluate("let obj = { name: 'JetCrab', version: '1.0' }; obj.name + ' ' + obj.version");
    println!("   Result: {:?}", result5);

    println!("\n=== Test Complete ===");
}
