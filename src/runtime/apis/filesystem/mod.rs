//! # File System APIs
//!
//! APIs for file system operations and data handling.

pub mod fs_native;
pub mod path;

pub use fs_native::NativeFsModule;
pub use path::PathAPI;
