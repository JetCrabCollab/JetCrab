//! Object Shapes Unit Tests
//! 
//! Tests for object shapes (hidden classes) system

use jetcrab::vm::memory::heap::object_shapes::{ObjectShape, ShapeTable, PropertyDescriptor};

#[test]
fn test_object_shape_creation() {
    let shape = ObjectShape::new("test_shape".to_string());
    
    assert_eq!(shape.name, "test_shape");
    assert!(shape.properties.is_empty());
    assert_eq!(shape.property_count, 0);
}

#[test]
fn test_object_shape_add_property() {
    let mut shape = ObjectShape::new("test".to_string());
    
    let descriptor = PropertyDescriptor::new("x".to_string(), "number".to_string(), true);
    shape.add_property(descriptor);
    
    assert_eq!(shape.property_count, 1);
    assert_eq!(shape.properties.len(), 1);
    assert_eq!(shape.properties[0].name, "x");
}

#[test]
fn test_object_shape_get_property() {
    let mut shape = ObjectShape::new("test".to_string());
    
    let descriptor = PropertyDescriptor::new("y".to_string(), "string".to_string(), false);
    shape.add_property(descriptor);
    
    let found_property = shape.get_property("y");
    assert!(found_property.is_some());
    assert_eq!(found_property.unwrap().name, "y");
    
    let not_found = shape.get_property("z");
    assert!(not_found.is_none());
}

#[test]
fn test_property_descriptor() {
    let descriptor = PropertyDescriptor::new("test_prop".to_string(), "boolean".to_string(), true);
    
    assert_eq!(descriptor.name, "test_prop");
    assert_eq!(descriptor.type_name, "boolean");
    assert!(descriptor.writable);
}

#[test]
fn test_shape_table_creation() {
    let table = ShapeTable::new();
    
    assert!(table.shapes.is_empty());
    assert_eq!(table.shape_count, 0);
}

#[test]
fn test_shape_table_add_shape() {
    let mut table = ShapeTable::new();
    
    let shape = ObjectShape::new("shape1".to_string());
    table.add_shape(shape);
    
    assert_eq!(table.shape_count, 1);
    assert_eq!(table.shapes.len(), 1);
}

#[test]
fn test_shape_table_find_shape() {
    let mut table = ShapeTable::new();
    
    let shape = ObjectShape::new("test_shape".to_string());
    table.add_shape(shape);
    
    let found = table.find_shape("test_shape");
    assert!(found.is_some());
    
    let not_found = table.find_shape("nonexistent");
    assert!(not_found.is_none());
}

#[test]
fn test_object_shape_clone() {
    let mut original = ObjectShape::new("original".to_string());
    let descriptor = PropertyDescriptor::new("x".to_string(), "number".to_string(), true);
    original.add_property(descriptor);
    
    let cloned = original.clone();
    
    assert_eq!(original.name, cloned.name);
    assert_eq!(original.property_count, cloned.property_count);
    assert_eq!(original.properties.len(), cloned.properties.len());
}

#[test]
fn test_property_descriptor_clone() {
    let original = PropertyDescriptor::new("prop".to_string(), "string".to_string(), false);
    let cloned = original.clone();
    
    assert_eq!(original.name, cloned.name);
    assert_eq!(original.type_name, cloned.type_name);
    assert_eq!(original.writable, cloned.writable);
}
