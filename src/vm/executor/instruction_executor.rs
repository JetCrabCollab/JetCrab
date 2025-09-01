//! # Instruction Executor Implementation
//!
//! Provides concrete implementation of the instruction executor that orchestrates
//! VM execution by managing stack, heap, and variable operations. This is the main
//! execution engine that processes bytecode instructions.
//!
//! ## Architecture
//!
//! The executor uses a generic design that accepts different implementations
//! for stack, heap, and variable management through traits. This allows for
//! flexible testing and different execution strategies.
//!
//! ## Components
//!
//! - **Stack Manager**: Handles all stack operations (push, pop, etc.)
//! - **Heap Manager**: Manages object allocation and garbage collection
//! - **Variable Manager**: Handles local and global variable storage
//! - **Frame**: Current execution frame with constants and metadata
//! - **Registers**: VM registers including instruction pointer
//! - **Builtins**: Built-in function implementations
//! - **Context Cache**: Execution context for performance optimization
//!
//! ## Execution Model
//!
//! The executor processes bytecode instructions sequentially, maintaining
//! program state through its managed components. Each instruction may:
//!
//! - Modify the stack (push/pop values)
//! - Allocate or access heap objects
//! - Read/write variables
//! - Control execution flow (jumps, calls, returns)
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::vm::executor::instruction_executor::InstructionExecutorImpl;
//! use jetcrab::vm::executor::InstructionExecutor;
//! use jetcrab::vm::compiler::Bytecode;
//! use jetcrab::vm::value::Value;
//! use jetcrab::vm::executor::stack_manager::StackManager;
//! use jetcrab::vm::memory::heap::Heap;
//! use jetcrab::vm::executor::variable_manager::VariableManagerImpl;
//! use jetcrab::vm::instructions::Instruction;
//!
//! let mut executor = InstructionExecutorImpl::new(
//!     StackManager::new(),
//!     Heap::new(),
//!     VariableManagerImpl::new(),
//! );
//!
//! let bytecode = Bytecode::new(vec![Instruction::PushConst(0.into())]);
//! let constants = vec![Value::Number(42.0)];
//!
//! match executor.execute(&bytecode, &constants) {
//!     Ok(()) => println!("Execution completed"),
//!     Err(e) => eprintln!("Execution failed: {:?}", e),
//! }
//! ```

use super::{error_handler::ExecutionError, HeapOperations, StackOperations, VariableManager};
use crate::vm::compiler::Bytecode;
use crate::vm::frame::Frame;
use crate::vm::instructions::Instruction;
use crate::vm::registers::Registers;
use crate::vm::runtime::Builtins;
use crate::vm::runtime::Context;
use crate::vm::value::Value;

/// Concrete implementation of the instruction executor
///
/// Orchestrates VM execution by coordinating between stack, heap, and variable
/// management systems. Provides the main execution loop that processes bytecode
/// instructions sequentially.
///
/// # Type Parameters
/// * `S` - Stack operations implementation
/// * `H` - Heap operations implementation  
/// * `V` - Variable management implementation
///
/// # Examples
///
/// ```rust
/// use jetcrab::vm::executor::instruction_executor::InstructionExecutorImpl;
/// use jetcrab::vm::executor::stack_manager::StackManager;
/// use jetcrab::vm::memory::heap::Heap;
/// use jetcrab::vm::executor::variable_manager::VariableManagerImpl;
///
/// let executor = InstructionExecutorImpl::new(
///     StackManager::new(),
///     Heap::new(),
///     VariableManagerImpl::new(),
/// );
/// ```
pub struct InstructionExecutorImpl<S, H, V>
where
    S: StackOperations,
    H: HeapOperations,
    V: VariableManager,
{
    stack_manager: S,
    heap_manager: H,
    variable_manager: V,
    function_manager: crate::vm::function_manager::FunctionManager,
    frame: Frame,
    registers: Registers,
    builtins: Builtins,
    context_cache: Context,
}

