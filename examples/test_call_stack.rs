use jetcrab::api::Engine;

fn main() {
    println!("=== Test Call Stack ===");

    let mut engine = Engine::new();

    println!("\n1. Testing nested function calls:");
    let result1 = engine.evaluate("function outer() { function inner() { return 42; } return inner(); } outer()");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing function call depth:");
    let result2 = engine.evaluate("function level1() { return level2(); } function level2() { return level3(); } function level3() { return 100; } level1()");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing recursive calls with state:");
    let result3 = engine.evaluate("function counter(n) { if (n <= 0) return 0; return 1 + counter(n - 1); } counter(5)");
    println!("   Result: {:?}", result3);

    println!("\n4. Testing mutual recursion:");
    let result4 = engine.evaluate("function isEven(n) { if (n === 0) return true; if (n === 1) return false; return isOdd(n - 1); } function isOdd(n) { if (n === 0) return false; if (n === 1) return true; return isEven(n - 1); } isEven(4)");
    println!("   Result: {:?}", result4);

    println!("\n5. Testing function call with local variables:");
    let result5 = engine.evaluate("function test() { let x = 10; function helper() { let y = 20; return x + y; } return helper(); } test()");
    println!("   Result: {:?}", result5);

    println!("\n=== Test Complete ===");
}
