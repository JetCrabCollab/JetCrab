//! # Cell Allocator
//!
//! High-performance allocator for small objects (≤ 16 bytes) using fixed-size cells.
//! Optimized for objects that are frequently allocated and deallocated.
//!
//! ## Characteristics
//!
//! - **Fixed-size cells**: All cells are the same size for fast allocation
//! - **No fragmentation**: Each cell is exactly the right size
//! - **Fast allocation**: O(1) allocation from free cell list
//! - **Memory efficient**: Minimal overhead per object
//! - **Perfect for small objects**: Strings, numbers, booleans, etc.

use super::{Allocator, AllocationStats, AllocationError};
use crate::vm::handle::HeapHandleId;
use crate::vm::types::MemorySize;
use super::alignment::{align_up, ALIGN_8};

/// Cell size for small objects
pub const CELL_SIZE: usize = 16;

/// Cell allocator for small objects
pub struct CellAllocator {
    /// Memory region divided into cells
    memory: Vec<u8>,
    /// Free cell list (stack-based for O(1) allocation)
    free_cells: Vec<usize>,
    /// Total number of cells
    total_cells: usize,
    /// Number of allocated cells
    allocated_cells: usize,
    /// Statistics
    stats: AllocationStats,
}

impl CellAllocator {
    /// Create a new cell allocator with the specified number of cells
    pub fn new(cell_count: usize) -> Self {
        let total_size = cell_count * CELL_SIZE;
        let memory = vec![0; total_size];
        let mut free_cells = Vec::with_capacity(cell_count);
        
        // Initialize free cell list (cells are numbered from 0)
        for i in 0..cell_count {
            free_cells.push(i);
        }
        
        Self {
            memory,
            free_cells,
            total_cells: cell_count,
            allocated_cells: 0,
            stats: AllocationStats::default(),
        }
    }
    
    /// Create a new cell allocator with the specified total size
    pub fn with_size(total_size: usize) -> Self {
        let cell_count = total_size / CELL_SIZE;
        Self::new(cell_count)
    }
    
    /// Get the number of available cells
    pub fn available_cells(&self) -> usize {
        self.free_cells.len()
    }
    
    /// Get the number of allocated cells
    pub fn allocated_cells(&self) -> usize {
        self.allocated_cells
    }
    
    /// Get the total number of cells
    pub fn total_cells(&self) -> usize {
        self.total_cells
    }
    
    /// Get cell usage percentage
    pub fn usage_percentage(&self) -> f64 {
        (self.allocated_cells as f64 / self.total_cells as f64) * 100.0
    }
    
    /// Get memory efficiency (cells used vs total cells)
    pub fn efficiency(&self) -> f64 {
        let used_memory = self.allocated_cells * CELL_SIZE;
        let total_memory = self.total_cells * CELL_SIZE;
        (used_memory as f64 / total_memory as f64) * 100.0
    }
    
    /// Get cell information
    pub fn cell_info(&self) -> CellInfo {
        CellInfo {
            total_cells: self.total_cells,
            allocated_cells: self.allocated_cells,
            free_cells: self.free_cells.len(),
            cell_size: CELL_SIZE,
            usage_percentage: self.usage_percentage(),
            efficiency: self.efficiency(),
        }
    }
    
    /// Check if a cell index is valid
    fn is_valid_cell(&self, cell_index: usize) -> bool {
        cell_index < self.total_cells
    }
    
    /// Get memory address for a cell index
    fn get_cell_address(&self, cell_index: usize) -> usize {
        cell_index * CELL_SIZE
    }
    
    /// Get cell index from memory address
    fn get_cell_index(&self, address: usize) -> usize {
        address / CELL_SIZE
    }
    
    /// Write data to a cell
    pub fn write_cell(&mut self, handle: HeapHandleId, data: &[u8]) -> Result<(), AllocationError> {
        let cell_index = self.get_cell_index(handle.as_usize());
        
        if !self.is_valid_cell(cell_index) {
            return Err(AllocationError::InvalidHandle { handle });
        }
        
        let start = self.get_cell_address(cell_index);
        let end = start + CELL_SIZE;
        
        if data.len() > CELL_SIZE {
            return Err(AllocationError::SizeTooLarge { 
                size: data.len(), 
                max: CELL_SIZE 
            });
        }
        
        // Copy data to cell
        self.memory[start..start + data.len()].copy_from_slice(data);
        
        // Zero out remaining bytes
        if data.len() < CELL_SIZE {
            self.memory[start + data.len()..end].fill(0);
        }
        
        Ok(())
    }
    
    /// Read data from a cell
    pub fn read_cell(&self, handle: HeapHandleId) -> Result<&[u8], AllocationError> {
        let cell_index = self.get_cell_index(handle.as_usize());
        
        if !self.is_valid_cell(cell_index) {
            return Err(AllocationError::InvalidHandle { handle });
        }
        
        let start = self.get_cell_address(cell_index);
        let end = start + CELL_SIZE;
        
        Ok(&self.memory[start..end])
    }
    
