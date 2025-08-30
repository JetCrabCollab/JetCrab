use jetcrab::api::Engine;

fn main() {
    println!("=== Test Error Handling ===");

    let mut engine = Engine::new();

    println!("\n1. Testing division by zero:");
    let result1 = engine.evaluate("10 / 0");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing undefined variable access:");
    let result2 = engine.evaluate("undefinedVariable");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing function call with wrong arguments:");
    let result3 = engine.evaluate("function add(a, b) { return a + b; } add(5)");
    println!("   Result: {:?}", result3);

    println!("\n4. Testing array out of bounds access:");
    let result4 = engine.evaluate("let arr = [1, 2, 3]; arr[10]");
    println!("   Result: {:?}", result4);

    println!("\n5. Testing object property access on null:");
    let result5 = engine.evaluate("let obj = null; obj.property");
    println!("   Result: {:?}", result5);

    println!("\n=== Test Complete ===");
}
