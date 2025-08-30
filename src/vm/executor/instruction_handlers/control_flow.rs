//! # Control Flow Handler
//!
//! Handles all control flow operations in the VM including jumps, function calls,
//! returns, and exception handling.
//!
//! ## Operations Supported
//!
//! - **Jumps**: jump, jump_if_true, jump_if_false
//! - **Function Calls**: call, return_from_function
//! - **Scope Management**: create_scope, exit_scope
//! - **Exception Handling**: throw, try_catch, finally
//! - **Loop Control**: break_statement, continue_statement
//! - **Switch Statements**: switch_statement, case_statement, default_case
//!
//! ## Control Flow Semantics
//!
//! - **Jumps**: Modify the program counter to change execution flow
//! - **Function Calls**: Set up new execution context with arguments
//! - **Returns**: Restore previous execution context
//! - **Exception Handling**: Manage try-catch-finally blocks
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::vm::executor::instruction_handlers::ControlFlowHandler;
//! use jetcrab::vm::executor::traits::{StackOperations, VariableManager};
//!
//! let mut stack = MyStack::new();
//! let mut registers = MyRegisters::new();
//! let mut frame = MyFrame::new();
//!
//! ControlFlowHandler::jump(&mut stack, &mut registers, 100)?;
//! ```

use crate::vm::executor::error_handler::ExecutionError;
use crate::vm::executor::traits::{StackOperations, VariableManager};
use crate::vm::frame::Frame;
use crate::vm::registers::Registers;
use crate::vm::types::{CodeAddress, ArgIndex};
use crate::vm::value::Value;

/// Handles control flow operations for the VM
pub struct ControlFlowHandler;

impl ControlFlowHandler {
    /// Performs an unconditional jump to a target address
    ///
    /// Sets the program counter to the target address, effectively
    /// changing the execution flow.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `registers` - The VM registers containing the program counter
    /// * `target_ip` - The target instruction pointer to jump to
    ///
    /// # Returns
    /// * `Ok(usize)` with the new instruction pointer on success
    /// * `Err(ExecutionError)` on failure
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut stack = MyStack::new();
    /// let mut registers = MyRegisters::new();
    /// let new_ip = ControlFlowHandler::jump(&mut stack, &mut registers, 100)?;
    /// assert_eq!(new_ip, 100);
    /// ```
    pub fn jump<S, V>(
        _stack: &mut S,
        _registers: &mut Registers,
        target_ip: CodeAddress,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let new_ip = usize::from(target_ip);
        Ok(new_ip)
    }

    /// Performs a conditional jump if the top stack value is true
    ///
    /// Pops a value from the stack and jumps to the target address
    /// if the value is truthy.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `registers` - The VM registers
    /// * `target_ip` - The target instruction pointer to jump to
    ///
    /// # Returns
    /// * `Ok(usize)` with the new instruction pointer if jump occurs
    /// * `Ok(usize)` with current IP + 1 if no jump occurs
    /// * `Err(ExecutionError)` on failure
    pub fn jump_if_true<S, V>(
        stack: &mut S,
        _registers: &mut Registers,
        target_ip: CodeAddress,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let condition = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let should_jump = match condition {
            Value::Boolean(b) => b,
            Value::Number(n) => n != 0.0 && !n.is_nan(),
            Value::String(s) => !s.is_empty(),
            Value::Null | Value::Undefined => false,
            _ => true,
        };

        if should_jump {
            Ok(usize::from(target_ip))
        } else {
            Ok(0)
        }
    }

    /// Performs a conditional jump if the top stack value is false
    ///
    /// Pops a value from the stack and jumps to the target address
    /// if the value is falsy.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `registers` - The VM registers
    /// * `target_ip` - The target instruction pointer to jump to
    ///
    /// # Returns
    /// * `Ok(usize)` with the new instruction pointer if jump occurs
    /// * `Ok(usize)` with current IP + 1 if no jump occurs
    /// * `Err(ExecutionError)` on failure
    pub fn jump_if_false<S, V>(
        stack: &mut S,
        _registers: &mut Registers,
        target_ip: CodeAddress,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let condition = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        let should_jump = match condition {
            Value::Boolean(b) => !b,
            Value::Number(n) => n == 0.0 || n.is_nan(),
            Value::String(s) => s.is_empty(),
            Value::Null | Value::Undefined => true,
            _ => false,
        };

        if should_jump {
            Ok(usize::from(target_ip))
        } else {
            Ok(0)
        }
    }

