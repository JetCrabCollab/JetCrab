use jetcrab::api::Engine;

fn main() {
    println!("=== Test Function Expressions ===");

    let mut engine = Engine::new();

    println!("\n1. Testing function expression:");
    let result1 = engine.evaluate("const foo = function() { return 100; }; foo()");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing function expression with parameters:");
    let result2 = engine.evaluate("const add = function(a, b) { return a + b; }; add(5, 3)");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing arrow function:");
    let result3 = engine.evaluate("const bar = () => 200; bar()");
    println!("   Result: {:?}", result3);

    println!("\n4. Testing arrow function with parameters:");
    let result4 = engine.evaluate("const multiply = (x, y) => x * y; multiply(4, 6)");
    println!("   Result: {:?}", result4);

    println!("\n5. Testing function assignment:");
    let result5 = engine.evaluate("let func = function() { return 42; }; func()");
    println!("   Result: {:?}", result5);

    println!("\n=== Test Complete ===");
}
