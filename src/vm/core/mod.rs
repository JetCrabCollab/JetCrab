//! Core module - Main entry point for the VM
//!
//! This module contains the core components of the JetCrab VM:
//! - Engine: Main VM execution engine
//! - Config: VM configuration and settings
//! - Core: Central VM orchestration

pub mod config;
pub mod engine;

pub use config::VmConfig;
pub use engine::VmEngine;

/// Core VM instance that orchestrates all components
pub struct VmCore {
    pub engine: VmEngine,
    pub config: VmConfig,
}

impl VmCore {
    /// Create a new VM core with default configuration
    pub fn new() -> Self {
        Self {
            engine: VmEngine::new(),
            config: VmConfig::default(),
        }
    }

    /// Create a new VM core with custom configuration
    pub fn with_config(config: VmConfig) -> Self {
        Self {
            engine: VmEngine::new(),
            config,
        }
    }

    /// Get a reference to the VM engine
    pub fn engine(&self) -> &VmEngine {
        &self.engine
    }

    /// Get a mutable reference to the VM engine
    pub fn engine_mut(&mut self) -> &mut VmEngine {
        &mut self.engine
    }

    /// Get the current VM configuration
    pub fn config(&self) -> &VmConfig {
        &self.config
    }

    /// Update the VM configuration
    pub fn update_config(&mut self, config: VmConfig) {
        self.config = config;
    }
}

impl Default for VmCore {
    fn default() -> Self {
        Self::new()
    }
}
