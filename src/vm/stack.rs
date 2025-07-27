use crate::vm::frame::Frame;
use crate::vm::types::StackIndex;
use crate::vm::value::Value;

#[derive(Debug, Default)]
pub struct Stack {
    pub values: Vec<Value>,
    pub frames: Vec<Frame>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            frames: Vec::new(),
        }
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.values.pop()
    }

    pub fn peek(&self) -> Option<&Value> {
        self.values.last()
    }

    pub fn peek_mut(&mut self) -> Option<&mut Value> {
        self.values.last_mut()
    }

    pub fn size(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn get(&self, index: StackIndex) -> Option<&Value> {
        self.values.get(index.as_usize())
    }

    pub fn set(&mut self, index: StackIndex, value: Value) -> Result<(), String> {
        let idx = index.as_usize();
        if idx < self.values.len() {
            self.values[idx] = value;
            Ok(())
        } else {
            Err("Stack index out of bounds".to_string())
        }
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub fn pop_frame(&mut self) -> Option<Frame> {
        self.frames.pop()
    }
}
