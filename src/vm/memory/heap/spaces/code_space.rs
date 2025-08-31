//! # Code Space
//!
//! Memory space for compiled bytecode and code objects.
//! Optimized for code that needs to persist across garbage collections.
//!
//! ## Characteristics
//!
//! - **Persistent storage**: Code objects survive multiple GC cycles
//! - **Read-only optimization**: Code is typically read-only after compilation
//! - **Hot code detection**: Identifies frequently executed code
//! - **Code sharing**: Enables sharing of identical bytecode
//! - **Perfect for**: Functions, classes, compiled expressions

use super::{MemorySpace, SpaceStats, SpaceType};
use crate::vm::handle::HeapHandleId;
use crate::vm::memory::heap::spaces::{DefragmentationStats, GcStats};
use crate::vm::types::MemorySize;
use crate::vm::value::Value;
use std::collections::HashMap;

/// Code space for compiled bytecode
pub struct CodeSpace {
    /// Total size of the space
    total_size: usize,
    /// Statistics
    stats: SpaceStats,
    /// Code objects tracking
    code_objects: HashMap<HeapHandleId, CodeObjectInfo>,
    /// Code blocks for different types
    code_blocks: Vec<CodeBlock>,
    /// Next block ID
    next_block_id: usize,
    /// Hot code detection
    hot_code_tracker: HotCodeTracker,
}

/// Information about a code object
#[derive(Debug, Clone)]
pub struct CodeObjectInfo {
    pub size: usize,
    pub block_id: usize,
    pub allocation_time: std::time::Instant,
    pub execution_count: usize,
    pub last_execution: std::time::Instant,
    pub code_type: CodeType,
    pub optimization_level: OptimizationLevel,
    pub is_hot: bool,
}

/// Types of code objects
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CodeType {
    Function,
    Class,
    Method,
    Expression,
    Module,
    Script,
    Builtin,
    Other,
}

/// Optimization levels for code
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum OptimizationLevel {
    None = 0,
    Basic = 1,
    Intermediate = 2,
    Advanced = 3,
    Aggressive = 4,
}

/// Code block for memory management
#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub id: usize,
    pub start_address: usize,
    pub size: usize,
    pub is_allocated: bool,
    pub code_handle: Option<HeapHandleId>,
    pub allocation_time: Option<std::time::Instant>,
    pub access_pattern: AccessPattern,
}

/// Access patterns for code blocks
#[derive(Debug, Clone)]
pub enum AccessPattern {
    ReadOnly,
    ReadWrite,
    Execute,
    Mixed,
}

/// Hot code tracking system
#[derive(Debug, Clone)]
pub struct HotCodeTracker {
    pub execution_threshold: usize,
    pub hot_code_count: usize,
    pub optimization_opportunities: Vec<HeapHandleId>,
    pub cold_code_candidates: Vec<HeapHandleId>,
}

impl Default for HotCodeTracker {
    fn default() -> Self {
        Self {
            execution_threshold: 1000,
            hot_code_count: 0,
            optimization_opportunities: Vec::new(),
            cold_code_candidates: Vec::new(),
        }
    }
}

impl CodeSpace {
    /// Create a new code space with the specified size
    pub fn new(size: usize) -> Self {
        let mut space = Self {
            total_size: size,
            stats: SpaceStats {
                space_type: SpaceType::CodeSpace,
                total_size: size,
                allocated_size: 0,
                free_size: size,
                object_count: 0,
                fragmentation_percentage: 0.0,
                allocation_count: 0,
                deallocation_count: 0,
            },
            code_objects: HashMap::new(),
            code_blocks: Vec::new(),
            next_block_id: 0,
            hot_code_tracker: HotCodeTracker::default(),
        };

        // Initialize with code-optimized memory blocks
        space.initialize_code_blocks();
        space
    }

