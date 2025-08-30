use jetcrab::api::Engine;

fn main() {
    println!("=== Test Call Stack System ===");

    let mut engine = Engine::new();

    println!("\n1. Testing simple function call:");
    let result1 = engine.evaluate("function test() { return 42; } test()");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing nested function calls:");
    let result2 = engine.evaluate("function outer() { function inner() { return 100; } return inner(); } outer()");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing recursive function:");
    let result3 = engine.evaluate("function countdown(n) { if (n <= 0) return 0; return n + countdown(n - 1); } countdown(5)");
    println!("   Result: {:?}", result3);

    println!("\n4. Testing multiple function calls:");
    let result4 = engine.evaluate("function add(a, b) { return a + b; } function multiply(x, y) { return x * y; } add(5, 3) + multiply(2, 4)");
    println!("   Result: {:?}", result4);

    println!("\n5. Testing function with local variables:");
    let result5 = engine.evaluate("function testLocal() { let x = 10; let y = 20; return x + y; } testLocal()");
    println!("   Result: {:?}", result5);

    println!("\n=== Test Complete ===");
}
