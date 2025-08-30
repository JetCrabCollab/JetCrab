use jetcrab::api::Engine;

fn main() {
    println!("=== Debug Simple Loop ===");

    let mut engine = Engine::new();

    println!("\n1. Testing simple for loop step by step:");

    println!("\n   Step 1: Initialize sum = 0");
    let result1 = engine.evaluate("let sum = 0");
    println!("   Result: {:?}", result1);

    println!("\n   Step 2: Check sum value");
    let result2 = engine.evaluate("sum");
    println!("   Result: {:?}", result2);

    println!("\n   Step 3: Test for loop");
    let result3 = engine.evaluate("for (let i = 0; i < 3; i++) { sum += i; }");
    println!("   Result: {:?}", result3);

    println!("\n   Step 4: Check final sum value");
    let result4 = engine.evaluate("sum");
    println!("   Result: {:?}", result4);

    println!("\n   Step 5: Check i value");
    let result5 = engine.evaluate("i");
    println!("   Result: {:?}", result5);
}
