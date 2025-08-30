pub mod arithmetic;
pub mod comparison;
pub mod control_flow;
pub mod stack_ops;
pub mod heap_ops;
pub mod builtin_calls;

pub use arithmetic::ArithmeticHandler;
pub use comparison::ComparisonHandler;
pub use control_flow::ControlFlowHandler;
pub use stack_ops::StackOpsHandler;
pub use heap_ops::HeapOpsHandler;
pub use builtin_calls::BuiltinCallsHandler;

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
}
