use jetcrab::vm::memory::heap::types::{HeapType, TypeInfo, TypeCategory};

#[test]
fn test_heap_type_creation() {
    let type_info = TypeInfo::new("test_type".to_string(), TypeCategory::Primitive);
    
    assert_eq!(type_info.name, "test_type");
    assert!(matches!(type_info.category, TypeCategory::Primitive));
}

#[test]
fn test_heap_type_categories() {
    let primitive = TypeInfo::new("primitive".to_string(), TypeCategory::Primitive);
    let object = TypeInfo::new("object".to_string(), TypeCategory::Object);
    let array = TypeInfo::new("array".to_string(), TypeCategory::Array);
    let function = TypeInfo::new("function".to_string(), TypeCategory::Function);
    
    assert!(matches!(primitive.category, TypeCategory::Primitive));
    assert!(matches!(object.category, TypeCategory::Object));
    assert!(matches!(array.category, TypeCategory::Array));
    assert!(matches!(function.category, TypeCategory::Function));
}

#[test]
fn test_heap_type_size() {
    let mut type_info = TypeInfo::new("test".to_string(), TypeCategory::Primitive);
    
    assert_eq!(type_info.size, 0);
    
    type_info.size = 64;
    assert_eq!(type_info.size, 64);
}

#[test]
fn test_heap_type_metadata() {
    let mut type_info = TypeInfo::new("test".to_string(), TypeCategory::Object);
    
    type_info.metadata.insert("version".to_string(), "1.0".to_string());
    type_info.metadata.insert("author".to_string(), "test".to_string());
    
    assert_eq!(type_info.metadata.len(), 2);
    assert_eq!(type_info.metadata.get("version"), Some(&"1.0".to_string()));
    assert_eq!(type_info.metadata.get("author"), Some(&"test".to_string()));
}

#[test]
fn test_heap_type_clone() {
    let mut original = TypeInfo::new("original".to_string(), TypeCategory::Array);
    original.size = 128;
    original.metadata.insert("key".to_string(), "value".to_string());
    
    let cloned = original.clone();
    
    assert_eq!(original.name, cloned.name);
    assert_eq!(original.category, cloned.category);
    assert_eq!(original.size, cloned.size);
    assert_eq!(original.metadata.len(), cloned.metadata.len());
}

#[test]
fn test_heap_type_debug() {
    let type_info = TypeInfo::new("debug_type".to_string(), TypeCategory::Function);
    
    let debug_str = format!("{:?}", type_info);
    assert!(debug_str.contains("debug_type"));
    assert!(debug_str.contains("Function"));
}

#[test]
fn test_heap_type_equality() {
    let type1 = TypeInfo::new("same_type".to_string(), TypeCategory::Primitive);
    let type2 = TypeInfo::new("same_type".to_string(), TypeCategory::Primitive);
    let type3 = TypeInfo::new("different_type".to_string(), TypeCategory::Primitive);
    
    assert_eq!(type1, type2);
    assert_ne!(type1, type3);
}

#[test]
fn test_heap_type_default() {
    let type_info = TypeInfo::default();
    
    assert_eq!(type_info.name, "");
    assert!(matches!(type_info.category, TypeCategory::Primitive));
    assert_eq!(type_info.size, 0);
    assert!(type_info.metadata.is_empty());
}
