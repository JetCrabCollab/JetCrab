use jetcrab::api::Engine;

fn main() {
    println!("=== Debug While Loop Simple ===");
    
    let mut engine = Engine::new();
    
    println!("\n1. Testing simple while loop:");
    
    println!("\n   Step 1: Test basic while loop");
    let result1 = engine.evaluate("let count = 0; let i = 0; while (i < 3) { count += i; i++; } count");
    println!("   Result: {:?}", result1);
    
    println!("\n2. Testing while loop with condition:");
    
    println!("\n   Step 2: Test while loop with condition");
    let result2 = engine.evaluate("let sum = 0; let num = 3; while (num > 0) { sum += num; num--; } sum");
    println!("   Result: {:?}", result2);
    
    println!("\n3. Testing while loop step by step:");
    
    println!("\n   Step 3: Initialize variables");
    let result3 = engine.evaluate("let total = 0; let i = 1");
    println!("   Result: {:?}", result3);
    
    println!("\n   Step 4: Test while loop body");
    let result4 = engine.evaluate("while (i <= 3) { total += i; i++; } total");
    println!("   Result: {:?}", result4);
    
    println!("\n4. Testing if statement:");
    
    println!("\n   Step 5: Test if statement");
    let result5 = engine.evaluate("let x = 5; if (x > 3) { x = 10; } x");
    println!("   Result: {:?}", result5);
}
