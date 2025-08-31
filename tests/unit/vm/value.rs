//! VM Value Unit Tests
//!
//! Tests for VM value system

use jetcrab::vm::value::Value;

#[test]
fn test_value_number() {
    let value = Value::Number(42.0);
    assert!(matches!(value, Value::Number(42.0)));
}

#[test]
fn test_value_string() {
    let value = Value::String("test".to_string());
    assert!(matches!(value, Value::String(ref s) if s == "test"));
}

#[test]
fn test_value_boolean() {
    let true_value = Value::Boolean(true);
    let false_value = Value::Boolean(false);
    assert!(matches!(true_value, Value::Boolean(true)));
    assert!(matches!(false_value, Value::Boolean(false)));
}

#[test]
fn test_value_null() {
    let value = Value::Null;
    assert!(matches!(value, Value::Null));
}

#[test]
fn test_value_undefined() {
    let value = Value::Undefined;
    assert!(matches!(value, Value::Undefined));
}

#[test]
fn test_value_object() {
    let value = Value::Object(1); // Assuming Object takes a handle ID
    assert!(matches!(value, Value::Object(_)));
}

#[test]
fn test_value_array() {
    let value = Value::Array(1); // Assuming Array takes a handle ID
    assert!(matches!(value, Value::Array(_)));
}

#[test]
fn test_value_function() {
    let value = Value::Function(1); // Assuming Function takes a handle ID
    assert!(matches!(value, Value::Function(_)));
}

#[test]
fn test_value_display() {
    let number = Value::Number(42.0);
    let string = Value::String("test".to_string());
    let boolean = Value::Boolean(true);
    let null = Value::Null;
    let undefined = Value::Undefined;

    assert_eq!(number.to_string(), "42");
    assert_eq!(string.to_string(), "test");
    assert_eq!(boolean.to_string(), "true");
    assert_eq!(null.to_string(), "null");
    assert_eq!(undefined.to_string(), "undefined");
}
