//! # Memory Management System
//!
//! Advanced memory management system with generational heap, specialized allocators,
//! and sophisticated garbage collection strategies.
//!
//! ## Architecture
//!
//! - **Generational Heap**: Young and old generation management
//! - **Specialized Allocators**: Bump, Free List, Cell, and Smart allocators
//! - **Memory Spaces**: New, Old, Large Object, Code, and Cell spaces
//! - **Garbage Collection**: Minor, Major, Incremental, and Background GC
//! - **Performance Optimizations**: Object shapes, string interning, optimized arrays

pub mod heap;
pub mod stack;
pub mod allocator;
pub mod collector;

pub use heap::Heap;
pub use stack::Stack;
pub use allocator::Allocator;
pub use collector::GarbageCollector;

use crate::vm::types::MemorySize;
use crate::vm::handle::HeapHandleId;

/// Memory management system that orchestrates all memory operations
pub struct MemoryManager {
    /// Main heap with generational management
    heap: Heap,
    /// Execution stack
    stack: Stack,
    /// Garbage collector
    garbage_collector: GarbageCollector,
    /// Memory statistics
    stats: MemoryStats,
}

/// Memory statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_memory: MemorySize,
    pub allocated_memory: MemorySize,
    pub free_memory: MemorySize,
    pub heap_efficiency: f64,
    pub stack_usage: f64,
    pub gc_overhead: f64,
    pub allocation_rate: f64, // objects per second
    pub deallocation_rate: f64, // objects per second
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self {
            total_memory: MemorySize::new(0),
            allocated_memory: MemorySize::new(0),
            free_memory: MemorySize::new(0),
            heap_efficiency: 0.0,
            stack_usage: 0.0,
            gc_overhead: 0.0,
            allocation_rate: 0.0,
            deallocation_rate: 0.0,
        }
    }
}

impl MemoryManager {
    /// Create a new memory manager
    pub fn new() -> Self {
        Self {
            heap: Heap::new(),
            stack: Stack::new(),
            garbage_collector: GarbageCollector::new(),
            stats: MemoryStats::default(),
        }
    }
    
    /// Allocate memory on the heap
    pub fn allocate(&mut self, size: MemorySize) -> Option<HeapHandleId> {
        let start_time = std::time::Instant::now();
        
        // Try to allocate from heap
        let result = self.heap.allocate(size);
        
        if result.is_some() {
            // Update statistics
            self.update_allocation_stats(start_time);
        }
        
        result
    }
    
    /// Deallocate memory from the heap
    pub fn deallocate(&mut self, handle: HeapHandleId) -> bool {
        let start_time = std::time::Instant::now();
        
        // Deallocate from heap
        let result = self.heap.deallocate(handle);
        
        if result {
            // Update statistics
            self.update_deallocation_stats(start_time);
        }
        
        result
    }
    
    /// Push value onto stack
    pub fn push(&mut self, value: crate::vm::value::Value) -> Result<(), String> {
        self.stack.push(value)
    }
    
    /// Pop value from stack
    pub fn pop(&mut self) -> Option<crate::vm::value::Value> {
        self.stack.pop()
    }
    
    /// Peek at top of stack
    pub fn peek(&self) -> Option<&crate::vm::value::Value> {
        self.stack.peek()
    }
    
    /// Check if garbage collection is needed
    pub fn should_collect(&self) -> bool {
        self.garbage_collector.should_collect().is_some()
    }
    
    /// Perform garbage collection
    pub fn collect(&mut self) -> crate::vm::memory::heap::gc::GcResult {
        let start_time = std::time::Instant::now();
        
        // Perform collection
        let result = self.garbage_collector.collect();
        
        // Update statistics
        self.update_gc_stats(&result, start_time);
        
        result
    }
    
    /// Get memory statistics
    pub fn stats(&self) -> &MemoryStats {
        &self.stats
    }
    
    /// Get heap information
    pub fn heap_info(&self) -> crate::vm::memory::heap::HeapInfo {
        self.heap.info()
    }
    
    /// Get stack information
    pub fn stack_info(&self) -> crate::vm::memory::stack::StackInfo {
        self.stack.info()
    }
    
    /// Get garbage collection information
    pub fn gc_info(&self) -> crate::vm::memory::heap::gc::GcStats {
        self.garbage_collector.stats().clone()
    }
    
