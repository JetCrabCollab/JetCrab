use jetcrab::api::Engine;

fn main() {
    println!("=== Test Labeled Statements ===");

    let mut engine = Engine::new();

    println!("\n1. Testing simple labeled statement:");

    println!("\n   Step 1: Test basic labeled statement");
    let result1 = engine.evaluate("let result = 0; myLabel: { result = 10; } result");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing labeled for loop:");

    println!("\n   Step 2: Test labeled for loop");
    let result2 =
        engine.evaluate("let sum = 0; outerLoop: for (let i = 0; i < 3; i++) { sum += i; } sum");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing labeled while loop:");

    println!("\n   Step 3: Test labeled while loop");
    let result3 = engine.evaluate("let count = 0; myLoop: while (count < 3) { count++; } count");
    println!("   Result: {:?}", result3);

    println!("\n=== Test Complete ===");
}
