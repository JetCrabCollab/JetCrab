use boa_engine::{Context, JsValue};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::info;

#[derive(Debug, Clone)]
pub struct EventListener {
    pub callback: JsValue,
    pub once: bool,
    pub prepend: bool,
}

#[derive(Debug, Clone)]
pub struct EventEmitter {
    pub listeners: HashMap<String, Vec<EventListener>>,
    pub max_listeners: usize,
    pub default_max_listeners: usize,
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
            max_listeners: 10,
            default_max_listeners: 10,
        }
    }

    pub fn on(&mut self, event: String, listener: JsValue, prepend: bool) -> Result<(), String> {
        if self.listeners.len() >= self.max_listeners {
            return Err(format!("Max listeners ({}) exceeded", self.max_listeners));
        }

        let event_listener = EventListener {
            callback: listener,
            once: false,
            prepend,
        };

        self.listeners
            .entry(event)
            .or_insert_with(Vec::new)
            .push(event_listener);

        Ok(())
    }

    pub fn once(&mut self, event: String, listener: JsValue, prepend: bool) -> Result<(), String> {
        if self.listeners.len() >= self.max_listeners {
            return Err(format!("Max listeners ({}) exceeded", self.max_listeners));
        }

        let event_listener = EventListener {
            callback: listener,
            once: true,
            prepend,
        };

        self.listeners
            .entry(event)
            .or_insert_with(Vec::new)
            .push(event_listener);

        Ok(())
    }

    pub fn emit(&mut self, event: String, args: Vec<JsValue>) -> Result<bool, String> {
        if let Some(listeners) = self.listeners.get_mut(&event) {
            if listeners.is_empty() {
                return Ok(false);
            }

            let mut to_remove = Vec::new();
            let mut listeners_copy = listeners.clone();

            for (index, listener) in listeners_copy.iter().enumerate() {
                if listener.once {
                    to_remove.push(index);
                }
            }

            for &index in to_remove.iter().rev() {
                listeners.remove(index);
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn remove_listener(&mut self, event: String, listener: JsValue) -> Result<(), String> {
        if let Some(listeners) = self.listeners.get_mut(&event) {
            listeners.retain(|l| l.callback != listener);
        }
        Ok(())
    }

    pub fn remove_all_listeners(&mut self, event: Option<String>) -> Result<(), String> {
        match event {
            Some(event_name) => {
                self.listeners.remove(&event_name);
            }
            None => {
                self.listeners.clear();
            }
        }
        Ok(())
    }

    pub fn listener_count(&self, event: &str) -> usize {
        self.listeners
            .get(event)
            .map_or(0, |listeners| listeners.len())
    }

    pub fn event_names(&self) -> Vec<String> {
        self.listeners.keys().cloned().collect()
    }

    pub fn set_max_listeners(&mut self, max: usize) {
        self.max_listeners = max;
    }

    pub fn get_max_listeners(&self) -> usize {
        self.max_listeners
    }
}

pub struct EventsAPI;

impl EventsAPI {
    pub fn new() -> Self {
        Self
    }

    pub fn register(&self, context: &mut Context) -> Result<(), Box<dyn std::error::Error>> {
        info!("📡 Registering Events API...");

        let events_code = r#"
        class EventEmitter {
            constructor() {
                this._events = {};
                this._maxListeners = 10;
                this._defaultMaxListeners = 10;
            }
            
            addListener(event, listener) {
                return this.on(event, listener);
            }
            
            on(event, listener) {
                if (!this._events[event]) {
                    this._events[event] = [];
                }
                
                if (this._events[event].length >= this._maxListeners) {
                    console.warn(`MaxListenersExceededWarning: Possible EventEmitter memory leak detected. ${this._events[event].length} listeners added. Use emitter.setMaxListeners() to increase limit`);
                }
                
                this._events[event].push(listener);
                return this;
            }
            
            once(event, listener) {
                const onceWrapper = (...args) => {
                    this.removeListener(event, onceWrapper);
                    listener.apply(this, args);
                };
                onceWrapper.listener = listener;
                this.on(event, onceWrapper);
                return this;
            }
            
            removeListener(event, listener) {
                if (!this._events[event]) return this;
                
                const listeners = this._events[event];
                const index = listeners.indexOf(listener);
                if (index !== -1) {
                    listeners.splice(index, 1);
                }
                
                return this;
            }
            
            off(event, listener) {
                return this.removeListener(event, listener);
            }
            
            removeAllListeners(event) {
                if (event) {
                    delete this._events[event];
                } else {
                    this._events = {};
                }
                return this;
            }
            
            setMaxListeners(n) {
                this._maxListeners = n;
                return this;
            }
            
            getMaxListeners() {
                return this._maxListeners;
            }
            
            listeners(event) {
                return this._events[event] ? [...this._events[event]] : [];
            }
            
            rawListeners(event) {
                return this._events[event] ? [...this._events[event]] : [];
            }
            
            listenerCount(event) {
                return this._events[event] ? this._events[event].length : 0;
            }
            
            eventNames() {
                return Object.keys(this._events);
            }
            
            emit(event, ...args) {
                if (!this._events[event]) return false;
                
                const listeners = this._events[event];
                for (const listener of listeners) {
                    try {
                        listener.apply(this, args);
                    } catch (error) {
                        console.error('Error in event listener:', error);
                    }
                }
                return true;
            }
            
            prependListener(event, listener) {
                if (!this._events[event]) {
                    this._events[event] = [];
                }
                this._events[event].unshift(listener);
                return this;
            }
            
            prependOnceListener(event, listener) {
                const onceWrapper = (...args) => {
                    this.removeListener(event, onceWrapper);
                    listener.apply(this, args);
                };
                onceWrapper.listener = listener;
                this.prependListener(event, onceWrapper);
                return this;
            }
        }
        
        globalThis.EventEmitter = EventEmitter;
        
        if (typeof process !== 'undefined') {
            process.EventEmitter = EventEmitter;
        }
        
        globalThis.events = {
            EventEmitter: EventEmitter,
            once: function(emitter, event) {
                return new Promise((resolve, reject) => {
                    emitter.once(event, resolve);
                    emitter.on('error', reject);
                });
            },
            on: function(emitter, event) {
                return new Promise((resolve, reject) => {
                    emitter.on(event, resolve);
                    emitter.on('error', reject);
                });
            }
        };
        "#;

        context.eval(boa_engine::Source::from_bytes(events_code))?;
        info!("✅ Events API registered successfully");
        Ok(())
    }
}