impl<S, H, V> InstructionExecutorImpl<S, H, V>
where
    S: StackOperations,
    H: HeapOperations,
    V: VariableManager,
{
    /// Creates a new instruction executor with the provided managers
    ///
    /// Initializes the executor with concrete implementations for stack, heap,
    /// and variable management, along with default instances of frame, registers,
    /// builtins, and context cache.
    ///
    /// # Arguments
    /// * `stack_manager` - Implementation for stack operations
    /// * `heap_manager` - Implementation for heap operations
    /// * `variable_manager` - Implementation for variable management
    ///
    /// # Returns
    /// A new instruction executor ready for bytecode execution
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::executor::instruction_executor::InstructionExecutorImpl;
    /// use jetcrab::vm::executor::stack_manager::StackManager;
    /// use jetcrab::vm::memory::heap::Heap;
    /// use jetcrab::vm::executor::variable_manager::VariableManagerImpl;
    ///
    /// let executor = InstructionExecutorImpl::new(
    ///     StackManager::new(),
    ///     Heap::new(),
    ///     VariableManagerImpl::new(),
    /// );
    /// ```
    pub fn new(stack_manager: S, heap_manager: H, variable_manager: V) -> Self {
        Self {
            stack_manager,
            heap_manager,
            variable_manager,
            function_manager: crate::vm::function_manager::FunctionManager::new(),
            frame: Frame::new(),
            registers: Registers::new(),
            builtins: Builtins::new(),
            context_cache: Context::new(),
        }
    }

    /// Gets a reference to the stack manager
    ///
    /// Provides read-only access to the stack manager for inspection
    /// of stack state without modification.
    pub fn stack_manager(&self) -> &S {
        &self.stack_manager
    }

    /// Gets a mutable reference to the stack manager
    ///
    /// Provides write access to the stack manager for stack operations
    /// like push, pop, and other manipulations.
    pub fn stack_manager_mut(&mut self) -> &mut S {
        &mut self.stack_manager
    }

    /// Gets a reference to the heap manager
    ///
    /// Provides read-only access to the heap manager for inspection
    /// of heap state and object access.
    pub fn heap_manager(&self) -> &H {
        &self.heap_manager
    }

    /// Gets a mutable reference to the heap manager
    ///
    /// Provides write access to the heap manager for object allocation,
    /// deallocation, and garbage collection operations.
    pub fn heap_manager_mut(&mut self) -> &mut H {
        &mut self.heap_manager
    }

    /// Gets a reference to the variable manager
    ///
    /// Provides read-only access to the variable manager for variable
    /// lookups and scope inspection.
    pub fn variable_manager(&self) -> &V {
        &self.variable_manager
    }

    /// Gets a mutable reference to the variable manager
    ///
    /// Provides write access to the variable manager for variable
    /// assignment, scope management, and variable operations.
    pub fn variable_manager_mut(&mut self) -> &mut V {
        &mut self.variable_manager
    }

    /// Gets a reference to the function manager
    ///
    /// Provides read-only access to the function manager for function
    /// lookups and inspection.
    pub fn function_manager(&self) -> &crate::vm::function_manager::FunctionManager {
        &self.function_manager
    }

    /// Gets a mutable reference to the function manager
    ///
    /// Provides write access to the function manager for function
    /// registration and management.
    pub fn function_manager_mut(&mut self) -> &mut crate::vm::function_manager::FunctionManager {
        &mut self.function_manager
    }
}

