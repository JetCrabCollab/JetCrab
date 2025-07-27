use crate::ast::node::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryError {
    AllocationError {
        message: String,
        size: usize,
        position: Option<Position>,
    },

    OutOfMemory {
        requested: usize,
        available: usize,
        position: Option<Position>,
    },

    GarbageCollectionError {
        message: String,
        position: Option<Position>,
    },

    InvalidPointer {
        pointer: usize,
        message: String,
        position: Option<Position>,
    },

    MemoryCorruption {
        address: usize,
        message: String,
        position: Option<Position>,
    },

    HeapFull {
        message: String,
        position: Option<Position>,
    },

    InvalidAllocation {
        size: usize,
        alignment: usize,
        position: Option<Position>,
    },

    FragmentationError {
        message: String,
        position: Option<Position>,
    },
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::AllocationError {
                message,
                size,
                position,
            } => {
                write!(f, "Memory allocation error ({} bytes): {}", size, message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            MemoryError::OutOfMemory {
                requested,
                available,
                position,
            } => {
                write!(
                    f,
                    "Out of memory: requested {} bytes, available {} bytes",
                    requested, available
                )?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            MemoryError::GarbageCollectionError { message, position } => {
                write!(f, "Garbage collection error: {}", message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            MemoryError::InvalidPointer {
                pointer,
                message,
                position,
            } => {
                write!(f, "Invalid pointer 0x{:x}: {}", pointer, message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            MemoryError::MemoryCorruption {
                address,
                message,
                position,
            } => {
                write!(f, "Memory corruption at 0x{:x}: {}", address, message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            MemoryError::HeapFull { message, position } => {
                write!(f, "Heap full: {}", message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            MemoryError::InvalidAllocation {
                size,
                alignment,
                position,
            } => {
                write!(
                    f,
                    "Invalid allocation: size {} bytes, alignment {} bytes",
                    size, alignment
                )?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
            MemoryError::FragmentationError { message, position } => {
                write!(f, "Memory fragmentation error: {}", message)?;
                if let Some(pos) = position {
                    write!(f, " at line {}, column {}", pos.line, pos.column)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for MemoryError {}
