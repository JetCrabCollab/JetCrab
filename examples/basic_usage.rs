use jetcrab::Engine;

fn main() {
    println!("=== JetCrab Engine - Practical JavaScript Examples ===\n");

    let mut engine = Engine::new();

    // Example 1: Basic arithmetic and expressions
    println!("1. Basic Arithmetic:");
    run_example(&mut engine, "2 + 3", "Simple addition");
    run_example(&mut engine, "10 - 5 * 2", "Order of operations");
    run_example(&mut engine, "(20 + 10) / 3", "Parentheses and division");
    run_example(&mut engine, "Math.pow(2, 10)", "Power function");
    println!();

    // Example 2: String operations
    println!("2. String Manipulation:");
    run_example(&mut engine, "'Hello' + ' ' + 'World'", "String concatenation");
    run_example(&mut engine, "'JavaScript'.length", "String length property");
    // TODO: Fix string methods
    // run_example(&mut engine, "'hello world'.toUpperCase()", "String method");
    // run_example(&mut engine, "'  spaced  '.trim()", "String trimming");
    println!();

    // Example 3: Variable declarations and usage
    println!("3. Variables and Scope:");
    run_example(&mut engine, "let x = 42; x", "Variable declaration and retrieval");
    run_example(&mut engine, "const PI = 3.14159; PI * 2", "Constant declaration");
    run_example(&mut engine, "let a = 5; let b = 3; a + b", "Multiple variables");
    println!();

    // Example 4: Object creation and manipulation
    println!("4. Object Operations:");
    run_example(&mut engine, 
        "let person = { name: 'Alice', age: 25 }; person.name",
        "Object creation and property access"
    );
    run_example(&mut engine, 
        "let config = { timeout: 5000, retries: 3 }; config.timeout + config.retries",
        "Object with numeric properties"
    );
    println!();

    // Example 5: Array operations
    println!("5. Array Manipulation:");
    run_example(&mut engine, 
        "let numbers = [1, 2, 3, 4, 5]; numbers.length",
        "Array creation and length"
    );
    run_example(&mut engine, 
        "let fruits = ['apple', 'banana']; fruits.push('orange'); fruits",
        "Array modification"
    );
    run_example(&mut engine, 
        "let scores = [85, 92, 78, 96]; scores.reduce((sum, score) => sum + score, 0)",
        "Array reduction"
    );
    println!();

    // Example 6: Function definition and execution
    println!("6. Functions:");
    run_example(&mut engine, 
        "function greet(name) { return 'Hello, ' + name + '!'; } greet('World')",
        "Function definition and call"
    );
    run_example(&mut engine, 
        "let add = (a, b) => a + b; add(10, 20)",
        "Arrow function"
    );
    println!();

    // Example 7: Conditional logic
    println!("7. Conditional Logic:");
    run_example(&mut engine, 
        "let age = 18; age >= 18 ? 'Adult' : 'Minor'",
        "Ternary operator"
    );
    run_example(&mut engine, 
        "let score = 85; if (score >= 90) 'A'; else if (score >= 80) 'B'; else 'C'",
        "If-else chain"
    );
    println!();

    // Example 8: Error handling and edge cases
    println!("8. Error Handling:");
    run_example(&mut engine, "1 / 0", "Division by zero");
    run_example(&mut engine, "undefined + 5", "Undefined arithmetic");
    run_example(&mut engine, "null == undefined", "Null comparison");
    println!();

    // Example 9: Real-world calculation
    println!("9. Real-world Calculation:");
    run_example(&mut engine, 
        "let principal = 1000; let rate = 0.05; let time = 2; principal * Math.pow(1 + rate, time)",
        "Compound interest calculation"
    );
    println!();

    // Example 10: Data transformation
    println!("10. Data Transformation:");
    run_example(&mut engine, 
        "let data = [1, 2, 3, 4, 5]; data.map(x => x * 2).filter(x => x > 5).reduce((sum, x) => sum + x, 0)",
        "Data pipeline: map -> filter -> reduce"
    );
    println!();

    println!("=== Examples Complete ===");
    println!("These examples demonstrate practical usage of JetCrab for:");
    println!("✅ Mathematical calculations");
    println!("✅ String processing");
    println!("✅ Data manipulation");
    println!("✅ Object-oriented programming");
    println!("✅ Functional programming patterns");
    println!("✅ Real-world problem solving");
}

fn run_example(engine: &mut Engine, code: &str, description: &str) {
    println!("  {}: {}", description, code);
    match engine.evaluate(code) {
        Ok(result) => println!("    Result: {}", result),
        Err(e) => println!("    Error: {}", e),
    }
    println!();
}
