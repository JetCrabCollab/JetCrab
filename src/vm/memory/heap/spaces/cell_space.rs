//! # Cell Space
//!
//! Specialized memory space for small objects (≤ 16 bytes) using fixed-size cells.
//! Optimized for objects that are frequently allocated and deallocated.
//!
//! ## Characteristics
//!
//! - **Fixed-size cells**: All cells are the same size for fast allocation
//! - **No fragmentation**: Each cell is exactly the right size
//! - **Fast allocation**: O(1) allocation from free cell list
//! - **Memory efficient**: Minimal overhead per object
//! - **Perfect for small objects**: Strings, numbers, booleans, etc.

use super::{MemorySpace, SpaceStats, SpaceType};
use crate::vm::handle::HeapHandleId;
use crate::vm::memory::heap::allocation::{Allocator, CellAllocator, CellInfo, CompactionStats};
use crate::vm::memory::heap::spaces::{DefragmentationStats, GcStats};
use crate::vm::types::MemorySize;
use crate::vm::value::Value;
use std::collections::HashMap;

/// Cell space for small objects
pub struct CellSpace {
    /// Cell allocator for small objects
    allocator: CellAllocator,
    /// Total size of the space
    total_size: usize,
    /// Statistics
    stats: SpaceStats,
    /// Object type tracking
    object_types: std::collections::HashMap<HeapHandleId, SmallObjectType>,
    /// Performance metrics
    metrics: CellMetrics,
}

/// Types of small objects
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SmallObjectType {
    String,
    Number,
    Boolean,
    Symbol,
    Undefined,
    Null,
    Other,
}

/// Performance metrics for cell space
#[derive(Debug, Clone)]
pub struct CellMetrics {
    pub allocation_time_ns: u64,
    pub deallocation_time_ns: u64,
    pub compaction_time_ns: u64,
    pub total_operations: usize,
    pub average_operation_time_ns: u64,
}

impl Default for CellMetrics {
    fn default() -> Self {
        Self {
            allocation_time_ns: 0,
            deallocation_time_ns: 0,
            compaction_time_ns: 0,
            total_operations: 0,
            average_operation_time_ns: 0,
        }
    }
}

impl CellSpace {
    /// Create a new cell space with the specified number of cells
    pub fn new(cell_count: usize) -> Self {
        let cell_size = 16; // 16 bytes per cell
        let total_size = cell_count * cell_size;

        Self {
            allocator: CellAllocator::new(cell_size, cell_count),
            total_size,
            stats: SpaceStats {
                space_type: SpaceType::CellSpace,
                total_size,
                allocated_size: 0,
                free_size: total_size,
                object_count: 0,
                fragmentation_percentage: 0.0,
                allocation_count: 0,
                deallocation_count: 0,
            },
            object_types: std::collections::HashMap::new(),
            metrics: CellMetrics::default(),
        }
    }

    /// Get cell information
    pub fn cell_info(&self) -> CellInfo {
        CellInfo {
            total_cells: self.total_size / 16,
            allocated_cells: self.allocator.total_allocated().as_usize() / 16,
            free_cells: (self.total_size - self.allocator.total_allocated().as_usize()) / 16,
            cell_size: 16, // Default cell size
            fragmentation: self.allocator.fragmentation(),
            efficiency: self.efficiency(),
        }
    }

    /// Get object type information
    pub fn object_type_info(&self) -> ObjectTypeInfo {
        let mut type_counts = std::collections::HashMap::new();

        for object_type in self.object_types.values() {
            *type_counts.entry(object_type.clone()).or_insert(0) += 1;
        }

        ObjectTypeInfo {
            total_objects: self.object_types.len(),
            type_distribution: type_counts.clone(),
            most_common_type: type_counts
                .iter()
                .max_by_key(|(_, &count)| count)
                .map(|(object_type, _)| object_type.clone()),
        }
    }

    /// Get performance metrics
    pub fn performance_metrics(&self) -> &CellMetrics {
        &self.metrics
    }

    /// Get space efficiency
    pub fn efficiency(&self) -> f64 {
        let used_cells = self.allocator.total_allocated().as_usize() / 16;
        let total_cells = self.total_size / 16;
        (used_cells as f64 / total_cells as f64) * 100.0
    }

