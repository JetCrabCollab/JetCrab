//! # Core APIs
//!
//! Essential APIs required for basic JetCrab functionality.

pub mod assert;
pub mod buffer;
pub mod console;
pub mod events;
pub mod require;

pub use assert::AssertAPI;
pub use buffer::BufferAPI;
pub use console::SimpleConsoleAPI;
pub use events::EventsAPI;
pub use require::RequireAPI;
