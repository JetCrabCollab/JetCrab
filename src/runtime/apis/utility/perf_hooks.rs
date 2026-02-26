use chitin::boa_engine::{Context, JsValue};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tracing::info;

#[derive(Debug, Clone)]
pub struct PerformanceMark {
    pub name: String,
    pub start_time: f64,
    pub detail: Option<JsValue>,
}

#[derive(Debug, Clone)]
pub struct PerformanceMeasure {
    pub name: String,
    pub start_time: f64,
    pub duration: f64,
    pub detail: Option<JsValue>,
}

#[derive(Debug, Clone)]
pub struct PerformanceEntry {
    pub name: String,
    pub entry_type: String,
    pub start_time: f64,
    pub duration: f64,
    pub detail: Option<JsValue>,
}

pub struct PerformanceObserver {
    pub callback: Box<dyn Fn(Vec<PerformanceEntry>) + Send + Sync>,
    pub entry_types: Vec<String>,
    pub buffered: bool,
}

pub struct PerformanceManager {
    marks: Arc<Mutex<HashMap<String, PerformanceMark>>>,
    measures: Arc<Mutex<HashMap<String, PerformanceMeasure>>>,
    entries: Arc<Mutex<Vec<PerformanceEntry>>>,
    observers: Arc<Mutex<Vec<PerformanceObserver>>>,
    start_time: Instant,
    time_origin: f64,
}

impl PerformanceManager {
    pub fn new() -> Self {
        let start_time = Instant::now();
        let time_origin = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as f64;

        Self {
            marks: Arc::new(Mutex::new(HashMap::new())),
            measures: Arc::new(Mutex::new(HashMap::new())),
            entries: Arc::new(Mutex::new(Vec::new())),
            observers: Arc::new(Mutex::new(Vec::new())),
            start_time,
            time_origin,
        }
    }

    pub fn now(&self) -> f64 {
        self.start_time.elapsed().as_nanos() as f64 / 1_000_000.0
    }

    pub fn mark(&self, name: String, detail: Option<JsValue>) -> Result<(), String> {
        let mark = PerformanceMark {
            name: name.clone(),
            start_time: self.now(),
            detail,
        };

        let mut marks = self.marks.lock().map_err(|e| e.to_string())?;
        marks.insert(name.clone(), mark.clone());

        let entry = PerformanceEntry {
            name,
            entry_type: "mark".to_string(),
            start_time: mark.start_time,
            duration: 0.0,
            detail: mark.detail,
        };

        let mut entries = self.entries.lock().map_err(|e| e.to_string())?;
        entries.push(entry);

        self.notify_observers("mark");
        Ok(())
    }

    pub fn measure(
        &self,
        name: String,
        start_mark: Option<String>,
        end_mark: Option<String>,
        detail: Option<JsValue>,
    ) -> Result<f64, String> {
        let start_time = if let Some(start) = start_mark {
            let marks = self.marks.lock().map_err(|e| e.to_string())?;
            marks
                .get(&start)
                .map(|m| m.start_time)
                .ok_or_else(|| format!("Mark '{}' not found", start))?
        } else {
            self.now()
        };

        let end_time = if let Some(end) = end_mark {
            let marks = self.marks.lock().map_err(|e| e.to_string())?;
            marks
                .get(&end)
                .map(|m| m.start_time)
                .ok_or_else(|| format!("Mark '{}' not found", end))?
        } else {
            self.now()
        };

        let duration = end_time - start_time;

        let measure = PerformanceMeasure {
            name: name.clone(),
            start_time,
            duration,
            detail,
        };

        let mut measures = self.measures.lock().map_err(|e| e.to_string())?;
        measures.insert(name.clone(), measure.clone());

        let entry = PerformanceEntry {
            name,
            entry_type: "measure".to_string(),
            start_time,
            duration,
            detail: measure.detail,
        };

        let mut entries = self.entries.lock().map_err(|e| e.to_string())?;
        entries.push(entry);

        self.notify_observers("measure");
        Ok(duration)
    }

    pub fn get_entries(&self, entry_type: Option<String>) -> Vec<PerformanceEntry> {
        let entries = self.entries.lock().unwrap();
        if let Some(entry_type) = entry_type {
            entries
                .iter()
                .filter(|e| e.entry_type == entry_type)
                .cloned()
                .collect()
        } else {
            entries.clone()
        }
    }

    pub fn clear_marks(&self, name: Option<String>) {
        let mut marks = self.marks.lock().unwrap();
        if let Some(name) = name {
            marks.remove(&name);
        } else {
            marks.clear();
        }
    }

    pub fn clear_measures(&self, name: Option<String>) {
        let mut measures = self.measures.lock().unwrap();
        if let Some(name) = name {
            measures.remove(&name);
        } else {
            measures.clear();
        }
    }

    pub fn clear_entries(&self, entry_type: Option<String>) {
        let mut entries = self.entries.lock().unwrap();
        if let Some(entry_type) = entry_type {
            entries.retain(|e| e.entry_type != entry_type);
        } else {
            entries.clear();
        }
    }

    fn notify_observers(&self, entry_type: &str) {
        let observers = self.observers.lock().unwrap();
        let entries = self.entries.lock().unwrap();

        for observer in observers.iter() {
            if observer.entry_types.contains(&entry_type.to_string()) {
                let relevant_entries: Vec<PerformanceEntry> = entries
                    .iter()
                    .filter(|e| observer.entry_types.contains(&e.entry_type))
                    .cloned()
                    .collect();

                if !relevant_entries.is_empty() {
                    (observer.callback)(relevant_entries);
                }
            }
        }
    }
}

pub struct PerfHooksAPI;

impl PerfHooksAPI {
    pub fn new() -> Self {
        Self
    }