    /// Initialize code-optimized memory blocks
    fn initialize_code_blocks(&mut self) {
        // Create blocks optimized for different code sizes
        let block_sizes = vec![
            4 * 1024,    // 4KB - Small functions
            16 * 1024,   // 16KB - Medium functions
            64 * 1024,   // 64KB - Large functions
            256 * 1024,  // 256KB - Classes/modules
            1024 * 1024, // 1MB - Large scripts
        ];

        let mut current_address = 0;
        for &size in &block_sizes {
            if current_address + size <= self.total_size {
                self.code_blocks.push(CodeBlock {
                    id: self.next_block_id,
                    start_address: current_address,
                    size,
                    is_allocated: false,
                    code_handle: None,
                    allocation_time: None,
                    access_pattern: AccessPattern::ReadOnly,
                });
                self.next_block_id += 1;
                current_address += size;
            }
        }
    }

    /// Find best fit code block
    fn find_best_fit_code_block(&self, size: usize) -> Option<usize> {
        let mut best_fit: Option<usize> = None;
        let mut best_fit_size = usize::MAX;

        for (index, block) in self.code_blocks.iter().enumerate() {
            if !block.is_allocated && block.size >= size {
                let waste = block.size - size;
                if waste < best_fit_size {
                    best_fit_size = waste;
                    best_fit = Some(index);
                }
            }
        }

        best_fit
    }

    /// Split code block if necessary
    fn split_code_block(&mut self, block_index: usize, requested_size: usize) {
        let block = &mut self.code_blocks[block_index];
        let remaining_size = block.size - requested_size;

        if remaining_size >= 4 * 1024 {
            // Minimum 4KB for new block
            // Resize current block
            block.size = requested_size;

            // Create new block with remaining space
            let new_block = CodeBlock {
                id: self.next_block_id,
                start_address: block.start_address + requested_size,
                size: remaining_size,
                is_allocated: false,
                code_handle: None,
                allocation_time: None,
                access_pattern: AccessPattern::ReadOnly,
            };

            self.code_blocks.push(new_block);
            self.next_block_id += 1;
        }
    }

    /// Get code object information
    pub fn code_object_info(&self) -> CodeSpaceInfo {
        let total_objects = self.code_objects.len();
        let total_size: usize = self.code_objects.values().map(|obj| obj.size).sum();
        let avg_size = if total_objects > 0 {
            total_size / total_objects
        } else {
            0
        };

        let mut type_distribution = HashMap::new();
        let mut optimization_distribution = HashMap::new();

        for obj in self.code_objects.values() {
            *type_distribution.entry(obj.code_type.clone()).or_insert(0) += 1;
            *optimization_distribution
                .entry(obj.optimization_level.clone())
                .or_insert(0) += 1;
        }

        CodeSpaceInfo {
            total_objects,
            total_size,
            average_size: avg_size,
            type_distribution,
            optimization_distribution,
            block_count: self.code_blocks.len(),
            allocated_blocks: self.code_blocks.iter().filter(|b| b.is_allocated).count(),
            hot_code_count: self.hot_code_tracker.hot_code_count,
        }
    }

    /// Get code block information
    pub fn code_block_info(&self) -> Vec<CodeBlockInfo> {
        self.code_blocks
            .iter()
            .map(|block| CodeBlockInfo {
                id: block.id,
                start_address: block.start_address,
                size: block.size,
                is_allocated: block.is_allocated,
                code_handle: block.code_handle,
                allocation_time: block.allocation_time,
                access_pattern: block.access_pattern.clone(),
            })
            .collect()
    }

    /// Record code execution for hot code detection
    pub fn record_execution(&mut self, handle: HeapHandleId) {
        if let Some(code_obj) = self.code_objects.get_mut(&handle) {
            code_obj.execution_count += 1;
            code_obj.last_execution = std::time::Instant::now();

            // Check if code became hot
            if !code_obj.is_hot
                && code_obj.execution_count >= self.hot_code_tracker.execution_threshold
            {
                code_obj.is_hot = true;
                self.hot_code_tracker.hot_code_count += 1;
                self.hot_code_tracker
                    .optimization_opportunities
                    .push(handle);
            }
        }
    }

