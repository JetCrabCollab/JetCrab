//! VM Value Unit Tests
//!
//! Tests for VM value system

use jetcrab::vm::value::Value;

#[test]
fn test_value_creation() {
    let null_value = Value::Null;
    let bool_value = Value::Boolean(true);
    let number_value = Value::Number(42.0);
    let string_value = Value::String("hello".to_string());
    
    assert!(matches!(null_value, Value::Null));
    assert!(matches!(bool_value, Value::Boolean(true)));
    assert!(matches!(number_value, Value::Number(42.0)));
    assert!(matches!(string_value, Value::String(ref s) if s == "hello"));
}

#[test]
fn test_value_size() {
    let null_value = Value::Null;
    let bool_value = Value::Boolean(true);
    let number_value = Value::Number(42.0);
    let string_value = Value::String("hello".to_string());
    
    assert_eq!(null_value.size(), 0);
    assert_eq!(bool_value.size(), 1);
    assert_eq!(number_value.size(), 8);
    assert_eq!(string_value.size(), 5);
}

#[test]
fn test_value_clone() {
    let original = Value::String("test".to_string());
    let cloned = original.clone();
    
    assert_eq!(original, cloned);
    assert_eq!(original.size(), cloned.size());
}

#[test]
fn test_value_debug() {
    let value = Value::Number(3.14);
    let debug_str = format!("{:?}", value);
    
    assert!(debug_str.contains("Number"));
    assert!(debug_str.contains("3.14"));
}
