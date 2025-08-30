//! # Variable Manager
//!
//! Provides concrete implementation of variable management for the VM executor.
//! Manages local and global variable storage and implements the `VariableManager`
//! trait for variable access and manipulation.
//!
//! ## Overview
//!
//! The variable manager provides a simple array-based storage system for
//! both local and global variables, supporting:
//!
//! - **Local Variables**: Function-scoped variables with limited lifetime
//! - **Global Variables**: Program-wide variables accessible across functions
//! - **Variable Access**: Getting and setting variables by index
//! - **Mutable Access**: Direct access to variable storage for advanced operations
//!
//! ## Storage Model
//!
//! Variables are stored in fixed-size vectors, with each variable slot
//! identified by a numeric index. Undefined variables are represented
//! as `Value::Undefined`.
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::vm::executor::variable_manager::VariableManagerImpl;
//! use jetcrab::vm::executor::traits::VariableManager;
//! use jetcrab::vm::value::Value;
//!
//! let mut var_manager = VariableManagerImpl::new();
//! var_manager.set_local(0, Value::Number(42.0));
//! let value = var_manager.get_local(0);
//! ```

use super::VariableManager;
use crate::vm::value::Value;

/// Concrete implementation of variable management for the VM
///
/// Provides array-based storage for local and global variables
/// with indexed access and modification capabilities.
pub struct VariableManagerImpl {
    locals: Vec<Value>,
    globals: Vec<Value>,
}

impl VariableManagerImpl {
    /// Creates a new variable manager with empty variable storage
    ///
    /// Initializes both local and global variable storage with
    /// 32 slots each, all set to `Value::Undefined`.
    pub fn new() -> Self {
        Self {
            locals: vec![Value::Undefined; 32],
            globals: vec![Value::Undefined; 32],
        }
    }

    pub fn locals(&self) -> &[Value] {
        &self.locals
    }

    pub fn locals_mut(&mut self) -> &mut [Value] {
        &mut self.locals
    }

    pub fn globals(&self) -> &[Value] {
        &self.globals
    }

    pub fn globals_mut(&mut self) -> &mut [Value] {
        &mut self.globals
    }
}

impl VariableManager for VariableManagerImpl {
    fn get_local(&self, idx: usize) -> Option<&Value> {
        self.locals.get(idx)
    }

    fn set_local(&mut self, idx: usize, value: Value) {
        if let Some(slot) = self.locals.get_mut(idx) {
            *slot = value;
        }
    }

    fn get_global(&self, idx: usize) -> Option<&Value> {
        self.globals.get(idx)
    }

    fn set_global(&mut self, idx: usize, value: Value) {
        if let Some(slot) = self.globals.get_mut(idx) {
            *slot = value;
        }
    }

    fn get_local_mut(&mut self, idx: usize) -> Option<&mut Value> {
        self.locals.get_mut(idx)
    }

    fn get_global_mut(&mut self, idx: usize) -> Option<&mut Value> {
        self.globals.get_mut(idx)
    }
}