    pub fn register(&self, context: &mut Context) -> Result<(), Box<dyn std::error::Error>> {
        info!("⚡ Registering Performance Hooks API...");

        let perf_hooks_code = r#"
        const performance = {
            now: function() {
                return Date.now();
            },
            
            mark: function(name, detail) {
                if (typeof name !== 'string') {
                    throw new TypeError('Mark name must be a string');
                }
                
                const mark = {
                    name: name,
                    startTime: this.now(),
                    detail: detail || null,
                    entryType: 'mark',
                    duration: 0
                };
                
                if (!this._marks) this._marks = new Map();
                this._marks.set(name, mark);
                
                if (!this._entries) this._entries = [];
                this._entries.push(mark);
                
                console.log(`⚡ Performance mark: ${name}`);
                return mark;
            },
            
            measure: function(name, startMark, endMark, detail) {
                if (typeof name !== 'string') {
                    throw new TypeError('Measure name must be a string');
                }
                
                const startTime = startMark ? this._marks.get(startMark)?.startTime : this.now();
                const endTime = endMark ? this._marks.get(endMark)?.startTime : this.now();
                
                if (startTime === undefined || endTime === undefined) {
                    throw new Error('Invalid mark name');
                }
                
                const duration = endTime - startTime;
                
                const measure = {
                    name: name,
                    startTime: startTime,
                    duration: duration,
                    detail: detail || null,
                    entryType: 'measure'
                };
                
                if (!this._measures) this._measures = new Map();
                this._measures.set(name, measure);
                
                if (!this._entries) this._entries = [];
                this._entries.push(measure);
                
                console.log(`⚡ Performance measure: ${name} = ${duration}ms`);
                return measure;
            },
            
            getEntries: function(options) {
                if (!this._entries) return [];
                
                if (options && options.entryType) {
                    return this._entries.filter(entry => entry.entryType === options.entryType);
                }
                
                return [...this._entries];
            },
            
            getEntriesByName: function(name, entryType) {
                if (!this._entries) return [];
                
                return this._entries.filter(entry => {
                    if (entry.name !== name) return false;
                    if (entryType && entry.entryType !== entryType) return false;
                    return true;
                });
            },
            
            getEntriesByType: function(entryType) {
                if (!this._entries) return [];
                
                return this._entries.filter(entry => entry.entryType === entryType);
            },
            
            clearMarks: function(name) {
                if (name) {
                    this._marks.delete(name);
                    this._entries = this._entries.filter(entry => 
                        !(entry.entryType === 'mark' && entry.name === name)
                    );
                } else {
                    this._marks.clear();
                    this._entries = this._entries.filter(entry => entry.entryType !== 'mark');
                }
            },
            
            clearMeasures: function(name) {
                if (name) {
                    this._measures.delete(name);
                    this._entries = this._entries.filter(entry => 
                        !(entry.entryType === 'measure' && entry.name === name)
                    );
                } else {
                    this._measures.clear();
                    this._entries = this._entries.filter(entry => entry.entryType !== 'measure');
                }
            },
            
            clearResourceTimings: function() {
                this._entries = this._entries.filter(entry => entry.entryType !== 'resource');
            },
            
            timeOrigin: Date.now()
        };

        class PerformanceObserver {
            constructor(callback) {
                this.callback = callback;
                this.entryTypes = [];
                this.buffered = false;
            }
            
            observe(options) {
                this.entryTypes = options.entryTypes || [];
                this.buffered = options.buffered || false;
                
                if (this.buffered) {
                    const entries = performance.getEntries().filter(entry => 
                        this.entryTypes.includes(entry.entryType)
                    );
                    if (entries.length > 0) {
                        this.callback({ getEntries: () => entries });
                    }
                }
            }
            
            disconnect() {
                this.entryTypes = [];
                this.buffered = false;
            }
            
            takeRecords() {
                return performance.getEntries().filter(entry => 
                    this.entryTypes.includes(entry.entryType)
                );
            }
        }

        globalThis.performance = performance;
        globalThis.PerformanceObserver = PerformanceObserver;

        globalThis.perf_hooks = {
            performance: performance,
            PerformanceObserver: PerformanceObserver,
            constants: {
                NODE_PERFORMANCE_GC_MAJOR: 1,
                NODE_PERFORMANCE_GC_MINOR: 2,
                NODE_PERFORMANCE_GC_INCREMENTAL: 4,
                NODE_PERFORMANCE_GC_WEAKCB: 8,
                NODE_PERFORMANCE_GC_FLAGS_NO: 0,
                NODE_PERFORMANCE_GC_FLAGS_CONSTRUCT_RETAINED: 1,
                NODE_PERFORMANCE_GC_FLAGS_FORCED: 2,
                NODE_PERFORMANCE_GC_FLAGS_SYNCHRONOUS_PHANTOM_PROCESSING: 4,
                NODE_PERFORMANCE_GC_FLAGS_ALL_AVAILABLE_GARBAGE: 8,
                NODE_PERFORMANCE_GC_FLAGS_ALL_EXTERNAL_MEMORY: 16,
                NODE_PERFORMANCE_GC_FLAGS_SCHEDULE_IDLE: 32,
                NODE_PERFORMANCE_GC_FLAGS_SCHEDULE_NEXT: 64,
                NODE_PERFORMANCE_GC_FLAGS_FORCE_ANNOTATE_UNREACHABLE: 128,
                NODE_PERFORMANCE_GC_FLAGS_SYNCHRONOUS_PHANTOM_PROCESSING: 256
            }
        };
        "#;

        context.eval(chitin::boa_engine::Source::from_bytes(perf_hooks_code))?;
        info!("✅ Performance Hooks API registered successfully");
        Ok(())
    }
}
