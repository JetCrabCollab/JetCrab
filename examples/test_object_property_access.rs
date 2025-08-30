use jetcrab::api::Engine;

fn main() {
    println!("=== Test Object Property Access ===");

    let mut engine = Engine::new();

    println!("\n1. Testing basic object creation:");
    let result1 = engine.evaluate("let obj = { name: 'John', age: 30 }");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing property access with dot notation:");
    let result2 = engine.evaluate("obj.name");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing property access with bracket notation:");
    let result3 = engine.evaluate("obj['age']");
    println!("   Result: {:?}", result3);

    println!("\n4. Testing property assignment:");
    let result4 = engine.evaluate("obj.city = 'New York'");
    println!("   Result: {:?}", result4);

    println!("\n5. Testing property assignment with bracket notation:");
    let result5 = engine.evaluate("obj['country'] = 'USA'");
    println!("   Result: {:?}", result5);

    println!("\n6. Testing property access after assignment:");
    let result6 = engine.evaluate("obj.city");
    println!("   Result: {:?}", result6);

    println!("\n7. Testing property access with variable:");
    let result7 = engine.evaluate("let prop = 'name'; obj[prop]");
    println!("   Result: {:?}", result7);

    println!("\n8. Testing nested object property access:");
    let result8 =
        engine.evaluate("let nested = { user: { profile: { email: 'john@example.com' } } }");
    println!("   Result: {:?}", result8);

    let result9 = engine.evaluate("nested.user.profile.email");
    println!("   Result: {:?}", result9);

    println!("\n9. Testing object with computed properties:");
    let result10 = engine.evaluate("let key = 'dynamic'; let dynamicObj = { [key]: 'value' }");
    println!("   Result: {:?}", result10);

    let result11 = engine.evaluate("dynamicObj.dynamic");
    println!("   Result: {:?}", result11);

    println!("\n=== Test Complete ===");
}
