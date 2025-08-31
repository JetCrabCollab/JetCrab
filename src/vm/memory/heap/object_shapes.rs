//! Object Shapes (Hidden Classes) - Optimization system for object property access
//! 
//! This module implements V8-style hidden classes that optimize:
//! - Property access patterns
//! - Memory layout optimization
//! - Property transition chains
//! - Shape sharing between similar objects

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};

/// Unique identifier for object shapes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ShapeId(u64);

impl ShapeId {
    /// Generate a new unique shape ID
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        Self(id)
    }

    /// Get the raw ID value
    pub fn value(&self) -> u64 {
        self.0
    }
}

/// Property attributes for object shapes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropertyAttributes {
    /// Property is writable
    pub writable: bool,
    /// Property is enumerable
    pub enumerable: bool,
    /// Property is configurable
    pub configurable: bool,
    /// Property is a getter/setter
    pub is_accessor: bool,
}

impl Default for PropertyAttributes {
    fn default() -> Self {
        Self {
            writable: true,
            enumerable: true,
            configurable: true,
            is_accessor: false,
        }
    }
}

impl PropertyAttributes {
    /// Create read-only property attributes
    pub fn read_only() -> Self {
        Self {
            writable: false,
            enumerable: true,
            configurable: true,
            is_accessor: false,
        }
    }

    /// Create non-enumerable property attributes
    pub fn non_enumerable() -> Self {
        Self {
            writable: true,
            enumerable: false,
            configurable: true,
            is_accessor: false,
        }
    }

    /// Create non-configurable property attributes
    pub fn non_configurable() -> Self {
        Self {
            writable: true,
            enumerable: true,
            configurable: false,
            is_accessor: false,
        }
    }

    /// Create accessor property attributes
    pub fn accessor() -> Self {
        Self {
            writable: false,
            enumerable: true,
            configurable: true,
            is_accessor: true,
        }
    }
}

/// Property descriptor for object shapes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropertyDescriptor {
    /// Property name
    pub name: String,
    /// Property attributes
    pub attributes: PropertyAttributes,
    /// Property offset in object layout
    pub offset: usize,
    /// Property type information
    pub property_type: PropertyType,
}

impl PropertyDescriptor {
    /// Create a new property descriptor
    pub fn new(name: String, offset: usize, property_type: PropertyType) -> Self {
        Self {
            name,
            attributes: PropertyAttributes::default(),
            offset,
            property_type,
        }
    }

    /// Create a property descriptor with custom attributes
    pub fn with_attributes(
        name: String,
        offset: usize,
        property_type: PropertyType,
        attributes: PropertyAttributes,
    ) -> Self {
        Self {
            name,
            attributes,
            offset,
            property_type,
        }
    }
}

/// Property type information
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PropertyType {
    /// Primitive value (number, string, boolean, null, undefined)
    Primitive,
    /// Object reference
    Object,
    /// Array reference
    Array,
    /// Function reference
    Function,
    /// Accessor (getter/setter)
    Accessor,
}

/// Object shape representing the structure of an object
#[derive(Debug, Clone)]
pub struct ObjectShape {
    /// Unique shape identifier
    pub id: ShapeId,
    /// Parent shape (for transition chains)
    pub parent: Option<ShapeId>,
    /// Property descriptors
    pub properties: Vec<PropertyDescriptor>,
    /// Property name to descriptor mapping
    pub property_map: HashMap<String, usize>,
    /// Object size in bytes
    pub object_size: usize,
    /// Property count
    pub property_count: usize,
    /// Shape depth in transition chain
    pub depth: u32,
}

impl ObjectShape {
    /// Create a new empty object shape
    pub fn new() -> Self {
        Self {
            id: ShapeId::new(),
            parent: None,
            properties: Vec::new(),
            property_map: HashMap::new(),
            object_size: 0,
            property_count: 0,
            depth: 0,
        }
    }

