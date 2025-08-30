use jetcrab::api::Engine;

fn main() {
    println!("=== Debug Object Bytecode ===");

    let mut engine = Engine::new();

    println!("\n1. Testing object literal bytecode generation:");

    println!("\n   Step 1: Test simple object literal");
    let result1 = engine.evaluate("{ x: 10 }");
    println!("   Result: {:?}", result1);

    println!("\n   Step 2: Test object literal with string property");
    let result2 = engine.evaluate("{ name: 'test' }");
    println!("   Result: {:?}", result2);

    println!("\n   Step 3: Test object literal with multiple properties");
    let result3 = engine.evaluate("{ a: 1, b: 2, c: 3 }");
    println!("   Result: {:?}", result3);

    println!("\n2. Testing object property access:");

    println!("\n   Step 4: Test property access on literal");
    let result4 = engine.evaluate("let obj = { x: 10 }; obj.x");
    println!("   Result: {:?}", result4);

    println!("\n   Step 5: Test property access with bracket notation");
    let result5 = engine.evaluate("obj['x']");
    println!("   Result: {:?}", result5);

    println!("\n3. Testing object property assignment:");

    println!("\n   Step 6: Test property assignment");
    let result6 = engine.evaluate("obj.y = 20");
    println!("   Result: {:?}", result6);

    println!("\n   Step 7: Test property access after assignment");
    let result7 = engine.evaluate("obj.y");
    println!("   Result: {:?}", result7);
}
