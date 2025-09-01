use crate::vm::value::Value;
use std::collections::HashMap;

pub struct Object {
    properties: HashMap<String, Value>,
    prototype: Option<Box<Object>>,
}

impl Default for Object {
    fn default() -> Self {
        Self::new()
    }
}

impl Object {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            prototype: None,
        }
    }

    pub fn set_property(&mut self, name: String, value: Value) {
        self.properties.insert(name, value);
    }

    pub fn get_property(&self, name: &str) -> Option<&Value> {
        if let Some(value) = self.properties.get(name) {
            Some(value)
        } else if let Some(ref prototype) = self.prototype {
            prototype.get_property(name)
        } else {
            None
        }
    }

    pub fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
            || self
                .prototype
                .as_ref()
                .is_some_and(|p| p.has_property(name))
    }

    pub fn delete_property(&mut self, name: &str) -> bool {
        self.properties.remove(name).is_some()
    }

    pub fn set_prototype(&mut self, prototype: Object) {
        self.prototype = Some(Box::new(prototype));
    }

    pub fn get_prototype(&self) -> Option<&Object> {
        self.prototype.as_ref().map(|p| p.as_ref())
    }
}