    /// Get hot code information
    pub fn hot_code_info(&self) -> HotCodeInfo {
        let hot_objects: Vec<_> = self
            .code_objects
            .iter()
            .filter(|(_, obj)| obj.is_hot)
            .collect();

        let total_executions: usize = hot_objects.iter().map(|(_, obj)| obj.execution_count).sum();
        let avg_executions = if !hot_objects.is_empty() {
            total_executions / hot_objects.len()
        } else {
            0
        };

        HotCodeInfo {
            hot_code_count: self.hot_code_tracker.hot_code_count,
            total_executions,
            average_executions: avg_executions,
            optimization_opportunities: self.hot_code_tracker.optimization_opportunities.len(),
            cold_code_candidates: self.hot_code_tracker.cold_code_candidates.len(),
        }
    }

    /// Optimize hot code
    pub fn optimize_hot_code(&mut self) -> OptimizationStats {
        let start_time = std::time::Instant::now();
        let mut optimizations_applied = 0;

        for &handle in &self.hot_code_tracker.optimization_opportunities {
            if let Some(code_obj) = self.code_objects.get_mut(&handle) {
                // Upgrade optimization level
                if code_obj.optimization_level < OptimizationLevel::Aggressive {
                    code_obj.optimization_level = match code_obj.optimization_level {
                        OptimizationLevel::None => OptimizationLevel::Basic,
                        OptimizationLevel::Basic => OptimizationLevel::Intermediate,
                        OptimizationLevel::Intermediate => OptimizationLevel::Advanced,
                        OptimizationLevel::Advanced => OptimizationLevel::Aggressive,
                        OptimizationLevel::Aggressive => OptimizationLevel::Aggressive,
                    };
                    optimizations_applied += 1;
                }
            }
        }

        let end_time = std::time::Instant::now();
        let duration = end_time.duration_since(start_time).as_micros() as u64;

        OptimizationStats {
            duration_micros: duration,
            optimizations_applied,
            hot_code_count: self.hot_code_tracker.hot_code_count,
        }
    }

    /// Check if compaction is needed
    pub fn should_compact(&self) -> bool {
        let allocated_blocks = self.code_blocks.iter().filter(|b| b.is_allocated).count();
        let total_blocks = self.code_blocks.len();

        // Compact if more than 80% of blocks are allocated
        allocated_blocks as f64 / total_blocks as f64 > 0.8
    }

    /// Compact code blocks
    pub fn compact(&mut self) -> CompactionStats {
        let start_time = std::time::Instant::now();
        let initial_fragmentation = self.calculate_fragmentation();

        // Move allocated blocks to the beginning
        let mut allocated_blocks: Vec<_> = self
            .code_blocks
            .iter()
            .filter(|b| b.is_allocated)
            .cloned()
            .collect();

        allocated_blocks.sort_by_key(|b| b.start_address);

        // Rebuild code blocks
        self.code_blocks.clear();
        let mut current_address = 0;

        for block in allocated_blocks {
            let new_block = CodeBlock {
                id: block.id,
                start_address: current_address,
                size: block.size,
                is_allocated: true,
                code_handle: block.code_handle,
                allocation_time: block.allocation_time,
                access_pattern: block.access_pattern,
            };

            self.code_blocks.push(new_block);
            current_address += block.size;
        }

        // Add free block at the end
        if current_address < self.total_size {
            let free_size = self.total_size - current_address;
            if free_size >= 4 * 1024 {
                // Minimum 4KB
                self.code_blocks.push(CodeBlock {
                    id: self.next_block_id,
                    start_address: current_address,
                    size: free_size,
                    is_allocated: false,
                    code_handle: None,
                    allocation_time: None,
                    access_pattern: AccessPattern::ReadOnly,
                });
                self.next_block_id += 1;
            }
        }

        let end_time = std::time::Instant::now();
        let duration = end_time.duration_since(start_time).as_micros() as u64;

        CompactionStats {
            duration_micros: duration,
            initial_fragmentation,
            final_fragmentation: self.calculate_fragmentation(),
            cells_moved: allocated_blocks.len(),
        }
    }

    /// Calculate fragmentation percentage
    fn calculate_fragmentation(&self) -> f64 {
        let total_free = self
            .code_blocks
            .iter()
            .filter(|b| !b.is_allocated)
            .map(|b| b.size)
            .sum::<usize>();

        let largest_free = self
            .code_blocks
            .iter()
            .filter(|b| !b.is_allocated)
            .map(|b| b.size)
            .max()
            .unwrap_or(0);

        if total_free == 0 {
            0.0
        } else {
            (1.0 - (largest_free as f64 / total_free as f64)) * 100.0
        }
    }

