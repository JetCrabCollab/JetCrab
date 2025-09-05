//! # Claw - Package Manager
//!
//! A modern package manager that combines JavaScript and Rust
//! through WebAssembly, offering:
//!
//! - **Package installation** JavaScript via npm
//! - **Automatic compilation** of Rust modules to WASM
//! - **Intelligent dependency resolution**
//! - **Optimized cache** for fast builds
//! - **Native integration** between JS and Rust

pub mod cache_manager;
pub mod dependency_resolver;
pub mod dev_tools;
pub mod package_manager;
pub mod registry;

pub use cache_manager::CacheManager;
pub use dependency_resolver::DependencyResolver;
pub use dev_tools::{DevConfig, DevTools};
pub use package_manager::Claw;
pub use registry::PackageRegistry;
