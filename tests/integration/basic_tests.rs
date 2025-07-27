use jetcrab::Engine;

#[test]
fn test_basic_number_literal() {
    let mut engine = Engine::new();
    let result = engine.evaluate("42");
    assert!(result.is_ok());
    if let Ok(value) = result {
        assert_eq!(value.to_string(), "42");
    }
}

#[test]
fn test_basic_string_literal() {
    let mut engine = Engine::new();
    let result = engine.evaluate("\"Hello, World!\"");
    assert!(result.is_ok());
    if let Ok(value) = result {
        assert_eq!(value.to_string(), "Hello, World!");
    }
}

#[test]
fn test_basic_addition() {
    let mut engine = Engine::new();
    let result = engine.evaluate("2 + 3");
    assert!(result.is_ok());
    if let Ok(value) = result {
        assert_eq!(value.to_string(), "5");
    }
}

#[test]
fn test_basic_multiplication() {
    let mut engine = Engine::new();
    let result = engine.evaluate("4 * 5");
    assert!(result.is_ok());
    if let Ok(value) = result {
        assert_eq!(value.to_string(), "20");
    }
}