    /// Get memory density
    pub fn memory_density(&self) -> f64 {
        let used_memory = self.allocator.total_allocated().as_usize();
        let total_memory = self.total_size;
        (used_memory as f64 / total_memory as f64) * 100.0
    }

    /// Check if compaction is needed
    pub fn should_compact(&self) -> bool {
        let fragmentation = self.allocator.fragmentation();
        let efficiency = self.efficiency();

        // Compact if fragmentation is high or efficiency is low
        fragmentation > 25.0 || efficiency < 50.0
    }

    /// Get space health score
    pub fn health_score(&self) -> f64 {
        let efficiency = self.efficiency();
        let density = self.memory_density();
        let fragmentation = self.allocator.fragmentation();

        // Higher efficiency, density, and lower fragmentation = better health
        let efficiency_score = efficiency / 100.0;
        let density_score = density / 100.0;
        let fragmentation_score = 1.0 - (fragmentation / 100.0);

        (efficiency_score + density_score + fragmentation_score) / 3.0 * 100.0
    }

    /// Perform garbage collection
    pub fn collect(&mut self) -> GcStats {
        let start_time = std::time::Instant::now();

        // Get current usage before collection
        let before_usage = self.allocator.total_allocated().as_usize();
        let before_objects = self.object_types.len();

        // Simple collection simulation
        // In a real implementation, this would mark live objects
        let mut objects_to_remove = Vec::new();

        // Simulate some objects becoming unreachable
        for (&handle, object_type) in &self.object_types {
            // Simulate collection based on object type
            match object_type {
                SmallObjectType::Undefined | SmallObjectType::Null => {
                    // These are always collected
                    objects_to_remove.push(handle);
                }
                SmallObjectType::Boolean => {
                    // Booleans have 50% chance of being collected (deterministic)
                    if (handle.as_usize() % 2) == 0 {
                        objects_to_remove.push(handle);
                    }
                }
                _ => {
                    // Other types have 10% chance of being collected (deterministic)
                    if (handle.as_usize() % 10) == 0 {
                        objects_to_remove.push(handle);
                    }
                }
            }
        }

        // Remove dead objects
        for handle in &objects_to_remove {
            if self.allocator.deallocate(handle.as_usize(), MemorySize::new(0)) {
                self.object_types.remove(handle);
            }
        }

        // Calculate collection statistics
        let objects_collected = before_objects - self.object_types.len();
        let bytes_freed = before_usage - self.allocator.total_allocated().as_usize();

        let end_time = std::time::Instant::now();
        let collection_time = end_time.duration_since(start_time).as_micros() as u64;

        // Update statistics
        self.stats.object_count = self.object_types.len();
        self.stats.allocated_size = self.allocator.total_allocated().as_usize();
        self.stats.free_size = self.allocator.total_free().as_usize();
        self.stats.fragmentation_percentage = self.allocator.fragmentation();

        GcStats {
            objects_collected,
            bytes_freed,
            collection_time,
        }
    }

    /// Compact the space
    pub fn compact(&mut self) -> CompactionStats {
        let start_time = std::time::Instant::now();

        // Perform compaction
        let stats = self.allocator.compact();

        // Update metrics
        let end_time = std::time::Instant::now();
        let compaction_time = end_time.duration_since(start_time).as_nanos() as u64;

        self.metrics.compaction_time_ns = compaction_time;
        self.metrics.total_operations += 1;
        self.metrics.average_operation_time_ns = (self.metrics.allocation_time_ns
            + self.metrics.deallocation_time_ns
            + self.metrics.compaction_time_ns)
            / self.metrics.total_operations as u64;

        // Update statistics
        self.stats.fragmentation_percentage = self.allocator.fragmentation();

        stats
    }

    /// Get detailed space information
    pub fn detailed_info(&self) -> CellSpaceInfo {
        CellSpaceInfo {
            cell_info: self.cell_info(),
            object_type_info: self.object_type_info(),
            performance_metrics: self.performance_metrics().clone(),
            efficiency: self.efficiency(),
            memory_density: self.memory_density(),
            health_score: self.health_score(),
            should_compact: self.should_compact(),
        }
    }
}

