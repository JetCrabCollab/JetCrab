//! VM Core - Core VM orchestration

use super::{VmConfig, VmEngine};

/// Core VM that orchestrates engine and configuration
pub struct VmCore {
    /// VM execution engine
    engine: VmEngine,
    /// VM configuration
    config: VmConfig,
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

    /// Get a reference to the engine
    pub fn engine(&self) -> &VmEngine {
        &self.engine
    }

    /// Get a mutable reference to the engine
    pub fn engine_mut(&mut self) -> &mut VmEngine {
        &mut self.engine
    }

    /// Get a reference to the configuration
    pub fn config(&self) -> &VmConfig {
        &self.config
    }

    /// Update the configuration
    pub fn update_config(&mut self, new_config: VmConfig) {
        self.config = new_config;
    }

    /// Force garbage collection
    pub fn force_gc(&mut self) {
        self.engine.force_gc();
    }

    /// Get memory statistics
    pub fn memory_stats(&self) -> String {
        self.engine.memory_stats()
    }

    /// Get memory manager
    pub fn memory_manager(&self) -> &crate::vm::vm::memory::MemoryManager {
        self.engine.memory_manager()
    }

    /// Get heap information
    pub fn heap_info(&self) -> String {
        self.engine.memory_manager().heap_info()
    }

    /// Get mutable memory manager
    pub fn memory_manager_mut(&mut self) -> &mut crate::vm::vm::memory::MemoryManager {
        self.engine.memory_manager_mut()
    }
}

impl Default for VmCore {
    fn default() -> Self {
        Self::new()
    }
}
