use jetcrab::vm::memory::heap::optimized_arrays::{OptimizedArray, ArrayType, ArrayConfig};

#[test]
fn test_optimized_array_creation() {
    let config = ArrayConfig::default();
    let array = OptimizedArray::new(ArrayType::Number, 100, config);
    
    assert!(matches!(array.array_type, ArrayType::Number));
    assert_eq!(array.capacity, 100);
    assert_eq!(array.length, 0);
}

#[test]
fn test_optimized_array_push() {
    let mut array = OptimizedArray::new(ArrayType::Number, 10, ArrayConfig::default());
    
    array.push_number(42.0);
    array.push_number(3.14);
    
    assert_eq!(array.length, 2);
    assert_eq!(array.get_number(0), Some(42.0));
    assert_eq!(array.get_number(1), Some(3.14));
}

#[test]
fn test_optimized_array_string_type() {
    let mut array = OptimizedArray::new(ArrayType::String, 5, ArrayConfig::default());
    
    array.push_string("hello".to_string());
    array.push_string("world".to_string());
    
    assert_eq!(array.length, 2);
    assert_eq!(array.get_string(0), Some("hello".to_string()));
    assert_eq!(array.get_string(1), Some("world".to_string()));
}

#[test]
fn test_optimized_array_boolean_type() {
    let mut array = OptimizedArray::new(ArrayType::Boolean, 3, ArrayConfig::default());
    
    array.push_boolean(true);
    array.push_boolean(false);
    array.push_boolean(true);
    
    assert_eq!(array.length, 3);
    assert_eq!(array.get_boolean(0), Some(true));
    assert_eq!(array.get_boolean(1), Some(false));
    assert_eq!(array.get_boolean(2), Some(true));
}

#[test]
fn test_optimized_array_capacity() {
    let array = OptimizedArray::new(ArrayType::Number, 50, ArrayConfig::default());
    
    assert_eq!(array.capacity, 50);
    assert!(array.has_capacity());
}

#[test]
fn test_optimized_array_resize() {
    let mut array = OptimizedArray::new(ArrayType::Number, 5, ArrayConfig::default());
    
    assert_eq!(array.capacity, 5);
    
    array.resize(10);
    assert_eq!(array.capacity, 10);
}

#[test]
fn test_optimized_array_clear() {
    let mut array = OptimizedArray::new(ArrayType::Number, 10, ArrayConfig::default());
    
    array.push_number(42.0);
    array.push_number(3.14);
    
    assert_eq!(array.length, 2);
    
    array.clear();
    assert_eq!(array.length, 0);
}

#[test]
fn test_array_config_default() {
    let config = ArrayConfig::default();
    
    assert_eq!(config.initial_capacity, 16);
    assert_eq!(config.growth_factor, 2.0);
    assert!(config.enable_compaction);
}

#[test]
fn test_array_config_custom() {
    let config = ArrayConfig::new()
        .with_initial_capacity(32)
        .with_growth_factor(1.5)
        .with_compaction(false);
    
    assert_eq!(config.initial_capacity, 32);
    assert_eq!(config.growth_factor, 1.5);
    assert!(!config.enable_compaction);
}

#[test]
fn test_array_type_variants() {
    let number_type = ArrayType::Number;
    let string_type = ArrayType::String;
    let boolean_type = ArrayType::Boolean;
    let mixed_type = ArrayType::Mixed;
    
    assert!(matches!(number_type, ArrayType::Number));
    assert!(matches!(string_type, ArrayType::String));
    assert!(matches!(boolean_type, ArrayType::Boolean));
    assert!(matches!(mixed_type, ArrayType::Mixed));
}
