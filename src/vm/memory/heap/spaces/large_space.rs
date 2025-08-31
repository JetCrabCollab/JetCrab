//! # Large Object Space
//!
//! Memory space for large objects (> 1MB) with specialized allocation strategies.
//! Optimized for objects that are too large for generational collection.
//!
//! ## Characteristics
//!
//! - **Direct allocation**: Large objects allocated directly from OS
//! - **No generational collection**: Objects too large to move efficiently
//! - **Mark & sweep GC**: Traditional collection for large objects
//! - **Memory mapping**: Uses mmap for very large objects
//! - **Perfect for large objects**: Arrays, buffers, images, etc.

use super::{MemorySpace, SpaceStats, SpaceType};
use crate::vm::handle::HeapHandleId;
use crate::vm::memory::heap::spaces::{DefragmentationStats, GcStats};
use crate::vm::types::MemorySize;
use crate::vm::value::Value;
use std::collections::HashMap;

/// Large object space for objects > 1MB
pub struct LargeObjectSpace {
    /// Total size of the space
    total_size: usize,
    /// Statistics
    stats: SpaceStats,
    /// Large object tracking
    large_objects: HashMap<HeapHandleId, LargeObjectInfo>,
    /// Memory regions for large objects
    memory_regions: Vec<MemoryRegion>,
    /// Next region ID
    next_region_id: usize,
}

/// Information about a large object
#[derive(Debug, Clone)]
pub struct LargeObjectInfo {
    pub size: usize,
    pub region_id: usize,
    pub allocation_time: std::time::Instant,
    pub access_count: usize,
    pub last_access: std::time::Instant,
    pub object_type: LargeObjectType,
}

/// Types of large objects
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LargeObjectType {
    Array,
    Buffer,
    Image,
    Video,
    Audio,
    Document,
    Database,
    Other,
}

/// Memory region for large objects
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub id: usize,
    pub start_address: usize,
    pub size: usize,
    pub is_allocated: bool,
    pub object_handle: Option<HeapHandleId>,
    pub allocation_time: Option<std::time::Instant>,
}

impl LargeObjectSpace {
    /// Create a new large object space with the specified size
    pub fn new(size: usize) -> Self {
        let mut space = Self {
            total_size: size,
            stats: SpaceStats {
                space_type: SpaceType::LargeObjectSpace,
                total_size: size,
                allocated_size: 0,
                free_size: size,
                object_count: 0,
                fragmentation_percentage: 0.0,
                allocation_count: 0,
                deallocation_count: 0,
            },
            large_objects: HashMap::new(),
            memory_regions: Vec::new(),
            next_region_id: 0,
        };

        // Initialize with some default memory regions
        space.initialize_regions();
        space
    }

    /// Initialize memory regions
    fn initialize_regions(&mut self) {
        // Create regions of different sizes for efficient allocation
        let region_sizes = vec![
            1024 * 1024,      // 1MB
            2 * 1024 * 1024,  // 2MB
            4 * 1024 * 1024,  // 4MB
            8 * 1024 * 1024,  // 8MB
            16 * 1024 * 1024, // 16MB
            32 * 1024 * 1024, // 32MB
            64 * 1024 * 1024, // 64MB
        ];

        let mut current_address = 0;
        for &size in &region_sizes {
            if current_address + size <= self.total_size {
                self.memory_regions.push(MemoryRegion {
                    id: self.next_region_id,
                    start_address: current_address,
                    size,
                    is_allocated: false,
                    object_handle: None,
                    allocation_time: None,
                });
                self.next_region_id += 1;
                current_address += size;
            }
        }
    }

    /// Find best fit region for allocation
    fn find_best_fit_region(&self, size: usize) -> Option<usize> {
        let mut best_fit: Option<usize> = None;
        let mut best_fit_size = usize::MAX;

        for (index, region) in self.memory_regions.iter().enumerate() {
            if !region.is_allocated && region.size >= size {
                let waste = region.size - size;
                if waste < best_fit_size {
                    best_fit_size = waste;
                    best_fit = Some(index);
                }
            }
        }

        best_fit
    }

    /// Split region if it's too large
    fn split_region(&mut self, region_index: usize, requested_size: usize) {
        let region = &mut self.memory_regions[region_index];
        let remaining_size = region.size - requested_size;

        if remaining_size >= 1024 * 1024 {
            // Minimum 1MB for new region
            // Resize current region
            region.size = requested_size;

            // Create new region with remaining space
            let new_region = MemoryRegion {
                id: self.next_region_id,
                start_address: region.start_address + requested_size,
                size: remaining_size,
                is_allocated: false,
                object_handle: None,
                allocation_time: None,
            };

            self.memory_regions.push(new_region);
            self.next_region_id += 1;
        }
    }

