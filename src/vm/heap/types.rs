use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HeapStats {
    pub total_entries: usize,
    pub object_count: usize,
    pub array_count: usize,
    pub function_count: usize,
    pub string_count: usize,
    pub memory_usage: usize,
    pub fragmentation: f64,
}

impl HeapStats {
    pub fn new() -> Self {
        Self {
            total_entries: 0,
            object_count: 0,
            array_count: 0,
            function_count: 0,
            string_count: 0,
            memory_usage: 0,
            fragmentation: 0.0,
        }
    }

    pub fn update_counts(&mut self, object_count: usize, array_count: usize, function_count: usize, string_count: usize) {
        self.object_count = object_count;
        self.array_count = array_count;
        self.function_count = function_count;
        self.string_count = string_count;
        self.total_entries = object_count + array_count + function_count + string_count;
    }

    pub fn set_memory_usage(&mut self, usage: usize) {
        self.memory_usage = usage;
    }

    pub fn set_fragmentation(&mut self, fragmentation: f64) {
        self.fragmentation = fragmentation;
    }

    pub fn get_efficiency(&self) -> f64 {
        if self.total_entries == 0 {
            1.0
        } else {
            (self.memory_usage as f64) / (self.total_entries as f64)
        }
    }
}

impl Default for HeapStats {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for HeapStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HeapStats {{ total: {}, objects: {}, arrays: {}, functions: {}, strings: {}, memory: {} bytes, fragmentation: {:.2}% }}",
            self.total_entries,
            self.object_count,
            self.array_count,
            self.function_count,
            self.string_count,
            self.memory_usage,
            self.fragmentation * 100.0
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HeapMetrics {
    pub allocation_count: usize,
    pub deallocation_count: usize,
    pub gc_cycles: usize,
    pub last_gc_duration: std::time::Duration,
}

impl HeapMetrics {
    pub fn new() -> Self {
        Self {
            allocation_count: 0,
            deallocation_count: 0,
            gc_cycles: 0,
            last_gc_duration: std::time::Duration::ZERO,
        }
    }

    pub fn record_allocation(&mut self) {
        self.allocation_count += 1;
    }

    pub fn record_deallocation(&mut self) {
        self.deallocation_count += 1;
    }

    pub fn record_gc_cycle(&mut self, duration: std::time::Duration) {
        self.gc_cycles += 1;
        self.last_gc_duration = duration;
    }

    pub fn get_allocation_rate(&self) -> f64 {
        if self.gc_cycles == 0 {
            0.0
        } else {
            self.allocation_count as f64 / self.gc_cycles as f64
        }
    }
}

impl Default for HeapMetrics {
    fn default() -> Self {
        Self::new()
    }
}
