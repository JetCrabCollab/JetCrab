use jetcrab::api::Engine;

fn main() {
    println!("=== Test Advanced Labeled Statements ===");

    let mut engine = Engine::new();

    println!("\n1. Testing labeled statement with break:");

    println!("\n   Step 1: Test labeled statement with break");
    let result1 = engine
        .evaluate("let result = 0; myLabel: { result = 10; break myLabel; result = 20; } result");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing labeled for loop with break label:");

    println!("\n   Step 2: Test labeled for loop with break label");
    let result2 = engine.evaluate("let sum = 0; outerLoop: for (let i = 0; i < 3; i++) { for (let j = 0; j < 3; j++) { if (i === 1 && j === 1) { break outerLoop; } sum += i + j; } } sum");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing labeled while loop with continue label:");

    println!("\n   Step 3: Test labeled while loop with continue label");
    let result3 = engine.evaluate("let count = 0; let i = 0; myLoop: while (i < 5) { i++; if (i === 3) { continue myLoop; } count++; } count");
    println!("   Result: {:?}", result3);

    println!("\n4. Testing nested labeled statements:");

    println!("\n   Step 4: Test nested labeled statements");
    let result4 = engine.evaluate("let result = 0; outer: { result = 10; inner: { result = 20; break outer; result = 30; } result = 40; } result");
    println!("   Result: {:?}", result4);

    println!("\n=== Test Complete ===");
}