    /// Clear a cell (set all bytes to 0)
    pub fn clear_cell(&mut self, handle: HeapHandleId) -> Result<(), AllocationError> {
        let cell_index = self.get_cell_index(handle.as_usize());
        
        if !self.is_valid_cell(cell_index) {
            return Err(AllocationError::InvalidHandle { handle });
        }
        
        let start = self.get_cell_address(cell_index);
        let end = start + CELL_SIZE;
        
        self.memory[start..end].fill(0);
        
        Ok(())
    }
    
    /// Get memory region for a cell (for direct access)
    pub fn get_cell_memory(&mut self, handle: HeapHandleId) -> Result<&mut [u8], AllocationError> {
        let cell_index = self.get_cell_index(handle.as_usize());
        
        if !self.is_valid_cell(cell_index) {
            return Err(AllocationError::InvalidHandle { handle });
        }
        
        let start = self.get_cell_address(cell_index);
        let end = start + CELL_SIZE;
        
        Ok(&mut self.memory[start..end])
    }
    
    /// Compact memory by moving allocated cells to the beginning
    pub fn compact(&mut self) -> CompactionStats {
        let start_time = std::time::Instant::now();
        let initial_fragmentation = self.calculate_fragmentation();
        
        // Create a map of old to new positions
        let mut old_to_new: Vec<Option<usize>> = vec![None; self.total_cells];
        let mut new_cell_index = 0;
        
        // Find all allocated cells and assign new positions
        for cell_index in 0..self.total_cells {
            if !self.free_cells.contains(&cell_index) {
                old_to_new[cell_index] = Some(new_cell_index);
                new_cell_index += 1;
            }
        }
        
        // Move cells to new positions
        let mut moved_cells = 0;
        for old_index in 0..self.total_cells {
            if let Some(new_index) = old_to_new[old_index] {
                if old_index != new_index {
                    let old_start = self.get_cell_address(old_index);
                    let new_start = self.get_cell_address(new_index);
                    
                    // Copy cell data
                    self.memory.copy_within(old_start..old_start + CELL_SIZE, new_start);
                    
                    // Clear old cell
                    self.memory[old_start..old_start + CELL_SIZE].fill(0);
                    
                    moved_cells += 1;
                }
            }
        }
        
        // Rebuild free cell list
        self.free_cells.clear();
        for i in new_cell_index..self.total_cells {
            self.free_cells.push(i);
        }
        
        let end_time = std::time::Instant::now();
        let duration = end_time.duration_since(start_time);
        
        CompactionStats {
            duration_micros: duration.as_micros() as u64,
            initial_fragmentation,
            final_fragmentation: self.calculate_fragmentation(),
            cells_moved: moved_cells,
        }
    }
    
    /// Calculate fragmentation percentage
    fn calculate_fragmentation(&self) -> f64 {
        if self.free_cells.is_empty() {
            return 0.0;
        }
        
        let total_free = self.free_cells.len();
        let largest_free_run = self.get_largest_free_run();
        
        if total_free == 0 {
            0.0
        } else {
            (1.0 - (largest_free_run as f64 / total_free as f64)) * 100.0
        }
    }
    
    /// Get the largest consecutive run of free cells
    fn get_largest_free_run(&self) -> usize {
        let mut free_cells_sorted = self.free_cells.clone();
        free_cells_sorted.sort();
        
        let mut max_run = 0;
        let mut current_run = 0;
        
        for i in 0..free_cells_sorted.len() {
            if i == 0 || free_cells_sorted[i] == free_cells_sorted[i - 1] + 1 {
                current_run += 1;
            } else {
                current_run = 1;
            }
            
            if current_run > max_run {
                max_run = current_run;
            }
        }
        
        max_run
    }
}

impl Allocator for CellAllocator {
    fn allocate(&mut self, size: MemorySize) -> Option<HeapHandleId> {
        let aligned_size = align_up(size.as_usize(), ALIGN_8);
        
        if aligned_size > CELL_SIZE {
            return None; // Too large for cell allocator
        }
        
        if self.free_cells.is_empty() {
            return None; // No free cells available
        }
        
        // Pop a free cell from the stack
        let cell_index = self.free_cells.pop().unwrap();
        let address = self.get_cell_address(cell_index);
        let handle = HeapHandleId::new(address);
        
        // Update statistics
        self.allocated_cells += 1;
        self.stats.total_allocations += 1;
        self.stats.current_allocations += 1;
        self.stats.total_allocated_bytes += CELL_SIZE;
        
        if self.stats.total_allocated_bytes > self.stats.peak_allocated_bytes {
            self.stats.peak_allocated_bytes = self.stats.total_allocated_bytes;
        }
        
        self.stats.average_allocation_size = 
            self.stats.total_allocated_bytes as f64 / self.stats.total_allocations as f64;
        
        Some(handle)
    }
    
