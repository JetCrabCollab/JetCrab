use jetcrab::api::Engine;

fn main() {
    println!("=== Test Garbage Collection ===");

    let mut engine = Engine::new();

    println!("\n1. Testing circular reference cleanup:");
    let result1 = engine.evaluate("let obj1 = {}; let obj2 = {}; obj1.ref = obj2; obj2.ref = obj1; obj1 = null; obj2 = null; 'cleaned'");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing function closure cleanup:");
    let result2 = engine.evaluate("function createClosure() { let data = 'large data'; return function() { return 'closure'; }; } let closure = createClosure(); closure()");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing array cleanup:");
    let result3 = engine.evaluate("let arr = [1, 2, 3, 4, 5]; arr = null; 'array cleaned'");
    println!("   Result: {:?}", result3);

    println!("\n4. Testing nested object cleanup:");
    let result4 = engine.evaluate(
        "let parent = { child: { grandchild: { data: 'deep' } } }; parent = null; 'nested cleaned'",
    );
    println!("   Result: {:?}", result4);

    println!("\n5. Testing memory leak prevention:");
    let result5 = engine.evaluate("let cache = {}; for (let i = 0; i < 100; i++) { cache[i] = 'data ' + i; } cache = null; 'cache cleaned'");
    println!("   Result: {:?}", result5);

    println!("\n=== Test Complete ===");
}
