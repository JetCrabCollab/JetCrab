use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::ast::Node;
use crate::semantic::core::Type;

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

pub struct TypeAnalyzer {
    types: std::collections::HashMap<String, Type>,
}

impl TypeAnalyzer {
    pub fn new() -> Self {
        Self {
            types: std::collections::HashMap::new(),
        }
    }

    pub fn analyze_types(&mut self, ast: &Node) -> Result<(), String> {
        self.visit_node(ast)?;
        Ok(())
    }

    fn visit_node(&mut self, node: &Node) -> Result<Type, String> {
        match node {
            Node::Program(statements) => {
                let mut last_type = Type::Undefined;
                for stmt in statements {
                    last_type = self.visit_node(stmt)?;
                }
                Ok(last_type)
            }
            Node::ExpressionStatement(expr) => self.visit_node(expr),
            Node::BinaryExpression { left, operator: _, right } => {
                let left_type = self.visit_node(left)?;
                let right_type = self.visit_node(right)?;
                self.infer_binary_expression_type(&left_type, &right_type)
            }
            Node::Identifier(name) => {
                Ok(self.types.get(name).cloned().unwrap_or(Type::Unknown))
            }
            Node::Literal(value) => {
                match value {
                    crate::ast::Literal::Number(_) => Ok(Type::Number),
                    crate::ast::Literal::String(_) => Ok(Type::String),
                    crate::ast::Literal::Boolean(_) => Ok(Type::Boolean),
                    crate::ast::Literal::Null => Ok(Type::Null),
                    crate::ast::Literal::Undefined => Ok(Type::Undefined),
                }
            }
            Node::VariableDeclaration { name, initializer, .. } => {
                if let Some(init) = initializer {
                    let init_type = self.visit_node(init)?;
                    self.types.insert(name.clone(), init_type.clone());
                    Ok(init_type)
                } else {
                    Ok(Type::Undefined)
                }
            }
            Node::MemberExpression { object, property } => {
                let object_type = self.visit_node(object)?;
                self.infer_member_expression_type(&object_type, property)
            }
            Node::CallExpression { callee, arguments: _ } => {
                let callee_type = self.visit_node(callee)?;
                match callee_type {
                    Type::Function { return_type, .. } => Ok(*return_type),
                    _ => Ok(Type::Unknown),
                }
            }
            Node::ArrayExpression { elements } => {
                let mut element_types = Vec::new();
                for element in elements {
                    element_types.push(self.visit_node(element)?);
                }
                Ok(Type::Array(Box::new(Type::Union(element_types))))
            }
            Node::ObjectExpression { properties } => {
                let mut property_types = Vec::new();
                for prop in properties {
                    if let crate::ast::Property { value, .. } = prop {
                        property_types.push(self.visit_node(value)?);
                    }
                }
                Ok(Type::Object(property_types))
            }
            Node::FunctionDeclaration { name, params, return_type, body: _ } => {
                let param_types: Vec<Type> = params.iter().map(|_| Type::Unknown).collect();
                let func_type = Type::Function {
                    params: param_types,
                    return_type: Box::new(return_type.clone().unwrap_or(Type::Undefined)),
                };
                self.types.insert(name.clone(), func_type.clone());
                Ok(func_type)
            }
            Node::ArrowFunctionExpression { params, return_type, body: _ } => {
                let param_types: Vec<Type> = params.iter().map(|_| Type::Unknown).collect();
                Ok(Type::Function {
                    params: param_types,
                    return_type: Box::new(return_type.clone().unwrap_or(Type::Undefined)),
                })
            }
            Node::IfStatement { condition: _, consequent, alternate } => {
                let consequent_type = self.visit_node(consequent)?;
                if let Some(alt) = alternate {
                    let alternate_type = self.visit_node(alt)?;
                    Ok(Type::Union(vec![consequent_type, alternate_type]))
                } else {
                    Ok(consequent_type)
                }
            }
            Node::WhileStatement { condition: _, body } => {
                self.visit_node(body)
            }
            Node::ForStatement { init: _, condition: _, update: _, body } => {
                self.visit_node(body)
            }
            Node::ReturnStatement { argument } => {
                if let Some(arg) = argument {
                    self.visit_node(arg)
                } else {
                    Ok(Type::Undefined)
                }
            }
            Node::BlockStatement { body } => {
                let mut last_type = Type::Undefined;
                for stmt in body {
                    last_type = self.visit_node(stmt)?;
                }
                Ok(last_type)
            }
            _ => Ok(Type::Unknown),
        }
    }

    fn infer_binary_expression_type(&self, left: &Type, right: &Type) -> Result<Type, String> {
        match (left, right) {
            (Type::Number, Type::Number) => Ok(Type::Number),
            (Type::String, Type::String) => Ok(Type::String),
            (Type::String, Type::Number) | (Type::Number, Type::String) => Ok(Type::String),
            (Type::Boolean, Type::Boolean) => Ok(Type::Boolean),
            _ => Ok(Type::Unknown),
        }
    }

    fn infer_member_expression_type(&mut self, object_type: &Type, property: &Node) -> Result<Type, String> {
        match (object_type, property) {
            (Type::Object(_), Node::Identifier(prop_name)) => {
                match prop_name.as_str() {
                    "length" => Ok(Type::Number),
                    _ => Ok(Type::Unknown),
                }
            }
            (Type::Array(_), Node::Identifier(prop_name)) => {
                match prop_name.as_str() {
                    "push" | "pop" => {
                        Ok(Type::Function {
                            params: vec![],
                            return_type: Box::new(Type::Number),
                        })
                    }
                    "length" => Ok(Type::Number),
                    _ => Ok(Type::Unknown),
                }
            }
            (Type::String, Node::Identifier(prop_name)) => {
                match prop_name.as_str() {
                    "length" => Ok(Type::Number),
                    "toUpperCase" | "toLowerCase" | "trim" => {
                        Ok(Type::Function {
                            params: vec![],
                            return_type: Box::new(Type::String),
                        })
                    }
                    _ => Ok(Type::Unknown),
                }
            }
            _ => Ok(Type::Unknown),
        }
    }
}
