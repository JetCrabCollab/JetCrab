use jetcrab::Engine;

fn main() {
    println!("JetCrab JavaScript Engine");
    println!("========================\n");

    let mut engine = Engine::new();

    println!("Number literal: 42");
    match engine.evaluate("42") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("String literal: \"Hello, World!\"");
    match engine.evaluate("\"Hello, World!\"") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("Addition: 2 + 3");
    match engine.evaluate("2 + 3") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("Multiplication: 4 * 5");
    match engine.evaluate("4 * 5") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("Subtraction: 10 - 3");
    match engine.evaluate("10 - 3") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("Division: 15 / 3");
    match engine.evaluate("15 / 3") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();
}
