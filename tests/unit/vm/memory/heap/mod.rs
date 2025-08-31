//! VM Memory Heap Unit Tests - Mirroring src/vm/memory/heap/ structure
//! 
//! This module contains unit tests for heap components:
//! - generational.rs: Generational heap
//! - optimized_arrays.rs: Optimized array representations
//! - object_shapes.rs: Object shapes (hidden classes)
//! - string_interning.rs: String interning system
//! - allocation/: Memory allocation strategies
//! - spaces/: Memory spaces
//! - gc/: Garbage collection

pub mod generational;
pub mod optimized_arrays;
pub mod object_shapes;
pub mod string_interning;
pub mod allocation;
pub mod gc;
pub mod spaces;

pub use allocation::*;
pub use gc::*;
pub use spaces::*;