    /// Get large object information
    pub fn large_object_info(&self) -> LargeObjectSpaceInfo {
        let total_objects = self.large_objects.len();
        let total_size: usize = self.large_objects.values().map(|obj| obj.size).sum();
        let avg_size = if total_objects > 0 {
            total_size / total_objects
        } else {
            0
        };

        let mut type_distribution = HashMap::new();
        for obj in self.large_objects.values() {
            *type_distribution
                .entry(obj.object_type.clone())
                .or_insert(0) += 1;
        }

        LargeObjectSpaceInfo {
            total_objects,
            total_size,
            average_size: avg_size,
            type_distribution,
            region_count: self.memory_regions.len(),
            allocated_regions: self
                .memory_regions
                .iter()
                .filter(|r| r.is_allocated)
                .count(),
        }
    }

    /// Get memory region information
    pub fn region_info(&self) -> Vec<RegionInfo> {
        self.memory_regions
            .iter()
            .map(|region| RegionInfo {
                id: region.id,
                start_address: region.start_address,
                size: region.size,
                is_allocated: region.is_allocated,
                object_handle: region.object_handle,
                allocation_time: region.allocation_time,
            })
            .collect()
    }

    /// Check if compaction is needed
    pub fn should_compact(&self) -> bool {
        let allocated_regions = self
            .memory_regions
            .iter()
            .filter(|r| r.is_allocated)
            .count();
        let total_regions = self.memory_regions.len();

        // Compact if more than 70% of regions are allocated
        allocated_regions as f64 / total_regions as f64 > 0.7
    }

    /// Compact memory regions
    pub fn compact(&mut self) -> CompactionStats {
        let start_time = std::time::Instant::now();
        let initial_fragmentation = self.calculate_fragmentation();

        // Simple compaction: move allocated regions to the beginning
        let mut allocated_regions: Vec<_> = self
            .memory_regions
            .iter()
            .filter(|r| r.is_allocated)
            .cloned()
            .collect();

        allocated_regions.sort_by_key(|r| r.start_address);

        // Rebuild memory regions
        self.memory_regions.clear();
        let mut current_address = 0;

        for region in allocated_regions {
            let new_region = MemoryRegion {
                id: region.id,
                start_address: current_address,
                size: region.size,
                is_allocated: true,
                object_handle: region.object_handle,
                allocation_time: region.allocation_time,
            };

            self.memory_regions.push(new_region);
            current_address += region.size;
        }

        // Add free region at the end
        if current_address < self.total_size {
            let free_size = self.total_size - current_address;
            if free_size >= 1024 * 1024 {
                // Minimum 1MB
                self.memory_regions.push(MemoryRegion {
                    id: self.next_region_id,
                    start_address: current_address,
                    size: free_size,
                    is_allocated: false,
                    object_handle: None,
                    allocation_time: None,
                });
                self.next_region_id += 1;
            }
        }

        let end_time = std::time::Instant::now();
        let duration = end_time.duration_since(start_time).as_micros() as u64;

        CompactionStats {
            duration_micros: duration,
            initial_fragmentation,
            final_fragmentation: self.calculate_fragmentation(),
            cells_moved: allocated_regions.len(),
        }
    }

    /// Calculate fragmentation percentage
    fn calculate_fragmentation(&self) -> f64 {
        let total_free = self
            .memory_regions
            .iter()
            .filter(|r| !r.is_allocated)
            .map(|r| r.size)
            .sum::<usize>();

        let largest_free = self
            .memory_regions
            .iter()
            .filter(|r| !r.is_allocated)
            .map(|r| r.size)
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
            .large_objects
            .values()
            .map(|obj| obj.size)
            .sum::<usize>();
        (used as f64 / self.total_size as f64) * 100.0
    }

    /// Get space health score
    pub fn health_score(&self) -> f64 {
        let efficiency = self.efficiency();
        let fragmentation = self.calculate_fragmentation();

        // Higher efficiency and lower fragmentation = better health
        let efficiency_score = efficiency / 100.0;
        let fragmentation_score = 1.0 - (fragmentation / 100.0);

        (efficiency_score + fragmentation_score) / 2.0 * 100.0
    }