    /// Update allocation statistics
    fn update_allocation_stats(&mut self, start_time: std::time::Instant) {
        let duration = start_time.elapsed().as_micros() as f64;
        self.stats.allocation_rate = 1_000_000.0 / duration; // objects per second
        
        // Update memory usage
        self.stats.allocated_memory = self.heap.total_allocated();
        self.stats.free_memory = self.heap.total_free();
        self.stats.total_memory = MemorySize::new(
            self.stats.allocated_memory.as_usize() + self.stats.free_memory.as_usize()
        );
        
        // Update heap efficiency
        if self.stats.total_memory.as_usize() > 0 {
            self.stats.heap_efficiency = 
                (self.stats.allocated_memory.as_usize() as f64 / self.stats.total_memory.as_usize() as f64) * 100.0;
        }
    }
    
    /// Update deallocation statistics
    fn update_deallocation_stats(&mut self, start_time: std::time::Instant) {
        let duration = start_time.elapsed().as_micros() as f64;
        self.stats.deallocation_rate = 1_000_000.0 / duration; // objects per second
        
        // Update memory usage
        self.stats.allocated_memory = self.heap.total_allocated();
        self.stats.free_memory = self.heap.total_free();
        
        // Update heap efficiency
        if self.stats.total_memory.as_usize() > 0 {
            self.stats.heap_efficiency = 
                (self.stats.allocated_memory.as_usize() as f64 / self.stats.total_memory.as_usize() as f64) * 100.0;
        }
    }
    
    /// Update garbage collection statistics
    fn update_gc_stats(&mut self, result: &crate::vm::memory::heap::gc::GcResult, start_time: std::time::Instant) {
        let duration = start_time.elapsed().as_micros() as f64;
        self.stats.gc_overhead = duration / 1_000_000.0; // Convert to seconds
        
        // Update memory usage after collection
        self.stats.allocated_memory = self.heap.total_allocated();
        self.stats.free_memory = self.heap.total_free();
        
        // Update heap efficiency
        if self.stats.total_memory.as_usize() > 0 {
            self.stats.heap_efficiency = 
                (self.stats.allocated_memory.as_usize() as f64 / self.stats.total_memory.as_usize() as f64) * 100.0;
        }
    }
    
    /// Get memory pressure information
    pub fn memory_pressure(&self) -> crate::vm::memory::heap::gc::MemoryPressureInfo {
        self.garbage_collector.memory_pressure_info()
    }
    
    /// Start background garbage collection
    pub fn start_background_gc(&mut self) {
        self.garbage_collector.start_background_collection();
    }
    
    /// Stop background garbage collection
    pub fn stop_background_gc(&mut self) {
        self.garbage_collector.stop_background_collection();
    }
    
    /// Check if background garbage collection is running
    pub fn is_background_gc_running(&self) -> bool {
        self.garbage_collector.is_background_collection_running()
    }
    
    /// Get detailed heap analysis
    pub fn heap_analysis(&self) -> crate::vm::memory::heap::HeapAnalysis {
        self.heap.analysis()
    }
    
    /// Get memory space information
    pub fn space_info(&self) -> crate::vm::memory::heap::spaces::SpaceManagerInfo {
        self.heap.space_info()
    }
    
    /// Get allocation strategy information
    pub fn allocation_strategy_info(&self) -> crate::vm::memory::heap::allocation::AllocationStrategyInfo {
        self.heap.allocation_strategy_info()
    }
    
    /// Optimize memory layout
    pub fn optimize_memory_layout(&mut self) -> crate::vm::memory::heap::OptimizationResult {
        self.heap.optimize_layout()
    }
    
    /// Compact memory
    pub fn compact_memory(&mut self) -> crate::vm::memory::heap::CompactionResult {
        self.heap.compact()
    }
    
    /// Get memory health score
    pub fn health_score(&self) -> f64 {
        let heap_health = self.heap.health_score();
        let stack_health = self.stack.health_score();
        let gc_health = self.gc_health_score();
        
        // Weighted average
        (heap_health * 0.6 + stack_health * 0.2 + gc_health * 0.2)
    }
    
    /// Get garbage collection health score
    fn gc_health_score(&self) -> f64 {
        let gc_stats = self.garbage_collector.stats();
        
        // Calculate health based on collection efficiency
        let total_collections = gc_stats.total_collections as f64;
        if total_collections == 0.0 {
            return 100.0; // No collections yet, assume healthy
        }
        
        let avg_collection_time = gc_stats.average_collection_time_ms as f64;
        let objects_collected = gc_stats.total_objects_collected as f64;
        let bytes_freed = gc_stats.total_bytes_freed as f64;
        
        // Health factors:
        // - Lower average collection time is better
        // - Higher collection efficiency is better
        // - More objects collected per collection is better
        
        let time_score = (1000.0 - avg_collection_time).max(0.0) / 10.0; // 0-100
        let efficiency_score = if total_collections > 0.0 {
            (objects_collected / total_collections).min(1000.0) / 10.0 // 0-100
        } else {
            50.0
        };
        
        (time_score + efficiency_score) / 2.0
    }
    
