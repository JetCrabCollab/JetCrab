use jetcrab::api::events::*;
use serde_json::json;
use std::time::Instant;

#[test]
fn test_event_data_creation() {
    let event_data = EventData::new("test_event".to_string(), json!("test_data"));
    assert_eq!(event_data.event_type, "test_event");
    assert_eq!(event_data.data, json!("test_data"));
    assert!(event_data.source.is_none());
}

#[test]
fn test_event_data_with_source() {
    let event_data = EventData::new("test_event".to_string(), json!("test_data"))
        .with_source("test_source".to_string());
    assert_eq!(event_data.event_type, "test_event");
    assert_eq!(event_data.data, json!("test_data"));
    assert_eq!(event_data.source, Some("test_source".to_string()));
}

#[test]
fn test_event_data_clone() {
    let event_data = EventData::new("test_event".to_string(), json!("test_data"));
    let cloned = event_data.clone();
    assert_eq!(cloned.event_type, "test_event");
    assert_eq!(cloned.data, json!("test_data"));
}

#[test]
fn test_event_data_debug() {
    let event_data = EventData::new("test_event".to_string(), json!("test_data"));
    let debug_str = format!("{:?}", event_data);
    assert!(debug_str.contains("test_event"));
}

#[test]
fn test_event_emitter_creation() {
    let emitter = EventEmitter::new();
    assert_eq!(emitter.get_event_history().len(), 0);
}

#[test]
fn test_event_emitter_default() {
    let emitter = EventEmitter::default();
    assert_eq!(emitter.get_event_history().len(), 0);
}

#[test]
fn test_event_emitter_on() {
    let mut emitter = EventEmitter::new();
    emitter.on("test_event", |_data| {});
    emitter.emit("test_event", json!("test_data"));
    assert_eq!(emitter.get_event_history().len(), 1);
}

#[test]
fn test_event_emitter_emit() {
    let mut emitter = EventEmitter::new();
    emitter.emit("test_event", json!("test_data"));
    assert_eq!(emitter.get_event_history().len(), 1);
    assert_eq!(emitter.get_event_history()[0].event_type, "test_event");
    assert_eq!(emitter.get_event_history()[0].data, json!("test_data"));
}

#[test]
fn test_event_emitter_emit_with_source() {
    let mut emitter = EventEmitter::new();
    emitter.emit_with_source("test_event", json!("test_data"), "test_source".to_string());
    assert_eq!(emitter.get_event_history().len(), 1);
    assert_eq!(emitter.get_event_history()[0].event_type, "test_event");
    assert_eq!(emitter.get_event_history()[0].data, json!("test_data"));
    assert_eq!(
        emitter.get_event_history()[0].source,
        Some("test_source".to_string())
    );
}

#[test]
fn test_event_emitter_remove_listener() {
    let mut emitter = EventEmitter::new();
    emitter.on("test_event", |_| {});
    emitter.on("test_event", |_| {});

    let result = emitter.remove_listener("test_event", 0);
    assert!(result.is_ok());

    let result = emitter.remove_listener("test_event", 10);
    assert!(result.is_err());

    let result = emitter.remove_listener("nonexistent", 0);
    assert!(result.is_err());
}

#[test]
fn test_event_emitter_clear_listeners() {
    let mut emitter = EventEmitter::new();
    emitter.on("test_event", |_| {});
    emitter.clear_listeners("test_event");
    assert!(true);
}

#[test]
fn test_event_emitter_get_event_history() {
    let mut emitter = EventEmitter::new();
    emitter.emit("test_event", json!("test_data"));
    let history = emitter.get_event_history();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].event_type, "test_event");
}

#[test]
fn test_event_emitter_set_max_history_size() {
    let mut emitter = EventEmitter::new();
    emitter.set_max_history_size(5);

    for i in 0..10 {
        emitter.emit("test_event", json!(i));
    }
    assert_eq!(emitter.get_event_history().len(), 5);
}

#[test]
fn test_callback_metadata_creation() {
    let metadata = CallbackMetadata {
        name: "test_callback".to_string(),
        description: "Test callback".to_string(),
        parameters: vec!["param1".to_string()],
        return_type: "string".to_string(),
        created_at: Instant::now(),
        call_count: 0,
    };
    assert_eq!(metadata.name, "test_callback");
    assert_eq!(metadata.description, "Test callback");
    assert_eq!(metadata.parameters.len(), 1);
    assert_eq!(metadata.return_type, "string");
    assert_eq!(metadata.call_count, 0);
}

