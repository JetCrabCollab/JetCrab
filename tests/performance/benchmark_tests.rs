use jetcrab::api::Engine;
use std::time::Instant;

fn main() {
    println!("=== Test Benchmark Final ===");

    let mut engine = Engine::new();

    println!("\n=== BASIC OPERATIONS ===");

    println!("\n1. Testing arithmetic operations:");
    let result1 = engine.evaluate("let a = 10; let b = 5; a + b * 2 - 3 / 1");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing variables and assignment:");
    let result2 = engine.evaluate("let x = 100; let y = 200; x + y");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing string operations:");
    let result3 = engine.evaluate("let str1 = 'Hello'; let str2 = 'World'; str1 + ' ' + str2");
    println!("   Result: {:?}", result3);

    println!("\n=== CONTROL FLOW ===");

    println!("\n4. Testing for loops:");
    let result4 = engine.evaluate("let sum = 0; for (let i = 0; i < 10; i++) { sum += i; } sum");
    println!("   Result: {:?}", result4);

    println!("\n5. Testing while loops:");
    let result5 =
        engine.evaluate("let count = 0; let i = 0; while (i < 5) { count += i; i++; } count");
    println!("   Result: {:?}", result5);

    println!("\n6. Testing if statements:");
    let result6 =
        engine.evaluate("let value = 15; if (value > 10) { value * 2; } else { value / 2; }");
    println!("   Result: {:?}", result6);

    println!("\n=== FUNCTIONS ===");

    println!("\n7. Testing function declarations:");
    let result7 = engine.evaluate("function add(a, b) { return a + b; } add(10, 20)");
    println!("   Result: {:?}", result7);

    println!("\n8. Testing arrow functions:");
    let result8 = engine.evaluate("const multiply = (x, y) => x * y; multiply(6, 7)");
    println!("   Result: {:?}", result8);

    println!("\n9. Testing function expressions:");
    let result9 = engine.evaluate("const divide = function(a, b) { return a / b; }; divide(20, 4)");
    println!("   Result: {:?}", result9);

    println!("\n10. Testing recursion:");
    let result10 = engine.evaluate(
        "function factorial(n) { if (n <= 1) return 1; return n * factorial(n - 1); } factorial(5)",
    );
    println!("   Result: {:?}", result10);

    println!("\n=== ARRAYS ===");

    println!("\n11. Testing array creation:");
    let result11 = engine.evaluate("let arr = [1, 2, 3, 4, 5]; arr.length");
    println!("   Result: {:?}", result11);

    println!("\n12. Testing array access:");
    let result12 = engine.evaluate("let arr = [10, 20, 30]; arr[0] + arr[1] + arr[2]");
    println!("   Result: {:?}", result12);

    println!("\n13. Testing array methods:");
    let result13 = engine.evaluate("let arr = []; arr.push(100); arr.push(200); arr.length");
    println!("   Result: {:?}", result13);

    println!("\n=== OBJECTS ===");

    println!("\n14. Testing object creation:");
    let result14 = engine.evaluate("let obj = { a: 10, b: 20 }; obj");
    println!("   Result: {:?}", result14);

    println!("\n15. Testing object property access:");
    let result15 = engine.evaluate("let obj = { x: 5, y: 15 }; obj.x + obj.y");
    println!("   Result: {:?}", result15);

    println!("\n=== MATH FUNCTIONS ===");

    println!("\n16. Testing Math.pow:");
    let result16 = engine.evaluate("Math.pow(2, 8)");
    println!("   Result: {:?}", result16);

    println!("\n17. Testing Math.sqrt:");
    let result17 = engine.evaluate("Math.sqrt(64)");
    println!("   Result: {:?}", result17);

    println!("\n18. Testing Math.round:");
    let result18 = engine.evaluate("Math.round(9.7)");
    println!("   Result: {:?}", result18);

    println!("\n=== PERFORMANCE TEST ===");

    println!("\n19. Testing complex performance:");
    let start = Instant::now();
    let result19 = engine.evaluate(
        "let total = 0; for (let i = 0; i < 1000; i++) { total += Math.pow(i, 2); } total",
    );
    let duration = start.elapsed();
    println!("   Result: {:?} (Time: {:?})", result19, duration);

    println!("\n20. Testing recursive performance:");
    let start = Instant::now();
    let result20 = engine.evaluate(
        "function fib(n) { if (n <= 1) return n; return fib(n - 1) + fib(n - 2); } fib(20)",
    );
    let duration = start.elapsed();
    println!("   Result: {:?} (Time: {:?})", result20, duration);

    println!("\n=== BENCHMARK FINAL COMPLETE ===");
}