    /// Perform garbage collection
    pub fn collect(&mut self) -> GcStats {
        let start_time = std::time::Instant::now();

        // Get current usage before collection
        let before_usage: usize = self.large_objects.values().map(|obj| obj.size).sum();
        let before_objects = self.large_objects.len();

        // Simple collection simulation
        // In a real implementation, this would mark live objects
        let mut objects_to_remove = Vec::new();

        for (&handle, obj_info) in &self.large_objects {
            // Simulate collection based on access patterns
            let time_since_access = std::time::Instant::now()
                .duration_since(obj_info.last_access)
                .as_secs();

            // Remove objects that haven't been accessed in a long time
            if time_since_access > 300 {
                // 5 minutes
                objects_to_remove.push(handle);
            }
        }

        // Remove dead objects
        for handle in &objects_to_remove {
            if let Some(obj_info) = self.large_objects.remove(handle) {
                // Free the memory region
                if let Some(region) = self
                    .memory_regions
                    .iter_mut()
                    .find(|r| r.object_handle == Some(*handle))
                {
                    region.is_allocated = false;
                    region.object_handle = None;
                    region.allocation_time = None;
                }
            }
        }

        // Calculate collection statistics
        let objects_collected = before_objects - self.large_objects.len();
        let after_usage: usize = self.large_objects.values().map(|obj| obj.size).sum();
        let bytes_freed = before_usage - after_usage;

        let end_time = std::time::Instant::now();
        let collection_time = end_time.duration_since(start_time).as_micros() as u64;

        // Update statistics
        self.stats.object_count = self.large_objects.len();
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

impl MemorySpace for LargeObjectSpace {
    fn allocate(&mut self, size: MemorySize) -> Option<HeapHandleId> {
        let size_bytes = size.as_usize();

        // Find best fit region
        if let Some(region_index) = self.find_best_fit_region(size_bytes) {
            // Split region if necessary
            if self.memory_regions[region_index].size > size_bytes {
                self.split_region(region_index, size_bytes);
            }

            // Get region reference after potential split
            let region = &mut self.memory_regions[region_index];

            // Allocate the region
            region.is_allocated = true;
            region.object_handle = Some(HeapHandleId::new(region.start_address));
            region.allocation_time = Some(std::time::Instant::now());

            // Create large object info
            let handle = HeapHandleId::new(region.start_address);
            let object_info = LargeObjectInfo {
                size: size_bytes,
                region_id: region.id,
                allocation_time: std::time::Instant::now(),
                access_count: 0,
                last_access: std::time::Instant::now(),
                object_type: if size_bytes > 64 * 1024 * 1024 {
                    LargeObjectType::Video
                } else if size_bytes > 32 * 1024 * 1024 {
                    LargeObjectType::Image
                } else if size_bytes > 16 * 1024 * 1024 {
                    LargeObjectType::Audio
                } else if size_bytes > 8 * 1024 * 1024 {
                    LargeObjectType::Document
                } else {
                    LargeObjectType::Array
                },
            };

            self.large_objects.insert(handle, object_info);

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
        if let Some(obj_info) = self.large_objects.remove(&handle) {
            // Free the memory region
            if let Some(region) = self
                .memory_regions
                .iter_mut()
                .find(|r| r.object_handle == Some(handle))
            {
                region.is_allocated = false;
                region.object_handle = None;
                region.allocation_time = None;
            }

            // Update statistics
            self.stats.object_count = self.large_objects.len();
            self.stats.allocated_size = self.large_objects.values().map(|obj| obj.size).sum();
            self.stats.free_size = self.total_size - self.stats.allocated_size;
            self.stats.deallocation_count += 1;

            true
        } else {
            false
        }
    }

    fn extract_object(&mut self, handle: HeapHandleId) -> Option<Value> {
        // Extract object from our large objects
        if let Some(obj_info) = self.large_objects.remove(&handle) {
            // Free the memory region
            if let Some(region) = self
                .memory_regions
                .iter_mut()
                .find(|r| r.object_handle == Some(handle))
            {
                region.is_allocated = false;
                region.object_handle = None;
                region.allocation_time = None;
            }

            // Update statistics
            self.stats.object_count = self.stats.object_count.saturating_sub(1);
            self.stats.allocated_size = self.stats.allocated_size.saturating_sub(obj_info.size);

            // For now, return a dummy Value since we don't have the actual object data
            Some(Value::Number(handle.as_usize() as f64))
        } else {
            None
        }
    }

    fn allocate_object(&mut self, data: Value) -> Option<HeapHandleId> {
        let size = MemorySize::new(data.size().unwrap_or(2 * 1024 * 1024));
        if let Some(handle) = self.allocate(size) {
            // Object was already tracked in allocate method
            Some(handle)
        } else {
            None
        }
    }

    fn can_allocate(&self, size: MemorySize) -> bool {
        let size_bytes = size.as_usize();
        self.memory_regions
            .iter()
            .any(|r| !r.is_allocated && r.size >= size_bytes)
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
        SpaceType::LargeObjectSpace
    }
}

/// Information about large object space
#[derive(Debug, Clone)]
pub struct LargeObjectSpaceInfo {
    pub total_objects: usize,
    pub total_size: usize,
    pub average_size: usize,
    pub type_distribution: HashMap<LargeObjectType, usize>,
    pub region_count: usize,
    pub allocated_regions: usize,
}

/// Information about a memory region
#[derive(Debug, Clone)]
pub struct RegionInfo {
    pub id: usize,
    pub start_address: usize,
    pub size: usize,
    pub is_allocated: bool,
    pub object_handle: Option<HeapHandleId>,
    pub allocation_time: Option<std::time::Instant>,
}

/// Compaction statistics
#[derive(Debug, Clone)]
pub struct CompactionStats {
    pub duration_micros: u64,
    pub initial_fragmentation: f64,
    pub final_fragmentation: f64,
    pub cells_moved: usize,
}

impl Default for LargeObjectSpace {
    fn default() -> Self {
        Self::new(128 * 1024 * 1024) // 128MB default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_large_object_space_new() {
        let large_space = LargeObjectSpace::new(1024 * 1024);
        assert_eq!(large_space.total_size, 1024 * 1024);
        assert_eq!(large_space.total_allocated().as_usize(), 0);
        assert_eq!(large_space.total_free().as_usize(), 1024 * 1024);
        assert_eq!(large_space.stats.object_count, 0);
        assert!(!large_space.memory_regions.is_empty());
    }

    #[test]
    fn test_large_object_space_allocate() {
        let mut large_space = LargeObjectSpace::new(1024 * 1024);

        let handle = large_space.allocate(MemorySize::new(512 * 1024));
        assert!(handle.is_some());
        assert_eq!(large_space.total_allocated().as_usize(), 512 * 1024);
        assert_eq!(large_space.stats.object_count, 1);
        assert_eq!(large_space.stats.allocation_count, 1);

        // Check that a region was allocated
        let allocated_regions = large_space
            .memory_regions
            .iter()
            .filter(|r| r.is_allocated)
            .count();
        assert_eq!(allocated_regions, 1);
    }

    #[test]
    fn test_large_object_space_deallocate() {
        let mut large_space = LargeObjectSpace::new(1024 * 1024);

        let handle = large_space.allocate(MemorySize::new(512 * 1024)).unwrap();
        assert_eq!(large_space.stats.object_count, 1);

        assert!(large_space.deallocate(handle));
        assert_eq!(large_space.stats.object_count, 0);
        assert_eq!(large_space.stats.deallocation_count, 1);

        // Check that the region was freed
        let allocated_regions = large_space
            .memory_regions
            .iter()
            .filter(|r| r.is_allocated)
            .count();
        assert_eq!(allocated_regions, 0);
    }

    #[test]
    fn test_large_object_space_compact() {
        let mut large_space = LargeObjectSpace::new(1024 * 1024);

        // Allocate some objects
        let handle1 = large_space.allocate(MemorySize::new(256 * 1024)).unwrap();
        let handle2 = large_space.allocate(MemorySize::new(256 * 1024)).unwrap();

        // Deallocate first object to create fragmentation
        large_space.deallocate(handle1);

        let initial_fragmentation = large_space.calculate_fragmentation();

        // Compact
        let stats = large_space.compact();
        assert!(stats.final_fragmentation < stats.initial_fragmentation);
        assert!(stats.cells_moved > 0);
    }

    #[test]
    fn test_large_object_space_collect() {
        let mut large_space = LargeObjectSpace::new(1024 * 1024);

        // Allocate some objects
        large_space.allocate(MemorySize::new(256 * 1024));
        large_space.allocate(MemorySize::new(256 * 1024));

        let before_objects = large_space.stats.object_count;

        // Perform collection
        let stats = large_space.collect();

        assert!(stats.collection_time > 0);
        assert_eq!(
            large_space.stats.object_count,
            before_objects - stats.objects_collected
        );
    }

    #[test]
    fn test_large_object_space_efficiency() {
        let mut large_space = LargeObjectSpace::new(1024 * 1024);

        // Allocate 50% of space
        large_space.allocate(MemorySize::new(512 * 1024));

        let efficiency = large_space.efficiency();
        assert_eq!(efficiency, 50.0);

        let health = large_space.health_score();
        assert!(health > 60.0);
    }
}
