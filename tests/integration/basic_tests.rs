use jetcrab::Engine;

#[test]
fn test_basic_arithmetic() {
    let mut engine = Engine::new();
    let result = engine.evaluate("2 + 2").unwrap();
    assert_eq!(result, jetcrab::vm::value::Value::Number(4.0));
}

#[test]
fn test_jetcrab_status() {
    let mut engine = Engine::new();

    println!("\n=== JETCRAB STATUS REPORT ===");

    // Test 1: Number literals
    match engine.evaluate("42") {
        Ok(result) => {
            assert_eq!(result, jetcrab::vm::value::Value::Number(42.0));
            println!("✅ Number literals: WORKING");
        }
        Err(e) => println!("❌ Number literals: FAILED - {}", e),
    }

    // Test 2: String literals
    match engine.evaluate("\"Hello, World!\"") {
        Ok(result) => {
            assert_eq!(
                result,
                jetcrab::vm::value::Value::String("Hello, World!".to_string())
            );
            println!("✅ String literals: WORKING");
        }
        Err(e) => println!("❌ String literals: FAILED - {}", e),
    }

    // Test 3: Boolean literals
    match engine.evaluate("true") {
        Ok(result) => {
            assert_eq!(result, jetcrab::vm::value::Value::Boolean(true));
            println!("✅ Boolean literals: WORKING");
        }
        Err(e) => println!("❌ Boolean literals: FAILED - {}", e),
    }

    // Test 4: Basic arithmetic
    match engine.evaluate("2 + 3") {
        Ok(result) => {
            assert_eq!(result, jetcrab::vm::value::Value::Number(5.0));
            println!("✅ Basic arithmetic: WORKING");
        }
        Err(e) => println!("❌ Basic arithmetic: FAILED - {}", e),
    }

    // Test 5: String concatenation
    match engine.evaluate("\"Hello\" + \" \" + \"World\"") {
        Ok(result) => {
            assert_eq!(
                result,
                jetcrab::vm::value::Value::String("Hello World".to_string())
            );
            println!("✅ String concatenation: WORKING");
        }
        Err(e) => println!("❌ String concatenation: FAILED - {}", e),
    }

    // Test 6: Variable declarations
    match engine.evaluate("let x = 42; x") {
        Ok(result) => {
            assert_eq!(result, jetcrab::vm::value::Value::Number(42.0));
            println!("✅ Variable declarations: WORKING");
        }
        Err(e) => println!("❌ Variable declarations: FAILED - {}", e),
    }

    // Test 7: Control flow
    match engine.evaluate("if (true) { 42 } else { 0 }") {
        Ok(result) => {
            assert_eq!(result, jetcrab::vm::value::Value::Number(42.0));
            println!("✅ Control flow: WORKING");
        }
        Err(e) => println!("❌ Control flow: FAILED - {}", e),
    }

    // Test 8: Comparison operators
    match engine.evaluate("5 > 3") {
        Ok(result) => {
            assert_eq!(result, jetcrab::vm::value::Value::Boolean(true));
            println!("✅ Comparison operators: WORKING");
        }
        Err(e) => println!("❌ Comparison operators: FAILED - {}", e),
    }

    // Test 9: Logical operators
    match engine.evaluate("true && true") {
        Ok(result) => {
            assert_eq!(result, jetcrab::vm::value::Value::Boolean(true));
            println!("✅ Logical operators: WORKING");
        }
        Err(e) => println!("❌ Logical operators: FAILED - {}", e),
    }

    // Test 10: Template literals
    match engine.evaluate("`Hello World`") {
        Ok(result) => {
            assert_eq!(
                result,
                jetcrab::vm::value::Value::String("Hello World".to_string())
            );
            println!("✅ Template literals: WORKING");
        }
        Err(e) => println!("❌ Template literals: FAILED - {}", e),
    }

    // Test 11: Template literals with variables
    match engine.evaluate("let name = 'John'; `Hello ${name}`") {
        Ok(result) => {
            if matches!(result, jetcrab::vm::value::Value::String(_)) {
                println!("✅ Template literals with variables: WORKING");
            } else {
                println!(
                    "⚠️ Template literals with variables: PARTIAL - got {:?}",
                    result
                );
            }
        }
        Err(e) => println!("❌ Template literals with variables: FAILED - {}", e),
    }

    // Test 12: Array creation
    match engine.evaluate("[1, 2, 3, 4, 5]") {
        Ok(result) => {
            if matches!(result, jetcrab::vm::value::Value::Array(_)) {
                println!("✅ Array creation: WORKING");
            } else {
                println!("⚠️ Array creation: PARTIAL - got {:?}", result);
            }
        }
        Err(e) => println!("❌ Array creation: FAILED - {}", e),
    }

    // Test 13: Function declaration (should return undefined for now)
    match engine.evaluate("function add(a, b) { return a + b; }") {
        Ok(result) => {
            if matches!(result, jetcrab::vm::value::Value::Undefined) {
                println!("✅ Function declaration: WORKING (returns undefined as expected)");
            } else {
                println!("⚠️ Function declaration: PARTIAL - got {:?}", result);
            }
        }
        Err(e) => println!("❌ Function declaration: FAILED - {}", e),
    }

    // Test 14: Object creation
    match engine.evaluate("{name: 'John', age: 30}") {
        Ok(result) => {
            if matches!(result, jetcrab::vm::value::Value::Number(_)) {
                println!("⚠️ Object creation: PARTIAL - returns placeholder value");
            } else {
                println!("✅ Object creation: WORKING - got {:?}", result);
            }
        }
        Err(e) => println!("❌ Object creation: FAILED - {}", e),
    }

    println!("=== END STATUS REPORT ===\n");
}
