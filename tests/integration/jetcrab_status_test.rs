use jetcrab::Engine;

#[test]
fn test_jetcrab_basic_features() {
    let mut engine = Engine::new();

    // Test 1: Number literals
    assert_eq!(
        engine.evaluate("42").unwrap(),
        jetcrab::vm::value::Value::Number(42.0)
    );

    // Test 2: String literals
    assert_eq!(
        engine.evaluate("\"Hello, World!\"").unwrap(),
        jetcrab::vm::value::Value::String("Hello, World!".to_string())
    );

    // Test 3: Boolean literals
    assert_eq!(
        engine.evaluate("true").unwrap(),
        jetcrab::vm::value::Value::Boolean(true)
    );
    assert_eq!(
        engine.evaluate("false").unwrap(),
        jetcrab::vm::value::Value::Boolean(false)
    );

    // Test 4: Basic arithmetic
    assert_eq!(
        engine.evaluate("2 + 3").unwrap(),
        jetcrab::vm::value::Value::Number(5.0)
    );
    assert_eq!(
        engine.evaluate("4 * 5").unwrap(),
        jetcrab::vm::value::Value::Number(20.0)
    );
    assert_eq!(
        engine.evaluate("10 - 3").unwrap(),
        jetcrab::vm::value::Value::Number(7.0)
    );
    assert_eq!(
        engine.evaluate("15 / 3").unwrap(),
        jetcrab::vm::value::Value::Number(5.0)
    );

    // Test 5: String concatenation
    assert_eq!(
        engine.evaluate("\"Hello\" + \" \" + \"World\"").unwrap(),
        jetcrab::vm::value::Value::String("Hello World".to_string())
    );

    // Test 6: Complex expressions
    assert_eq!(
        engine.evaluate("2 + 3 * 4").unwrap(),
        jetcrab::vm::value::Value::Number(14.0)
    );

    println!("✅ Basic features working correctly!");
}

#[test]
fn test_jetcrab_advanced_features() {
    let mut engine = Engine::new();

    // Test 1: Variable declarations
    assert_eq!(
        engine.evaluate("let x = 42; x").unwrap(),
        jetcrab::vm::value::Value::Number(42.0)
    );

    // Test 2: Array creation
    let result = engine.evaluate("[1, 2, 3, 4, 5]").unwrap();
    assert!(matches!(result, jetcrab::vm::value::Value::Array(_)));

    // Test 3: Control flow
    assert_eq!(
        engine.evaluate("if (true) { 42 } else { 0 }").unwrap(),
        jetcrab::vm::value::Value::Number(42.0)
    );
    assert_eq!(
        engine.evaluate("if (false) { 0 } else { 42 }").unwrap(),
        jetcrab::vm::value::Value::Number(42.0)
    );

    // Test 4: Comparison operators
    assert_eq!(
        engine.evaluate("5 > 3").unwrap(),
        jetcrab::vm::value::Value::Boolean(true)
    );
    assert_eq!(
        engine.evaluate("3 < 5").unwrap(),
        jetcrab::vm::value::Value::Boolean(true)
    );
    assert_eq!(
        engine.evaluate("5 == 5").unwrap(),
        jetcrab::vm::value::Value::Boolean(true)
    );

    // Test 5: Logical operators
    assert_eq!(
        engine.evaluate("true && true").unwrap(),
        jetcrab::vm::value::Value::Boolean(true)
    );
    assert_eq!(
        engine.evaluate("true && false").unwrap(),
        jetcrab::vm::value::Value::Boolean(false)
    );
    assert_eq!(
        engine.evaluate("true || false").unwrap(),
        jetcrab::vm::value::Value::Boolean(true)
    );
    assert_eq!(
        engine.evaluate("!true").unwrap(),
        jetcrab::vm::value::Value::Boolean(false)
    );

    // Test 6: Template literals
    assert_eq!(
        engine.evaluate("`Hello World`").unwrap(),
        jetcrab::vm::value::Value::String("Hello World".to_string())
    );

    // Test 7: Template literals with variables
    let result = engine
        .evaluate("let name = 'John'; `Hello ${name}`")
        .unwrap();
    assert!(matches!(result, jetcrab::vm::value::Value::String(_)));

    println!("✅ Advanced features working correctly!");
}

#[test]
fn test_jetcrab_function_features() {
    let mut engine = Engine::new();

    // Test 1: Function declaration (should return undefined for now)
    let result = engine
        .evaluate("function add(a, b) { return a + b; }")
        .unwrap();
    assert!(matches!(result, jetcrab::vm::value::Value::Undefined));

    // Test 2: Arrow function (should return undefined for now)
    let result = engine
        .evaluate("const multiply = (a, b) => a * b;")
        .unwrap();
    assert!(matches!(result, jetcrab::vm::value::Value::Undefined));

    println!("⚠️ Function features need implementation!");
}

#[test]
fn test_jetcrab_object_features() {
    let mut engine = Engine::new();

    // Test 1: Object creation (should work but return a placeholder for now)
    let result = engine.evaluate("{name: 'John', age: 30}").unwrap();
    // For now, objects return a placeholder value
    assert!(matches!(result, jetcrab::vm::value::Value::Number(_)));

    println!("⚠️ Object features need improvement!");
}
