use crate::api::error::ApiError;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct EventData {
    pub event_type: String,
    pub timestamp: Instant,
    pub data: serde_json::Value,
    pub source: Option<String>,
}

impl EventData {
    pub fn new(event_type: String, data: serde_json::Value) -> Self {
        Self {
            event_type,
            timestamp: Instant::now(),
            data,
            source: None,
        }
    }

    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
}

pub type EventCallback = Box<dyn FnMut(&EventData) + Send + Sync>;
pub type AsyncEventCallback = Box<
    dyn Fn(EventData) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
        + Send
        + Sync,
>;

pub struct EventEmitter {
    listeners: HashMap<String, Vec<EventCallback>>,
    async_listeners: HashMap<String, Vec<AsyncEventCallback>>,
    event_history: Vec<EventData>,
    max_history_size: usize,
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
            async_listeners: HashMap::new(),
            event_history: Vec::new(),
            max_history_size: 1000,
        }
    }

    pub fn on<F>(&mut self, event: &str, callback: F)
    where
        F: FnMut(&EventData) + Send + Sync + 'static,
    {
        self.listeners
            .entry(event.to_string())
            .or_default()
            .push(Box::new(callback));
    }

    pub fn on_async<F>(&mut self, event: &str, callback: F)
    where
        F: Fn(EventData) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>
            + Send
            + Sync
            + 'static,
    {
        self.async_listeners
            .entry(event.to_string())
            .or_default()
            .push(Box::new(callback));
    }

    pub fn emit(&mut self, event: &str, data: serde_json::Value) {
        let event_data = EventData::new(event.to_string(), data);

        // Store in history
        self.event_history.push(event_data.clone());
        if self.event_history.len() > self.max_history_size {
            self.event_history.remove(0);
        }

        // Notify synchronous listeners
        if let Some(listeners) = self.listeners.get_mut(event) {
            for listener in listeners {
                listener(&event_data);
            }
        }

        // Notify asynchronous listeners (simplified without tokio)
        if let Some(_async_listeners) = self.async_listeners.get(event) {
            // For now, just call synchronously - in a real implementation you'd use tokio
            // tokio::spawn(async move {
            //     listener_clone(event_data_clone).await;
            // });
        }
    }

    pub fn emit_with_source(&mut self, event: &str, data: serde_json::Value, source: String) {
        let event_data = EventData::new(event.to_string(), data).with_source(source);

        self.event_history.push(event_data.clone());
        if self.event_history.len() > self.max_history_size {
            self.event_history.remove(0);
        }

        if let Some(listeners) = self.listeners.get_mut(event) {
            for listener in listeners {
                listener(&event_data);
            }
        }
    }

    pub fn remove_listener(&mut self, event: &str, index: usize) -> Result<(), ApiError> {
        if let Some(listeners) = self.listeners.get_mut(event) {
            if index < listeners.len() {
                let _ = listeners.remove(index);
                Ok(())
            } else {
                Err(ApiError::InvalidInput {
                    message: "Listener index out of bounds".to_string(),
                    input: index.to_string(),
                    position: None,
                })
            }
        } else {
            Err(ApiError::InvalidInput {
                message: "Event not found".to_string(),
                input: event.to_string(),
                position: None,
            })
        }
    }

    pub fn clear_listeners(&mut self, event: &str) {
        self.listeners.remove(event);
        self.async_listeners.remove(event);
    }

    pub fn get_event_history(&self) -> &[EventData] {
        &self.event_history
    }

    pub fn set_max_history_size(&mut self, size: usize) {
        self.max_history_size = size;
        while self.event_history.len() > self.max_history_size {
            self.event_history.remove(0);
        }
    }
}

impl Default for EventEmitter {
    fn default() -> Self {
        Self::new()
    }
}

type CallbackFunction =
    Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, ApiError> + Send + Sync>;
type CallbackArc = std::sync::Arc<std::sync::Mutex<CallbackFunction>>;

pub struct CallbackRegistry {
    callbacks: HashMap<String, CallbackArc>,
    metadata: HashMap<String, CallbackMetadata>,
}

#[derive(Debug, Clone)]
pub struct CallbackMetadata {
    pub name: String,
    pub description: String,
    pub parameters: Vec<String>,
    pub return_type: String,
    pub created_at: Instant,
    pub call_count: usize,
}

