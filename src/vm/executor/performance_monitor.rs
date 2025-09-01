//! # Performance Monitor
//!
//! Provides performance monitoring and metrics collection for VM execution.
//! Tracks various execution statistics including instruction counts, timing,
//! and memory operation metrics.
//!
//! ## Overview
//!
//! The performance monitor system consists of:
//!
//! - **PerformanceMetrics**: Stores collected performance data
//! - **PerformanceMonitor**: Trait defining monitoring interface
//! - **DefaultPerformanceMonitor**: Concrete implementation
//!
//! ## Tracked Metrics
//!
//! - **Total Instructions**: Number of executed instructions
//! - **Execution Time**: Duration of execution cycles
//! - **Memory Allocations**: Count of heap allocations
//! - **Stack Operations**: Number of stack manipulations
//! - **Heap Operations**: Number of heap operations
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::vm::executor::performance_monitor::{DefaultPerformanceMonitor, PerformanceMonitor};
//!
//! let mut monitor = DefaultPerformanceMonitor::new();
//! monitor.start_execution();
//! monitor.record_instruction();
//! monitor.record_stack_operation();
//! monitor.end_execution();
//!
//! let metrics = monitor.get_metrics();
//! println!("Executed {} instructions", metrics.total_instructions);
//! ```

use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub total_instructions: usize,
    pub execution_time: Duration,
    pub memory_allocations: usize,
    pub stack_operations: usize,
    pub heap_operations: usize,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            total_instructions: 0,
            execution_time: Duration::ZERO,
            memory_allocations: 0,
            stack_operations: 0,
            heap_operations: 0,
        }
    }
}

pub trait PerformanceMonitor {
    fn start_execution(&mut self);
    fn end_execution(&mut self);
    fn record_instruction(&mut self);
    fn record_memory_allocation(&mut self);
    fn record_stack_operation(&mut self);
    fn record_heap_operation(&mut self);
    fn get_metrics(&self) -> &PerformanceMetrics;
    fn reset(&mut self);
}

pub struct DefaultPerformanceMonitor {
    metrics: PerformanceMetrics,
    start_time: Option<Instant>,
}

impl DefaultPerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: PerformanceMetrics::default(),
            start_time: None,
        }
    }
}

impl PerformanceMonitor for DefaultPerformanceMonitor {
    fn start_execution(&mut self) {
        self.start_time = Some(Instant::now());
    }

    fn end_execution(&mut self) {
        if let Some(start_time) = self.start_time {
            self.metrics.execution_time = start_time.elapsed();
        }
    }

    fn record_instruction(&mut self) {
        self.metrics.total_instructions += 1;
    }

    fn record_memory_allocation(&mut self) {
        self.metrics.memory_allocations += 1;
    }

    fn record_stack_operation(&mut self) {
        self.metrics.stack_operations += 1;
    }

    fn record_heap_operation(&mut self) {
        self.metrics.heap_operations += 1;
    }

    fn get_metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }

    fn reset(&mut self) {
        self.metrics = PerformanceMetrics::default();
        self.start_time = None;
    }
}