    /// Get space efficiency
    pub fn efficiency(&self) -> f64 {
        let used = self
            .code_objects
            .values()
            .map(|obj| obj.size)
            .sum::<usize>();
        (used as f64 / self.total_size as f64) * 100.0
    }

    /// Get space health score
    pub fn health_score(&self) -> f64 {
        let efficiency = self.efficiency();
        let fragmentation = self.calculate_fragmentation();
        let hot_code_ratio = if self.code_objects.is_empty() {
            1.0
        } else {
            self.hot_code_tracker.hot_code_count as f64 / self.code_objects.len() as f64
        };

        // Higher efficiency, lower fragmentation, and good hot code ratio = better health
        let efficiency_score = efficiency / 100.0;
        let fragmentation_score = 1.0 - (fragmentation / 100.0);
        let hot_code_score = hot_code_ratio;

        (efficiency_score + fragmentation_score + hot_code_score) / 3.0 * 100.0
    }

    /// Perform garbage collection
    pub fn collect(&mut self) -> GcStats {
        let start_time = std::time::Instant::now();

        // Get current usage before collection
        let before_usage: usize = self.code_objects.values().map(|obj| obj.size).sum();
        let before_objects = self.code_objects.len();

        // Simple collection simulation for code space
        // In a real implementation, this would mark live code objects
        let mut objects_to_remove = Vec::new();

        for (&handle, code_obj) in &self.code_objects {
            // Simulate collection based on execution patterns
            let time_since_execution = std::time::Instant::now()
                .duration_since(code_obj.last_execution)
                .as_secs();

            // Remove cold code that hasn't been executed recently
            if !code_obj.is_hot && time_since_execution > 600 {
                // 10 minutes
                objects_to_remove.push(handle);
            }
        }

        // Remove dead code objects
        for handle in &objects_to_remove {
            if let Some(code_obj) = self.code_objects.remove(handle) {
                // Free the code block
                if let Some(block) = self
                    .code_blocks
                    .iter_mut()
                    .find(|b| b.code_handle == Some(*handle))
                {
                    block.is_allocated = false;
                    block.code_handle = None;
                    block.allocation_time = None;
                }

                // Update hot code tracker
                if code_obj.is_hot {
                    self.hot_code_tracker.hot_code_count =
                        self.hot_code_tracker.hot_code_count.saturating_sub(1);
                }
            }
        }

        // Calculate collection statistics
        let objects_collected = before_objects - self.code_objects.len();
        let after_usage: usize = self.code_objects.values().map(|obj| obj.size).sum();
        let bytes_freed = before_usage - after_usage;

        let end_time = std::time::Instant::now();
        let collection_time = end_time.duration_since(start_time).as_micros() as u64;

        // Update statistics
        self.stats.object_count = self.code_objects.len();
        self.stats.allocated_size = after_usage;
        self.stats.free_size = self.total_size - after_usage;
        self.stats.fragmentation_percentage = self.calculate_fragmentation();

        GcStats {
            objects_collected,
            bytes_freed,
            collection_time,
        }
    }
}

impl MemorySpace for CodeSpace {
    fn allocate(&mut self, size: MemorySize) -> Option<HeapHandleId> {
        let size_bytes = size.as_usize();

        // Find best fit code block
        if let Some(block_index) = self.find_best_fit_code_block(size_bytes) {
            // Split block if necessary
            if self.code_blocks[block_index].size > size_bytes {
                self.split_code_block(block_index, size_bytes);
            }

            // Get block reference after potential split
            let block = &mut self.code_blocks[block_index];

            // Allocate the block
            block.is_allocated = true;
            block.code_handle = Some(HeapHandleId::new(block.start_address));
            block.allocation_time = Some(std::time::Instant::now());

            // Create code object info
            let handle = HeapHandleId::new(block.start_address);
            let code_obj_info = CodeObjectInfo {
                size: size_bytes,
                block_id: block.id,
                allocation_time: std::time::Instant::now(),
                execution_count: 0,
                last_execution: std::time::Instant::now(),
                code_type: if size_bytes > 1024 * 1024 {
                    CodeType::Script
                } else if size_bytes > 256 * 1024 {
                    CodeType::Module
                } else if size_bytes > 64 * 1024 {
                    CodeType::Class
                } else if size_bytes > 16 * 1024 {
                    CodeType::Method
                } else {
                    CodeType::Function
                },
                optimization_level: OptimizationLevel::None,
                is_hot: false,
            };

            self.code_objects.insert(handle, code_obj_info);

            // Update statistics
            self.stats.allocated_size += size_bytes;
            self.stats.object_count += 1;
            self.stats.allocation_count += 1;
            self.stats.free_size = self.total_size - self.stats.allocated_size;

            Some(handle)
        } else {
            None
        }
    }