    /// Create a shape with a parent (for transitions)
    pub fn with_parent(parent: ShapeId, properties: Vec<PropertyDescriptor>) -> Self {
        let mut property_map = HashMap::new();
        let mut object_size = 0;
        
        for (index, prop) in properties.iter().enumerate() {
            property_map.insert(prop.name.clone(), index);
            object_size = object_size.max(prop.offset + prop.size());
        }

        let depth = Self::calculate_depth(parent);
        
        Self {
            id: ShapeId::new(),
            parent: Some(parent),
            properties,
            property_map,
            object_size,
            property_count: property_map.len(),
            depth,
        }
    }

    /// Calculate shape depth based on parent chain
    fn calculate_depth(parent: ShapeId) -> u32 {
        // This would need to traverse the parent chain
        // For now, we'll use a simple increment
        1
    }

    /// Add a property to the shape
    pub fn add_property(&mut self, name: String, property_type: PropertyType) -> Result<usize, String> {
        if self.property_map.contains_key(&name) {
            return Err(format!("Property '{}' already exists in shape", name));
        }

        let offset = self.object_size;
        let descriptor = PropertyDescriptor::new(name.clone(), offset, property_type);
        let index = self.properties.len();
        
        self.properties.push(descriptor);
        self.property_map.insert(name, index);
        self.object_size = offset + descriptor.size();
        self.property_count += 1;

        Ok(index)
    }

    /// Get property descriptor by name
    pub fn get_property(&self, name: &str) -> Option<&PropertyDescriptor> {
        self.property_map.get(name).map(|&index| &self.properties[index])
    }

    /// Get property descriptor by index
    pub fn get_property_by_index(&self, index: usize) -> Option<&PropertyDescriptor> {
        self.properties.get(index)
    }

    /// Check if shape has a property
    pub fn has_property(&self, name: &str) -> bool {
        self.property_map.contains_key(name)
    }

    /// Get property offset by name
    pub fn get_property_offset(&self, name: &str) -> Option<usize> {
        self.get_property(name).map(|prop| prop.offset)
    }

    /// Get property count
    pub fn property_count(&self) -> usize {
        self.property_count
    }

    /// Get object size
    pub fn object_size(&self) -> usize {
        self.object_size
    }

    /// Get shape depth
    pub fn depth(&self) -> u32 {
        self.depth
    }

    /// Check if this shape is a leaf (no children)
    pub fn is_leaf(&self) -> bool {
        // This would need to check if any other shapes have this as parent
        true
    }

    /// Get all property names
    pub fn property_names(&self) -> Vec<&String> {
        self.properties.iter().map(|prop| &prop.name).collect()
    }

    /// Get all property names that are enumerable
    pub fn enumerable_property_names(&self) -> Vec<&String> {
        self.properties
            .iter()
            .filter(|prop| prop.attributes.enumerable)
            .map(|prop| &prop.name)
            .collect()
    }
}

impl PartialEq for ObjectShape {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ObjectShape {}

impl Hash for ObjectShape {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Shape transition manager
#[derive(Debug)]
pub struct ShapeTransitionManager {
    /// Shape registry
    shapes: HashMap<ShapeId, ObjectShape>,
    /// Transition cache for fast lookups
    transition_cache: HashMap<(ShapeId, String, PropertyType), ShapeId>,
    /// Root shape (empty object)
    root_shape: ShapeId,
}

impl ShapeTransitionManager {
    /// Create a new shape transition manager
    pub fn new() -> Self {
        let mut manager = Self {
            shapes: HashMap::new(),
            transition_cache: HashMap::new(),
            root_shape: ShapeId::new(),
        };

        // Create the root shape (empty object)
        let root_shape = ObjectShape::new();
        manager.root_shape = root_shape.id;
        manager.shapes.insert(root_shape.id, root_shape);

        manager
    }

    /// Get the root shape
    pub fn root_shape(&self) -> ShapeId {
        self.root_shape
    }

