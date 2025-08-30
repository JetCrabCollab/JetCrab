use std::fs;
use std::path::Path;
use jetcrab::api::Engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== JetCrab Sorting Algorithms Example ===\n");
    
    // Create a new JetCrab engine
    let mut engine = Engine::new();
    
    // Read the JavaScript file
    let js_path = Path::new("examples/sorting/sorting.js");
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
    
    // Test individual sorting functions
    println!("\n=== Testing Individual Sorting Functions ===");
    
    let test_cases = vec![
        "bubbleSort([3, 1, 4, 1, 5, 9, 2, 6])",
        "selectionSort([3, 1, 4, 1, 5, 9, 2, 6])",
        "insertionSort([3, 1, 4, 1, 5, 9, 2, 6])",
        "quickSort([3, 1, 4, 1, 5, 9, 2, 6])",
        "mergeSort([3, 1, 4, 1, 5, 9, 2, 6])"
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
    
    // Performance test with larger arrays
    println!("\n=== Performance Test with Larger Arrays ===");
    
    let performance_tests = vec![
        ("bubbleSort([5, 2, 8, 1, 9, 3, 7, 4, 6])", "Bubble Sort"),
        ("quickSort([5, 2, 8, 1, 9, 3, 7, 4, 6])", "Quick Sort"),
        ("mergeSort([5, 2, 8, 1, 9, 3, 7, 4, 6])", "Merge Sort")
    ];
    
    for (test_code, test_name) in performance_tests {
        println!("\nTesting {}:", test_name);
        let start = std::time::Instant::now();
        
        match engine.evaluate(test_code) {
            Ok(result) => {
                let duration = start.elapsed();
                println!("✅ Result: {:?}", result);
                println!("⏱️  Execution time: {:?}", duration);
            }
            Err(error) => {
                println!("❌ Error: {}", error);
            }
        }
    }
    
    // Test utility functions
    println!("\n=== Testing Utility Functions ===");
    
    let utility_tests = vec![
        "isSorted([1, 2, 3, 4, 5])",
        "isSorted([5, 4, 3, 2, 1])",
        "isSorted([1, 3, 2, 4, 5])"
    ];
    
    for test_case in utility_tests {
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
    
    Ok(())
}
