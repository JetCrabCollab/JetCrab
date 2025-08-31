//! VM Configuration - Settings and configuration for the JetCrab VM

use serde::{Deserialize, Serialize};

/// VM configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmConfig {
    /// Memory management settings
    pub memory: MemoryConfig,
    /// Garbage collection settings
    pub gc: GcConfig,
    /// Performance settings
    pub performance: PerformanceConfig,
    /// Debug and logging settings
    pub debug: DebugConfig,
}

impl Default for VmConfig {
    fn default() -> Self {
        Self {
            memory: MemoryConfig::default(),
            gc: GcConfig::default(),
            performance: PerformanceConfig::default(),
            debug: DebugConfig::default(),
        }
    }
}

/// Memory management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Initial heap size in bytes
    pub initial_heap_size: usize,
    /// Maximum heap size in bytes
    pub max_heap_size: usize,
    /// New space size (young generation)
    pub new_space_size: usize,
    /// Old space size (old generation)
    pub old_space_size: usize,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            initial_heap_size: 64 * 1024 * 1024, // 64MB
            max_heap_size: 1024 * 1024 * 1024,   // 1GB
            new_space_size: 16 * 1024 * 1024,    // 16MB
            old_space_size: 256 * 1024 * 1024,   // 256MB
        }
    }
}

/// Garbage collection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcConfig {
    /// Enable minor GC
    pub enable_minor_gc: bool,
    /// Enable major GC
    pub enable_major_gc: bool,
    /// GC threshold percentage
    pub gc_threshold: f64,
}

impl Default for GcConfig {
    fn default() -> Self {
        Self {
            enable_minor_gc: true,
            enable_major_gc: true,
            gc_threshold: 0.75, // 75% heap usage
        }
    }
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable JIT compilation
    pub enable_jit: bool,
    /// Enable optimizations
    pub enable_optimization: bool,
    /// Enable inline caching
    pub enable_inline_caching: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_jit: false,
            enable_optimization: true,
            enable_inline_caching: true,
        }
    }
}

/// Debug configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    /// Enable debug mode
    pub debug_mode: bool,
    /// Enable verbose logging
    pub verbose_logging: bool,
    /// Log level (0-5)
    pub log_level: u8,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            debug_mode: false,
            verbose_logging: false,
            log_level: 2,
        }
    }
}

impl VmConfig {
    /// Create a performance-optimized configuration
    pub fn performance() -> Self {
        Self {
            memory: MemoryConfig {
                initial_heap_size: 128 * 1024 * 1024, // 128MB
                max_heap_size: 2048 * 1024 * 1024,    // 2GB
                new_space_size: 32 * 1024 * 1024,     // 32MB
                old_space_size: 512 * 1024 * 1024,    // 512MB
            },
            gc: GcConfig {
                enable_minor_gc: true,
                enable_major_gc: true,
                gc_threshold: 0.85, // 85% heap usage
            },
            performance: PerformanceConfig {
                enable_jit: false,
                enable_optimization: true,
                enable_inline_caching: true,
            },
            debug: DebugConfig::default(),
        }
    }

    /// Create a debug-optimized configuration
    pub fn debug() -> Self {
        Self {
            memory: MemoryConfig::default(),
            gc: GcConfig::default(),
            performance: PerformanceConfig::default(),
            debug: DebugConfig {
                debug_mode: true,
                verbose_logging: true,
                log_level: 4,
            },
        }
    }

    /// Create a memory-efficient configuration
    pub fn memory_efficient() -> Self {
        Self {
            memory: MemoryConfig {
                initial_heap_size: 16 * 1024 * 1024, // 16MB
                max_heap_size: 256 * 1024 * 1024,    // 256MB
                new_space_size: 4 * 1024 * 1024,     // 4MB
                old_space_size: 64 * 1024 * 1024,    // 64MB
            },
            gc: GcConfig {
                enable_minor_gc: true,
                enable_major_gc: true,
                gc_threshold: 0.6, // 60% heap usage
            },
            performance: PerformanceConfig::default(),
            debug: DebugConfig::default(),
        }
    }
}
