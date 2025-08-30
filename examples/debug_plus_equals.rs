use jetcrab::api::Engine;

fn main() {
    println!("=== Debug Plus Equals Operator ===");

    let mut engine = Engine::new();

    println!("\n1. Testing += operator:");

    println!("\n   Step 1: Initialize sum = 0");
    let result1 = engine.evaluate("let sum = 0");
    println!("   Result: {:?}", result1);

    println!("\n   Step 2: Check sum value");
    let result2 = engine.evaluate("sum");
    println!("   Result: {:?}", result2);

    println!("\n   Step 3: Test sum += 1");
    let result3 = engine.evaluate("sum += 1");
    println!("   Result: {:?}", result3);

    println!("\n   Step 4: Check sum value after += 1");
    let result4 = engine.evaluate("sum");
    println!("   Result: {:?}", result4);

    println!("\n   Step 5: Test sum += 2");
    let result5 = engine.evaluate("sum += 2");
    println!("   Result: {:?}", result5);

    println!("\n   Step 6: Check final sum value");
    let result6 = engine.evaluate("sum");
    println!("   Result: {:?}", result6);

    println!("\n2. Testing += with variable:");

    println!("\n   Step 7: Initialize i = 5");
    let result7 = engine.evaluate("let i = 5");
    println!("   Result: {:?}", result7);

    println!("\n   Step 8: Test sum += i");
    let result8 = engine.evaluate("sum += i");
    println!("   Result: {:?}", result8);

    println!("\n   Step 9: Check final sum value");
    let result9 = engine.evaluate("sum");
    println!("   Result: {:?}", result9);

    println!("\n3. Testing += in expression:");

    println!("\n   Step 10: Test expression with +=");
    let result10 = engine.evaluate("let x = 10; x += 5; x");
    println!("   Result: {:?}", result10);
}
