use jetcrab::api::Engine;

#[test]
fn test_string_length() {
    let mut engine = Engine::new();
    let result = engine.evaluate("'Hello World'.length");
    assert!(result.is_ok());
}

#[test]
fn test_string_concatenation() {
    let mut engine = Engine::new();
    let result = engine.evaluate("'Hello' + ' ' + 'World'");
    assert!(result.is_ok());
}

#[test]
fn test_string_literal() {
    let mut engine = Engine::new();
    let result = engine.evaluate("'Hello'");
    assert!(result.is_ok());
}
