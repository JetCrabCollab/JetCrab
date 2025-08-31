//! VM Configuration - Settings and configuration for the JetCrab VM
//! 
//! This module provides configuration options for:
//! - Memory management
//! - Garbage collection
//! - Performance tuning
//! - Debug and logging

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
    /// Large object threshold in bytes
    pub large_object_threshold: usize,
    /// Cell space size for small objects
    pub cell_space_size: usize,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            initial_heap_size: 64 * 1024 * 1024, // 64MB
            max_heap_size: 1024 * 1024 * 1024,   // 1GB
            new_space_size: 16 * 1024 * 1024,    // 16MB
            old_space_size: 256 * 1024 * 1024,   // 256MB
            large_object_threshold: 1024 * 1024,  // 1MB
            cell_space_size: 1024 * 1024,         // 1MB
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
    /// Enable incremental GC
    pub enable_incremental_gc: bool,
    /// Enable background GC
    pub enable_background_gc: bool,
    /// GC threshold percentage
    pub gc_threshold: f64,
    /// Minor GC frequency (collections per major GC)
    pub minor_gc_frequency: u32,
    /// Major GC frequency (collections per 1000 allocations)
    pub major_gc_frequency: u32,
}

impl Default for GcConfig {
    fn default() -> Self {
        Self {
            enable_minor_gc: true,
            enable_major_gc: true,
            enable_incremental_gc: true,
            enable_background_gc: false,
            gc_threshold: 0.75,        // 75% heap usage
            minor_gc_frequency: 8,     // 8 minor GCs per major GC
            major_gc_frequency: 1000,  // Major GC every 1000 allocations
        }
    }
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable JIT compilation
    pub enable_jit: bool,
    /// Enable instruction optimization
    pub enable_optimization: bool,
    /// Enable inline caching
    pub enable_inline_caching: bool,
    /// Enable hidden classes (object shapes)
    pub enable_hidden_classes: bool,
    /// Enable string interning
    pub enable_string_interning: bool,
    /// Maximum inline cache size
    pub max_inline_cache_size: usize,
    /// Optimization threshold
    pub optimization_threshold: u32,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_jit: false,           // JIT not implemented yet
            enable_optimization: true,
            enable_inline_caching: true,
            enable_hidden_classes: true,
            enable_string_interning: true,
            max_inline_cache_size: 1024,
            optimization_threshold: 100,
        }
    }
}

/// Debug and logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    /// Enable debug mode
    pub debug_mode: bool,
    /// Enable verbose logging
    pub verbose_logging: bool,
    /// Enable memory tracing
    pub memory_tracing: bool,
    /// Enable instruction tracing
    pub instruction_tracing: bool,
    /// Enable GC tracing
    pub gc_tracing: bool,
    /// Log level (0=Error, 1=Warn, 2=Info, 3=Debug, 4=Trace)
    pub log_level: u8,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            debug_mode: false,
            verbose_logging: false,
            memory_tracing: false,
            instruction_tracing: false,
            gc_tracing: false,
            log_level: 1, // Warn level by default
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
                ..Default::default()
            },
            gc: GcConfig {
                enable_background_gc: true,
                gc_threshold: 0.85, // Higher threshold for performance
                ..Default::default()
            },
            performance: PerformanceConfig {
                enable_optimization: true,
                enable_inline_caching: true,
                enable_hidden_classes: true,
                enable_string_interning: true,
                ..Default::default()
            },
            debug: DebugConfig {
                debug_mode: false,
                verbose_logging: false,
                ..Default::default()
            },
        }
    }

    /// Create a debug configuration
    pub fn debug() -> Self {
        Self {
            debug: DebugConfig {
                debug_mode: true,
                verbose_logging: true,
                memory_tracing: true,
                instruction_tracing: true,
                gc_tracing: true,
                log_level: 4, // Trace level
            },
            ..Default::default()
        }
    }

    /// Create a memory-efficient configuration
    pub fn memory_efficient() -> Self {
        Self {
            memory: MemoryConfig {
                initial_heap_size: 16 * 1024 * 1024,  // 16MB
                max_heap_size: 256 * 1024 * 1024,     // 256MB
                new_space_size: 4 * 1024 * 1024,      // 4MB
                old_space_size: 64 * 1024 * 1024,     // 64MB
                ..Default::default()
            },
            gc: GcConfig {
                gc_threshold: 0.6, // Lower threshold for memory efficiency
                minor_gc_frequency: 4,  // More frequent minor GCs
                major_gc_frequency: 500, // More frequent major GCs
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
