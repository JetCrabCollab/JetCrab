use jetcrab::api::Engine;

fn main() {
    println!("=== Test Do-While Loops ===");

    let mut engine = Engine::new();

    println!("\n1. Testing simple do-while loop:");

    println!("\n   Step 1: Test basic do-while loop");
    let result1 = engine.evaluate("let i = 0; do { i++; } while (i < 5); i");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing do-while with break:");

    println!("\n   Step 2: Test do-while with break");
    let result2 = engine.evaluate(
        "let sum = 0; let i = 1; do { sum += i; i++; if (i > 5) break; } while (i <= 10); sum",
    );
    println!("   Result: {:?}", result2);

    println!("\n3. Testing do-while with continue:");

    println!("\n   Step 3: Test do-while with continue");
    let result3 = engine.evaluate("let count = 0; let i = 0; do { i++; if (i % 2 == 0) continue; count++; } while (i < 6); count");
    println!("   Result: {:?}", result3);

    println!("\n=== Test Complete ===");
}
