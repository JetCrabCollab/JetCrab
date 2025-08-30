use crate::vm::value::Value;
use crate::vm::executor::traits::StackOperations;
use crate::vm::executor::error_handler::ExecutionError;

pub struct ArithmeticHandler;

impl ArithmeticHandler {
    pub fn add<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => Value::Number(a_val + b_val),
            (Value::String(a_str), b) => Value::String(format!("{}{}", a_str, b)),
            (a, Value::String(b_str)) => Value::String(format!("{}{}", a, b_str)),
            _ => Value::Number(f64::NAN),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn subtract<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => Value::Number(a_val - b_val),
            _ => Value::Number(f64::NAN),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn multiply<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => Value::Number(a_val * b_val),
            _ => Value::Number(f64::NAN),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn divide<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => {
                if b_val == 0.0 {
                    Value::Number(f64::INFINITY)
                } else {
                    Value::Number(a_val / b_val)
                }
            }
            _ => Value::Number(f64::NAN),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn modulo<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => {
                if b_val == 0.0 {
                    Value::Number(f64::NAN)
                } else {
                    Value::Number(a_val % b_val)
                }
            }
            _ => Value::Number(f64::NAN),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn power<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let b = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        let a = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => Value::Number(a_val.powf(b_val)),
            _ => Value::Number(f64::NAN),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn negate<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match value {
            Value::Number(val) => Value::Number(-val),
            _ => Value::Number(f64::NAN),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn increment<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match value {
            Value::Number(val) => Value::Number(val + 1.0),
            _ => Value::Number(f64::NAN),
        };
        
        stack.push(result);
        Ok(())
    }

    pub fn decrement<S>(stack: &mut S) -> Result<(), ExecutionError>
    where
        S: StackOperations,
    {
        let value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let result = match value {
            Value::Number(val) => Value::Number(val - 1.0),
            _ => Value::Number(f64::NAN),
        };
        
        stack.push(result);
        Ok(())
    }
}
