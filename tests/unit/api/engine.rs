use jetcrab::api::engine::Engine;
use jetcrab::api::config::EngineConfig;

#[test]
fn test_engine_creation() {
    let config = EngineConfig::new();
    let engine = Engine::new(config);
    
    assert!(engine.is_initialized());
}

#[test]
fn test_engine_with_custom_config() {
    let config = EngineConfig::new()
        .with_optimization(jetcrab::api::config::OptimizationLevel::Aggressive)
        .with_strict_mode(true);
    
    let engine = Engine::new(config);
    
    assert!(engine.is_initialized());
    assert!(engine.config().strict_mode);
}

#[test]
fn test_engine_initialization() {
    let engine = Engine::default();
    
    assert!(engine.is_initialized());
    assert!(engine.is_ready());
}

#[test]
fn test_engine_config_access() {
    let config = EngineConfig::new()
        .with_timeout(std::time::Duration::from_secs(30));
    
    let engine = Engine::new(config);
    
    let engine_config = engine.config();
    assert_eq!(engine_config.timeout, Some(std::time::Duration::from_secs(30)));
}

#[test]
fn test_engine_status() {
    let engine = Engine::default();
    
    assert!(engine.is_initialized());
    assert!(engine.is_ready());
    assert!(!engine.is_shutdown());
}

#[test]
fn test_engine_shutdown() {
    let mut engine = Engine::default();
    
    assert!(engine.is_ready());
    
    engine.shutdown();
    
    assert!(engine.is_shutdown());
    assert!(!engine.is_ready());
}
