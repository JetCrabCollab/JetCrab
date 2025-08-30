use jetcrab::api::Engine;

fn main() {
    println!("=== Test Switch Statement ===");
    
    let mut engine = Engine::new();
    
    println!("\n1. Testing simple switch statement:");
    
    println!("\n   Step 1: Test basic switch");
    let result1 = engine.evaluate("let x = 2; let result = 0; switch (x) { case 1: result = 10; break; case 2: result = 20; break; case 3: result = 30; break; default: result = 0; } result");
    println!("   Result: {:?}", result1);
    
    println!("\n2. Testing switch with default case:");
    
    println!("\n   Step 2: Test switch with default");
    let result2 = engine.evaluate("let x = 5; let result = 0; switch (x) { case 1: result = 10; break; case 2: result = 20; break; default: result = 100; } result");
    println!("   Result: {:?}", result2);
    
    println!("\n3. Testing switch with string values:");
    
    println!("\n   Step 3: Test switch with strings");
    let result3 = engine.evaluate("let x = 'banana'; let result = 0; switch (x) { case 'apple': result = 10; break; case 'banana': result = 20; break; case 'orange': result = 30; break; default: result = 0; } result");
    println!("   Result: {:?}", result3);
    
    println!("\n=== Test Complete ===");
}
