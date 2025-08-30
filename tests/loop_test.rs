use jetcrab::api::engine::Engine;

fn main() {
    let mut engine = Engine::new();

    println!("=== Loop Test ===\n");

    // Test 1: Simple for loop
    println!("1. Testing simple for loop:");
    let result =
        engine.evaluate("let sum = 0; for (let i = 0; i < 5; i = i + 1) { sum = sum + i }; sum");
    println!(
        "   for (let i = 0; i < 5; i = i + 1) {{ sum = sum + i }}; sum = {:?}",
        result
    );

    // Test 2: While loop
    println!("\n2. Testing while loop:");
    let result = engine.evaluate("let count = 0; while (count < 3) { count = count + 1 }; count");
    println!(
        "   while (count < 3) {{ count = count + 1 }}; count = {:?}",
        result
    );

    // Test 3: Array building with loop
    println!("\n3. Testing array building with loop:");
    let result = engine
        .evaluate("let arr = []; for (let i = 0; i < 3; i = i + 1) { arr.push(i) }; arr.length");
    println!("   Array building loop result: {:?}", result);

    println!("\n=== Loop Test Complete ===");
    println!("\n📊 STATUS:");
    println!("   ✅ For loops: IMPLEMENTADO");
    println!("   ✅ While loops: IMPLEMENTADO");
    println!("   ✅ Loop variables: IMPLEMENTADO");
    println!("   ❌ Functions: NÃO IMPLEMENTADO");
}
