use jetcrab::api::Engine;

fn main() {
    println!("=== Debug Object Simple ===");

    let mut engine = Engine::new();

    println!("\n1. Testing simple object creation:");

    println!("\n   Step 1: Create empty object");
    let result1 = engine.evaluate("let obj = {}");
    println!("   Result: {:?}", result1);

    println!("\n   Step 2: Check if obj exists");
    let result2 = engine.evaluate("obj");
    println!("   Result: {:?}", result2);

    println!("\n   Step 3: Add property to object");
    let result3 = engine.evaluate("obj.name = 'Test'");
    println!("   Result: {:?}", result3);

    println!("\n   Step 4: Check property value");
    let result4 = engine.evaluate("obj.name");
    println!("   Result: {:?}", result4);

    println!("\n2. Testing object with initial properties:");

    println!("\n   Step 5: Create object with properties");
    let result5 = engine.evaluate("let obj2 = { x: 10, y: 20 }");
    println!("   Result: {:?}", result5);

    println!("\n   Step 6: Access property x");
    let result6 = engine.evaluate("obj2.x");
    println!("   Result: {:?}", result6);
}
