use jetcrab::api::Engine;

fn main() {
    println!("=== Debug Parser Plus Equals ===");

    let mut engine = Engine::new();

    println!("\n1. Testing simple expression:");

    println!("\n   Step 1: Test simple addition");
    let result1 = engine.evaluate("1 + 2");
    println!("   Result: {:?}", result1);

    println!("\n   Step 2: Test simple assignment");
    let result2 = engine.evaluate("let x = 5");
    println!("   Result: {:?}", result2);

    println!("\n   Step 3: Test += operator");
    let result3 = engine.evaluate("x += 3");
    println!("   Result: {:?}", result3);

    println!("\n   Step 4: Check x value");
    let result4 = engine.evaluate("x");
    println!("   Result: {:?}", result4);

    println!("\n2. Testing += in for loop:");

    println!("\n   Step 5: Test for loop with +=");
    let result5 = engine.evaluate("let sum = 0; for (let i = 0; i < 3; i++) { sum += i; } sum");
    println!("   Result: {:?}", result5);

    println!("\n   Step 6: Check individual values");
    let result6 = engine.evaluate("sum");
    println!("   sum: {:?}", result6);
    let result7 = engine.evaluate("i");
    println!("   i: {:?}", result7);
}
