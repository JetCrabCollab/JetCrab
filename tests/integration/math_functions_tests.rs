use jetcrab::api::Engine;

#[test]
fn test_math_sqrt() {
    let mut engine = Engine::new();
    let result = engine.evaluate("Math.sqrt(16)");
    assert!(result.is_ok());
}

#[test]
fn test_math_max() {
    let mut engine = Engine::new();
    let result = engine.evaluate("Math.max(10, 20, 5)");
    assert!(result.is_ok());
}

#[test]
fn test_math_min() {
    let mut engine = Engine::new();
    let result = engine.evaluate("Math.min(10, 20, 5)");
    assert!(result.is_ok());
}
