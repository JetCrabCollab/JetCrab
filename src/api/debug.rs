use crate::api::error::ApiError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakpoint {
    pub id: String,
    pub line: usize,
    pub column: usize,
    pub condition: Option<String>,
    pub enabled: bool,
    pub hit_count: usize,
}

impl Breakpoint {
    pub fn new(id: String, line: usize, column: usize) -> Self {
        Self {
            id,
            line,
            column,
            condition: None,
            enabled: true,
            hit_count: 0,
        }
    }

    pub fn with_condition(mut self, condition: String) -> Self {
        self.condition = Some(condition);
        self
    }

    pub fn hit(&mut self) {
        self.hit_count += 1;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallFrame {
    pub function_name: String,
    pub line: usize,
    pub column: usize,
    pub variables: HashMap<String, crate::vm::value::Value>,
    pub this_value: Option<crate::vm::value::Value>,
    pub arguments: Vec<crate::vm::value::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugInfo {
    pub current_line: usize,
    pub current_column: usize,
    pub call_stack: Vec<CallFrame>,
    pub breakpoints: Vec<Breakpoint>,
    pub variables: HashMap<String, crate::vm::value::Value>,
    pub is_paused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingMetrics {
    pub execution_time: Duration,
    pub memory_usage: usize,
    pub instruction_count: usize,
    pub function_calls: usize,
    pub gc_cycles: usize,
}

impl ProfilingMetrics {
    pub fn generate_report(&self) -> String {
        format!(
            "Profiling Report:\n\
             Execution Time: {:?}\n\
             Memory Usage: {} bytes\n\
             Instructions Executed: {}\n\
             Function Calls: {}\n\
             GC Cycles: {}\n\
             Functions Profiled: {}",
            self.execution_time,
            self.memory_usage,
            self.instruction_count,
            self.function_calls,
            self.gc_cycles,
            0 // Placeholder for function timings count
        )
    }
}

pub struct Debugger {
    breakpoints: HashMap<String, Breakpoint>,
    is_enabled: bool,
    step_mode: bool,
    current_info: Option<DebugInfo>,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            breakpoints: HashMap::new(),
            is_enabled: false,
            step_mode: false,
            current_info: None,
        }
    }

    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    pub fn add_breakpoint(&mut self, breakpoint: Breakpoint) {
        self.breakpoints.insert(breakpoint.id.clone(), breakpoint);
    }

    pub fn remove_breakpoint(&mut self, id: &str) -> Option<Breakpoint> {
        self.breakpoints.remove(id)
    }

    pub fn enable_breakpoint(&mut self, id: &str) -> Result<(), ApiError> {
        if let Some(breakpoint) = self.breakpoints.get_mut(id) {
            breakpoint.enabled = true;
            Ok(())
        } else {
            Err(ApiError::InvalidInput {
                message: "Breakpoint not found".to_string(),
                input: id.to_string(),
                position: None,
            })
        }
    }

    pub fn disable_breakpoint(&mut self, id: &str) -> Result<(), ApiError> {
        if let Some(breakpoint) = self.breakpoints.get_mut(id) {
            breakpoint.enabled = false;
            Ok(())
        } else {
            Err(ApiError::InvalidInput {
                message: "Breakpoint not found".to_string(),
                input: id.to_string(),
                position: None,
            })
        }
    }

    pub fn should_pause(&self, line: usize, column: usize) -> bool {
        if !self.is_enabled {
            return false;
        }

        for breakpoint in self.breakpoints.values() {
            if breakpoint.enabled && breakpoint.line == line && breakpoint.column == column {
                return true;
            }
        }

        self.step_mode
    }

    pub fn update_debug_info(&mut self, info: DebugInfo) {
        self.current_info = Some(info);
    }

    pub fn get_debug_info(&self) -> Option<&DebugInfo> {
        self.current_info.as_ref()
    }

    pub fn step_into(&mut self) {
        self.step_mode = true;
    }

    pub fn step_over(&mut self) {
        self.step_mode = false;
    }

    pub fn continue_execution(&mut self) {
        self.step_mode = false;
    }
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Profiler {
    start_time: Option<Instant>,
    metrics: ProfilingMetrics,
    function_timings: HashMap<String, Duration>,
    memory_snapshots: Vec<(Instant, usize)>,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            start_time: None,
            metrics: ProfilingMetrics {
                execution_time: Duration::ZERO,
                memory_usage: 0,
                instruction_count: 0,
                function_calls: 0,
                gc_cycles: 0,
            },
            function_timings: HashMap::new(),
            memory_snapshots: Vec::new(),
        }
    }

    pub fn start_profiling(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn stop_profiling(&mut self) -> ProfilingMetrics {
        if let Some(start_time) = self.start_time {
            self.metrics.execution_time = start_time.elapsed();
        }
        self.metrics.clone()
    }

    pub fn record_function_call(&mut self, function_name: String, duration: Duration) {
        *self
            .function_timings
            .entry(function_name)
            .or_insert(Duration::ZERO) += duration;
        self.metrics.function_calls += 1;
    }

    pub fn record_instruction(&mut self) {
        self.metrics.instruction_count += 1;
    }

    pub fn record_memory_usage(&mut self, usage: usize) {
        self.metrics.memory_usage = usage;
        self.memory_snapshots.push((Instant::now(), usage));
    }

    pub fn record_gc_cycle(&mut self) {
        self.metrics.gc_cycles += 1;
    }

    pub fn get_function_timings(&self) -> &HashMap<String, Duration> {
        &self.function_timings
    }

    pub fn get_memory_snapshots(&self) -> &Vec<(Instant, usize)> {
        &self.memory_snapshots
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Inspector {
    debugger: Debugger,
    profiler: Profiler,
    event_listeners: HashMap<String, Vec<Box<dyn Fn(String) + Send + Sync>>>,
}

impl Inspector {
    pub fn new() -> Self {
        Self {
            debugger: Debugger::new(),
            profiler: Profiler::new(),
            event_listeners: HashMap::new(),
        }
    }

    pub fn get_debugger(&mut self) -> &mut Debugger {
        &mut self.debugger
    }

    pub fn get_profiler(&mut self) -> &mut Profiler {
        &mut self.profiler
    }

    pub fn add_event_listener(
        &mut self,
        event: String,
        listener: Box<dyn Fn(String) + Send + Sync>,
    ) {
        self.event_listeners
            .entry(event)
            .or_default()
            .push(listener);
    }

    pub fn emit_event(&self, event: &str, data: String) {
        if let Some(listeners) = self.event_listeners.get(event) {
            for listener in listeners {
                listener(data.clone());
            }
        }
    }

    pub fn start_inspection(&mut self) {
        self.debugger.enable();
        self.profiler.start_profiling();
        self.emit_event(
            "inspection_started",
            "Debugging and profiling started".to_string(),
        );
    }

    pub fn stop_inspection(&mut self) -> String {
        self.debugger.disable();
        let report = self.profiler.stop_profiling();
        self.emit_event(
            "inspection_stopped",
            "Debugging and profiling stopped".to_string(),
        );
        report.generate_report()
    }
}

impl Default for Inspector {
    fn default() -> Self {
        Self::new()
    }
}
