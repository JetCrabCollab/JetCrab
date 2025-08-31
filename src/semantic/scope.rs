use crate::semantic::error::SemanticError;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scope {
    pub variables: HashMap<String, VariableInfo>,
    pub parent: Option<Box<Scope>>,
}

#[derive(Debug)]
pub struct VariableInfo {
    pub name: String,
    pub declared: bool,
    pub used: bool,
    pub position: Option<(u32, u32)>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Scope) -> Self {
        Self {
            variables: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn declare_variable(&mut self, name: String, position: Option<(u32, u32)>) -> Result<(), SemanticError> {
        if self.variables.contains_key(&name) {
            return Err(SemanticError {
                message: format!("Variable '{}' already declared in this scope", name),
                position,
            });
        }

        self.variables.insert(name.clone(), VariableInfo {
            name,
            declared: true,
            used: false,
            position,
        });

        Ok(())
    }

    pub fn use_variable(&mut self, name: &str) -> Result<(), SemanticError> {
        if let Some(var_info) = self.variables.get_mut(name) {
            var_info.used = true;
            Ok(())
        } else if let Some(ref mut parent) = self.parent {
            parent.use_variable(name)
        } else {
            Err(SemanticError {
                message: format!("Variable '{}' is not declared", name),
                position: None,
            })
        }
    }

    pub fn is_declared(&self, name: &str) -> bool {
        self.variables.contains_key(name) || 
        self.parent.as_ref().map(|p| p.is_declared(name)).unwrap_or(false)
    }
}
