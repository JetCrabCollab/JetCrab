use crate::vm::executor::error_handler::ExecutionError;
use crate::vm::executor::traits::{StackOperations, VariableManager};
use crate::vm::frame::Frame;
use crate::vm::registers::Registers;
use crate::vm::value::Value;

pub struct ControlFlowHandler;

impl ControlFlowHandler {
    pub fn jump<S, V>(
        _stack: &mut S,
        _registers: &mut Registers,
        target_ip: usize,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        Ok(target_ip)
    }

    pub fn jump_if_false<S, V>(
        stack: &mut S,
        _registers: &mut Registers,
        target_ip: usize,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let condition = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let should_jump = match condition {
            Value::Boolean(val) => !val,
            Value::Number(val) => val == 0.0,
            Value::String(val) => val.is_empty(),
            Value::Null | Value::Undefined => true,
            _ => false,
        };

        if should_jump {
            Ok(target_ip)
        } else {
            Ok(target_ip + 1)
        }
    }

    pub fn jump_if_true<S, V>(
        stack: &mut S,
        _registers: &mut Registers,
        target_ip: usize,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let condition = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        let should_jump = match condition {
            Value::Boolean(val) => val,
            Value::Number(val) => val != 0.0,
            Value::String(val) => !val.is_empty(),
            Value::Null | Value::Undefined => false,
            _ => true,
        };

        if should_jump {
            Ok(target_ip)
        } else {
            Ok(target_ip + 1)
        }
    }

    pub fn call<S, V>(
        stack: &mut S,
        registers: &mut Registers,
        frame: &mut Frame,
        arg_count: usize,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let function_value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        match function_value {
            Value::Function(function_handle) => {
                let mut args = Vec::new();
                for _ in 0..arg_count {
                    args.push(stack.pop().ok_or(ExecutionError::StackUnderflow)?);
                }
                args.reverse();
                
                frame.arguments = args;
                frame.function_handle = Some(function_handle);
                Ok(())
            }
            _ => Err(ExecutionError::RuntimeError("Cannot call non-function value".to_string())),
        }
    }

    pub fn return_from_function<S, V>(
        stack: &mut S,
        registers: &mut Registers,
        _frame: &mut Frame,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let return_value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let return_address = usize::from(registers.program_counter);
        stack.push(return_value);
        Ok(return_address)
    }

    pub fn create_scope<S, V>(_stack: &mut S, _frame: &mut Frame) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        Ok(())
    }

    pub fn exit_scope<S, V>(_stack: &mut S, _frame: &mut Frame) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        Ok(())
    }

    pub fn throw<S, V>(stack: &mut S, _registers: &mut Registers) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let error_value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;

        Err(ExecutionError::RuntimeError(format!(
            "Thrown: {:?}",
            error_value
        )))
    }

    pub fn try_catch<S, V>(
        _stack: &mut S,
        registers: &mut Registers,
        _frame: &mut Frame,
        try_block_size: usize,
        _catch_block_size: usize,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let current_pc = usize::from(registers.program_counter);
        Ok(current_pc + try_block_size)
    }

    pub fn finally<S, V>(
        _stack: &mut S,
        registers: &mut Registers,
        _frame: &mut Frame,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let current_pc = usize::from(registers.program_counter);
        Ok(current_pc + 1)
    }

    pub fn break_statement<S, V>(
        _stack: &mut S,
        _registers: &mut Registers,
        _frame: &mut Frame,
        _target_label: Option<String>,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        Ok(0)
    }

    pub fn continue_statement<S, V>(
        _stack: &mut S,
        _registers: &mut Registers,
        _frame: &mut Frame,
        _target_label: Option<String>,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        Ok(0)
    }

    pub fn switch_statement<S, V>(
        stack: &mut S,
        _registers: &mut Registers,
        _frame: &mut Frame,
        _case_count: usize,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let _switch_value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        Ok(())
    }

    pub fn case_statement<S, V>(
        _stack: &mut S,
        _registers: &mut Registers,
        _frame: &mut Frame,
        _case_value: Value,
        target_ip: usize,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        Ok(target_ip)
    }

    pub fn default_case<S, V>(
        _stack: &mut S,
        _registers: &mut Registers,
        _frame: &mut Frame,
        target_ip: usize,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        Ok(target_ip)
    }
}
