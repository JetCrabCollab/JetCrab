use jetcrab::api::Engine;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== JetCrab Fibonacci Example ===\n");

    // Create a new JetCrab engine
    let mut engine = Engine::new();

    // Read the JavaScript file
    let js_path = Path::new("examples/fibonacci/fibonacci.js");
    let js_code = fs::read_to_string(js_path)?;

    println!("Executing JavaScript code from: {}", js_path.display());
    println!("Code length: {} characters\n", js_code.len());

    // Execute the JavaScript code
    match engine.evaluate(&js_code) {
        Ok(result) => {
            println!("✅ Execution completed successfully!");
            println!("Final result: {:?}", result);
        }
        Err(error) => {
            println!("❌ Execution failed with error:");
            println!("{}", error);
        }
    }

    // Test individual Fibonacci calculations
    println!("\n=== Testing Individual Fibonacci Calculations ===");

    let test_cases = vec![
        "fibonacciRecursive(5)",
        "fibonacciIterative(5)",
        "fibonacciMemo(5)",
        "generateFibonacciSequence(5)",
    ];

    for test_case in test_cases {
        println!("\nTesting: {}", test_case);
        match engine.evaluate(test_case) {
            Ok(result) => {
                println!("✅ Result: {:?}", result);
            }
            Err(error) => {
                println!("❌ Error: {}", error);
            }
        }
    }

    // Performance test
    println!("\n=== Performance Test ===");
    let performance_test = "fibonacciIterative(30)";
    println!("Testing: {}", performance_test);

    let start = std::time::Instant::now();
    match engine.evaluate(performance_test) {
        Ok(result) => {
            let duration = start.elapsed();
            println!("✅ Result: {:?}", result);
            println!("⏱️  Execution time: {:?}", duration);
        }
        Err(error) => {
            println!("❌ Error: {}", error);
        }
    }

    Ok(())
}
