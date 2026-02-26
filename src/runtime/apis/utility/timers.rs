use chitin::boa_engine::Context;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::info;

/// Timer ID type
type TimerId = u64;

/// Timer information
#[derive(Debug, Clone)]
struct TimerInfo {
    id: TimerId,
    delay: Duration,
    repeat: bool,
    callback: String,
}

/// Timers API implementation for JetCrab
pub struct TimersAPI {
    timers: Arc<Mutex<HashMap<TimerId, TimerInfo>>>,
    next_id: Arc<Mutex<TimerId>>,
}

impl TimersAPI {
    /// Create a new Timers API
    pub fn new() -> Self {
        Self {
            timers: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Get next timer ID
    fn get_next_id(&self) -> TimerId {
        let mut next_id = self.next_id.lock().unwrap();
        let id = *next_id;
        *next_id += 1;
        id
    }

    /// Register the Timers API in the JavaScript context
    pub fn register(&self, context: &mut Context) -> Result<(), Box<dyn std::error::Error>> {
        info!("⏰ Registering Timers API...");

        let timers_code = r#"
        let _timerIdCounter = 1;
        let _timers = new Map();
        let _immediates = new Map();

        function getNextTimerId() {
            return _timerIdCounter++;
        }

        function setTimeout(callback, delay, ...args) {
            if (typeof callback !== 'function') {
                throw new TypeError('Callback must be a function');
            }
            
            const id = getNextTimerId();
            const timer = {
                id: id,
                callback: callback,
                delay: delay || 0,
                args: args,
                repeat: false,
                created: Date.now()
            };
            
            _timers.set(id, timer);
            
            console.log(`⏰ setTimeout scheduled: ID=${id}, delay=${delay}ms`);
            
            return id;
        }

        function setInterval(callback, delay, ...args) {
            if (typeof callback !== 'function') {
                throw new TypeError('Callback must be a function');
            }
            
            const id = getNextTimerId();
            const timer = {
                id: id,
                callback: callback,
                delay: delay || 0,
                args: args,
                repeat: true,
                created: Date.now()
            };
            
            _timers.set(id, timer);
            
            console.log(`⏰ setInterval scheduled: ID=${id}, interval=${delay}ms`);
            
            return id;
        }

        function clearTimeout(id) {
            if (_timers.has(id)) {
                _timers.delete(id);
                console.log(`⏰ clearTimeout: ID=${id}`);
            }
        }

        function clearInterval(id) {
            if (_timers.has(id)) {
                _timers.delete(id);
                console.log(`⏰ clearInterval: ID=${id}`);
            }
        }

        function setImmediate(callback, ...args) {
            if (typeof callback !== 'function') {
                throw new TypeError('Callback must be a function');
            }
            
            const id = getNextTimerId();
            const immediate = {
                id: id,
                callback: callback,
                args: args,
                created: Date.now()
            };
            
            _immediates.set(id, immediate);
            
            console.log(`⏰ setImmediate scheduled: ID=${id}`);
            
            return id;
        }

        function clearImmediate(id) {
            if (_immediates.has(id)) {
                _immediates.delete(id);
                console.log(`⏰ clearImmediate: ID=${id}`);
            }
        }

        globalThis.setTimeout = setTimeout;
        globalThis.setInterval = setInterval;
        globalThis.clearTimeout = clearTimeout;
        globalThis.clearInterval = clearInterval;
        globalThis.setImmediate = setImmediate;
        globalThis.clearImmediate = clearImmediate;

        globalThis.timers = {
            setTimeout: setTimeout,
            setInterval: setInterval,
            clearTimeout: clearTimeout,
            clearInterval: clearInterval,
            setImmediate: setImmediate,
            clearImmediate: clearImmediate,
            active: function() {
                return _timers.size;
            },
            unref: function() {
                console.log('⏰ Timer unref called');
            },
            ref: function() {
                console.log('⏰ Timer ref called');
            }
        };
        "#;

        context.eval(chitin::boa_engine::Source::from_bytes(timers_code))?;
        info!("✅ Timers API registered successfully");
        Ok(())
    }

    /// Clear a specific timer
    pub fn clear_timer(&self, id: TimerId) -> bool {
        let mut timers = self.timers.lock().unwrap();
        timers.remove(&id).is_some()
    }

    /// Clear all timers
    pub fn clear_all_timers(&self) {
        let mut timers = self.timers.lock().unwrap();
        timers.clear();
    }

    /// Get active timer count
    pub fn active_count(&self) -> usize {
        let timers = self.timers.lock().unwrap();
        timers.len()
    }
}

impl Default for TimersAPI {
    fn default() -> Self {
        Self::new()
    }
}
