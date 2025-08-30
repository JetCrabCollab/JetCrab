pub mod addresses;
pub mod indices;
pub mod names;
pub mod sizes;

// Re-export all types for easy access


// Re-export commonly used types directly
pub use addresses::{CharOffset, CodeAddress, ColumnNumber, LineNumber, SourcePosition};
pub use indices::{
    ArgIndex, ArraySize, ConstantIndex, FramePointer, FunctionIndex, GlobalIndex, HeapId,
    LocalIndex, ObjectId, StackIndex,
};
pub use names::{ClassName, FunctionName, ModuleName, PropertyName, VariableName};
pub use sizes::{
    AllocationCount, ErrorCount, IndentLevel, MemorySize, NodeCount, ObjectCount, ObjectSize,
    ScopeDepth, VariableCount,
};
