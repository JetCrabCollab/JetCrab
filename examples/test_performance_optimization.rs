use jetcrab::api::Engine;
use std::time::Instant;

fn main() {
    println!("=== Test Performance Optimization ===");

    let mut engine = Engine::new();

    println!("\n1. Testing loop performance:");
    let start1 = Instant::now();
    let result1 = engine.evaluate("let sum = 0; for (let i = 0; i < 1000; i++) { sum = sum + i; } sum");
    let duration1 = start1.elapsed();
    println!("   Result: {:?} (Time: {:?})", result1, duration1);

    println!("\n2. Testing function call performance:");
    let start2 = Instant::now();
    let result2 = engine.evaluate("function add(a, b) { return a + b; } let sum = 0; for (let i = 0; i < 1000; i++) { sum = add(sum, i); } sum");
    let duration2 = start2.elapsed();
    println!("   Result: {:?} (Time: {:?})", result2, duration2);

    println!("\n3. Testing array operation performance:");
    let start3 = Instant::now();
    let result3 = engine.evaluate("let arr = []; for (let i = 0; i < 100; i++) { arr.push(i); } arr.length");
    let duration3 = start3.elapsed();
    println!("   Result: {:?} (Time: {:?})", result3, duration3);

    println!("\n4. Testing object property access performance:");
    let start4 = Instant::now();
    let result4 = engine.evaluate("let obj = { a: 1, b: 2, c: 3, d: 4, e: 5 }; let sum = 0; for (let i = 0; i < 1000; i++) { sum = sum + obj.a + obj.b + obj.c + obj.d + obj.e; } sum");
    let duration4 = start4.elapsed();
    println!("   Result: {:?} (Time: {:?})", result4, duration4);

    println!("\n5. Testing recursive function performance:");
    let start5 = Instant::now();
    let result5 = engine.evaluate("function fibonacci(n) { if (n <= 1) return n; return fibonacci(n - 1) + fibonacci(n - 2); } fibonacci(20)");
    let duration5 = start5.elapsed();
    println!("   Result: {:?} (Time: {:?})", result5, duration5);

    println!("\n=== Test Complete ===");
}
