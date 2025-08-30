use jetcrab::api::Engine;

fn main() {
    println!("=== Test Continue Statement ===");

    let mut engine = Engine::new();

    println!("\n1. Testing continue in while loop:");

    println!("\n   Step 1: Test continue in while loop");
    let result1 = engine.evaluate(
        "let sum = 0; let i = 0; while (i < 5) { i++; if (i % 2 == 0) continue; sum += i; } sum",
    );
    println!("   Result: {:?}", result1);

    println!("\n2. Testing continue in for loop:");

    println!("\n   Step 2: Test continue in for loop");
    let result2 = engine.evaluate(
        "let sum = 0; for (let i = 1; i <= 10; i++) { if (i % 2 == 0) continue; sum += i; } sum",
    );
    println!("   Result: {:?}", result2);

    println!("\n3. Testing continue with while(true):");

    println!("\n   Step 3: Test continue in while(true)");
    let result3 = engine.evaluate("let count = 0; let i = 0; while (true) { i++; if (i > 10) break; if (i % 2 == 0) continue; count++; } count");
    println!("   Result: {:?}", result3);

    println!("\n=== Test Complete ===");
}