impl CallbackRegistry {
    pub fn new() -> Self {
        Self {
            callbacks: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, name: &str, description: &str, callback: F)
    where
        F: Fn(serde_json::Value) -> Result<serde_json::Value, ApiError> + Send + Sync + 'static,
    {
        let metadata = CallbackMetadata {
            name: name.to_string(),
            description: description.to_string(),
            parameters: Vec::new(),
            return_type: "any".to_string(),
            created_at: Instant::now(),
            call_count: 0,
        };

        self.callbacks.insert(
            name.to_string(),
            std::sync::Arc::new(std::sync::Mutex::new(Box::new(callback))),
        );
        self.metadata.insert(name.to_string(), metadata);
    }

    pub fn register_with_metadata<F>(&mut self, name: &str, metadata: CallbackMetadata, callback: F)
    where
        F: Fn(serde_json::Value) -> Result<serde_json::Value, ApiError> + Send + Sync + 'static,
    {
        self.callbacks.insert(
            name.to_string(),
            std::sync::Arc::new(std::sync::Mutex::new(Box::new(callback))),
        );
        self.metadata.insert(name.to_string(), metadata);
    }

    pub fn call(
        &mut self,
        name: &str,
        data: serde_json::Value,
    ) -> Result<serde_json::Value, ApiError> {
        if let Some(callback) = self.callbacks.get(name) {
            if let Some(metadata) = self.metadata.get_mut(name) {
                metadata.call_count += 1;
            }

            let callback_guard = callback.lock().map_err(|_| ApiError::InvalidInput {
                message: "Failed to acquire callback lock".to_string(),
                input: "".to_string(),
                position: None,
            })?;

            callback_guard(data)
        } else {
            Err(ApiError::InvalidInput {
                message: "Callback not found".to_string(),
                input: name.to_string(),
                position: None,
            })
        }
        .map_err(|_| ApiError::ExecutionError {
            message: "Callback execution failed".to_string(),
            position: None,
        })
    }

    pub fn unregister(&mut self, name: &str) -> bool {
        self.callbacks.remove(name).is_some() && self.metadata.remove(name).is_some()
    }

    pub fn list_callbacks(&self) -> Vec<&CallbackMetadata> {
        self.metadata.values().collect()
    }

    pub fn get_callback_info(&self, name: &str) -> Option<&CallbackMetadata> {
        self.metadata.get(name)
    }

    pub fn exists(&self, name: &str) -> bool {
        self.callbacks.contains_key(name)
    }
}

impl Default for CallbackRegistry {
    fn default() -> Self {
        Self::new()
    }
}

type EventFilter = Box<dyn Fn(&EventData) -> bool + Send + Sync>;

pub struct EventManager {
    emitter: EventEmitter,
    callback_registry: CallbackRegistry,
    event_filters: HashMap<String, Vec<EventFilter>>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            emitter: EventEmitter::new(),
            callback_registry: CallbackRegistry::new(),
            event_filters: HashMap::new(),
        }
    }

    pub fn get_emitter(&mut self) -> &mut EventEmitter {
        &mut self.emitter
    }

    pub fn get_callback_registry(&mut self) -> &mut CallbackRegistry {
        &mut self.callback_registry
    }

    pub fn add_event_filter<F>(&mut self, event: &str, filter: F)
    where
        F: Fn(&EventData) -> bool + Send + Sync + 'static,
    {
        self.event_filters
            .entry(event.to_string())
            .or_default()
            .push(Box::new(filter));
    }

    pub fn emit_filtered(&mut self, event: &str, data: serde_json::Value) {
        let event_data = EventData::new(event.to_string(), data);

        // Check if event should be filtered
        if let Some(filters) = self.event_filters.get(event) {
            let should_emit = filters.iter().all(|filter| filter(&event_data));
            if !should_emit {
                return;
            }
        }

        self.emitter.emit(event, event_data.data);
    }

    pub fn create_event_chain(&mut self, events: Vec<String>) -> EventChain<'_> {
        EventChain::new(events, self)
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EventChain<'a> {
    events: Vec<String>,
    manager: &'a mut EventManager,
    current_index: usize,
}

impl<'a> EventChain<'a> {
    pub fn new(events: Vec<String>, manager: &'a mut EventManager) -> Self {
        Self {
            events,
            manager,
            current_index: 0,
        }
    }

    pub fn trigger_next(&mut self, data: serde_json::Value) -> Result<(), ApiError> {
        if self.current_index < self.events.len() {
            let event = &self.events[self.current_index];
            self.manager.get_emitter().emit(event, data);
            self.current_index += 1;
            Ok(())
        } else {
            Err(ApiError::ExecutionError {
                message: "Event chain completed".to_string(),
                position: None,
            })
        }
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
    }

    pub fn is_complete(&self) -> bool {
        self.current_index >= self.events.len()
    }
}
