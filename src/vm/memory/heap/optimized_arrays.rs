//! # Optimized Array Implementation
//!
//! High-performance array implementation with multiple representations
//! for different usage patterns and type optimizations.
//!
//! ## Array Representations
//!
//! - **Dense Arrays**: Contiguous indices, optimized for sequential access
//! - **Sparse Arrays**: Non-contiguous indices, optimized for random access
//! - **Typed Arrays**: Single-type arrays with raw data alignment
//! - **Mixed Arrays**: Hybrid approach for complex scenarios

use crate::vm::value::Value;
use std::collections::HashMap;

/// Array representation optimized for different access patterns
#[derive(Debug, Clone)]
pub enum ArrayRepresentation {
    /// Dense arrays with contiguous indices (most common)
    Dense {
        elements: Vec<Value>,
        length: usize,
        capacity: usize,
    },
    
    /// Sparse arrays with non-contiguous indices
    Sparse {
        elements: HashMap<usize, Value>,
        length: usize,
        capacity: usize,
    },
    
    /// Typed arrays for single-type optimization
    Typed {
        element_type: ElementType,
        data: Vec<u8>,
        length: usize,
        capacity: usize,
    },
    
    /// Mixed arrays combining multiple strategies
    Mixed {
        dense_elements: Vec<Value>,
        sparse_elements: HashMap<usize, Value>,
        length: usize,
        capacity: usize,
        threshold: usize, // When to switch representations
    },
}

/// Element type for typed arrays
#[derive(Debug, Clone, PartialEq)]
pub enum ElementType {
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float32,
    Float64,
    Boolean,
}

/// Optimized array with automatic representation switching
pub struct OptimizedArray {
    representation: ArrayRepresentation,
    length: usize,
    capacity: usize,
    growth_factor: f64,
}

impl OptimizedArray {
    pub fn new() -> Self {
        Self {
            representation: ArrayRepresentation::Dense {
                elements: Vec::new(),
                length: 0,
                capacity: 0,
            },
            length: 0,
            capacity: 0,
            growth_factor: 1.5,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            representation: ArrayRepresentation::Dense {
                elements: Vec::with_capacity(capacity),
                length: 0,
                capacity,
            },
            length: 0,
            capacity,
            growth_factor: 1.5,
        }
    }

    pub fn from_elements(elements: Vec<Value>) -> Self {
        let length = elements.len();
        let capacity = elements.capacity();
        
        Self {
            representation: ArrayRepresentation::Dense {
                elements,
                length,
                capacity,
            },
            length,
            capacity,
            growth_factor: 1.5,
        }
    }

    /// Get element at index with bounds checking
    pub fn get(&self, index: usize) -> Option<&Value> {
        if index >= self.length {
            return None;
        }

        match &self.representation {
            ArrayRepresentation::Dense { elements, .. } => {
                elements.get(index)
            }
            ArrayRepresentation::Sparse { elements, .. } => {
                elements.get(&index)
            }
            ArrayRepresentation::Typed { element_type, data, .. } => {
                self.get_typed_element(index, element_type, data)
            }
            ArrayRepresentation::Mixed { dense_elements, sparse_elements, .. } => {
                if index < dense_elements.len() {
                    dense_elements.get(index)
                } else {
                    sparse_elements.get(&index)
                }
            }
        }
    }

    /// Set element at index
    pub fn set(&mut self, index: usize, value: Value) -> Result<(), String> {
        if index >= self.length {
            return Err(format!("Index {} out of bounds (length: {})", index, self.length));
        }

        match &mut self.representation {
            ArrayRepresentation::Dense { elements, .. } => {
                if index < elements.len() {
                    elements[index] = value;
                    Ok(())
                } else {
                    Err("Index out of bounds".to_string())
                }
            }
            ArrayRepresentation::Sparse { elements, .. } => {
                elements.insert(index, value);
                Ok(())
            }
            ArrayRepresentation::Typed { element_type, data, .. } => {
                self.set_typed_element(index, value, element_type, data)
            }
            ArrayRepresentation::Mixed { dense_elements, sparse_elements, .. } => {
                if index < dense_elements.len() {
                    dense_elements[index] = value;
                } else {
                    sparse_elements.insert(index, value);
                }
                Ok(())
            }
        }
    }