    fn deallocate(&mut self, handle: HeapHandleId) -> bool {
        if let Some(code_obj) = self.code_objects.remove(&handle) {
            // Free the code block
            if let Some(block) = self
                .code_blocks
                .iter_mut()
                .find(|b| b.code_handle == Some(handle))
            {
                block.is_allocated = false;
                block.code_handle = None;
                block.allocation_time = None;
            }

            // Update hot code tracker
            if code_obj.is_hot {
                self.hot_code_tracker.hot_code_count =
                    self.hot_code_tracker.hot_code_count.saturating_sub(1);
            }

            // Update statistics
            self.stats.object_count = self.code_objects.len();
            self.stats.allocated_size = self.code_objects.values().map(|obj| obj.size).sum();
            self.stats.free_size = self.total_size - self.stats.allocated_size;
            self.stats.deallocation_count += 1;

            true
        } else {
            false
        }
    }

    fn can_allocate(&self, size: MemorySize) -> bool {
        let size_bytes = size.as_usize();
        self.code_blocks
            .iter()
            .any(|b| !b.is_allocated && b.size >= size_bytes)
    }

    fn total_allocated(&self) -> MemorySize {
        MemorySize::new(self.stats.allocated_size)
    }

    fn total_free(&self) -> MemorySize {
        MemorySize::new(self.stats.free_size)
    }

    fn stats(&self) -> SpaceStats {
        self.stats.clone()
    }

    fn space_type(&self) -> SpaceType {
        SpaceType::CodeSpace
    }
}

/// Information about code space
#[derive(Debug, Clone)]
pub struct CodeSpaceInfo {
    pub total_objects: usize,
    pub total_size: usize,
    pub average_size: usize,
    pub type_distribution: HashMap<CodeType, usize>,
    pub optimization_distribution: HashMap<OptimizationLevel, usize>,
    pub block_count: usize,
    pub allocated_blocks: usize,
    pub hot_code_count: usize,
}

/// Information about a code block
#[derive(Debug, Clone)]
pub struct CodeBlockInfo {
    pub id: usize,
    pub start_address: usize,
    pub size: usize,
    pub is_allocated: bool,
    pub code_handle: Option<HeapHandleId>,
    pub allocation_time: Option<std::time::Instant>,
    pub access_pattern: AccessPattern,
}

/// Hot code information
#[derive(Debug, Clone)]
pub struct HotCodeInfo {
    pub hot_code_count: usize,
    pub total_executions: usize,
    pub average_executions: usize,
    pub optimization_opportunities: usize,
    pub cold_code_candidates: usize,
}

/// Optimization statistics
#[derive(Debug, Clone)]
pub struct OptimizationStats {
    pub duration_micros: u64,
    pub optimizations_applied: usize,
    pub hot_code_count: usize,
}

/// Compaction statistics
#[derive(Debug, Clone)]
pub struct CompactionStats {
    pub duration_micros: u64,
    pub initial_fragmentation: f64,
    pub final_fragmentation: f64,
    pub cells_moved: usize,
}

impl Default for CodeSpace {
    fn default() -> Self {
        Self::new(32 * 1024 * 1024) // 32MB default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_space_new() {
        let code_space = CodeSpace::new(1024 * 1024);
        assert_eq!(code_space.total_size, 1024 * 1024);
        assert_eq!(code_space.total_allocated().as_usize(), 0);
        assert_eq!(code_space.total_free().as_usize(), 1024 * 1024);
        assert_eq!(code_space.stats.object_count, 0);
        assert!(!code_space.code_blocks.is_empty());
    }