#[test]
fn test_callback_metadata_clone() {
    let metadata = CallbackMetadata {
        name: "test_callback".to_string(),
        description: "Test callback".to_string(),
        parameters: vec![],
        return_type: "string".to_string(),
        created_at: Instant::now(),
        call_count: 0,
    };
    let cloned = metadata.clone();
    assert_eq!(cloned.name, "test_callback");
    assert_eq!(cloned.description, "Test callback");
}

#[test]
fn test_callback_metadata_debug() {
    let metadata = CallbackMetadata {
        name: "test_callback".to_string(),
        description: "Test callback".to_string(),
        parameters: vec![],
        return_type: "string".to_string(),
        created_at: Instant::now(),
        call_count: 0,
    };
    let debug_str = format!("{:?}", metadata);
    assert!(debug_str.contains("test_callback"));
}

#[test]
fn test_callback_registry_creation() {
    let registry = CallbackRegistry::new();
    assert_eq!(registry.list_callbacks().len(), 0);
}

#[test]
fn test_callback_registry_default() {
    let registry = CallbackRegistry::default();
    assert_eq!(registry.list_callbacks().len(), 0);
}

#[test]
fn test_callback_registry_register() {
    let mut registry = CallbackRegistry::new();
    registry.register("test_callback", "Test callback", |data| Ok(data));
    assert!(registry.exists("test_callback"));
    assert_eq!(registry.list_callbacks().len(), 1);
}

#[test]
fn test_callback_registry_register_with_metadata() {
    let mut registry = CallbackRegistry::new();
    let metadata = CallbackMetadata {
        name: "test_callback".to_string(),
        description: "Test callback".to_string(),
        parameters: vec!["param1".to_string()],
        return_type: "string".to_string(),
        created_at: Instant::now(),
        call_count: 0,
    };
    registry.register_with_metadata("test_callback", metadata, |data| Ok(data));
    assert!(registry.exists("test_callback"));
    assert_eq!(registry.list_callbacks().len(), 1);
}

#[test]
fn test_callback_registry_call() {
    let mut registry = CallbackRegistry::new();
    registry.register("test_callback", "Test callback", |data| Ok(data));
    let result = registry.call("test_callback", json!("test_data"));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), json!("test_data"));
}

#[test]
fn test_callback_registry_call_nonexistent() {
    let mut registry = CallbackRegistry::new();
    let result = registry.call("nonexistent", json!("test_data"));
    assert!(result.is_err());
}

#[test]
fn test_callback_registry_unregister() {
    let mut registry = CallbackRegistry::new();
    registry.register("test_callback", "Test callback", |data| Ok(data));
    assert!(registry.exists("test_callback"));
    let removed = registry.unregister("test_callback");
    assert!(removed);
    assert!(!registry.exists("test_callback"));
}

#[test]
fn test_callback_registry_list_callbacks() {
    let mut registry = CallbackRegistry::new();
    registry.register("callback1", "Callback 1", |data| Ok(data));
    registry.register("callback2", "Callback 2", |data| Ok(data));
    let callbacks = registry.list_callbacks();
    assert_eq!(callbacks.len(), 2);
}

#[test]
fn test_callback_registry_get_callback_info() {
    let mut registry = CallbackRegistry::new();
    registry.register("test_callback", "Test callback", |data| Ok(data));
    let info = registry.get_callback_info("test_callback");
    assert!(info.is_some());
    assert_eq!(info.unwrap().name, "test_callback");
    assert_eq!(info.unwrap().description, "Test callback");
}

#[test]
fn test_callback_registry_get_callback_info_nonexistent() {
    let registry = CallbackRegistry::new();
    let info = registry.get_callback_info("nonexistent");
    assert!(info.is_none());
}

#[test]
fn test_callback_registry_exists() {
    let mut registry = CallbackRegistry::new();
    assert!(!registry.exists("test_callback"));
    registry.register("test_callback", "Test callback", |data| Ok(data));
    assert!(registry.exists("test_callback"));
}

