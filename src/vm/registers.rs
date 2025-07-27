use crate::vm::types::{CodeAddress, FramePointer, StackIndex};
use crate::vm::value::Value;

pub struct Registers {
    pub accumulator: Value,
    pub program_counter: CodeAddress,
    pub stack_pointer: StackIndex,
    pub base_pointer: FramePointer,
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    pub fn new() -> Self {
        Self {
            accumulator: Value::Undefined,
            program_counter: CodeAddress::new(0),
            stack_pointer: StackIndex::new(0),
            base_pointer: FramePointer::new(0),
        }
    }

    pub fn reset(&mut self) {
        self.accumulator = Value::Undefined;
        self.program_counter = CodeAddress::new(0);
        self.stack_pointer = StackIndex::new(0);
        self.base_pointer = FramePointer::new(0);
    }

    pub fn set_accumulator(&mut self, value: Value) {
        self.accumulator = value;
    }

    pub fn get_accumulator(&self) -> &Value {
        &self.accumulator
    }

    pub fn increment_pc(&mut self) {
        self.program_counter.increment();
    }

    pub fn set_pc(&mut self, address: CodeAddress) {
        self.program_counter = address;
    }

    pub fn get_pc(&self) -> CodeAddress {
        self.program_counter
    }
}
