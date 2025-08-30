use jetcrab::Engine;

fn main() {
    println!("=== JetCrab Engine - Real World Examples ===\n");

    let mut engine = Engine::new();

    // Example 1: Simple mathematical expressions
    println!("1. Mathematical Expressions:");
    evaluate_and_print(&mut engine, "2 + 3 * 4", "Basic arithmetic with precedence");
    evaluate_and_print(
        &mut engine,
        "(10 - 5) / 2 + 3",
        "Complex expression with parentheses",
    );
    evaluate_and_print(&mut engine, "Math.pow(2, 8)", "Built-in function call");
    println!();

    // Example 2: String manipulation
    println!("2. String Operations:");
    evaluate_and_print(
        &mut engine,
        "'Hello' + ' ' + 'World'",
        "String concatenation",
    );
    evaluate_and_print(&mut engine, "'JavaScript'.length", "String property access");
    evaluate_and_print(
        &mut engine,
        "'hello world'.toUpperCase()",
        "String method call",
    );
    println!();

    // Example 3: Variable usage and scope
    println!("3. Variables and Scope:");
    evaluate_and_print(
        &mut engine,
        "let x = 42; x * 2",
        "Variable declaration and usage",
    );
    evaluate_and_print(
        &mut engine,
        "const PI = 3.14159; PI * 2",
        "Constant declaration",
    );
    println!();

    // Example 4: Object creation and manipulation
    println!("4. Object Operations:");
    evaluate_and_print(
        &mut engine,
        "let person = { name: 'John', age: 30 }; person.name + ' is ' + person.age + ' years old'",
        "Object creation and property access",
    );
    println!();

    // Example 5: Array operations
    println!("5. Array Operations:");
    evaluate_and_print(
        &mut engine,
        "let numbers = [1, 2, 3, 4, 5]; numbers.reduce((a, b) => a + b, 0)",
        "Array creation and reduction",
    );
    println!();

    // Example 6: Function definition and execution
    println!("6. Function Definition:");
    evaluate_and_print(
        &mut engine,
        "function factorial(n) { return n <= 1 ? 1 : n * factorial(n - 1); } factorial(5)",
        "Recursive function definition and execution",
    );
    println!();

    // Example 7: Error handling demonstration
    println!("7. Error Handling:");
    evaluate_and_print(&mut engine, "1 / 0", "Division by zero error");
    evaluate_and_print(
        &mut engine,
        "undefined.property",
        "Property access on undefined",
    );
    println!();

    println!("=== Examples Complete ===");
    println!("These examples demonstrate real usage of the JetCrab JavaScript engine");
    println!("for practical JavaScript evaluation and execution.");
}

fn evaluate_and_print(engine: &mut Engine, code: &str, description: &str) {
    println!("  {}: {}", description, code);
    match engine.evaluate(code) {
        Ok(result) => println!("    Result: {}", result),
        Err(e) => println!("    Error: {}", e),
    }
    println!();
}
