use jetcrab::api::Engine;

fn main() {
    println!("=== Test Memory Management ===");

    let mut engine = Engine::new();

    println!("\n1. Testing variable cleanup:");
    let result1 = engine.evaluate("let x = 100; { let y = 200; } x");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing function memory cleanup:");
    let result2 = engine.evaluate("function test() { let temp = 42; return temp; } test()");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing nested scope cleanup:");
    let result3 = engine.evaluate("let outer = 10; { let inner = 20; { let deep = 30; } } outer");
    println!("   Result: {:?}", result3);

    println!("\n4. Testing array memory management:");
    let result4 = engine.evaluate("let arr = [1, 2, 3, 4, 5]; arr.length");
    println!("   Result: {:?}", result4);

    println!("\n5. Testing object memory management:");
    let result5 = engine.evaluate("let obj = { a: 1, b: 2, c: 3 }; obj.a + obj.b");
    println!("   Result: {:?}", result5);

    println!("\n=== Test Complete ===");
}