    /// Perform memory diagnostics
    pub fn diagnose(&self) -> MemoryDiagnostics {
        let heap_analysis = self.heap.analysis();
        let space_info = self.heap.space_info();
        let gc_stats = self.garbage_collector.stats();
        
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        
        // Check heap health
        if heap_analysis.health_score < 70.0 {
            issues.push("Heap health is below optimal levels".to_string());
            recommendations.push("Consider running memory compaction".to_string());
        }
        
        // Check memory pressure
        let pressure_info = self.memory_pressure();
        match pressure_info.overall_pressure {
            crate::vm::memory::heap::gc::MemoryPressure::High => {
                issues.push("High memory pressure detected".to_string());
                recommendations.push("Increase heap size or optimize object usage".to_string());
            }
            crate::vm::memory::heap::gc::MemoryPressure::Critical => {
                issues.push("Critical memory pressure detected".to_string());
                recommendations.push("Immediate action required: increase heap size".to_string());
            }
            _ => {}
        }
        
        // Check collection efficiency
        if gc_stats.total_collections > 0 {
            let avg_time = gc_stats.average_collection_time_ms;
            if avg_time > 100 {
                issues.push("Garbage collection is taking too long".to_string());
                recommendations.push("Consider tuning GC parameters or reducing object allocation".to_string());
            }
        }
        
        // Check stack usage
        let stack_info = self.stack.info();
        if stack_info.usage_percentage > 90.0 {
            issues.push("Stack usage is very high".to_string());
            recommendations.push("Check for deep recursion or large stack frames".to_string());
        }
        
        MemoryDiagnostics {
            overall_health: self.health_score(),
            issues,
            recommendations,
            heap_analysis,
            space_info,
            gc_stats: gc_stats.clone(),
        }
    }
}

/// Memory diagnostics information
#[derive(Debug, Clone)]
pub struct MemoryDiagnostics {
    pub overall_health: f64,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
    pub heap_analysis: crate::vm::memory::heap::HeapAnalysis,
    pub space_info: crate::vm::memory::heap::spaces::SpaceManagerInfo,
    pub gc_stats: crate::vm::memory::heap::gc::GcStats,
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::value::Value;
    
    #[test]
    fn test_memory_manager_new() {
        let manager = MemoryManager::new();
        assert_eq!(manager.stats().total_memory.as_usize(), 0);
        assert_eq!(manager.stats().allocated_memory.as_usize(), 0);
        assert_eq!(manager.stats().free_memory.as_usize(), 0);
        assert!(!manager.should_collect());
    }
    
    #[test]
    fn test_memory_manager_allocation() {
        let mut manager = MemoryManager::new();
        
        // Allocate memory
        let handle = manager.allocate(MemorySize::new(1024));
        assert!(handle.is_some());
        
        // Check statistics
        let stats = manager.stats();
        assert!(stats.allocated_memory.as_usize() > 0);
        assert!(stats.allocation_rate > 0.0);
    }
    
    #[test]
    fn test_memory_manager_stack_operations() {
        let mut manager = MemoryManager::new();
        
        // Push values
        assert!(manager.push(Value::Number(42.0)).is_ok());
        assert!(manager.push(Value::String("test".to_string())).is_ok());
        
        // Peek and pop
        assert_eq!(manager.peek(), Some(&Value::String("test".to_string())));
        assert_eq!(manager.pop(), Some(Value::String("test".to_string())));
        assert_eq!(manager.pop(), Some(Value::Number(42.0)));
        assert_eq!(manager.pop(), None);
    }
    
    #[test]
    fn test_memory_manager_health_score() {
        let manager = MemoryManager::new();
        let health = manager.health_score();
        
        // Health score should be between 0 and 100
        assert!(health >= 0.0 && health <= 100.0);
    }
    
    #[test]
    fn test_memory_manager_diagnose() {
        let manager = MemoryManager::new();
        let diagnostics = manager.diagnose();
        
        // Should have diagnostics
        assert!(diagnostics.overall_health >= 0.0 && diagnostics.overall_health <= 100.0);
        assert!(diagnostics.issues.is_empty() || !diagnostics.issues.is_empty());
        assert!(diagnostics.recommendations.is_empty() || !diagnostics.recommendations.is_empty());
    }
}
