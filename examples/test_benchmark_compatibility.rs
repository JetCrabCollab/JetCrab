use jetcrab::api::Engine;

fn main() {
    println!("=== Test Benchmark Compatibility ===");

    let mut engine = Engine::new();

    println!("\n1. Testing basic arithmetic operations:");
    let result1 = engine.evaluate("let a = 10; let b = 5; a + b * 2 - 3 / 1");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing string operations:");
    let result2 = engine.evaluate("let str1 = 'Hello'; let str2 = 'World'; str1 + ' ' + str2 + '!'.repeat(3)");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing array operations:");
    let result3 = engine.evaluate("let arr = [1, 2, 3]; arr.push(4); arr.push(5); arr.length + arr[0] + arr[arr.length - 1]");
    println!("   Result: {:?}", result3);

    println!("\n4. Testing object operations:");
    let result4 = engine.evaluate("let obj = { x: 10, y: 20 }; obj.z = 30; obj.x + obj.y + obj.z");
    println!("   Result: {:?}", result4);

    println!("\n5. Testing control flow:");
    let result5 = engine.evaluate("let sum = 0; for (let i = 0; i < 10; i++) { if (i % 2 === 0) { sum += i; } else { sum -= i; } } sum");
    println!("   Result: {:?}", result5);

    println!("\n=== Test Complete ===");
}