impl MemorySpace for CellSpace {
    fn allocate(&mut self, size: MemorySize) -> Option<HeapHandleId> {
        let start_time = std::time::Instant::now();

        if let Some(handle) = self.allocator.allocate(size) {
            // Determine object type based on size and context
            let object_type = if size.as_usize() <= 8 {
                SmallObjectType::Number
            } else if size.as_usize() <= 16 {
                SmallObjectType::String
            } else {
                SmallObjectType::Other
            };

            // Track object type
            self.object_types
                .insert(HeapHandleId::from(handle), object_type);

            // Update statistics
            self.stats.allocated_size += size.as_usize();
            self.stats.object_count += 1;
            self.stats.allocation_count += 1;
            self.stats.free_size = self.allocator.total_free().as_usize();

            // Update metrics
            let end_time = std::time::Instant::now();
            let allocation_time = end_time.duration_since(start_time).as_nanos() as u64;

            self.metrics.allocation_time_ns += allocation_time;
            self.metrics.total_operations += 1;
            self.metrics.average_operation_time_ns = (self.metrics.allocation_time_ns
                + self.metrics.deallocation_time_ns
                + self.metrics.compaction_time_ns)
                / self.metrics.total_operations as u64;

            Some(HeapHandleId::from(handle))
        } else {
            None
        }
    }

    fn deallocate(&mut self, handle: HeapHandleId) -> bool {
        let start_time = std::time::Instant::now();

        if self
            .allocator
            .deallocate(handle.as_usize(), MemorySize::new(0))
        {
            // Remove type tracking
            self.object_types.remove(&handle);

            // Update statistics
            self.stats.object_count = self.object_types.len();
            self.stats.allocated_size = self.allocator.total_allocated().as_usize();
            self.stats.free_size = self.allocator.total_free().as_usize();
            self.stats.deallocation_count += 1;

            // Update metrics
            let end_time = std::time::Instant::now();
            let deallocation_time = end_time.duration_since(start_time).as_nanos() as u64;

            self.metrics.deallocation_time_ns += deallocation_time;
            self.metrics.total_operations += 1;
            self.metrics.average_operation_time_ns = (self.metrics.allocation_time_ns
                + self.metrics.deallocation_time_ns
                + self.metrics.compaction_time_ns)
                / self.metrics.total_operations as u64;

            true
        } else {
            false
        }
    }

    fn can_allocate(&self, size: MemorySize) -> bool {
        self.allocator.can_allocate(size)
    }

    fn total_allocated(&self) -> MemorySize {
        self.allocator.total_allocated()
    }

    fn total_free(&self) -> MemorySize {
        self.allocator.total_free()
    }

    fn stats(&self) -> SpaceStats {
        self.stats.clone()
    }

    fn space_type(&self) -> SpaceType {
        SpaceType::CellSpace
    }

    fn extract_object(&mut self, handle: HeapHandleId) -> Option<Value> {
        // Extract object from allocator
        if let Some(object_data) = self.allocator.extract_object(handle.as_usize()) {
            // Remove type tracking
            self.object_types.remove(&handle);

            // Update statistics
            self.stats.object_count = self.stats.object_count.saturating_sub(1);
            self.stats.allocated_size = self
                .stats
                .allocated_size
                .saturating_sub(object_data.size().unwrap_or(0));

            Some(object_data)
        } else {
            None
        }
    }

    fn allocate_object(&mut self, data: Value) -> Option<HeapHandleId> {
        let size = MemorySize::new(data.size().unwrap_or(16));
        if let Some(handle) = self.allocator.allocate_object(data.clone()) {
            // Determine object type based on value
            let object_type = match data {
                Value::String(_) => SmallObjectType::String,
                Value::Number(_) => SmallObjectType::Number,
                Value::Boolean(_) => SmallObjectType::Boolean,
                Value::Undefined => SmallObjectType::Undefined,
                Value::Null => SmallObjectType::Null,
                _ => SmallObjectType::Other,
            };

            // Track object type
            self.object_types
                .insert(HeapHandleId::from(handle), object_type);

            // Update statistics
            self.stats.allocated_size += size.bytes();
            self.stats.object_count += 1;
            self.stats.allocation_count += 1;

            // Update free space
            self.stats.free_size = self.allocator.total_free().bytes();

            Some(HeapHandleId::from(handle))
        } else {
            None
        }
    }
}

