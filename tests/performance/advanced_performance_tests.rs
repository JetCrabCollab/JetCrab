use jetcrab::api::Engine;
use std::time::Instant;

fn main() {
    println!("=== Test Advanced Performance ===");

    let mut engine = Engine::new();

    println!("\n1. Testing complex loop performance:");
    let start1 = Instant::now();
    let result1 = engine.evaluate("let total = 0; for (let i = 0; i < 10000; i++) { for (let j = 0; j < 10; j++) { total += i * j; } } total");
    let duration1 = start1.elapsed();
    println!("   Result: {:?} (Time: {:?})", result1, duration1);

    println!("\n2. Testing recursive performance:");
    let start2 = Instant::now();
    let result2 = engine.evaluate("function fibonacci(n) { if (n <= 1) return n; return fibonacci(n - 1) + fibonacci(n - 2); } fibonacci(25)");
    let duration2 = start2.elapsed();
    println!("   Result: {:?} (Time: {:?})", result2, duration2);

    println!("\n3. Testing array operation performance:");
    let start3 = Instant::now();
    let result3 = engine.evaluate("let arr = []; for (let i = 0; i < 5000; i++) { arr.push(i * 2); } let sum = 0; for (let i = 0; i < arr.length; i++) { sum += arr[i]; } sum");
    let duration3 = start3.elapsed();
    println!("   Result: {:?} (Time: {:?})", result3, duration3);

    println!("\n4. Testing object creation performance:");
    let start4 = Instant::now();
    let result4 = engine.evaluate("let objects = []; for (let i = 0; i < 1000; i++) { objects.push({ id: i, value: i * 10, name: 'obj' + i }); } objects.length");
    let duration4 = start4.elapsed();
    println!("   Result: {:?} (Time: {:?})", result4, duration4);

    println!("\n5. Testing complex calculation performance:");
    let start5 = Instant::now();
    let result5 = engine.evaluate("function complex(n) { let result = 0; for (let i = 0; i < n; i++) { result += Math.pow(i, 2) * Math.sqrt(i + 1); } return result; } complex(1000)");
    let duration5 = start5.elapsed();
    println!("   Result: {:?} (Time: {:?})", result5, duration5);

    println!("\n=== Test Complete ===");
}
