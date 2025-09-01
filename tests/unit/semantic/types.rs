use jetcrab::semantic::types::{Type, TypeKind, TypeInfo};

#[test]
fn test_type_creation() {
    let type_info = TypeInfo::new(
        TypeKind::Number,
        "number".to_string(),
        Some("A numeric type".to_string()),
    );
    
    assert!(matches!(type_info.kind, TypeKind::Number));
    assert_eq!(type_info.name, "number");
    assert_eq!(type_info.description, Some("A numeric type".to_string()));
}

#[test]
fn test_type_kinds() {
    let number_type = TypeInfo::new(TypeKind::Number, "number".to_string(), None);
    let string_type = TypeInfo::new(TypeKind::String, "string".to_string(), None);
    let boolean_type = TypeInfo::new(TypeKind::Boolean, "boolean".to_string(), None);
    let object_type = TypeInfo::new(TypeKind::Object, "object".to_string(), None);
    let function_type = TypeInfo::new(TypeKind::Function, "function".to_string(), None);
    
    assert!(matches!(number_type.kind, TypeKind::Number));
    assert!(matches!(string_type.kind, TypeKind::String));
    assert!(matches!(boolean_type.kind, TypeKind::Boolean));
    assert!(matches!(object_type.kind, TypeKind::Object));
    assert!(matches!(function_type.kind, TypeKind::Function));
}

#[test]
fn test_type_without_description() {
    let type_info = TypeInfo::new(
        TypeKind::String,
        "string".to_string(),
        None,
    );
    
    assert!(matches!(type_info.kind, TypeKind::String));
    assert_eq!(type_info.name, "string");
    assert!(type_info.description.is_none());
}

#[test]
fn test_type_clone() {
    let original = TypeInfo::new(
        TypeKind::Boolean,
        "boolean".to_string(),
        Some("A boolean type".to_string()),
    );
    
    let cloned = original.clone();
    
    assert_eq!(original.kind, cloned.kind);
    assert_eq!(original.name, cloned.name);
    assert_eq!(original.description, cloned.description);
}

#[test]
fn test_type_debug() {
    let type_info = TypeInfo::new(
        TypeKind::Object,
        "object".to_string(),
        Some("An object type".to_string()),
    );
    
    let debug_str = format!("{:?}", type_info);
    assert!(debug_str.contains("Object"));
    assert!(debug_str.contains("object"));
    assert!(debug_str.contains("An object type"));
}

#[test]
fn test_type_equality() {
    let type1 = TypeInfo::new(TypeKind::Number, "number".to_string(), None);
    let type2 = TypeInfo::new(TypeKind::Number, "number".to_string(), None);
    let type3 = TypeInfo::new(TypeKind::String, "string".to_string(), None);
    
    assert_eq!(type1, type2);
    assert_ne!(type1, type3);
}
