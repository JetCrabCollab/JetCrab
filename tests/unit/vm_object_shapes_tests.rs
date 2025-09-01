//! Unit tests for VM Object Shapes

use jetcrab::vm::memory::heap::object_shapes::{
    ObjectShape, PropertyAttributes, PropertyDescriptor, PropertyType, ShapeId, ShapeStats,
    ShapeTransitionManager,
};

#[test]
fn test_shape_id_creation() {
    let shape_id = ShapeId::new();
    assert!(shape_id.value() > 0);
}

#[test]
fn test_shape_id_default() {
    let shape_id = ShapeId::default();
    assert!(shape_id.value() > 0);
}

#[test]
fn test_shape_id_value() {
    let shape_id = ShapeId::new();
    let value = shape_id.value();
    assert!(value > 0);
}

#[test]
fn test_shape_id_equality() {
    let shape_id1 = ShapeId::new();
    let shape_id2 = ShapeId::new();
    assert_ne!(shape_id1, shape_id2);
    assert_eq!(shape_id1, shape_id1);
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
fn test_property_attributes_non_configurable() {
    let attrs = PropertyAttributes::non_configurable();
    assert!(attrs.writable);
    assert!(attrs.enumerable);
    assert!(!attrs.configurable);
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
fn test_property_attributes_equality() {
    let attrs1 = PropertyAttributes::default();
    let attrs2 = PropertyAttributes::default();
    assert_eq!(attrs1, attrs2);

    let attrs3 = PropertyAttributes::read_only();
    assert_ne!(attrs1, attrs3);
}

#[test]
fn test_property_descriptor_new() {
    let desc = PropertyDescriptor::new("test".to_string(), 0, PropertyType::Primitive);
    assert_eq!(desc.name, "test");
    assert_eq!(desc.offset, 0);
    assert_eq!(desc.property_type, PropertyType::Primitive);
    assert!(desc.attributes.writable);
}

#[test]
fn test_property_descriptor_with_attributes() {
    let attrs = PropertyAttributes::read_only();
    let desc = PropertyDescriptor::with_attributes(
        "test".to_string(),
        8,
        PropertyType::Object,
        attrs.clone(),
    );
    assert_eq!(desc.name, "test");
    assert_eq!(desc.offset, 8);
    assert_eq!(desc.property_type, PropertyType::Object);
    assert_eq!(desc.attributes, attrs);
}

#[test]
fn test_property_type_enum() {
    let primitive = PropertyType::Primitive;
    assert!(matches!(primitive, PropertyType::Primitive));

    let object = PropertyType::Object;
    assert!(matches!(object, PropertyType::Object));

    let array = PropertyType::Array;
    assert!(matches!(array, PropertyType::Array));

    let function = PropertyType::Function;
    assert!(matches!(function, PropertyType::Function));

    let accessor = PropertyType::Accessor;
    assert!(matches!(accessor, PropertyType::Accessor));
}

#[test]
fn test_object_shape_new() {
    let shape = ObjectShape::new();
    assert!(shape.id.value() > 0);
    assert!(shape.parent.is_none());
    assert!(shape.properties.is_empty());
    assert!(shape.property_map.is_empty());
    assert_eq!(shape.object_size, 0);
    assert_eq!(shape.property_count, 0);
    assert_eq!(shape.depth, 0);
}

#[test]
fn test_object_shape_default() {
    let shape = ObjectShape::default();
    assert!(shape.id.value() > 0);
    assert!(shape.parent.is_none());
    assert!(shape.properties.is_empty());
}

#[test]
fn test_object_shape_with_parent() {
    let parent_id = ShapeId::new();
    let properties = vec![
        PropertyDescriptor::new("prop1".to_string(), 0, PropertyType::Primitive),
        PropertyDescriptor::new("prop2".to_string(), 8, PropertyType::Object),
    ];

    let shape = ObjectShape::with_parent(parent_id, properties.clone());
    assert_eq!(shape.parent, Some(parent_id));
    assert_eq!(shape.properties.len(), 2);
    assert_eq!(shape.property_count, 2);
    assert_eq!(shape.depth, 1);
    assert!(shape.property_map.contains_key("prop1"));
    assert!(shape.property_map.contains_key("prop2"));
}

#[test]
fn test_object_shape_add_property() {
    let mut shape = ObjectShape::new();

    let result = shape.add_property("test".to_string(), PropertyType::Primitive);
    assert!(result.is_ok());
    assert_eq!(shape.property_count, 1);
    assert_eq!(shape.properties.len(), 1);
    assert!(shape.property_map.contains_key("test"));
}

#[test]
fn test_object_shape_get_property() {
    let mut shape = ObjectShape::new();
    let _ = shape.add_property("test".to_string(), PropertyType::Primitive);

    let found = shape.get_property("test");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "test");

    let not_found = shape.get_property("nonexistent");
    assert!(not_found.is_none());
}

#[test]
fn test_object_shape_has_property() {
    let mut shape = ObjectShape::new();
    let _ = shape.add_property("test".to_string(), PropertyType::Primitive);

    assert!(shape.has_property("test"));
    assert!(!shape.has_property("nonexistent"));
}

#[test]
fn test_object_shape_get_property_offset() {
    let mut shape = ObjectShape::new();
    let _ = shape.add_property("test".to_string(), PropertyType::Object);

    let offset = shape.get_property_offset("test");
    assert_eq!(offset, Some(0));

    let no_offset = shape.get_property_offset("nonexistent");
    assert!(no_offset.is_none());
}

#[test]
fn test_shape_transition_manager_new() {
    let manager = ShapeTransitionManager::new();
    let stats = manager.stats();
    assert_eq!(stats.total_shapes, 1);
    assert_eq!(stats.total_transitions, 0);
}

#[test]
fn test_shape_transition_manager_default() {
    let manager = ShapeTransitionManager::default();
    let stats = manager.stats();
    assert_eq!(stats.total_shapes, 1);
}

#[test]
fn test_shape_transition_manager_get_shape() {
    let manager = ShapeTransitionManager::new();
    let root_shape = manager.root_shape();

    let retrieved = manager.get_shape(root_shape);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, root_shape);
}

