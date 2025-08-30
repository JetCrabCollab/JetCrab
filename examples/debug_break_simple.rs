use jetcrab::api::Engine;

fn main() {
    println!("=== Debug Break Statement ===");

    let mut engine = Engine::new();

    println!("\n1. Testing simple break statement:");

    println!("\n   Step 1: Test break in while loop");
    let result1 = engine.evaluate("let i = 0; while (i < 5) { if (i > 2) break; i++; } i");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing break with infinite loop:");

    println!("\n   Step 2: Test while(true) with break");
    let result2 = engine.evaluate(
        "let count = 0; let i = 1; while (true) { count += i; i++; if (i > 3) break; } count",
    );
    println!("   Result: {:?}", result2);

    println!("\n3. Testing break step by step:");

    println!("\n   Step 3: Initialize variables");
    let result3 = engine.evaluate("let total = 0; let i = 1");
    println!("   Result: {:?}", result3);

    println!("\n   Step 4: Test while loop without break");
    let result4 = engine.evaluate("while (i <= 3) { total += i; i++; } total");
    println!("   Result: {:?}", result4);

    println!("\n4. Testing if statement with break:");

    println!("\n   Step 5: Test if statement");
    let result5 = engine.evaluate("let x = 5; if (x > 3) { x = 10; } x");
    println!("   Result: {:?}", result5);
}
