use jetcrab::api::Engine;

fn main() {
    println!("=== Test Try-Catch Statement ===");

    let mut engine = Engine::new();

    println!("\n1. Testing simple try-catch:");

    println!("\n   Step 1: Test try-catch without error");
    let result1 =
        engine.evaluate("let result = 0; try { result = 10; } catch (e) { result = 20; } result");
    println!("   Result: {:?}", result1);

    println!("\n2. Testing try-catch with finally:");

    println!("\n   Step 2: Test try-catch with finally");
    let result2 = engine.evaluate("let result = 0; try { result = 10; } catch (e) { result = 20; } finally { result = 30; } result");
    println!("   Result: {:?}", result2);

    println!("\n3. Testing throw statement:");

    println!("\n   Step 3: Test throw statement");
    let result3 =
        engine.evaluate("let result = 0; try { throw 'error'; } catch (e) { result = 50; } result");
    println!("   Result: {:?}", result3);

    println!("\n=== Test Complete ===");
}
