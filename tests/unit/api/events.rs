use jetcrab::api::events::{EventEmitter, CallbackRegistry, EventManager, EventChain};
use jetcrab::api::error::ApiError;
use serde_json::json;
use std::sync::{Arc, Mutex};

#[test]
fn test_event_emitter() {
    let mut emitter = EventEmitter::new();
    let received = Arc::new(Mutex::new(false));
    let received_clone = Arc::clone(&received);

    emitter.on("test", move |_| {
        *received_clone.lock().unwrap() = true;
    });

    emitter.emit("test", json!("data"));

    let is_received = *received.lock().unwrap();
    assert!(is_received);
}

#[test]
fn test_callback_registry() {
    let mut registry = CallbackRegistry::new();

    registry.register("test", "Test callback", |data| {
        Ok(json!({ "result": data }))
    });

    let result = registry.call("test", json!("input")).unwrap();
    assert_eq!(result["result"], "input");
}

#[test]
fn test_event_manager() {
    let mut manager = EventManager::new();
    let received = Arc::new(Mutex::new(false));
    let received_clone = Arc::clone(&received);

    manager.get_emitter().on("test", move |_| {
        *received_clone.lock().unwrap() = true;
    });

    manager.emit_filtered("test", json!("data"));

    let is_received = *received.lock().unwrap();
    assert!(is_received);
}

#[test]
fn test_event_chain() {
    let mut manager = EventManager::new();
    let events = vec!["step1".to_string(), "step2".to_string()];

    // Add listener first
    manager.get_emitter().on("step1", move |_| {
        // This will be called when the event is emitted
    });

    // Then create and execute the chain
    let mut chain = manager.create_event_chain(events);

    assert!(!chain.is_complete());
    chain.trigger_next(json!("data")).unwrap();
    assert!(!chain.is_complete());
    chain.trigger_next(json!("data")).unwrap();
    assert!(chain.is_complete());
}
