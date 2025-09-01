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

use crate::vm::memory::heap::allocation::Allocator;
use crate::vm::memory::heap::types::CellId;
use crate::vm::types::MemorySize;

/// Cell in the cell allocator
#[derive(Debug, Clone)]
struct Cell {
    /// Start address of the cell
    start: usize,
    /// Size of the cell
    size: usize,
    /// Whether the cell is allocated
    allocated: bool,
    /// Next cell in the chain (for chaining)
    next: Option<CellId>,
}

/// Cell allocator for small, fixed-size objects
///
/// This allocator is optimized for objects of similar sizes,
/// reducing fragmentation and improving allocation speed.
pub struct CellAllocator {
    /// All cells
    cells: Vec<Cell>,
    /// Free cells
    free_cells: Vec<CellId>,
    /// Cell size
    cell_size: usize,
    /// Total number of cells
    total_cells: usize,
    /// Total allocated memory
    total_allocated: MemorySize,
    /// Total freed memory
    total_freed: MemorySize,
    /// Peak memory usage
    peak_usage: MemorySize,
}

impl CellAllocator {
    /// Create a new cell allocator
    pub fn new(cell_size: usize, cell_count: usize) -> Self {
        let mut cells = Vec::with_capacity(cell_count);
        let mut free_cells = Vec::with_capacity(cell_count);

        for i in 0..cell_count {
            let cell = Cell {
                start: i * cell_size,
                size: cell_size,
                allocated: false,
                next: None,
            };
            cells.push(cell);
            free_cells.push(CellId::new(i));
        }

        Self {
            cells,
            free_cells,
            cell_size,
            total_cells: cell_count,
            total_allocated: MemorySize::new(0),
            total_freed: MemorySize::new(0),
            peak_usage: MemorySize::new(0),
        }
    }

    /// Get the number of free cells
    pub fn free_cell_count(&self) -> usize {
        self.free_cells.len()
    }

    /// Get the number of allocated cells
    pub fn allocated_cell_count(&self) -> usize {
        self.total_cells - self.free_cells.len()
    }

    /// Get cell usage percentage
    pub fn usage_percentage(&self) -> f64 {
        let allocated = self.allocated_cell_count();
        (allocated as f64 / self.total_cells as f64) * 100.0
    }

    /// Compact the cell space by moving allocated cells together
    pub fn compact(&mut self) -> crate::vm::memory::heap::allocation::CompactionStats {
        let start_time = std::time::Instant::now();
        let fragmentation_before = self.fragmentation();

        // Create a map of old to new positions
        let mut old_to_new: Vec<Option<CellId>> = vec![None; self.total_cells];
        let mut new_cell_index = 0;

        // First pass: mark cells that should stay
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.allocated {
                old_to_new[i] = Some(CellId::new(new_cell_index));
                new_cell_index += 1;
            }
        }

        // Second pass: rebuild cells array
        let mut new_cells = Vec::with_capacity(self.total_cells);
        let mut new_free_cells = Vec::new();

        // Add allocated cells first
        for cell in self.cells.iter() {
            if cell.allocated {
                let new_cell = Cell {
                    start: new_cells.len() * self.cell_size,
                    size: self.cell_size,
                    allocated: true,
                    next: None,
                };
                new_cells.push(new_cell);
            }
        }

        // Add free cells
        for _i in new_cell_index..self.total_cells {
            let new_cell = Cell {
                start: new_cells.len() * self.cell_size,
                size: self.cell_size,
                allocated: false,
                next: None,
            };
            new_cells.push(new_cell);
            new_free_cells.push(CellId::new(_i));
        }

        // Update internal state
        self.cells = new_cells;
        self.free_cells = new_free_cells;

        let _end_time = std::time::Instant::now();
        let _duration = _end_time.duration_since(start_time);

        crate::vm::memory::heap::allocation::CompactionStats {
            objects_moved: self.allocated_cell_count(),
            memory_compacted: MemorySize::new(self.total_cells * self.cell_size),
            fragmentation_before,
            fragmentation_after: self.fragmentation(),
        }
    }
}

impl Allocator for CellAllocator {
    fn allocate(&mut self, size: MemorySize) -> Option<usize> {
        let size_bytes = size.bytes();

        // Check if the size fits in a cell
        if size_bytes > self.cell_size {
            return None;
        }

        // Get a free cell
        if let Some(cell_id) = self.free_cells.pop() {
            if let Some(cell) = self.cells.get_mut(cell_id.as_usize()) {
                cell.allocated = true;
                self.total_allocated =
                    MemorySize::new(self.total_allocated.bytes() + self.cell_size);

                if self.total_allocated.bytes() > self.peak_usage.bytes() {
                    self.peak_usage = self.total_allocated;
                }

                return Some(cell.start);
            }
        }

        None
    }

    fn deallocate(&mut self, address: usize, _size: MemorySize) -> bool {
        let cell_index = address / self.cell_size;

        if cell_index >= self.total_cells {
            return false;
        }

        if let Some(cell) = self.cells.get_mut(cell_index) {
            if cell.allocated {
                cell.allocated = false;
                self.free_cells.push(CellId::new(cell_index));
                self.total_freed = MemorySize::new(self.total_freed.bytes() + self.cell_size);
                return true;
            }
        }

        false
    }

    fn can_allocate(&self, size: MemorySize) -> bool {
        let size_bytes = size.bytes();
        size_bytes <= self.cell_size && !self.free_cells.is_empty()
    }

    fn total_allocated(&self) -> MemorySize {
        self.total_allocated
    }

    fn total_free(&self) -> MemorySize {
        MemorySize::new(self.free_cells.len() * self.cell_size)
    }

    fn fragmentation(&self) -> f64 {
        let allocated_count = self.allocated_cell_count();
        if allocated_count == 0 {
            return 0.0;
        }

        // Calculate fragmentation based on how spread out allocated cells are
        let mut gaps = 0;
        let mut last_allocated = None;

        for (i, cell) in self.cells.iter().enumerate() {
            if cell.allocated {
                if let Some(last) = last_allocated {
                    if i - last > 1 {
                        gaps += 1;
                    }
                }
                last_allocated = Some(i);
            }
        }

        if gaps == 0 {
            0.0
        } else {
            (gaps as f64 / allocated_count as f64) * 100.0
        }
    }
}

impl Default for CellAllocator {
    fn default() -> Self {
        Self::new(64, 1024) // 64-byte cells, 1024 cells
    }
}
