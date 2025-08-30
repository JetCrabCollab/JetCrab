use jetcrab::api::Engine;

fn main() {
    println!("=== Debug For Loop Simple Test ===");

    let mut engine = Engine::new();

    println!("\n1. Testing simple for loop with sum:");

    println!("\n   Step 1: Initialize sum = 0");
    let result1 = engine.evaluate("let sum = 0");
    println!("   Result: {:?}", result1);

    println!("\n   Step 2: Check initial sum");
    let result2 = engine.evaluate("sum");
    println!("   Result: {:?}", result2);

    println!("\n   Step 3: Execute for loop");
    let result3 = engine.evaluate("for (let i = 0; i < 3; i++) { sum += i; }");
    println!("   Result: {:?}", result3);

    println!("\n   Step 4: Check final sum");
    let result4 = engine.evaluate("sum");
    println!("   Result: {:?}", result4);

    println!("\n   Step 5: Check final i");
    let result5 = engine.evaluate("i");
    println!("   Result: {:?}", result5);

    println!("\n2. Testing for loop with explicit return:");

    println!("\n   Step 6: Test for loop that returns a value");
    let result6 =
        engine.evaluate("let result = 0; for (let j = 0; j < 3; j++) { result += j; } result");
    println!("   Result: {:?}", result6);

    println!("\n3. Testing for loop with block expression:");

    println!("\n   Step 7: Test for loop as block expression");
    let result7 =
        engine.evaluate("{ let total = 0; for (let k = 0; k < 3; k++) { total += k; } total }");
    println!("   Result: {:?}", result7);
}
