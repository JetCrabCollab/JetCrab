use super::StackOperations;
use crate::vm::stack::Stack;
use crate::vm::value::Value;

pub struct StackManager {
    stack: Stack,
}

impl StackManager {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
        }
    }

    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.stack
    }
}

impl StackOperations for StackManager {
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    fn peek(&self) -> Option<&Value> {
        self.stack.values.last()
    }

    fn clear(&mut self) {
        self.stack.values.clear();
    }

    fn len(&self) -> usize {
        self.stack.values.len()
    }

    fn is_empty(&self) -> bool {
        self.stack.values.is_empty()
    }

    fn stack_mut(&mut self) -> &mut crate::vm::stack::Stack {
        &mut self.stack
    }
}
