use jetcrab::api::Engine;

fn main() {
    println!("=== Test Advanced Functions ===");

    let mut engine = Engine::new();

    println!("\n1. Testing function declaration:");

    println!("\n   Step 1: Test function declaration");
    let result1 = engine.evaluate("function add(a, b) { return a + b; } add(5, 3)");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing function expression:");

    println!("\n   Step 2: Test function expression");
    let result2 =
        engine.evaluate("let multiply = function(x, y) { return x * y; }; multiply(4, 6)");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing function with parameters:");

    println!("\n   Step 3: Test function with parameters");
    let result3 =
        engine.evaluate("function greet(name) { return 'Hello ' + name; } greet('World')");
    println!("   Result: {:?}", result3);

    println!("\n=== Test Complete ===");
}
