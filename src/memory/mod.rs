//! # Memory Management Module
//!
//! Provides memory management capabilities for the JavaScript engine,
//! including allocation, garbage collection, and memory error handling.
//!
//! ## Overview
//!
//! The memory module handles:
//!
//! - **Memory Allocation**: Dynamic memory allocation for objects
//! - **Garbage Collection**: Automatic memory reclamation
//! - **Heap Management**: Memory heap organization and access
//! - **Error Handling**: Memory-related error types and recovery
//!
//! ## Components
//!
//! - **Allocator**: Memory allocation strategies
//! - **Collector**: Garbage collection algorithms
//! - **Heap**: Memory heap structure and management
//! - **Errors**: Memory error types and handling
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::memory::{MemoryHeap, GarbageCollector, MemoryError};
//!
//! let heap = MemoryHeap::new();
//! let collector = GarbageCollector::new();
//! ```

pub mod allocator;
pub mod collector;
pub mod error;
pub mod heap;

pub use collector::GarbageCollector;
pub use error::MemoryError;
pub use heap::MemoryHeap;
