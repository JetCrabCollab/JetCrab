use crate::vm::handle::FunctionHandle;
use crate::vm::types::{ArgIndex, CodeAddress, ConstantIndex, FramePointer};
use crate::vm::value::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Frame {
    pub return_address: CodeAddress,
    pub arg_count: ArgIndex,
    pub base_pointer: FramePointer,
    pub arguments: Vec<Value>,
    pub closure_vars: HashMap<String, Value>,
    pub function_handle: Option<FunctionHandle>,
    pub this_value: Option<Value>,
    pub constants: Vec<Value>,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            return_address: CodeAddress::new(0),
            arg_count: ArgIndex::new(0),
            base_pointer: FramePointer::new(0),
            arguments: Vec::new(),
            closure_vars: HashMap::new(),
            function_handle: None,
            this_value: None,
            constants: Vec::new(),
        }
    }

    pub fn with_return_address(return_address: CodeAddress) -> Self {
        Self {
            return_address,
            arg_count: ArgIndex::new(0),
            base_pointer: FramePointer::new(0),
            arguments: Vec::new(),
            closure_vars: HashMap::new(),
            function_handle: None,
            this_value: None,
            constants: Vec::new(),
        }
    }

    pub fn with_constants(constants: Vec<Value>) -> Self {
        Self {
            return_address: CodeAddress::new(0),
            arg_count: ArgIndex::new(0),
            base_pointer: FramePointer::new(0),
            arguments: Vec::new(),
            closure_vars: HashMap::new(),
            function_handle: None,
            this_value: None,
            constants,
        }
    }

    pub fn get_constant(&self, index: ConstantIndex) -> Option<Value> {
        let idx = usize::from(index);
        self.constants.get(idx).cloned()
    }

    pub fn set_constants(&mut self, constants: Vec<Value>) {
        self.constants = constants;
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            return_address: CodeAddress::new(0),
            arg_count: ArgIndex::new(0),
            base_pointer: FramePointer::new(0),
            arguments: Vec::new(),
            closure_vars: HashMap::new(),
            function_handle: None,
            this_value: None,
            constants: Vec::new(),
        }
    }
}
