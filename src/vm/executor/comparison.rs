use crate::vm::value::Value;
use super::StackOperations;

pub trait ComparisonOperations<S>
where
    S: StackOperations,
{
    fn execute_eq(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_ne(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_lt(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_le(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_gt(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_ge(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_strict_eq(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_strict_ne(&mut self, stack: &mut S) -> Result<(), String>;
}

pub struct ComparisonExecutor;

impl ComparisonExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl<S> ComparisonOperations<S> for ComparisonExecutor
where
    S: StackOperations,
{
    fn execute_eq(&mut self, stack: &mut S) -> Result<(), String> {
        let b = stack.pop().ok_or("Stack underflow")?;
        let a = stack.pop().ok_or("Stack underflow")?;
        
        let result = match (&a, &b) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Undefined, Value::Undefined) => true,
            (Value::Number(a), Value::String(b)) => {
                if let Ok(b_num) = b.parse::<f64>() {
                    *a == b_num
                } else {
                    false
                }
            }
            (Value::String(a), Value::Number(b)) => {
                if let Ok(a_num) = a.parse::<f64>() {
                    a_num == *b
                } else {
                    false
                }
            }
            _ => false,
        };
        
        stack.push(Value::Boolean(result));
        Ok(())
    }

    fn execute_ne(&mut self, stack: &mut S) -> Result<(), String> {
        let b = stack.pop().ok_or("Stack underflow")?;
        let a = stack.pop().ok_or("Stack underflow")?;
        
        let result = match (&a, &b) {
            (Value::Number(a), Value::Number(b)) => a != b,
            (Value::String(a), Value::String(b)) => a != b,
            (Value::Boolean(a), Value::Boolean(b)) => a != b,
            (Value::Null, Value::Null) => false,
            (Value::Undefined, Value::Undefined) => false,
            (Value::Number(a), Value::String(b)) => {
                if let Ok(b_num) = b.parse::<f64>() {
                    *a != b_num
                } else {
                    true
                }
            }
            (Value::String(a), Value::Number(b)) => {
                if let Ok(a_num) = a.parse::<f64>() {
                    a_num != *b
                } else {
                    true
                }
            }
            _ => true,
        };
        
        stack.push(Value::Boolean(result));
        Ok(())
    }

    fn execute_lt(&mut self, stack: &mut S) -> Result<(), String> {
        let b = stack.pop().ok_or("Stack underflow")?;
        let a = stack.pop().ok_or("Stack underflow")?;
        
        let result = match (&a, &b) {
            (Value::Number(a), Value::Number(b)) => a < b,
            (Value::String(a), Value::String(b)) => a < b,
            (Value::Number(a), Value::String(b)) => {
                if let Ok(b_num) = b.parse::<f64>() {
                    *a < b_num
                } else {
                    false
                }
            }
            (Value::String(a), Value::Number(b)) => {
                if let Ok(a_num) = a.parse::<f64>() {
                    a_num < *b
                } else {
                    false
                }
            }
            _ => false,
        };
        
        stack.push(Value::Boolean(result));
        Ok(())
    }

    fn execute_le(&mut self, stack: &mut S) -> Result<(), String> {
        let b = stack.pop().ok_or("Stack underflow")?;
        let a = stack.pop().ok_or("Stack underflow")?;
        
        let result = match (&a, &b) {
            (Value::Number(a), Value::Number(b)) => a <= b,
            (Value::String(a), Value::String(b)) => a <= b,
            (Value::Number(a), Value::String(b)) => {
                if let Ok(b_num) = b.parse::<f64>() {
                    *a <= b_num
                } else {
                    false
                }
            }
            (Value::String(a), Value::Number(b)) => {
                if let Ok(a_num) = a.parse::<f64>() {
                    a_num <= *b
                } else {
                    false
                }
            }
            _ => false,
        };
        
        stack.push(Value::Boolean(result));
        Ok(())
    }

    fn execute_gt(&mut self, stack: &mut S) -> Result<(), String> {
        let b = stack.pop().ok_or("Stack underflow")?;
        let a = stack.pop().ok_or("Stack underflow")?;
        
        let result = match (&a, &b) {
            (Value::Number(a), Value::Number(b)) => a > b,
            (Value::String(a), Value::String(b)) => a > b,
            (Value::Number(a), Value::String(b)) => {
                if let Ok(b_num) = b.parse::<f64>() {
                    *a > b_num
                } else {
                    false
                }
            }
            (Value::String(a), Value::Number(b)) => {
                if let Ok(a_num) = a.parse::<f64>() {
                    a_num > *b
                } else {
                    false
                }
            }
            _ => false,
        };
        
        stack.push(Value::Boolean(result));
        Ok(())
    }

    fn execute_ge(&mut self, stack: &mut S) -> Result<(), String> {
        let b = stack.pop().ok_or("Stack underflow")?;
        let a = stack.pop().ok_or("Stack underflow")?;
        
        let result = match (&a, &b) {
            (Value::Number(a), Value::Number(b)) => a >= b,
            (Value::String(a), Value::String(b)) => a >= b,
            (Value::Number(a), Value::String(b)) => {
                if let Ok(b_num) = b.parse::<f64>() {
                    *a >= b_num
                } else {
                    false
                }
            }
            (Value::String(a), Value::Number(b)) => {
                if let Ok(a_num) = a.parse::<f64>() {
                    a_num >= *b
                } else {
                    false
                }
            }
            _ => false,
        };
        
        stack.push(Value::Boolean(result));
        Ok(())
    }

    fn execute_strict_eq(&mut self, stack: &mut S) -> Result<(), String> {
        let b = stack.pop().ok_or("Stack underflow")?;
        let a = stack.pop().ok_or("Stack underflow")?;
        
        let result = match (&a, &b) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Undefined, Value::Undefined) => true,
            _ => false,
        };
        
        stack.push(Value::Boolean(result));
        Ok(())
    }

    fn execute_strict_ne(&mut self, stack: &mut S) -> Result<(), String> {
        let b = stack.pop().ok_or("Stack underflow")?;
        let a = stack.pop().ok_or("Stack underflow")?;
        
        let result = match (&a, &b) {
            (Value::Number(a), Value::Number(b)) => a != b,
            (Value::String(a), Value::String(b)) => a != b,
            (Value::Boolean(a), Value::Boolean(b)) => a != b,
            (Value::Null, Value::Null) => false,
            (Value::Undefined, Value::Undefined) => false,
            _ => true,
        };
        
        stack.push(Value::Boolean(result));
        Ok(())
    }
}
