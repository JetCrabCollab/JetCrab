use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum Type {
    Undefined,
    Null,
    Boolean,
    Number,
    String,
    Symbol,
    Object,
    Array(Box<Type>),
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    Union(Vec<Type>),
    #[default]
    Any,
    Never,
    Unknown,
}

impl Type {
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Any, _) | (_, Type::Any) => true,
            (Type::Never, _) | (_, Type::Never) => false,
            (Type::Union(types), other) => types.iter().any(|t| t.is_compatible_with(other)),
            (Type::Array(inner1), Type::Array(inner2)) => inner1.is_compatible_with(inner2),
            (
                Type::Function {
                    params: p1,
                    return_type: r1,
                },
                Type::Function {
                    params: p2,
                    return_type: r2,
                },
            ) => {
                p1.len() == p2.len()
                    && p1
                        .iter()
                        .zip(p2.iter())
                        .all(|(a, b)| a.is_compatible_with(b))
                    && r1.is_compatible_with(r2)
            }
            _ => self == other,
        }
    }

    pub fn common_type(&self, other: &Type) -> Type {
        if self.is_compatible_with(other) {
            self.clone()
        } else if other.is_compatible_with(self) {
            other.clone()
        } else {
            Type::Union(vec![self.clone(), other.clone()])
        }
    }

    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Type::Undefined
                | Type::Null
                | Type::Boolean
                | Type::Number
                | Type::String
                | Type::Symbol
        )
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Type::Object | Type::Array(_) | Type::Function { .. })
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Number)
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Type::String)
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Type::Boolean)
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Type::Array(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Type::Function { .. })
    }
}

#[derive(Default)]
pub struct TypeEnvironment {
    types: HashMap<String, Type>,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }

    pub fn declare(&mut self, name: &str, type_info: Type) {
        self.types.insert(name.to_string(), type_info);
    }

    pub fn get_type(&self, name: &str) -> Option<&Type> {
        self.types.get(name)
    }

    pub fn is_declared(&self, name: &str) -> bool {
        self.types.contains_key(name)
    }

    pub fn update_type(&mut self, name: &str, type_info: Type) -> bool {
        if self.types.contains_key(name) {
            self.types.insert(name.to_string(), type_info);
            true
        } else {
            false
        }
    }
}