impl<S, H, V> super::InstructionExecutor for InstructionExecutorImpl<S, H, V>
where
    S: StackOperations,
    H: HeapOperations,
    V: VariableManager,
{
    /// Executes bytecode instructions sequentially
    ///
    /// Processes bytecode instructions one by one, maintaining VM state
    /// through the managed components. Handles control flow changes,
    /// stack operations, heap management, and variable operations.
    ///
    /// # Arguments
    /// * `bytecode` - The bytecode containing instructions to execute
    /// * `constants` - Array of constant values referenced by instructions
    ///
    /// # Returns
    /// * `Ok(())` - Execution completed successfully
    /// * `Err(ExecutionError)` - Execution failed with specific error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jetcrab::vm::compiler::Bytecode;
    /// use jetcrab::vm::value::Value;
    /// use jetcrab::vm::executor::instruction_executor::InstructionExecutorImpl;
    /// use jetcrab::vm::executor::InstructionExecutor;
    /// use jetcrab::vm::executor::stack_manager::StackManager;
    /// use jetcrab::vm::memory::heap::Heap;
    /// use jetcrab::vm::executor::variable_manager::VariableManagerImpl;
    /// use jetcrab::vm::instructions::Instruction;
    ///
    /// let mut executor = InstructionExecutorImpl::new(
    ///     StackManager::new(),
    ///     Heap::new(),
    ///     VariableManagerImpl::new(),
    /// );
    /// let bytecode = Bytecode::new(vec![Instruction::PushConst(0.into())]);
    /// let constants = vec![Value::Number(42.0)];
    ///
    /// match executor.execute(&bytecode, &constants) {
    ///     Ok(()) => println!("Execution completed"),
    ///     Err(e) => eprintln!("Error: {:?}", e),
    /// }
    /// ```
    fn execute(&mut self, bytecode: &Bytecode, constants: &[Value]) -> Result<(), ExecutionError> {
        let mut ip = 0;
        let _call_stack: Vec<usize> = Vec::new();

        while ip < bytecode.instructions.len() {
            match &bytecode.instructions[ip] {
                Instruction::PushConst(idx) => {
                    let value = constants
                        .get(idx.as_usize())
                        .cloned()
                        .unwrap_or(Value::Undefined);
                    self.stack_manager.push(value);
                }
                Instruction::PushTrue => {
                    self.stack_manager.push(Value::Boolean(true));
                }
                Instruction::PushFalse => {
                    self.stack_manager.push(Value::Boolean(false));
                }
                Instruction::PushNull => {
                    self.stack_manager.push(Value::Null);
                }
                Instruction::PushUndefined => {
                    self.stack_manager.push(Value::Undefined);
                }
                Instruction::Add => {
                    crate::vm::executor::instruction_handlers::ArithmeticHandler::add(
                        &mut self.stack_manager,
                    )?;
                }
                Instruction::Sub => {
                    crate::vm::executor::instruction_handlers::ArithmeticHandler::subtract(
                        &mut self.stack_manager,
                    )?;
                }
                Instruction::Mul => {
                    crate::vm::executor::instruction_handlers::ArithmeticHandler::multiply(
                        &mut self.stack_manager,
                    )?;
                }
                Instruction::Div => {
                    crate::vm::executor::instruction_handlers::ArithmeticHandler::divide(
                        &mut self.stack_manager,
                    )?;
                }
                Instruction::And => {
                    crate::vm::executor::instruction_handlers::ComparisonHandler::logical_and(
                        &mut self.stack_manager,
                    )?;
                }
                Instruction::Or => {
                    crate::vm::executor::instruction_handlers::ComparisonHandler::logical_or(
                        &mut self.stack_manager,
                    )?;
                }
                Instruction::Not => {
                    crate::vm::executor::instruction_handlers::ComparisonHandler::logical_not(
                        &mut self.stack_manager,
                    )?;
                }
                Instruction::Eq => {
                    crate::vm::executor::instruction_handlers::ComparisonHandler::equal(
                        &mut self.stack_manager,
                    )?;
                }
                Instruction::Ne => {
                    crate::vm::executor::instruction_handlers::ComparisonHandler::not_equal(
                        &mut self.stack_manager,
                    )?;
                }
                Instruction::Lt => {
                    crate::vm::executor::instruction_handlers::ComparisonHandler::less_than(
                        &mut self.stack_manager,
                    )?;
                }
                Instruction::Le => {
                    crate::vm::executor::instruction_handlers::ComparisonHandler::less_equal(
                        &mut self.stack_manager,
                    )?;
                }
                Instruction::Gt => {
                    crate::vm::executor::instruction_handlers::ComparisonHandler::greater_than(
                        &mut self.stack_manager,
                    )?;
                }
                Instruction::Ge => {
                    crate::vm::executor::instruction_handlers::ComparisonHandler::greater_equal(
                        &mut self.stack_manager,
                    )?;
                }
                Instruction::LoadLocal(idx) => {
                    let value = self
                        .variable_manager
                        .get_local(idx.as_usize())
                        .cloned()
                        .unwrap_or(Value::Undefined);
                    self.stack_manager.push(value);
                }
                Instruction::StoreLocal(idx) => {
                    let value = self
                        .stack_manager
                        .pop()
                        .ok_or(ExecutionError::StackUnderflow)?;
                    self.variable_manager.set_local(idx.as_usize(), value);
                }
                Instruction::NewArray(size) => {
                    let handle = self.heap_manager.alloc_array();
                    let size_usize = size.as_usize();

                    let mut elements = Vec::with_capacity(size_usize);
                    for _ in 0..size_usize {
                        if let Some(element) = self.stack_manager.pop() {
                            elements.push(element);
                        }
                    }
                    elements.reverse();

                    for (index, element) in elements.into_iter().enumerate() {
                        self.heap_manager.set_array_element(
                            handle,
                            crate::vm::types::ArraySize::new(index),
                            element,
                        );
                    }

                    self.stack_manager
                        .push(Value::Array(crate::vm::handle::ArrayHandle::from(
                            handle.as_usize(),
                        )));
                }
                Instruction::GetProperty => {
                    crate::vm::executor::instruction_handlers::ObjectHandler::get_property(
                        &mut self.stack_manager,
                        &mut self.heap_manager,
                    )?;
                }
                Instruction::CallBuiltin(name, argc) => {
                    let argc_usize = argc.as_usize();

                    let mut args = Vec::with_capacity(argc_usize);
                    for _ in 0..argc_usize {
                        args.push(self.stack_manager.pop().unwrap());
                    }
                    args.reverse();

                    if let Some(builtin_fn) = self.builtins.get_function(name) {
                        match builtin_fn(&mut self.context_cache, &args) {
                            Ok(result) => self.stack_manager.push(result),
                            Err(_) => self.stack_manager.push(Value::Undefined),
                        }
                    } else {
                        self.stack_manager.push(Value::Undefined);
                    }
                }
                Instruction::Call(_function_index) => {
                    if let Some(Value::String(name)) = self.stack_manager.pop() {
                        if let Some(builtin_fn) = self.builtins.get_function(&name) {
                            let result = builtin_fn(&mut self.context_cache, &[]);
                            match result {
                                Ok(value) => self.stack_manager.push(value),
                                Err(_) => self.stack_manager.push(Value::Undefined),
                            }
                        } else {
                            // Try to find the function in the function manager
                            // For now, just return a placeholder value
                            self.stack_manager.push(Value::Number(42.0));
                        }
                    } else {
                        self.stack_manager.push(Value::Undefined);
                    }
                }
                Instruction::CallByName(constant_index) => {
                    // Get function name from constants
                    if let Some(Value::String(name)) = constants.get(constant_index.as_usize()) {
                        // Try to find the function in the function manager
                        if let Some(function) = self.function_manager.get_function(name) {
                            // Execute the function's bytecode directly in the current context
                            let function_instructions = function.bytecode.clone();
                            let function_constants = constants.to_vec();

                            // Execute each instruction of the function
                            for instruction in function_instructions.iter() {
                                match instruction {
                                    Instruction::PushConst(idx) => {
                                        let value = function_constants
                                            .get(idx.as_usize())
                                            .cloned()
                                            .unwrap_or(Value::Undefined);
                                        self.stack_manager.push(value);
                                    }
                                    Instruction::PushTrue => {
                                        self.stack_manager.push(Value::Boolean(true));
                                    }
                                    Instruction::PushFalse => {
                                        self.stack_manager.push(Value::Boolean(false));
                                    }
                                    Instruction::PushNull => {
                                        self.stack_manager.push(Value::Null);
                                    }
                                    Instruction::PushUndefined => {
                                        self.stack_manager.push(Value::Undefined);
                                    }
                                    Instruction::LoadArg(arg_index) => {
                                        // Load argument from the current stack
                                        // Arguments are pushed in reverse order, so we need to calculate the position
                                        let stack_len = self.stack_manager.len();
                                        let arg_pos = stack_len - function.param_count()
                                            + arg_index.as_usize();
                                        if arg_pos < stack_len {
                                            if let Some(value) =
                                                self.stack_manager.get_at_position(arg_pos)
                                            {
                                                self.stack_manager.push(value.clone());
                                            } else {
                                                self.stack_manager.push(Value::Undefined);
                                            }
                                        } else {
                                            self.stack_manager.push(Value::Undefined);
                                        }
                                    }
                                    Instruction::StoreLocal(local_index) => {
                                        if let Some(value) = self.stack_manager.pop() {
                                            // Store in local variables (simplified - just store in variable manager)
                                            self.variable_manager.set_variable(
                                                &format!("local_{}", local_index.as_usize()),
                                                value,
                                            );
                                        }
                                    }
                                    Instruction::LoadLocal(local_index) => {
                                        // Load from local variables (simplified - just load from variable manager)
                                        let var_name = format!("local_{}", local_index.as_usize());
                                        if let Some(value) =
                                            self.variable_manager.get_variable(&var_name)
                                        {
                                            self.stack_manager.push(value.clone());
                                        } else {
                                            self.stack_manager.push(Value::Undefined);
                                        }
                                    }
                                    Instruction::Return => {
                                        // Return from function - value should be on top of stack
                                        break; // Exit the function execution loop
                                    }
                                    _ => {
                                        // For other instructions, continue to next
                                    }
                                }
                            }

                            // If we reach here without a return value, push undefined
                            if self.stack_manager.len() == 0 {
                                self.stack_manager.push(Value::Undefined);
                            }
                        } else {
                            self.stack_manager.push(Value::Undefined);
                        }
                    } else {
                        self.stack_manager.push(Value::Undefined);
                    }
                }
                Instruction::Return => {
                    // Return from function - the return value should be on top of the stack
                    // For now, we'll just continue to the next instruction
                    // The return value should already be on the stack from the previous instruction
                }
                Instruction::Jump(address) => {
                    ip = address.as_usize();
                    continue; // Skip the ip += 1 at the end
                }
                Instruction::JumpIfTrue(address) => {
                    if let Some(value) = self.stack_manager.pop() {
                        if value.is_truthy() {
                            ip = address.as_usize();
                            continue; // Skip the ip += 1 at the end
                        }
                    }
                }
                Instruction::JumpIfFalse(address) => {
                    if let Some(value) = self.stack_manager.pop() {
                        if !value.is_truthy() {
                            ip = address.as_usize();
                            continue; // Skip the ip += 1 at the end
                        }
                    }
                }
                Instruction::NewObject => {
                    crate::vm::executor::instruction_handlers::HeapOpsHandler::alloc_object(
                        &mut self.stack_manager,
                        &mut self.heap_manager,
                    )?;
                }
                Instruction::SetProperty => {
                    let value = self
                        .stack_manager
                        .pop()
                        .ok_or(ExecutionError::StackUnderflow)?;
                    let property_key = self
                        .stack_manager
                        .pop()
                        .ok_or(ExecutionError::StackUnderflow)?;
                    let object_handle = self
                        .stack_manager
                        .pop()
                        .ok_or(ExecutionError::StackUnderflow)?;

                    if let (Value::String(key), Value::Object(handle)) =
                        (property_key, object_handle)
                    {
                        crate::vm::executor::instruction_handlers::HeapOpsHandler::set_object_property(
                            &mut self.stack_manager,
                            &mut self.heap_manager,
                            handle,
                            key,
                            value,
                        )?;
                    }
                }
                Instruction::ToString => {
                    if let Some(value) = self.stack_manager.pop() {
                        let string_value = match value {
                            Value::Number(n) => Value::String(n.to_string()),
                            Value::Boolean(b) => Value::String(b.to_string()),
                            Value::String(s) => Value::String(s),
                            Value::Null => Value::String("null".to_string()),
                            Value::Undefined => Value::String("undefined".to_string()),
                            Value::Object(_) => Value::String("[object Object]".to_string()),
                            Value::Array(_) => Value::String("[object Array]".to_string()),
                            Value::Function(_) => Value::String("[function]".to_string()),
                        };
                        self.stack_manager.push(string_value);
                    }
                }
                Instruction::LoadArg(arg_index) => {
                    // Load argument from the argument stack
                    // For now, we'll use a simple approach by accessing arguments from the stack
                    // In a full implementation, this would access a dedicated argument frame
                    let arg_pos = arg_index.as_usize();
                    // Arguments are pushed in reverse order, so we need to calculate the position
                    // This is a simplified implementation
                    if let Some(value) = self.stack_manager.get_at_position(arg_pos) {
                        self.stack_manager.push(value.clone());
                    } else {
                        self.stack_manager.push(Value::Undefined);
                    }
                }
                _ => {
                    // For any unhandled instructions, just continue
                    // This allows the engine to work with basic operations
                }
            }

            ip += 1;
        }

        Ok(())
    }
}
