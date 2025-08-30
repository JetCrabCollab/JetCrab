use super::StackOperations;

pub trait StackUtilityOperations<S>
where
    S: StackOperations,
{
    fn execute_dup(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_pop(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_swap(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_rot(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_over(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_drop(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_nip(&mut self, stack: &mut S) -> Result<(), String>;
    fn execute_tuck(&mut self, stack: &mut S) -> Result<(), String>;
}

pub struct StackUtilityExecutor;

impl StackUtilityExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl<S> StackUtilityOperations<S> for StackUtilityExecutor
where
    S: StackOperations,
{
    fn execute_dup(&mut self, stack: &mut S) -> Result<(), String> {
        let value = stack.peek().ok_or("Stack underflow")?.clone();
        stack.push(value);
        Ok(())
    }

    fn execute_pop(&mut self, stack: &mut S) -> Result<(), String> {
        stack.pop().ok_or("Stack underflow")?;
        Ok(())
    }

    fn execute_swap(&mut self, stack: &mut S) -> Result<(), String> {
        let a = stack.pop().ok_or("Stack underflow")?;
        let b = stack.pop().ok_or("Stack underflow")?;
        stack.push(a);
        stack.push(b);
        Ok(())
    }

    fn execute_rot(&mut self, stack: &mut S) -> Result<(), String> {
        let c = stack.pop().ok_or("Stack underflow")?;
        let b = stack.pop().ok_or("Stack underflow")?;
        let a = stack.pop().ok_or("Stack underflow")?;
        stack.push(b);
        stack.push(c);
        stack.push(a);
        Ok(())
    }

    fn execute_over(&mut self, stack: &mut S) -> Result<(), String> {
        let a = stack.peek().ok_or("Stack underflow")?.clone();
        stack.push(a);
        Ok(())
    }

    fn execute_drop(&mut self, stack: &mut S) -> Result<(), String> {
        stack.pop().ok_or("Stack underflow")?;
        Ok(())
    }

    fn execute_nip(&mut self, stack: &mut S) -> Result<(), String> {
        let top = stack.pop().ok_or("Stack underflow")?;
        stack.pop().ok_or("Stack underflow")?;
        stack.push(top);
        Ok(())
    }

    fn execute_tuck(&mut self, stack: &mut S) -> Result<(), String> {
        let a = stack.pop().ok_or("Stack underflow")?;
        let b = stack.pop().ok_or("Stack underflow")?;
        stack.push(a.clone());
        stack.push(b);
        stack.push(a);
        Ok(())
    }
}
