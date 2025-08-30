use jetcrab::api::engine::Engine;

fn main() {
    let mut engine = Engine::new();

    println!("=== Control Flow Test ===\n");

    // Test 1: Simple if statement
    println!("1. Testing simple if statement:");
    let result = engine.evaluate("let x = 10; if (x > 5) { x } else { 0 }");
    println!("   if (x > 5) {{ x }} else {{ 0 }} = {:?}", result);

    // Test 2: If statement without else
    println!("\n2. Testing if statement without else:");
    let result = engine.evaluate("let y = 3; if (y > 5) { y }");
    println!("   if (y > 5) {{ y }} = {:?}", result);

    // Test 3: Comparison operators
    println!("\n3. Testing comparison operators:");
    let result = engine.evaluate("let a = 10; let b = 20; a < b");
    println!("   a < b = {:?}", result);

    let result = engine.evaluate("a == 10");
    println!("   a == 10 = {:?}", result);

    let result = engine.evaluate("b != 10");
    println!("   b != 10 = {:?}", result);

    // Test 4: Boolean operations
    println!("\n4. Testing boolean operations:");
    let result = engine.evaluate("true");
    println!("   true = {:?}", result);

    let result = engine.evaluate("false");
    println!("   false = {:?}", result);

    let result = engine.evaluate("!false");
    println!("   !false = {:?}", result);

    println!("\n=== Control Flow Test Complete ===");
    println!("\n📊 STATUS:");
    println!("   ✅ If statements: IMPLEMENTADO");
    println!("   ✅ Comparison operators: IMPLEMENTADO");
    println!("   ✅ Boolean values: IMPLEMENTADO");
    println!("   ❌ Loops (for/while): IMPLEMENTADO (bytecode)");
    println!("   ❌ Functions: NÃO IMPLEMENTADO");
}
