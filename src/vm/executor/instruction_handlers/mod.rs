//! # Instruction Handlers Module
//!
//! This module contains specialized handlers for different types of VM instructions.
//! Each handler is responsible for executing a specific category of operations.
//!
//! ## Handlers
//!
//! - **ArithmeticHandler**: Mathematical operations (add, subtract, multiply, etc.)
//! - **ComparisonHandler**: Comparison and logical operations
//! - **ControlFlowHandler**: Control flow operations (jumps, calls, returns)
//! - **StackOpsHandler**: Stack manipulation operations
//! - **HeapOpsHandler**: Memory allocation and management
//! - **BuiltinCallsHandler**: Built-in function calls
//!
//! ## Design Principles
//!
//! Each handler follows these principles:
//!
//! - **Single Responsibility**: Each handler handles one category of operations
//! - **Generic Implementation**: Uses trait bounds for flexibility
//! - **Error Handling**: Comprehensive error handling with `ExecutionError`
//! - **Performance**: Optimized for common operations
//!
//! ## Usage Example
//!
//! ```rust
//! use jetcrab::vm::executor::instruction_handlers::ArithmeticHandler;
//! use jetcrab::vm::executor::traits::StackOperations;
//! use jetcrab::vm::executor::stack_manager::StackManager;
//! use jetcrab::vm::value::Value;
//!
//! let mut stack = StackManager::new();
//! stack.push(Value::Number(3.0));
//! stack.push(Value::Number(5.0));
//! ArithmeticHandler::add(&mut stack).unwrap();
//! ```

pub mod arithmetic;
pub mod builtin_calls;
pub mod comparison;
pub mod control_flow;
pub mod heap_ops;
pub mod object;
pub mod stack_ops;

pub use arithmetic::ArithmeticHandler;
pub use builtin_calls::BuiltinCallsHandler;
pub use comparison::ComparisonHandler;
pub use control_flow::ControlFlowHandler;
pub use heap_ops::HeapOpsHandler;
pub use object::ObjectHandler;
pub use stack_ops::StackOpsHandler;

/// Unified instruction handler that provides access to all instruction handlers
pub struct InstructionHandlers;

impl InstructionHandlers {
    /// Get a reference to the arithmetic handler
    pub fn arithmetic() -> &'static ArithmeticHandler {
        &ArithmeticHandler
    }

    /// Get a reference to the comparison handler
    pub fn comparison() -> &'static ComparisonHandler {
        &ComparisonHandler
    }

    /// Get a reference to the control flow handler
    pub fn control_flow() -> &'static ControlFlowHandler {
        &ControlFlowHandler
    }

    /// Get a reference to the stack operations handler
    pub fn stack_ops() -> &'static StackOpsHandler {
        &StackOpsHandler
    }

    /// Get a reference to the heap operations handler
    pub fn heap_ops() -> &'static HeapOpsHandler {
        &HeapOpsHandler
    }

    /// Get a reference to the builtin calls handler
    pub fn builtin_calls() -> &'static BuiltinCallsHandler {
        &BuiltinCallsHandler
    }

    /// Get a reference to the object handler
    pub fn object() -> &'static ObjectHandler {
        &ObjectHandler
    }
}