    /// Push element to end of array
    pub fn push(&mut self, value: Value) {
        self.length += 1;
        
        match &mut self.representation {
            ArrayRepresentation::Dense { elements, capacity, .. } => {
                if self.length > *capacity {
                    *capacity = ((*capacity as f64) * self.growth_factor) as usize;
                    elements.reserve(*capacity - elements.len());
                }
                elements.push(value);
            }
            ArrayRepresentation::Sparse { elements, capacity, .. } => {
                if self.length > *capacity {
                    *capacity = ((*capacity as f64) * self.growth_factor) as usize;
                }
                elements.insert(self.length - 1, value);
            }
            ArrayRepresentation::Typed { element_type, data, capacity, .. } => {
                if self.length > *capacity {
                    *capacity = ((*capacity as f64) * self.growth_factor) as usize;
                    data.reserve(*capacity * element_type.size());
                }
                self.push_typed_element(value, element_type, data);
            }
            ArrayRepresentation::Mixed { dense_elements, sparse_elements, capacity, threshold, .. } => {
                if self.length <= *threshold {
                    if self.length > dense_elements.len() {
                        dense_elements.resize(self.length, Value::Undefined);
                    }
                    dense_elements[self.length - 1] = value;
                } else {
                    sparse_elements.insert(self.length - 1, value);
                }
                
                if self.length > *capacity {
                    *capacity = ((*capacity as f64) * self.growth_factor) as usize;
                }
            }
        }
    }

    /// Pop element from end of array
    pub fn pop(&mut self) -> Option<Value> {
        if self.length == 0 {
            return None;
        }

        self.length -= 1;
        let result = match &mut self.representation {
            ArrayRepresentation::Dense { elements, .. } => {
                elements.pop()
            }
            ArrayRepresentation::Sparse { elements, .. } => {
                elements.remove(&self.length)
            }
            ArrayRepresentation::Typed { element_type, data, .. } => {
                self.pop_typed_element(element_type, data)
            }
            ArrayRepresentation::Mixed { dense_elements, sparse_elements, .. } => {
                if self.length < dense_elements.len() {
                    dense_elements.pop()
                } else {
                    sparse_elements.remove(&self.length)
                }
            }
        };

        // Consider switching representations based on usage patterns
        self.consider_representation_switch();
        result
    }

    /// Get array length
    pub fn len(&self) -> usize {
        self.length
    }

    /// Check if array is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Get array capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Convert to dense representation for better performance
    pub fn to_dense(&mut self) {
        match &self.representation {
            ArrayRepresentation::Dense { .. } => {
                // Already dense, no conversion needed
            }
            ArrayRepresentation::Sparse { elements, .. } => {
                let mut dense_elements = Vec::with_capacity(self.length);
                for i in 0..self.length {
                    dense_elements.push(elements.get(&i).cloned().unwrap_or(Value::Undefined));
                }
                self.representation = ArrayRepresentation::Dense {
                    elements: dense_elements,
                    length: self.length,
                    capacity: self.capacity,
                };
            }
            ArrayRepresentation::Typed { element_type, data, .. } => {
                let mut dense_elements = Vec::with_capacity(self.length);
                for i in 0..self.length {
                    dense_elements.push(self.get_typed_element(i, element_type, data).cloned().unwrap_or(Value::Undefined));
                }
                self.representation = ArrayRepresentation::Dense {
                    elements: dense_elements,
                    length: self.length,
                    capacity: self.capacity,
                };
            }
            ArrayRepresentation::Mixed { dense_elements, sparse_elements, .. } => {
                let mut all_elements = dense_elements.clone();
                for (index, value) in sparse_elements {
                    if *index >= all_elements.len() {
                        all_elements.resize(*index + 1, Value::Undefined);
                    }
                    all_elements[*index] = value.clone();
                }
                self.representation = ArrayRepresentation::Dense {
                    elements: all_elements,
                    length: self.length,
                    capacity: self.capacity,
                };
            }
        }
    }

    /// Convert to typed representation if possible
    pub fn to_typed(&mut self) -> Result<(), String> {
        // Analyze elements to determine if they can be typed
        let element_type = self.analyze_element_type()?;
        
        match &self.representation {
            ArrayRepresentation::Dense { elements, .. } => {
                let mut data = Vec::new();
                for element in elements {
                    self.push_typed_element(element.clone(), &element_type, &mut data);
                }
                self.representation = ArrayRepresentation::Typed {
                    element_type,
                    data,
                    length: self.length,
                    capacity: self.capacity,
                };
            }
            _ => {
                // Convert to dense first, then to typed
                self.to_dense();
                self.to_typed()?;
            }
        }
        
        Ok(())
    }