/// Information about object types
#[derive(Debug, Clone)]
pub struct ObjectTypeInfo {
    pub total_objects: usize,
    pub type_distribution: std::collections::HashMap<SmallObjectType, usize>,
    pub most_common_type: Option<SmallObjectType>,
}

/// Detailed information about cell space
#[derive(Debug, Clone)]
pub struct CellSpaceInfo {
    pub cell_info: CellInfo,
    pub object_type_info: ObjectTypeInfo,
    pub performance_metrics: CellMetrics,
    pub efficiency: f64,
    pub memory_density: f64,
    pub health_score: f64,
    pub should_compact: bool,
}

impl Default for CellSpace {
    fn default() -> Self {
        Self::new(1024 * 1024) // 1M cells = 16MB default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_space_new() {
        let cell_space = CellSpace::new(100);
        assert_eq!(cell_space.total_size, 100 * 16);
        assert_eq!(cell_space.total_allocated().as_usize(), 0);
        assert_eq!(cell_space.total_free().as_usize(), 100 * 16);
        assert_eq!(cell_space.stats.object_count, 0);
    }

    #[test]
    fn test_cell_space_allocate() {
        let mut cell_space = CellSpace::new(100);

        let handle = cell_space.allocate(MemorySize::new(8));
        assert!(handle.is_some());
        assert_eq!(cell_space.total_allocated().as_usize(), 16); // Cell size
        assert_eq!(cell_space.stats.object_count, 1);
        assert_eq!(cell_space.stats.allocation_count, 1);

        // Check object type tracking
        assert_eq!(
            cell_space.object_types.get(&handle.unwrap()),
            Some(&SmallObjectType::Number)
        );
    }

    #[test]
    fn test_cell_space_deallocate() {
        let mut cell_space = CellSpace::new(100);

        let handle = cell_space.allocate(MemorySize::new(8)).unwrap();
        assert_eq!(cell_space.stats.object_count, 1);

        assert!(cell_space.deallocate(handle));
        assert_eq!(cell_space.stats.object_count, 0);
        assert_eq!(cell_space.stats.deallocation_count, 1);

        // Type tracking should be removed
        assert!(cell_space.object_types.is_empty());
    }

    #[test]
    fn test_cell_space_efficiency() {
        let mut cell_space = CellSpace::new(100);

        // Allocate 50 cells
        for _ in 0..50 {
            cell_space.allocate(MemorySize::new(8));
        }

        let efficiency = cell_space.efficiency();
        assert_eq!(efficiency, 50.0); // 50/100 = 50%

        let density = cell_space.memory_density();
        assert_eq!(density, 50.0); // 50*16/100*16 = 50%
    }

    #[test]
    fn test_cell_space_compact() {
        let mut cell_space = CellSpace::new(100);

        // Allocate some cells
        let handles: Vec<HeapHandleId> = (0..20)
            .map(|_| cell_space.allocate(MemorySize::new(8)).unwrap())
            .collect();

        // Deallocate some cells to create fragmentation
        cell_space.deallocate(handles[5]);
        cell_space.deallocate(handles[10]);
        cell_space.deallocate(handles[15]);

        let initial_fragmentation = cell_space.allocator.fragmentation();

        // Compact
        let stats = cell_space.compact();
        assert!(stats.final_fragmentation < stats.initial_fragmentation);
        assert!(stats.cells_moved > 0);
    }

    #[test]
    fn test_cell_space_health_score() {
        let mut cell_space = CellSpace::new(100);

        // Empty space should have good health
        let health = cell_space.health_score();
        assert!(health > 80.0);

        // Allocate some cells
        for _ in 0..50 {
            cell_space.allocate(MemorySize::new(8));
        }

        // Space with 50% usage should have good health
        let health = cell_space.health_score();
        assert!(health > 60.0);
    }
}
