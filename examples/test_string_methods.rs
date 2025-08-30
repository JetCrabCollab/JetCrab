use jetcrab::api::Engine;

fn main() {
    println!("=== Test String Methods ===");

    let mut engine = Engine::new();

    println!("\n1. Testing string repeat method:");
    let result1 = engine.evaluate("'Hello'.repeat(3)");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing string length property:");
    let result2 = engine.evaluate("'Hello World'.length");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing string concatenation:");
    let result3 = engine.evaluate("let str1 = 'Hello'; let str2 = 'World'; str1 + ' ' + str2");
    println!("   Result: {:?}", result3);

    println!("\n4. Testing string repeat with variable:");
    let result4 = engine.evaluate("let str = 'Hi'; let count = 5; str.repeat(count)");
    println!("   Result: {:?}", result4);

    println!("\n5. Testing string repeat with expression:");
    let result5 = engine.evaluate("'*'.repeat(2 + 3)");
    println!("   Result: {:?}", result5);

    println!("\n=== Test Complete ===");
}