#[test]
fn test_event_manager_creation() {
    let mut manager = EventManager::new();
    assert_eq!(manager.get_emitter().get_event_history().len(), 0);
    assert_eq!(manager.get_callback_registry().list_callbacks().len(), 0);
}

#[test]
fn test_event_manager_default() {
    let mut manager = EventManager::default();
    assert_eq!(manager.get_emitter().get_event_history().len(), 0);
    assert_eq!(manager.get_callback_registry().list_callbacks().len(), 0);
}

#[test]
fn test_event_manager_get_emitter() {
    let mut manager = EventManager::new();
    let emitter = manager.get_emitter();
    emitter.emit("test_event", json!("test_data"));
    assert_eq!(emitter.get_event_history().len(), 1);
}

#[test]
fn test_event_manager_get_callback_registry() {
    let mut manager = EventManager::new();
    let registry = manager.get_callback_registry();
    registry.register("test_callback", "Test callback", |data| Ok(data));
    assert!(registry.exists("test_callback"));
}

#[test]
fn test_event_manager_add_event_filter() {
    let mut manager = EventManager::new();
    manager.add_event_filter("test_event", |_| true);
    assert!(true);
}

#[test]
fn test_event_manager_emit_filtered() {
    let mut manager = EventManager::new();
    manager.add_event_filter("test_event", |_| true);
    manager.emit_filtered("test_event", json!("test_data"));
    assert_eq!(manager.get_emitter().get_event_history().len(), 1);
}

#[test]
fn test_event_manager_emit_filtered_blocked() {
    let mut manager = EventManager::new();
    manager.add_event_filter("test_event", |_| false);
    manager.emit_filtered("test_event", json!("test_data"));
    assert_eq!(manager.get_emitter().get_event_history().len(), 0);
}

#[test]
fn test_event_manager_create_event_chain() {
    let mut manager = EventManager::new();
    let events = vec!["event1".to_string(), "event2".to_string()];
    let chain = manager.create_event_chain(events);
    assert!(!chain.is_complete());
}

#[test]
fn test_event_chain_creation() {
    let mut manager = EventManager::new();
    let events = vec!["event1".to_string(), "event2".to_string()];
    let chain = EventChain::new(events, &mut manager);
    assert!(!chain.is_complete());
}

#[test]
fn test_event_chain_trigger_next() {
    let mut manager = EventManager::new();
    let events = vec!["event1".to_string(), "event2".to_string()];
    let mut chain = EventChain::new(events, &mut manager);

    let result = chain.trigger_next(json!("test_data"));
    assert!(result.is_ok());
    assert!(!chain.is_complete());
    assert_eq!(manager.get_emitter().get_event_history().len(), 1);
}

#[test]
fn test_event_chain_trigger_next_complete() {
    let mut manager = EventManager::new();
    let events = vec!["event1".to_string()];
    let mut chain = EventChain::new(events, &mut manager);

    let result = chain.trigger_next(json!("test_data"));
    assert!(result.is_ok());

    let result = chain.trigger_next(json!("test_data"));
    assert!(result.is_err());
}

#[test]
fn test_event_chain_reset() {
    let mut manager = EventManager::new();
    let events = vec!["event1".to_string(), "event2".to_string()];
    let mut chain = EventChain::new(events, &mut manager);

    chain.trigger_next(json!("test_data"));
    assert!(!chain.is_complete());

    chain.reset();
    assert!(!chain.is_complete());
}

#[test]
fn test_event_chain_is_complete() {
    let mut manager = EventManager::new();
    let events = vec!["event1".to_string()];
    let mut chain = EventChain::new(events, &mut manager);

    assert!(!chain.is_complete());
    chain.trigger_next(json!("test_data"));
    assert!(chain.is_complete());
}

#[test]
fn test_event_chain_is_complete_multiple_events() {
    let mut manager = EventManager::new();
    let events = vec!["event1".to_string(), "event2".to_string()];
    let mut chain = EventChain::new(events, &mut manager);

    assert!(!chain.is_complete());
    chain.trigger_next(json!("test_data"));
    assert!(!chain.is_complete());
    chain.trigger_next(json!("test_data"));
    assert!(chain.is_complete());
}
