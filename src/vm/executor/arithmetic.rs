use super::StackOperations;
use crate::vm::value::Value;

pub trait ArithmeticOperations<S>
where
    S: StackOperations,
{
    fn execute_add(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_sub(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_mul(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_div(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_mod(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_pow(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_neg(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_inc(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_dec(&mut self, stack: &mut S) -> Result<(), String>;
}

pub struct ArithmeticExecutor;

impl ArithmeticExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl<S> ArithmeticOperations<S> for ArithmeticExecutor
where
    S: StackOperations,
{
    fn execute_add(&mut self, stack: &mut S) -> Result<(), String> {
        let b = stack.pop().ok_or("Stack underflow")?;
        let a = stack.pop().ok_or("Stack underflow")?;

        match (a.clone(), b.clone()) {
            (Value::Number(a), Value::Number(b)) => {
                stack.push(Value::Number(a + b));
            }
            _ => {
                let a_str = a.to_string();
                let b_str = b.to_string();
                stack.push(Value::String(format!("{a_str}{b_str}")));
            }
        }
        Ok(())
    }

    fn execute_sub(&mut self, stack: &mut S) -> Result<(), String> {
        let b = stack.pop().ok_or("Stack underflow")?;
        let a = stack.pop().ok_or("Stack underflow")?;

        match (a, b) {
            (Value::Number(a), Value::Number(b)) => {
                stack.push(Value::Number(a - b));
            }
            _ => {
                stack.push(Value::Number(f64::NAN));
            }
        }
        Ok(())
    }

    fn execute_mul(&mut self, stack: &mut S) -> Result<(), String> {
        let b = stack.pop().ok_or("Stack underflow")?;
        let a = stack.pop().ok_or("Stack underflow")?;

        match (a, b) {
            (Value::Number(a), Value::Number(b)) => {
                stack.push(Value::Number(a * b));
            }
            _ => {
                stack.push(Value::Number(f64::NAN));
            }
        }
        Ok(())
    }

    fn execute_div(&mut self, stack: &mut S) -> Result<(), String> {
        let b = stack.pop().ok_or("Stack underflow")?;
        let a = stack.pop().ok_or("Stack underflow")?;

        match (a, b) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    stack.push(Value::Number(f64::INFINITY));
                } else {
                    stack.push(Value::Number(a / b));
                }
            }
            _ => {
                stack.push(Value::Number(f64::NAN));
            }
        }
        Ok(())
    }

    fn execute_mod(&mut self, stack: &mut S) -> Result<(), String> {
        let b = stack.pop().ok_or("Stack underflow")?;
        let a = stack.pop().ok_or("Stack underflow")?;

        match (a, b) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    stack.push(Value::Number(f64::NAN));
                } else {
                    stack.push(Value::Number(a % b));
                }
            }
            _ => {
                stack.push(Value::Number(f64::NAN));
            }
        }
        Ok(())
    }

    fn execute_pow(&mut self, stack: &mut S) -> Result<(), String> {
        let b = stack.pop().ok_or("Stack underflow")?;
        let a = stack.pop().ok_or("Stack underflow")?;

        match (a, b) {
            (Value::Number(a), Value::Number(b)) => {
                stack.push(Value::Number(a.powf(b)));
            }
            _ => {
                stack.push(Value::Number(f64::NAN));
            }
        }
        Ok(())
    }

    fn execute_neg(&mut self, stack: &mut S) -> Result<(), String> {
        let a = stack.pop().ok_or("Stack underflow")?;

        match a {
            Value::Number(a) => {
                stack.push(Value::Number(-a));
            }
            _ => {
                stack.push(Value::Number(f64::NAN));
            }
        }
        Ok(())
    }

    fn execute_inc(&mut self, stack: &mut S) -> Result<(), String> {
        let a = stack.pop().ok_or("Stack underflow")?;

        match a {
            Value::Number(a) => {
                stack.push(Value::Number(a + 1.0));
            }
            _ => {
                stack.push(Value::Number(f64::NAN));
            }
        }
        Ok(())
    }

    fn execute_dec(&mut self, stack: &mut S) -> Result<(), String> {
        let a = stack.pop().ok_or("Stack underflow")?;

        match a {
            Value::Number(a) => {
                stack.push(Value::Number(a - 1.0));
            }
            _ => {
                stack.push(Value::Number(f64::NAN));
            }
        }
        Ok(())
    }
}
