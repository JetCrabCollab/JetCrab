use jetcrab::api::interpreter::Interpreter;
use jetcrab::api::config::EngineConfig;

#[test]
fn test_interpreter_creation() {
    let config = EngineConfig::new();
    let interpreter = Interpreter::new(config);
    
    assert!(interpreter.is_initialized());
}

#[test]
fn test_interpreter_with_custom_config() {
    let config = EngineConfig::new()
        .with_optimization(jetcrab::api::config::OptimizationLevel::Aggressive)
        .with_strict_mode(true);
    
    let interpreter = Interpreter::new(config);
    
    assert!(interpreter.is_initialized());
    assert!(interpreter.config().strict_mode);
}

#[test]
fn test_interpreter_initialization() {
    let interpreter = Interpreter::default();
    
    assert!(interpreter.is_initialized());
    assert!(interpreter.is_ready());
}

#[test]
fn test_interpreter_config_access() {
    let config = EngineConfig::new()
        .with_timeout(std::time::Duration::from_secs(30));
    
    let interpreter = Interpreter::new(config);
    
    let interpreter_config = interpreter.config();
    assert_eq!(interpreter_config.timeout, Some(std::time::Duration::from_secs(30)));
}

#[test]
fn test_interpreter_status() {
    let interpreter = Interpreter::default();
    
    assert!(interpreter.is_initialized());
    assert!(interpreter.is_ready());
    assert!(!interpreter.is_shutdown());
}

#[test]
fn test_interpreter_shutdown() {
    let mut interpreter = Interpreter::default();
    
    assert!(interpreter.is_ready());
    
    interpreter.shutdown();
    
    assert!(interpreter.is_shutdown());
    assert!(!interpreter.is_ready());
}
