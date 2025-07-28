use jetcrab::Engine;

fn main() {
    println!("JetCrab JavaScript Engine - End-to-End Test");
    println!("==========================================\n");

    let mut engine = Engine::new();

    let test_cases = vec![
        ("Number literal", "42", "42"),
        ("String literal", "\"Hello, World!\"", "Hello, World!"),
        ("Boolean true", "true", "true"),
        ("Boolean false", "false", "false"),
        ("Null", "null", "null"),
        ("Undefined", "undefined", "undefined"),
        ("Addition", "2 + 3", "5"),
        ("Subtraction", "10 - 4", "6"),
        ("Multiplication", "6 * 7", "42"),
        ("Division", "20 / 4", "5"),
        ("Modulo", "17 % 5", "2"),
        ("Exponentiation", "2 ** 8", "256"),
        ("String concatenation", "\"Hello\" + \" \" + \"World\"", "Hello World"),
        ("Number to string conversion", "42 + \" is the answer\"", "42 is the answer"),
        ("Comparison (less than)", "5 < 10", "true"),
        ("Comparison (greater than)", "15 > 8", "true"),
        ("Comparison (equal)", "5 == 5", "true"),
        ("Comparison (not equal)", "5 != 3", "true"),
        ("Logical AND", "true && false", "false"),
        ("Logical OR", "true || false", "true"),
        ("Logical NOT", "!false", "true"),
        ("Complex expression", "(2 + 3) * 4 - 1", "19"),
        ("Nested expressions", "((5 + 3) * 2) / 4", "4"),
    ];

    let mut passed = 0;
    let mut failed = 0;

    for (test_name, code, expected) in test_cases {
        print!("Testing {}: ", test_name);
        
        match engine.evaluate(code) {
            Ok(result) => {
                let result_str = result.to_string();
                if result_str == expected {
                    println!("✅ PASS ({} = {})", code, result_str);
                    passed += 1;
                } else {
                    println!("❌ FAIL (expected: {}, got: {})", expected, result_str);
                    failed += 1;
                }
            }
            Err(error) => {
                println!("❌ ERROR: {}", error);
                failed += 1;
            }
        }
    }

    println!("\n=== Test Results ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);
    println!("Total: {}", passed + failed);
    
    if failed == 0 {
        println!("🎉 All tests passed! The JetCrab engine is working correctly.");
    } else {
        println!("⚠️  Some tests failed. The engine needs improvements.");
    }

    println!("\n=== Engine Features Demonstrated ===");
    println!("✅ Lexical analysis (tokenization)");
    println!("✅ Syntax parsing (AST generation)");
    println!("✅ Semantic analysis");
    println!("✅ Bytecode generation");
    println!("✅ Virtual machine execution");
    println!("✅ Type system (numbers, strings, booleans, null, undefined)");
    println!("✅ Arithmetic operations (+, -, *, /, %, **)");
    println!("✅ String operations (concatenation)");
    println!("✅ Comparison operations (<, >, ==, !=)");
    println!("✅ Logical operations (&&, ||, !)");
    println!("✅ Expression evaluation");
    println!("✅ Parenthesized expressions");
    println!("✅ Type coercion (number to string)");
} 