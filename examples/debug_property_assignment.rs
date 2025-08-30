use jetcrab::api::Engine;

fn main() {
    println!("=== Debug Property Assignment ===");

    let mut engine = Engine::new();

    println!("\n1. Testing property assignment step by step:");

    println!("\n   Step 1: Create object with initial property");
    let result1 = engine.evaluate("let obj = { x: 10 }");
    println!("   Result: {:?}", result1);

    println!("\n   Step 2: Check initial property");
    let result2 = engine.evaluate("obj.x");
    println!("   Result: {:?}", result2);

    println!("\n   Step 3: Assign new property");
    let result3 = engine.evaluate("obj.y = 20");
    println!("   Result: {:?}", result3);

    println!("\n   Step 4: Check new property");
    let result4 = engine.evaluate("obj.y");
    println!("   Result: {:?}", result4);

    println!("\n   Step 5: Check original property still exists");
    let result5 = engine.evaluate("obj.x");
    println!("   Result: {:?}", result5);

    println!("\n2. Testing property assignment with bracket notation:");

    println!("\n   Step 6: Assign property with bracket notation");
    let result6 = engine.evaluate("obj['z'] = 30");
    println!("   Result: {:?}", result6);

    println!("\n   Step 7: Check property with bracket notation");
    let result7 = engine.evaluate("obj['z']");
    println!("   Result: {:?}", result7);

    println!("\n3. Testing property assignment with variable:");

    println!("\n   Step 8: Assign property with variable");
    let result8 = engine.evaluate("let key = 'w'; obj[key] = 40");
    println!("   Result: {:?}", result8);

    println!("\n   Step 9: Check property with variable");
    let result9 = engine.evaluate("obj.w");
    println!("   Result: {:?}", result9);
}
