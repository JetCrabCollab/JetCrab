use jetcrab::api::Engine;

fn main() {
    println!("=== Debug Member Expression ===");
    
    let mut engine = Engine::new();
    
    println!("\n1. Testing member expression access:");
    
    println!("\n   Step 1: Create object");
    let result1 = engine.evaluate("let obj = { x: 10 }");
    println!("   Result: {:?}", result1);
    
    println!("\n   Step 2: Access property with dot notation");
    let result2 = engine.evaluate("obj.x");
    println!("   Result: {:?}", result2);
    
    println!("\n   Step 3: Access property with bracket notation");
    let result3 = engine.evaluate("obj['x']");
    println!("   Result: {:?}", result3);
    
    println!("\n2. Testing member expression assignment:");
    
    println!("\n   Step 4: Assign property with dot notation");
    let result4 = engine.evaluate("obj.y = 20");
    println!("   Result: {:?}", result4);
    
    println!("\n   Step 5: Check assigned property");
    let result5 = engine.evaluate("obj.y");
    println!("   Result: {:?}", result5);
    
    println!("\n   Step 6: Check original property still exists");
    let result6 = engine.evaluate("obj.x");
    println!("   Result: {:?}", result6);
    
    println!("\n3. Testing member expression with variable:");
    
    println!("\n   Step 7: Assign property with variable key");
    let result7 = engine.evaluate("let key = 'z'; obj[key] = 30");
    println!("   Result: {:?}", result7);
    
    println!("\n   Step 8: Check property with variable key");
    let result8 = engine.evaluate("obj.z");
    println!("   Result: {:?}", result8);
}
