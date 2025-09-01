use jetcrab::api::Engine;

fn main() {
    println!("=== Test Math Functions ===");

    let mut engine = Engine::new();

    println!("\n1. Testing Math.pow function:");
    let result1 = engine.evaluate("Math.pow(2, 3)");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing Math.sqrt function:");
    let result2 = engine.evaluate("Math.sqrt(16)");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing Math.abs function:");
    let result3 = engine.evaluate("Math.abs(-42)");
    println!("   Result: {:?}", result3);

    println!("\n4. Testing Math.round function:");
    let result4 = engine.evaluate("Math.round(3.7)");
    println!("   Result: {:?}", result4);

    println!("\n5. Testing Math.floor function:");
    let result5 = engine.evaluate("Math.floor(3.9)");
    println!("   Result: {:?}", result5);

    println!("\n6. Testing Math.ceil function:");
    let result6 = engine.evaluate("Math.ceil(3.1)");
    println!("   Result: {:?}", result6);

    println!("\n7. Testing Math.min function:");
    let result7 = engine.evaluate("Math.min(5, 3, 8, 1)");
    println!("   Result: {:?}", result7);

    println!("\n8. Testing Math.max function:");
    let result8 = engine.evaluate("Math.max(5, 3, 8, 1)");
    println!("   Result: {:?}", result8);

    println!("\n=== Test Complete ===");
}
