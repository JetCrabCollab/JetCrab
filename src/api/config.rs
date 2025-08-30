use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OptimizationLevel {
    None,
    Basic,
    Aggressive,
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        Self::Basic
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModuleSystem {
    None,
    CommonJS,
    ES6,
}

impl Default for ModuleSystem {
    fn default() -> Self {
        Self::ES6
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SecurityLevel {
    Permissive,
    Standard,
    Strict,
}

impl Default for SecurityLevel {
    fn default() -> Self {
        Self::Standard
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub initial_heap_size: usize,
    pub max_heap_size: usize,
    pub gc_threshold: usize,
    pub gc_interval: Duration,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            initial_heap_size: 1024 * 1024,   // 1MB
            max_heap_size: 100 * 1024 * 1024, // 100MB
            gc_threshold: 512 * 1024,         // 512KB
            gc_interval: Duration::from_millis(100),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    pub optimization_level: OptimizationLevel,
    pub memory_config: MemoryConfig,
    pub timeout: Option<Duration>,
    pub strict_mode: bool,
    pub module_system: ModuleSystem,
    pub security_level: SecurityLevel,
    pub enable_debugging: bool,
    pub enable_profiling: bool,
    pub allow_unsafe_operations: bool,
    pub max_execution_depth: usize,
    pub max_loop_iterations: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            optimization_level: OptimizationLevel::default(),
            memory_config: MemoryConfig::default(),
            timeout: Some(Duration::from_secs(30)),
            strict_mode: false,
            module_system: ModuleSystem::default(),
            security_level: SecurityLevel::default(),
            enable_debugging: false,
            enable_profiling: false,
            allow_unsafe_operations: false,
            max_execution_depth: 1000,
            max_loop_iterations: 1_000_000,
        }
    }
}

impl EngineConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_optimization(mut self, level: OptimizationLevel) -> Self {
        self.optimization_level = level;
        self
    }

    pub fn with_memory_config(mut self, config: MemoryConfig) -> Self {
        self.memory_config = config;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    pub fn with_module_system(mut self, system: ModuleSystem) -> Self {
        self.module_system = system;
        self
    }

    pub fn with_security_level(mut self, level: SecurityLevel) -> Self {
        self.security_level = level;
        self
    }

    pub fn with_debugging(mut self, enable: bool) -> Self {
        self.enable_debugging = enable;
        self
    }

    pub fn with_profiling(mut self, enable: bool) -> Self {
        self.enable_profiling = enable;
        self
    }

    pub fn with_execution_limits(mut self, max_depth: usize, max_iterations: usize) -> Self {
        self.max_execution_depth = max_depth;
        self.max_loop_iterations = max_iterations;
        self
    }

    pub fn is_production_ready(&self) -> bool {
        !self.enable_debugging
            && !self.enable_profiling
            && !self.allow_unsafe_operations
            && matches!(self.security_level, SecurityLevel::Strict)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.memory_config.initial_heap_size > self.memory_config.max_heap_size {
            return Err("Initial heap size cannot be larger than max heap size".to_string());
        }

        if self.memory_config.gc_threshold > self.memory_config.max_heap_size {
            return Err("GC threshold cannot be larger than max heap size".to_string());
        }

        if self.max_execution_depth == 0 {
            return Err("Max execution depth must be greater than 0".to_string());
        }

        if self.max_loop_iterations == 0 {
            return Err("Max loop iterations must be greater than 0".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = EngineConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_builder() {
        let config = EngineConfig::new()
            .with_optimization(OptimizationLevel::Aggressive)
            .with_strict_mode(true)
            .with_timeout(Duration::from_secs(60));

        assert_eq!(config.optimization_level, OptimizationLevel::Aggressive);
        assert!(config.strict_mode);
        assert_eq!(config.timeout, Some(Duration::from_secs(60)));
    }

    #[test]
    fn test_memory_config_validation() {
        let mut memory_config = MemoryConfig::default();
        memory_config.initial_heap_size = 200 * 1024 * 1024; // 200MB
        memory_config.max_heap_size = 100 * 1024 * 1024; // 100MB

        let config = EngineConfig::new().with_memory_config(memory_config);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_production_ready() {
        let production_config = EngineConfig::new()
            .with_security_level(SecurityLevel::Strict)
            .with_debugging(false)
            .with_profiling(false);

        assert!(production_config.is_production_ready());
    }
}
