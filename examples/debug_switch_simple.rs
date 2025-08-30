use jetcrab::api::Engine;

fn main() {
    println!("=== Debug Switch Simple ===");

    let mut engine = Engine::new();

    println!("\n1. Testing simple switch step by step:");

    println!("\n   Step 1: Test just the discriminant");
    let result1 = engine.evaluate("let x = 2; x");
    println!("   Result: {:?}", result1);

    println!("\n   Step 2: Test simple case without break");
    let result2 =
        engine.evaluate("let x = 2; let result = 0; switch (x) { case 2: result = 20; } result");
    println!("   Result: {:?}", result2);

    println!("\n   Step 3: Test case with break");
    let result3 = engine
        .evaluate("let x = 2; let result = 0; switch (x) { case 2: result = 20; break; } result");
    println!("   Result: {:?}", result3);
}