    /// Analyze elements to determine optimal element type
    fn analyze_element_type(&self) -> Result<ElementType, String> {
        let mut int_count = 0;
        let mut float_count = 0;
        let mut bool_count = 0;
        let mut other_count = 0;

        for i in 0..self.length {
            if let Some(value) = self.get(i) {
                match value {
                    Value::Number(n) => {
                        if n.fract() == 0.0 {
                            int_count += 1;
                        } else {
                            float_count += 1;
                        }
                    }
                    Value::Boolean(_) => bool_count += 1,
                    _ => other_count += 1,
                }
            }
        }

        let total = self.length;
        if other_count > 0 {
            return Err("Cannot convert to typed array: contains non-numeric/boolean values".to_string());
        }

        if bool_count == total {
            Ok(ElementType::Boolean)
        } else if int_count == total {
            // Determine optimal integer size
            let max_value = self.get_max_int_value();
            if max_value <= i8::MAX as f64 {
                Ok(ElementType::Int8)
            } else if max_value <= i16::MAX as f64 {
                Ok(ElementType::Int16)
            } else if max_value <= i32::MAX as f64 {
                Ok(ElementType::Int32)
            } else {
                Ok(ElementType::Int64)
            }
        } else if float_count > 0 {
            Ok(ElementType::Float64)
        } else {
            Err("Cannot determine element type".to_string())
        }
    }

    /// Get maximum integer value in array
    fn get_max_int_value(&self) -> f64 {
        let mut max_value = f64::NEG_INFINITY;
        for i in 0..self.length {
            if let Some(Value::Number(n)) = self.get(i) {
                if n.fract() == 0.0 && *n > max_value {
                    max_value = *n;
                }
            }
        }
        max_value
    }

    /// Get typed element from raw data
    fn get_typed_element(&self, index: usize, element_type: &ElementType, data: &[u8]) -> Option<&Value> {
        let element_size = element_type.size();
        let start = index * element_size;
        let end = start + element_size;
        
        if end > data.len() {
            return None;
        }

        let element_data = &data[start..end];
        Some(&self.deserialize_typed_element(element_data, element_type))
    }

    /// Set typed element in raw data
    fn set_typed_element(&mut self, index: usize, value: Value, element_type: &ElementType, data: &mut [u8]) -> Result<(), String> {
        let element_size = element_type.size();
        let start = index * element_size;
        let end = start + element_size;
        
        if end > data.len() {
            return Err("Index out of bounds".to_string());
        }

        let serialized = self.serialize_typed_element(value, element_type)?;
        data[start..end].copy_from_slice(&serialized);
        Ok(())
    }

    /// Push typed element to raw data
    fn push_typed_element(&mut self, value: Value, element_type: &ElementType, data: &mut Vec<u8>) {
        let serialized = self.serialize_typed_element(value, element_type).unwrap_or_default();
        data.extend_from_slice(&serialized);
    }

    /// Pop typed element from raw data
    fn pop_typed_element(&mut self, element_type: &ElementType, data: &mut Vec<u8>) -> Option<Value> {
        let element_size = element_type.size();
        if data.len() < element_size {
            return None;
        }

        let start = data.len() - element_size;
        let element_data = data[start..].to_vec();
        data.truncate(start);
        
        Some(self.deserialize_typed_element(&element_data, element_type))
    }

