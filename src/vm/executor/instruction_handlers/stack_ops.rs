use crate::vm::executor::error_handler::ExecutionError;
use crate::vm::executor::traits::StackOperations;
use crate::vm::value::Value;

pub struct StackOpsHandler;

impl StackOpsHandler {
    pub fn push<S>(stack: &mut S, value: Value) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        stack.push(value);
        Ok(())
    }

    pub fn pop<S>(stack: &mut S) -> Result<Value, ExecutionError>
    where
        S: StackOperations,
    {
        stack.pop().ok_or(ExecutionError::StackUnderflow)
    }

    pub fn dup<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let value = stack.peek().ok_or(ExecutionError::StackUnderflow)?;
        stack.push(value.clone());
        Ok(())
    }

    pub fn dup2<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        if stack.len() < 2 {
            return Err(ExecutionError::StackUnderflow);
        }

        // Clone the value first to avoid borrow checker issues
        let value = stack.peek().ok_or(ExecutionError::StackUnderflow)?.clone();
        stack.push(value.clone());
        stack.push(value);
        Ok(())
    }

    pub fn swap<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        if stack.len() < 2 {
            return Err(ExecutionError::StackUnderflow);
        }

        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        stack.push(b);
        stack.push(a);
        Ok(())
    }

    pub fn rot<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        if stack.len() < 3 {
            return Err(ExecutionError::StackUnderflow);
        }

        let c = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        stack.push(b);
        stack.push(c);
        stack.push(a);
        Ok(())
    }

    pub fn over<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        if stack.len() < 2 {
            return Err(ExecutionError::StackUnderflow);
        }

        // For now, we'll just push the top value again
        let value = stack.peek().ok_or(ExecutionError::StackUnderflow)?.clone();
        stack.push(value);
        Ok(())
    }

    pub fn drop<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        Ok(())
    }

    pub fn drop2<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        if stack.len() < 2 {
            return Err(ExecutionError::StackUnderflow);
        }

        stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        Ok(())
    }

    pub fn clear<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        stack.clear();
        Ok(())
    }

    pub fn size<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let size = stack.len();
        stack.push(Value::Number(size as f64));
        Ok(())
    }

    pub fn is_empty<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let is_empty = stack.is_empty();
        stack.push(Value::Boolean(is_empty));
        Ok(())
    }

    pub fn peek<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let value = stack.peek().ok_or(ExecutionError::StackUnderflow)?;
        stack.push(value.clone());
        Ok(())
    }

    pub fn depth<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let depth = stack.len();
        stack.push(Value::Number(depth as f64));
        Ok(())
    }

    pub fn reserve<S>(stack: &mut S, count: usize) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        for _ in 0..count {
            stack.push(Value::Undefined);
        }
        Ok(())
    }

    pub fn truncate<S>(stack: &mut S, new_size: usize) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let current_size = stack.len();
        if new_size < current_size {
            let to_remove = current_size - new_size;
            for _ in 0..to_remove {
                stack.pop().ok_or(ExecutionError::StackUnderflow)?;
            }
        }
        Ok(())
    }
}