    fn deallocate(&mut self, handle: HeapHandleId) -> bool {
        let cell_index = self.get_cell_index(handle.as_usize());
        
        if !self.is_valid_cell(cell_index) {
            return false;
        }
        
        // Check if cell is actually allocated
        if self.free_cells.contains(&cell_index) {
            return false; // Already free
        }
        
        // Clear the cell
        let start = self.get_cell_address(cell_index);
        let end = start + CELL_SIZE;
        self.memory[start..end].fill(0);
        
        // Add back to free list
        self.free_cells.push(cell_index);
        self.allocated_cells -= 1;
        
        // Update statistics
        self.stats.total_deallocations += 1;
        self.stats.current_allocations = self.stats.current_allocations.saturating_sub(1);
        self.stats.total_deallocated_bytes += CELL_SIZE;
        
        true
    }
    
    fn can_allocate(&self, size: usize) -> bool {
        let aligned_size = align_up(size, ALIGN_8);
        aligned_size <= CELL_SIZE && !self.free_cells.is_empty()
    }
    
    fn total_allocated(&self) -> MemorySize {
        MemorySize::new(self.allocated_cells * CELL_SIZE)
    }
    
    fn total_free(&self) -> MemorySize {
        MemorySize::new(self.free_cells.len() * CELL_SIZE)
    }
    
    fn fragmentation(&self) -> f64 {
        self.calculate_fragmentation()
    }
    
    fn stats(&self) -> AllocationStats {
        self.stats.clone()
    }
}

/// Cell information
#[derive(Debug, Clone)]
pub struct CellInfo {
    pub total_cells: usize,
    pub allocated_cells: usize,
    pub free_cells: usize,
    pub cell_size: usize,
    pub usage_percentage: f64,
    pub efficiency: f64,
}

/// Compaction statistics
#[derive(Debug, Clone)]
pub struct CompactionStats {
    pub duration_micros: u64,
    pub initial_fragmentation: f64,
    pub final_fragmentation: f64,
    pub cells_moved: usize,
}

impl Default for CellAllocator {
    fn default() -> Self {
        Self::new(1024) // 1024 cells = 16KB default
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cell_allocator_new() {
        let allocator = CellAllocator::new(100);
        assert_eq!(allocator.total_cells(), 100);
        assert_eq!(allocator.allocated_cells(), 0);
        assert_eq!(allocator.available_cells(), 100);
        assert_eq!(allocator.usage_percentage(), 0.0);
    }
    
    #[test]
    fn test_cell_allocator_allocate() {
        let mut allocator = CellAllocator::new(100);
        
        let handle = allocator.allocate(MemorySize::new(8));
        assert!(handle.is_some());
        assert_eq!(allocator.allocated_cells(), 1);
        assert_eq!(allocator.available_cells(), 99);
        assert_eq!(allocator.usage_percentage(), 1.0);
    }
    
    #[test]
    fn test_cell_allocator_deallocate() {
        let mut allocator = CellAllocator::new(100);
        
        let handle = allocator.allocate(MemorySize::new(8)).unwrap();
        assert_eq!(allocator.allocated_cells(), 1);
        
        assert!(allocator.deallocate(handle));
        assert_eq!(allocator.allocated_cells(), 0);
        assert_eq!(allocator.available_cells(), 100);
    }
    
    #[test]
    fn test_cell_allocator_write_read() {
        let mut allocator = CellAllocator::new(100);
        let handle = allocator.allocate(MemorySize::new(8)).unwrap();
        
        let test_data = b"Hello";
        assert!(allocator.write_cell(handle, test_data).is_ok());
        
        let read_data = allocator.read_cell(handle).unwrap();
        assert_eq!(&read_data[..5], test_data);
    }
    
    #[test]
    fn test_cell_allocator_size_limit() {
        let mut allocator = CellAllocator::new(100);
        
        // Try to allocate more than CELL_SIZE
        let handle = allocator.allocate(MemorySize::new(CELL_SIZE + 1));
        assert!(handle.is_none());
    }
    
    #[test]
    fn test_cell_allocator_compact() {
        let mut allocator = CellAllocator::new(100);
        
        // Allocate some cells
        let handles: Vec<HeapHandleId> = (0..10)
            .map(|_| allocator.allocate(MemorySize::new(8)).unwrap())
            .collect();
        
        // Deallocate some cells to create fragmentation
        allocator.deallocate(handles[2]);
        allocator.deallocate(handles[5]);
        allocator.deallocate(handles[8]);
        
        let initial_fragmentation = allocator.fragmentation();
        
        // Compact
        let stats = allocator.compact();
        assert!(stats.final_fragmentation < stats.initial_fragmentation);
        assert!(stats.cells_moved > 0);
    }
}
