use std::fs;
use std::path::Path;
use jetcrab::api::Engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== JetCrab Data Structures Example ===\n");
    
    // Create a new JetCrab engine
    let mut engine = Engine::new();
    
    // Read the JavaScript file
    let js_path = Path::new("examples/data_structures/data_structures.js");
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
    
    // Test individual data structure operations
    println!("\n=== Testing Individual Data Structure Operations ===");
    
    let test_cases = vec![
        "new Stack()",
        "new Queue()",
        "new LinkedList()",
        "new BinarySearchTree()",
        "new HashTable()"
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
    
    // Test Stack operations
    println!("\n=== Testing Stack Operations ===");
    let stack_tests = vec![
        "let s = new Stack(); s.push(1); s.push(2); s.toString()",
        "let s = new Stack(); s.push(1); s.push(2); s.pop()",
        "let s = new Stack(); s.push(1); s.push(2); s.peek()",
        "let s = new Stack(); s.push(1); s.push(2); s.size()"
    ];
    
    for test_case in stack_tests {
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
    
    // Test Queue operations
    println!("\n=== Testing Queue Operations ===");
    let queue_tests = vec![
        "let q = new Queue(); q.enqueue('A'); q.enqueue('B'); q.toString()",
        "let q = new Queue(); q.enqueue('A'); q.enqueue('B'); q.dequeue()",
        "let q = new Queue(); q.enqueue('A'); q.enqueue('B'); q.front()"
    ];
    
    for test_case in queue_tests {
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
    
    // Test LinkedList operations
    println!("\n=== Testing LinkedList Operations ===");
    let linkedlist_tests = vec![
        "let ll = new LinkedList(); ll.add(10); ll.add(20); ll.toString()",
        "let ll = new LinkedList(); ll.add(10); ll.add(20); ll.find(20)",
        "let ll = new LinkedList(); ll.add(10); ll.add(20); ll.remove(20); ll.toString()"
    ];
    
    for test_case in linkedlist_tests {
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
    
    // Test Binary Search Tree operations
    println!("\n=== Testing Binary Search Tree Operations ===");
    let bst_tests = vec![
        "let bst = new BinarySearchTree(); bst.insert(15); bst.insert(10); bst.insert(20); bst.search(15)",
        "let bst = new BinarySearchTree(); bst.insert(15); bst.insert(10); bst.insert(20); bst.search(99)"
    ];
    
    for test_case in bst_tests {
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
    
    // Test Hash Table operations
    println!("\n=== Testing Hash Table Operations ===");
    let hashtable_tests = vec![
        "let ht = new HashTable(); ht.set('name', 'John'); ht.get('name')",
        "let ht = new HashTable(); ht.set('age', 30); ht.set('city', 'NY'); ht.keys()"
    ];
    
    for test_case in hashtable_tests {
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
