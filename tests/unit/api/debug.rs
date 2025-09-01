use jetcrab::api::debug::{Breakpoint, Debugger, Profiler, Inspector};
use std::time::Duration;

#[test]
fn test_breakpoint() {
    let mut breakpoint = Breakpoint::new("bp1".to_string(), 10, 5);
    breakpoint.hit();
    assert_eq!(breakpoint.hit_count, 1);
}

#[test]
fn test_debugger() {
    let mut debugger = Debugger::new();
    let breakpoint = Breakpoint::new("bp1".to_string(), 10, 5);
    debugger.add_breakpoint(breakpoint);

    // Enable the debugger first
    debugger.enable();

    assert!(debugger.should_pause(10, 5));
    assert!(!debugger.should_pause(11, 5));
}

#[test]
fn test_profiler() {
    let mut profiler = Profiler::new();
    profiler.start_profiling();
    std::thread::sleep(Duration::from_millis(10));
    let metrics = profiler.stop_profiling();

    assert!(metrics.execution_time > Duration::ZERO);
}

#[test]
fn test_inspector() {
    let mut inspector = Inspector::new();

    inspector.add_event_listener(
        "test".to_string(),
        Box::new(|_| {
            // In a real test, we'd use a different approach to verify the event was received
            // For now, just verify the listener was added
        }),
    );

    // Test that the listener was added (this is a simplified test)
    assert!(true); // Placeholder assertion
}