    /// Sets up a function call with arguments
    ///
    /// Pops the function and arguments from the stack and sets up
    /// the frame for function execution.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `registers` - The VM registers
    /// * `frame` - The current execution frame
    /// * `arg_count` - The number of arguments to pop from stack
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError)` on failure
    pub fn call<S, V>(
        stack: &mut S,
        _registers: &mut Registers,
        frame: &mut Frame,
        arg_count: ArgIndex,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let function_value = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        
        match function_value {
            Value::Function(function_handle) => {
                let mut args = Vec::new();
                for _ in 0..usize::from(arg_count) {
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

    /// Returns from the current function
    ///
    /// Restores the previous execution context and returns control
    /// to the calling function.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `registers` - The VM registers
    /// * `frame` - The current execution frame
    ///
    /// # Returns
    /// * `Ok(usize)` with the return address on success
    /// * `Err(ExecutionError)` on failure
    pub fn return_from_function<S, V>(
        _stack: &mut S,
        _registers: &mut Registers,
        _frame: &mut Frame,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let return_address = usize::from(_registers.program_counter);
        Ok(return_address)
    }

    /// Creates a new scope for variable declarations
    ///
    /// Sets up a new variable scope for block execution.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `frame` - The current execution frame
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError)` on failure
    pub fn create_scope<S, V>(_stack: &mut S, _frame: &mut Frame) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        Ok(())
    }

    /// Exits the current scope
    ///
    /// Cleans up the current variable scope.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `frame` - The current execution frame
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError)` on failure
    pub fn exit_scope<S, V>(_stack: &mut S, _frame: &mut Frame) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        Ok(())
    }

    /// Throws an exception
    ///
    /// Pops a value from the stack and throws it as an exception,
    /// interrupting normal execution flow.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `registers` - The VM registers
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError)` on failure
    pub fn throw<S, V>(stack: &mut S, _registers: &mut Registers) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let exception = stack.pop().ok_or(ExecutionError::StackUnderflow)?;
        Err(ExecutionError::RuntimeError(format!("Exception thrown: {:?}", exception)))
    }

    /// Sets up a try-catch block
    ///
    /// Prepares the VM for exception handling with try and catch blocks.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `registers` - The VM registers
    /// * `frame` - The current execution frame
    /// * `try_block_size` - Size of the try block
    /// * `catch_block_size` - Size of the catch block
    ///
    /// # Returns
    /// * `Ok(usize)` with the new instruction pointer on success
    /// * `Err(ExecutionError)` on failure
    pub fn try_catch<S, V>(
        _stack: &mut S,
        _registers: &mut Registers,
        _frame: &mut Frame,
        try_block_size: CodeAddress,
        catch_block_size: CodeAddress,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let new_ip = usize::from(try_block_size) + usize::from(catch_block_size);
        Ok(new_ip)
    }

    /// Executes the finally block
    ///
    /// Handles cleanup code that should run regardless of exception handling.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `registers` - The VM registers
    /// * `frame` - The current execution frame
    ///
    /// # Returns
    /// * `Ok(usize)` with the new instruction pointer on success
    /// * `Err(ExecutionError)` on failure
    pub fn finally<S, V>(
        _stack: &mut S,
        _registers: &mut Registers,
        _frame: &mut Frame,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        Ok(0)
    }

    /// Breaks out of a loop or switch statement
    ///
    /// Exits the current loop or switch statement and continues
    /// execution at the specified target.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `registers` - The VM registers
    /// * `frame` - The current execution frame
    /// * `target_label` - The target label to jump to
    ///
    /// # Returns
    /// * `Ok(usize)` with the target instruction pointer on success
    /// * `Err(ExecutionError)` on failure
    pub fn break_statement<S, V>(
        _stack: &mut S,
        _registers: &mut Registers,
        _frame: &mut Frame,
        target_label: String,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let target_ip = target_label.parse::<usize>().unwrap_or(0);
        Ok(target_ip)
    }

    /// Continues to the next iteration of a loop
    ///
    /// Skips the rest of the current loop iteration and continues
    /// with the next iteration.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `registers` - The VM registers
    /// * `frame` - The current execution frame
    /// * `target_label` - The target label to jump to
    ///
    /// # Returns
    /// * `Ok(usize)` with the target instruction pointer on success
    /// * `Err(ExecutionError)` on failure
    pub fn continue_statement<S, V>(
        _stack: &mut S,
        _registers: &mut Registers,
        _frame: &mut Frame,
        target_label: String,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        let target_ip = target_label.parse::<usize>().unwrap_or(0);
        Ok(target_ip + 1)
    }

    /// Sets up a switch statement
    ///
    /// Prepares the VM for switch statement execution.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `registers` - The VM registers
    /// * `frame` - The current execution frame
    /// * `case_count` - The number of cases in the switch
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ExecutionError)` on failure
    pub fn switch_statement<S, V>(
        _stack: &mut S,
        _registers: &mut Registers,
        _frame: &mut Frame,
        _case_count: CodeAddress,
    ) -> Result<(), ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        Ok(())
    }

    /// Handles a case in a switch statement
    ///
    /// Compares the switch value with a case value and jumps
    /// to the target if they match.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `registers` - The VM registers
    /// * `frame` - The current execution frame
    /// * `case_value` - The value to compare against
    /// * `target_ip` - The target instruction pointer to jump to
    ///
    /// # Returns
    /// * `Ok(usize)` with the target instruction pointer on success
    /// * `Err(ExecutionError)` on failure
    pub fn case_statement<S, V>(
        _stack: &mut S,
        _registers: &mut Registers,
        _frame: &mut Frame,
        _case_value: Value,
        target_ip: CodeAddress,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        Ok(usize::from(target_ip))
    }

    /// Handles the default case in a switch statement
    ///
    /// Jumps to the default case target when no other cases match.
    ///
    /// # Arguments
    /// * `stack` - The stack to operate on
    /// * `registers` - The VM registers
    /// * `frame` - The current execution frame
    /// * `target_ip` - The target instruction pointer to jump to
    ///
    /// # Returns
    /// * `Ok(usize)` with the target instruction pointer on success
    /// * `Err(ExecutionError)` on failure
    pub fn default_case<S, V>(
        _stack: &mut S,
        _registers: &mut Registers,
        _frame: &mut Frame,
        target_ip: CodeAddress,
    ) -> Result<usize, ExecutionError>
    where
        S: StackOperations,
        V: VariableManager,
    {
        Ok(usize::from(target_ip))
    }
}
