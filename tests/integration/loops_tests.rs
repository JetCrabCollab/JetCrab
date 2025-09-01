use jetcrab::api::Engine;

fn main() {
    println!("=== Test Loops Correction ===");

    let mut engine = Engine::new();

    println!("\n=== FOR LOOPS ===");

    println!("\n1. Testing simple for loop:");
    let result1 = engine.evaluate("let sum = 0; for (let i = 0; i < 5; i++) { sum += i; } sum");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing for loop with multiplication:");
    let result2 =
        engine.evaluate("let product = 1; for (let i = 1; i <= 5; i++) { product *= i; } product");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing for loop with string concatenation:");
    let result3 =
        engine.evaluate("let result = ''; for (let i = 0; i < 3; i++) { result += 'x'; } result");
    println!("   Result: {:?}", result3);

    println!("\n=== WHILE LOOPS ===");

    println!("\n4. Testing simple while loop:");
    let result4 =
        engine.evaluate("let count = 0; let i = 0; while (i < 5) { count += i; i++; } count");
    println!("   Result: {:?}", result4);

    println!("\n5. Testing while loop with condition:");
    let result5 =
        engine.evaluate("let sum = 0; let num = 10; while (num > 0) { sum += num; num--; } sum");
    println!("   Result: {:?}", result5);

    println!("\n6. Testing while loop with break condition:");
    let result6 = engine.evaluate(
        "let total = 0; let i = 1; while (true) { total += i; i++; if (i > 5) break; } total",
    );
    println!("   Result: {:?}", result6);

    println!("\n=== NESTED LOOPS ===");

    println!("\n7. Testing nested for loops:");
    let result7 = engine.evaluate("let sum = 0; for (let i = 0; i < 3; i++) { for (let j = 0; j < 3; j++) { sum += i + j; } } sum");
    println!("   Result: {:?}", result7);

    println!("\n8. Testing mixed loop types:");
    let result8 = engine.evaluate("let result = 0; for (let i = 0; i < 3; i++) { let j = 0; while (j < 3) { result += i * j; j++; } } result");
    println!("   Result: {:?}", result8);

    println!("\n=== LOOP WITH FUNCTIONS ===");

    println!("\n9. Testing loop with function calls:");
    let result9 = engine.evaluate("function add(x, y) { return x + y; } let sum = 0; for (let i = 0; i < 4; i++) { sum = add(sum, i); } sum");
    println!("   Result: {:?}", result9);

    println!("\n10. Testing loop with recursion:");
    let result10 = engine.evaluate("function factorial(n) { if (n <= 1) return 1; return n * factorial(n - 1); } let sum = 0; for (let i = 1; i <= 3; i++) { sum += factorial(i); } sum");
    println!("   Result: {:?}", result10);

    println!("\n=== Test Complete ===");
}