    #[test]
    fn test_code_space_allocate() {
        let mut code_space = CodeSpace::new(1024 * 1024);

        let handle = code_space.allocate(MemorySize::new(32 * 1024));
        assert!(handle.is_some());
        assert_eq!(code_space.total_allocated().as_usize(), 32 * 1024);
        assert_eq!(code_space.stats.object_count, 1);
        assert_eq!(code_space.stats.allocation_count, 1);

        // Check that a block was allocated
        let allocated_blocks = code_space
            .code_blocks
            .iter()
            .filter(|b| b.is_allocated)
            .count();
        assert_eq!(allocated_blocks, 1);
    }

    #[test]
    fn test_code_space_deallocate() {
        let mut code_space = CodeSpace::new(1024 * 1024);

        let handle = code_space.allocate(MemorySize::new(32 * 1024)).unwrap();
        assert_eq!(code_space.stats.object_count, 1);

        assert!(code_space.deallocate(handle));
        assert_eq!(code_space.stats.object_count, 0);
        assert_eq!(code_space.stats.deallocation_count, 1);

        // Check that the block was freed
        let allocated_blocks = code_space
            .code_blocks
            .iter()
            .filter(|b| b.is_allocated)
            .count();
        assert_eq!(allocated_blocks, 0);
    }

    #[test]
    fn test_code_space_hot_code_detection() {
        let mut code_space = CodeSpace::new(1024 * 1024);

        let handle = code_space.allocate(MemorySize::new(16 * 1024)).unwrap();

        // Record executions to make it hot
        for _ in 0..1000 {
            code_space.record_execution(handle);
        }

        let hot_info = code_space.hot_code_info();
        assert_eq!(hot_info.hot_code_count, 1);
        assert!(hot_info.optimization_opportunities.len() > 0);
    }

    #[test]
    fn test_code_space_optimization() {
        let mut code_space = CodeSpace::new(1024 * 1024);

        let handle = code_space.allocate(MemorySize::new(16 * 1024)).unwrap();

        // Make it hot
        for _ in 0..1000 {
            code_space.record_execution(handle);
        }

        // Apply optimization
        let stats = code_space.optimize_hot_code();
        assert!(stats.optimizations_applied > 0);

        // Check optimization level was upgraded
        let code_obj = code_space.code_objects.get(&handle).unwrap();
        assert!(code_obj.optimization_level > OptimizationLevel::None);
    }

    #[test]
    fn test_code_space_compact() {
        let mut code_space = CodeSpace::new(1024 * 1024);

        // Allocate some code objects
        let handle1 = code_space.allocate(MemorySize::new(32 * 1024)).unwrap();
        let handle2 = code_space.allocate(MemorySize::new(32 * 1024)).unwrap();

        // Deallocate first object to create fragmentation
        code_space.deallocate(handle1);

        let initial_fragmentation = code_space.calculate_fragmentation();

        // Compact
        let stats = code_space.compact();
        assert!(stats.final_fragmentation < stats.initial_fragmentation);
        assert!(stats.cells_moved > 0);
    }

    #[test]
    fn test_code_space_collect() {
        let mut code_space = CodeSpace::new(1024 * 1024);

        // Allocate some code objects
        code_space.allocate(MemorySize::new(32 * 1024));
        code_space.allocate(MemorySize::new(32 * 1024));

        let before_objects = code_space.stats.object_count;

        // Perform collection
        let stats = code_space.collect();

        assert!(stats.collection_time > 0);
        assert_eq!(
            code_space.stats.object_count,
            before_objects - stats.objects_collected
        );
    }

    #[test]
    fn test_code_space_efficiency() {
        let mut code_space = CodeSpace::new(1024 * 1024);

        // Allocate 25% of space
        code_space.allocate(MemorySize::new(256 * 1024));

        let efficiency = code_space.efficiency();
        assert_eq!(efficiency, 25.0);

        let health = code_space.health_score();
        assert!(health > 60.0);
    }
}
