use crate::semantic::types::Type;
use crate::vm::types::LineNumber;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Scope {
    variables: HashMap<String, VariableInfo>,
    functions: HashMap<String, FunctionInfo>,
    parent: Option<Box<Scope>>,
    scope_type: ScopeType,
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
            scope_type: ScopeType::Function,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub type_info: Type,
    pub mutable: bool,
    pub initialized: bool,
    pub line: LineNumber,
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub param_types: Vec<Type>,
    pub return_type: Type,
    pub is_method: bool,
    pub line: LineNumber,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeType {
    Global,
    Function,
    Block,
    Class,
    Module,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
            scope_type: ScopeType::Global,
        }
    }

    pub fn new_global() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
            scope_type: ScopeType::Global,
        }
    }

    pub fn with_parent(parent: Scope) -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: Some(Box::new(parent)),
            scope_type: ScopeType::Block,
        }
    }

    pub fn new_child(parent: Scope, scope_type: ScopeType) -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: Some(Box::new(parent)),
            scope_type,
        }
    }

    pub fn declare_variable(&mut self, name: String, var_type: Type, line: LineNumber) {
        self.variables.insert(
            name.clone(),
            VariableInfo {
                name,
                type_info: var_type,
                mutable: true,
                initialized: true,
                line,
            },
        );
    }

    pub fn declare_variable_with_details(
        &mut self,
        name: &str,
        type_info: Type,
        mutable: bool,
        line: LineNumber,
    ) -> bool {
        if self.variables.contains_key(name) {
            false
        } else {
            self.variables.insert(
                name.to_string(),
                VariableInfo {
                    name: name.to_string(),
                    type_info,
                    mutable,
                    initialized: false,
                    line,
                },
            );
            true
        }
    }

    pub fn initialize_variable(&mut self, name: &str) -> bool {
        if let Some(var) = self.variables.get_mut(name) {
            var.initialized = true;
            true
        } else {
            false
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&VariableInfo> {
        self.variables
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|p| p.get_variable(name)))
    }

    pub fn get_variable_type(&self, name: &str) -> Option<Type> {
        if let Some(var_type) = self.variables.get(name) {
            Some(var_type.type_info.clone())
        } else if let Some(ref parent) = self.parent {
            parent.get_variable_type(name)
        } else {
            None
        }
    }

    pub fn has_variable(&self, name: &str) -> bool {
        self.variables.contains_key(name)
            || self.parent.as_ref().is_some_and(|p| p.has_variable(name))
    }

    pub fn is_variable_declared(&self, name: &str) -> bool {
        self.variables.contains_key(name)
            || self
                .parent
                .as_ref()
                .is_some_and(|p| p.is_variable_declared(name))
    }

    pub fn is_variable_declared_in_current_scope(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    pub fn declare_function(
        &mut self,
        name: &str,
        param_types: Vec<Type>,
        return_type: Type,
        is_method: bool,
        line: LineNumber,
    ) -> bool {
        if self.functions.contains_key(name) {
            false
        } else {
            self.functions.insert(
                name.to_string(),
                FunctionInfo {
                    name: name.to_string(),
                    param_types,
                    return_type,
                    is_method,
                    line,
                },
            );
            true
        }
    }

    pub fn get_function(&self, name: &str) -> Option<&FunctionInfo> {
        self.functions
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|p| p.get_function(name)))
    }

    pub fn is_function_declared(&self, name: &str) -> bool {
        self.functions.contains_key(name)
            || self
                .parent
                .as_ref()
                .is_some_and(|p| p.is_function_declared(name))
    }

    pub fn scope_type(&self) -> &ScopeType {
        &self.scope_type
    }

    pub fn get_local_variables(&self) -> &HashMap<String, VariableInfo> {
        &self.variables
    }

    pub fn get_local_functions(&self) -> &HashMap<String, FunctionInfo> {
        &self.functions
    }

    pub fn is_function_scope(&self) -> bool {
        matches!(self.scope_type, ScopeType::Function)
    }

    pub fn is_block_scope(&self) -> bool {
        matches!(self.scope_type, ScopeType::Block)
    }

    pub fn is_global_scope(&self) -> bool {
        matches!(self.scope_type, ScopeType::Global)
    }

    pub fn enter_scope(&mut self) {
        let new_scope = Scope::with_parent(std::mem::replace(self, Scope::new()));
        *self = new_scope;
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.parent.take() {
            *self = *parent;
        }
    }
}