    /// Get a shape by ID
    pub fn get_shape(&self, id: ShapeId) -> Option<&ObjectShape> {
        self.shapes.get(&id)
    }

    /// Get a mutable shape by ID
    pub fn get_shape_mut(&mut self, id: ShapeId) -> Option<&mut ObjectShape> {
        self.shapes.get_mut(&id)
    }

    /// Add a property to an existing shape, creating a transition
    pub fn add_property_transition(
        &mut self,
        base_shape: ShapeId,
        name: String,
        property_type: PropertyType,
    ) -> Result<ShapeId, String> {
        // Check if transition already exists
        if let Some(&transition_id) = self.transition_cache.get(&(base_shape, name.clone(), property_type.clone())) {
            return Ok(transition_id);
        }

        // Get base shape
        let base_shape = self.shapes.get(&base_shape)
            .ok_or_else(|| format!("Base shape {} not found", base_shape.value()))?;

        // Create new properties list with the new property
        let mut new_properties = base_shape.properties.clone();
        let offset = base_shape.object_size;
        let descriptor = PropertyDescriptor::new(name.clone(), offset, property_type.clone());
        new_properties.push(descriptor);

        // Create new shape
        let new_shape = ObjectShape::with_parent(base_shape.id, new_properties);
        let new_shape_id = new_shape.id;

        // Register the new shape
        self.shapes.insert(new_shape_id, new_shape);

        // Cache the transition
        self.transition_cache.insert((base_shape.id, name, property_type), new_shape_id);

        Ok(new_shape_id)
    }

    /// Remove a property from a shape, creating a transition
    pub fn remove_property_transition(
        &mut self,
        base_shape: ShapeId,
        name: &str,
    ) -> Result<ShapeId, String> {
        let base_shape = self.shapes.get(&base_shape)
            .ok_or_else(|| format!("Base shape {} not found", base_shape.value()))?;

        // Create new properties list without the specified property
        let new_properties: Vec<PropertyDescriptor> = base_shape
            .properties
            .iter()
            .filter(|prop| prop.name != name)
            .cloned()
            .collect();

        // Create new shape
        let new_shape = ObjectShape::with_parent(base_shape.id, new_properties);
        let new_shape_id = new_shape.id;

        // Register the new shape
        self.shapes.insert(new_shape_id, new_shape);

        Ok(new_shape_id)
    }

    /// Get all shapes
    pub fn all_shapes(&self) -> &HashMap<ShapeId, ObjectShape> {
        &self.shapes
    }

    /// Get shape statistics
    pub fn stats(&self) -> ShapeStats {
        let total_shapes = self.shapes.len();
        let total_transitions = self.transition_cache.len();
        let max_depth = self.shapes.values().map(|s| s.depth).max().unwrap_or(0);
        let avg_properties = if total_shapes > 0 {
            self.shapes.values().map(|s| s.property_count).sum::<usize>() / total_shapes
        } else {
            0
        };

        ShapeStats {
            total_shapes,
            total_transitions,
            max_depth,
            avg_properties,
        }
    }
}

impl Default for ShapeTransitionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Shape statistics
#[derive(Debug, Clone)]
pub struct ShapeStats {
    /// Total number of shapes
    pub total_shapes: usize,
    /// Total number of transitions
    pub total_transitions: usize,
    /// Maximum shape depth
    pub max_depth: u32,
    /// Average properties per shape
    pub avg_properties: usize,
}

/// Extension trait for PropertyDescriptor to calculate size
trait PropertySize {
    fn size(&self) -> usize;
}

impl PropertySize for PropertyDescriptor {
    fn size(&self) -> usize {
        match self.property_type {
            PropertyType::Primitive => 8, // 64-bit value
            PropertyType::Object => 8,    // 64-bit pointer
            PropertyType::Array => 8,     // 64-bit pointer
            PropertyType::Function => 8,  // 64-bit pointer
            PropertyType::Accessor => 16, // 64-bit getter + 64-bit setter
        }
    }
}
