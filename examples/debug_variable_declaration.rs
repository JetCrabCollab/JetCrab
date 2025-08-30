use jetcrab::api::Engine;

fn main() {
    println!("=== Debug Variable Declaration ===");

    let mut engine = Engine::new();

    println!("\n1. Testing variable declaration in for loop:");

    println!("\n   Step 1: Test simple variable declaration");
    let result1 = engine.evaluate("let x = 5");
    println!("   Result: {:?}", result1);

    println!("\n   Step 2: Check x value");
    let result2 = engine.evaluate("x");
    println!("   Result: {:?}", result2);

    println!("\n   Step 3: Test for loop with variable declaration");
    let result3 = engine.evaluate("for (let i = 0; i < 2; i++) { let temp = i; }");
    println!("   Result: {:?}", result3);

    println!("\n   Step 4: Check if i exists");
    let result4 = engine.evaluate("i");
    println!("   Result: {:?}", result4);

    println!("\n   Step 5: Check if temp exists");
    let result5 = engine.evaluate("temp");
    println!("   Result: {:?}", result5);

    println!("\n2. Testing variable scope:");

    println!("\n   Step 6: Test block scope");
    let result6 = engine.evaluate("{ let blockVar = 10; blockVar }");
    println!("   Result: {:?}", result6);

    println!("\n   Step 7: Check if blockVar exists outside block");
    let result7 = engine.evaluate("blockVar");
    println!("   Result: {:?}", result7);
}
