pub mod bytecode;
pub mod error;
pub mod executor;
pub mod frame;
pub mod handle;
pub mod heap;
pub mod instructions;
pub mod registers;
pub mod stack;
pub mod types;
pub mod value;

pub use bytecode::Bytecode;
pub use error::VmError;
pub use executor::Executor;
pub use handle::{ArrayHandle, FunctionHandle, HeapHandleId, ObjectHandle, INVALID_HANDLE};
pub use instructions::Instruction;
pub use types::*;
pub use value::Value;