    /// Serialize value to typed format
    fn serialize_typed_element(&self, value: Value, element_type: &ElementType) -> Result<Vec<u8>, String> {
        match (value, element_type) {
            (Value::Number(n), ElementType::Int8) => {
                if n.fract() == 0.0 && n >= i8::MIN as f64 && n <= i8::MAX as f64 {
                    Ok(vec![n as i8 as u8])
                } else {
                    Err("Value out of range for Int8".to_string())
                }
            }
            (Value::Number(n), ElementType::Int16) => {
                if n.fract() == 0.0 && n >= i16::MIN as f64 && n <= i16::MAX as f64 {
                    Ok((n as i16).to_le_bytes().to_vec())
                } else {
                    Err("Value out of range for Int16".to_string())
                }
            }
            (Value::Number(n), ElementType::Int32) => {
                if n.fract() == 0.0 && n >= i32::MIN as f64 && n <= i32::MAX as f64 {
                    Ok((n as i32).to_le_bytes().to_vec())
                } else {
                    Err("Value out of range for Int32".to_string())
                }
            }
            (Value::Number(n), ElementType::Int64) => {
                if n.fract() == 0.0 {
                    Ok((n as i64).to_le_bytes().to_vec())
                } else {
                    Err("Value out of range for Int64".to_string())
                }
            }
            (Value::Number(n), ElementType::Float32) => {
                Ok((n as f32).to_le_bytes().to_vec())
            }
            (Value::Number(n), ElementType::Float64) => {
                Ok(n.to_le_bytes().to_vec())
            }
            (Value::Boolean(b), ElementType::Boolean) => {
                Ok(vec![if b { 1 } else { 0 }])
            }
            _ => Err("Cannot serialize value to specified type".to_string()),
        }
    }

    /// Deserialize typed data to value
    fn deserialize_typed_element(&self, data: &[u8], element_type: &ElementType) -> Value {
        match element_type {
            ElementType::Int8 => Value::Number(data[0] as i8 as f64),
            ElementType::Int16 => {
                let bytes: [u8; 2] = [data[0], data[1]];
                Value::Number(i16::from_le_bytes(bytes) as f64)
            }
            ElementType::Int32 => {
                let bytes: [u8; 4] = [data[0], data[1], data[2], data[3]];
                Value::Number(i32::from_le_bytes(bytes) as f64)
            }
            ElementType::Int64 => {
                let bytes: [u8; 8] = [data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]];
                Value::Number(i64::from_le_bytes(bytes) as f64)
            }
            ElementType::Float32 => {
                let bytes: [u8; 4] = [data[0], data[1], data[2], data[3]];
                Value::Number(f32::from_le_bytes(bytes) as f64)
            }
            ElementType::Float64 => {
                let bytes: [u8; 8] = [data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]];
                Value::Number(f64::from_le_bytes(bytes))
            }
            ElementType::Boolean => Value::Boolean(data[0] != 0),
            _ => Value::Undefined,
        }
    }

    /// Consider switching array representation based on usage patterns
    fn consider_representation_switch(&mut self) {
        // TODO: Implement smart representation switching
        // - Switch to sparse if many holes
        // - Switch to dense if mostly contiguous
        // - Switch to typed if all elements are same type
    }
}

impl ElementType {
    /// Get size in bytes for this element type
    pub fn size(&self) -> usize {
        match self {
            ElementType::Int8 | ElementType::Uint8 | ElementType::Boolean => 1,
            ElementType::Int16 | ElementType::Uint16 => 2,
            ElementType::Int32 | ElementType::Uint32 | ElementType::Float32 => 4,
            ElementType::Int64 | ElementType::Uint64 | ElementType::Float64 => 8,
        }
    }
}

impl Default for OptimizedArray {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<Value>> for OptimizedArray {
    fn from(elements: Vec<Value>) -> Self {
        Self::from_elements(elements)
    }
}

impl From<OptimizedArray> for Vec<Value> {
    fn from(array: OptimizedArray) -> Self {
        match array.representation {
            ArrayRepresentation::Dense { elements, .. } => elements,
            ArrayRepresentation::Sparse { elements, length, .. } => {
                let mut result = Vec::with_capacity(length);
                for i in 0..length {
                    result.push(elements.get(&i).cloned().unwrap_or(Value::Undefined));
                }
                result
            }
            ArrayRepresentation::Typed { element_type, data, length, .. } => {
                let mut result = Vec::with_capacity(length);
                for i in 0..length {
                    result.push(array.get_typed_element(i, &element_type, &data).cloned().unwrap_or(Value::Undefined));
                }
                result
            }
            ArrayRepresentation::Mixed { dense_elements, sparse_elements, length, .. } => {
                let mut result = dense_elements;
                for (index, value) in sparse_elements {
                    if index >= result.len() {
                        result.resize(index + 1, Value::Undefined);
                    }
                    result[index] = value;
                }
                result
            }
        }
    }
}
