use crate::vm::value::Value;
use crate::vm::executor::traits::StackOperations;
use crate::vm::executor::error_handler::ExecutionError;

pub struct ComparisonHandler;

impl ComparisonHandler {
    pub fn equal<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = Value::Boolean(a == b);
        stack.push(result);
        Ok(())
    }

    pub fn not_equal<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = Value::Boolean(a != b);
        stack.push(result);
        Ok(())
    }

    pub fn strict_equal<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => Value::Boolean(a_val == b_val),
            (Value::String(a_str), Value::String(b_str)) => Value::Boolean(a_str == b_str),
            (Value::Boolean(a_val), Value::Boolean(b_val)) => Value::Boolean(a_val == b_val),
            (Value::Undefined, Value::Undefined) => Value::Boolean(true),
            (Value::Null, Value::Null) => Value::Boolean(true),
            _ => Value::Boolean(false),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn strict_not_equal<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => Value::Boolean(a_val != b_val),
            (Value::String(a_str), Value::String(b_str)) => Value::Boolean(a_str != b_str),
            (Value::Boolean(a_val), Value::Boolean(b_val)) => Value::Boolean(a_val != b_val),
            (Value::Undefined, Value::Undefined) => Value::Boolean(false),
            (Value::Null, Value::Null) => Value::Boolean(false),
            _ => Value::Boolean(true),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn less_than<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => Value::Boolean(a_val < b_val),
            (Value::String(a_str), Value::String(b_str)) => Value::Boolean(a_str < b_str),
            _ => Value::Boolean(false),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn less_equal<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => Value::Boolean(a_val <= b_val),
            (Value::String(a_str), Value::String(b_str)) => Value::Boolean(a_str <= b_str),
            _ => Value::Boolean(false),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn greater_than<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => Value::Boolean(a_val > b_val),
            (Value::String(a_str), Value::String(b_str)) => Value::Boolean(a_str > b_str),
            _ => Value::Boolean(false),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn greater_equal<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => Value::Boolean(a_val >= b_val),
            (Value::String(a_str), Value::String(b_str)) => Value::Boolean(a_str >= b_str),
            _ => Value::Boolean(false),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn logical_and<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Boolean(a_val), Value::Boolean(b_val)) => Value::Boolean(a_val && b_val),
            _ => Value::Boolean(false),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn logical_or<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Boolean(a_val), Value::Boolean(b_val)) => Value::Boolean(a_val || b_val),
            _ => Value::Boolean(false),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn logical_not<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match value {
            Value::Boolean(val) => Value::Boolean(!val),
            Value::Number(val) => Value::Boolean(val == 0.0),
            Value::String(val) => Value::Boolean(val.is_empty()),
            Value::Null | Value::Undefined => Value::Boolean(true),
            _ => Value::Boolean(false),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn bitwise_and<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => {
                Value::Number((a_val as i64 & b_val as i64) as f64)
            }
            _ => Value::Number(0.0),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn bitwise_or<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => {
                Value::Number((a_val as i64 | b_val as i64) as f64)
            }
            _ => Value::Number(0.0),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn bitwise_xor<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => {
                Value::Number((a_val as i64 ^ b_val as i64) as f64)
            }
            _ => Value::Number(0.0),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn bitwise_not<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match value {
            Value::Number(val) => Value::Number(!(val as i64) as f64),
            _ => Value::Number(-1.0),
        };
        
        stack.push(result);
        Ok(())
    }
}
