//! Object Shapes Unit Tests
//! 
//! Tests for object shapes (hidden classes) system

use jetcrab::vm::memory::heap::object_shapes::{
    ShapeId, PropertyAttributes, PropertyDescriptor, PropertyType, 
    ObjectShape, ShapeTransitionManager
};

#[test]
fn test_shape_id_generation() {
    let id1 = ShapeId::new();
    let id2 = ShapeId::new();
    assert_ne!(id1, id2);
    assert!(id1.value() > 0);
    assert!(id2.value() > id1.value());
}

#[test]
fn test_property_attributes_default() {
    let attrs = PropertyAttributes::default();
    assert!(attrs.writable);
    assert!(attrs.enumerable);
    assert!(attrs.configurable);
    assert!(!attrs.is_accessor);
}

#[test]
fn test_property_attributes_read_only() {
    let attrs = PropertyAttributes::read_only();
    assert!(!attrs.writable);
    assert!(attrs.enumerable);
    assert!(attrs.configurable);
    assert!(!attrs.is_accessor);
}

#[test]
fn test_property_attributes_non_enumerable() {
    let attrs = PropertyAttributes::non_enumerable();
    assert!(attrs.writable);
    assert!(!attrs.enumerable);
    assert!(attrs.configurable);
    assert!(!attrs.is_accessor);
}

#[test]
fn test_property_attributes_accessor() {
    let attrs = PropertyAttributes::accessor();
    assert!(!attrs.writable);
    assert!(attrs.enumerable);
    assert!(attrs.configurable);
    assert!(attrs.is_accessor);
}

#[test]
fn test_property_descriptor_creation() {
    let descriptor = PropertyDescriptor::new("test".to_string(), 0, PropertyType::Primitive);
    assert_eq!(descriptor.name, "test");
    assert_eq!(descriptor.offset, 0);
    assert!(matches!(descriptor.property_type, PropertyType::Primitive));
}

#[test]
fn test_object_shape_creation() {
    let shape = ObjectShape::new();
    assert_eq!(shape.property_count, 0);
    assert_eq!(shape.object_size, 0);
    assert_eq!(shape.depth, 0);
    assert!(shape.parent.is_none());
}

#[test]
fn test_object_shape_add_property() {
    let mut shape = ObjectShape::new();
    let result = shape.add_property("test".to_string(), PropertyType::Primitive);
    assert!(result.is_ok());
    assert_eq!(shape.property_count, 1);
    assert!(shape.has_property("test"));
}

#[test]
fn test_object_shape_duplicate_property() {
    let mut shape = ObjectShape::new();
    shape.add_property("test".to_string(), PropertyType::Primitive).unwrap();
    let result = shape.add_property("test".to_string(), PropertyType::Object);
    assert!(result.is_err());
}

#[test]
fn test_object_shape_get_property() {
    let mut shape = ObjectShape::new();
    shape.add_property("test".to_string(), PropertyType::Primitive).unwrap();
    
    let property = shape.get_property("test");
    assert!(property.is_some());
    let property = property.unwrap();
    assert_eq!(property.name, "test");
    assert!(matches!(property.property_type, PropertyType::Primitive));
}

#[test]
fn test_object_shape_property_offset() {
    let mut shape = ObjectShape::new();
    shape.add_property("test".to_string(), PropertyType::Primitive).unwrap();
    
    let offset = shape.get_property_offset("test");
    assert!(offset.is_some());
    assert_eq!(offset.unwrap(), 0);
}

#[test]
fn test_object_shape_enumerable_properties() {
    let mut shape = ObjectShape::new();
    shape.add_property("enumerable".to_string(), PropertyType::Primitive).unwrap();
    
    let mut non_enum_descriptor = PropertyDescriptor::new("non_enumerable".to_string(), 8, PropertyType::Primitive);
    non_enum_descriptor.attributes = PropertyAttributes::non_enumerable();
    shape.properties.push(non_enum_descriptor);
    shape.property_map.insert("non_enumerable".to_string(), 1);
    shape.property_count = 2;
    
    let enumerable_names = shape.enumerable_property_names();
    assert_eq!(enumerable_names.len(), 1);
    assert_eq!(enumerable_names[0], "enumerable");
}

#[test]
fn test_shape_transition_manager_creation() {
    let manager = ShapeTransitionManager::new();
    assert!(manager.get_shape(manager.root_shape()).is_some());
}

#[test]
fn test_shape_transition_add_property() {
    let mut manager = ShapeTransitionManager::new();
    let root_shape = manager.root_shape();
    
    let new_shape = manager.add_property_transition(
        root_shape,
        "test".to_string(),
        PropertyType::Primitive
    );
    assert!(new_shape.is_ok());
    
    let new_shape_id = new_shape.unwrap();
    let new_shape = manager.get_shape(new_shape_id);
    assert!(new_shape.is_some());
    assert!(new_shape.unwrap().has_property("test"));
}

#[test]
fn test_shape_transition_cache() {
    let mut manager = ShapeTransitionManager::new();
    let root_shape = manager.root_shape();
    
    let shape1 = manager.add_property_transition(
        root_shape,
        "test".to_string(),
        PropertyType::Primitive
    ).unwrap();
    
    let shape2 = manager.add_property_transition(
        root_shape,
        "test".to_string(),
        PropertyType::Primitive
    ).unwrap();
    
    assert_eq!(shape1, shape2);
}

#[test]
fn test_shape_transition_remove_property() {
    let mut manager = ShapeTransitionManager::new();
    let root_shape = manager.root_shape();
    
    let shape_with_prop = manager.add_property_transition(
        root_shape,
        "test".to_string(),
        PropertyType::Primitive
    ).unwrap();
    
    let shape_without_prop = manager.remove_property_transition(
        shape_with_prop,
        "test"
    );
    assert!(shape_without_prop.is_ok());
    
    let final_shape = manager.get_shape(shape_without_prop.unwrap());
    assert!(final_shape.is_some());
    assert!(!final_shape.unwrap().has_property("test"));
}

#[test]
fn test_shape_transition_manager_stats() {
    let mut manager = ShapeTransitionManager::new();
    let root_shape = manager.root_shape();
    
    manager.add_property_transition(
        root_shape,
        "test1".to_string(),
        PropertyType::Primitive
    ).unwrap();
    
    manager.add_property_transition(
        root_shape,
        "test2".to_string(),
        PropertyType::Object
    ).unwrap();
    
    let stats = manager.stats();
    assert_eq!(stats.total_shapes, 3); // root + 2 transitions
    assert_eq!(stats.total_transitions, 2);
    assert!(stats.max_depth > 0);
}
