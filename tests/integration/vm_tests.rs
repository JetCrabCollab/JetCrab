use jetcrab::api::Engine;

#[test]
fn test_execute_basic_arithmetic() {
    let mut engine = Engine::new();
    let result = engine.evaluate("2 + 3");
    assert!(result.is_ok());
}

#[test]
fn test_execute_basic_comparison() {
    let mut engine = Engine::new();
    let result = engine.evaluate("5 > 3");
    assert!(result.is_ok());
}

#[test]
fn test_execute_basic_string() {
    let mut engine = Engine::new();
    let result = engine.evaluate("\"hello\"");
    assert!(result.is_ok());
}

#[test]
fn test_execute_basic_number() {
    let mut engine = Engine::new();
    let result = engine.evaluate("42");
    assert!(result.is_ok());
}

#[test]
fn test_execute_basic_boolean() {
    let mut engine = Engine::new();
    let result = engine.evaluate("true");
    assert!(result.is_ok());
}