#[test]
fn test_shape_transition_manager_add_property_transition() {
    let mut manager = ShapeTransitionManager::new();
    let root_shape = manager.root_shape();

    let result = manager.add_property_transition(
        root_shape,
        "new_prop".to_string(),
        PropertyType::Primitive,
    );
    assert!(result.is_ok());

    let new_shape_id = result.unwrap();
    let new_shape = manager.get_shape(new_shape_id);
    assert!(new_shape.is_some());
    assert!(new_shape.unwrap().has_property("new_prop"));
}

#[test]
fn test_shape_transition_manager_remove_property_transition() {
    let mut manager = ShapeTransitionManager::new();
    let root_shape = manager.root_shape();

    let result = manager.remove_property_transition(root_shape, "nonexistent");
    assert!(result.is_ok());
}

#[test]
fn test_shape_transition_manager_all_shapes() {
    let manager = ShapeTransitionManager::new();
    let shapes = manager.all_shapes();
    assert_eq!(shapes.len(), 1);
}

#[test]
fn test_shape_transition_manager_stats() {
    let manager = ShapeTransitionManager::new();
    let stats = manager.stats();
    assert_eq!(stats.total_shapes, 1);
    assert_eq!(stats.total_transitions, 0);
    assert_eq!(stats.max_depth, 0);
    assert_eq!(stats.avg_properties, 0);
}

#[test]
fn test_shape_stats() {
    let stats = ShapeStats {
        total_shapes: 5,
        total_transitions: 3,
        max_depth: 2,
        avg_properties: 2,
    };

    assert_eq!(stats.total_shapes, 5);
    assert_eq!(stats.total_transitions, 3);
    assert_eq!(stats.max_depth, 2);
    assert_eq!(stats.avg_properties, 2);
}
