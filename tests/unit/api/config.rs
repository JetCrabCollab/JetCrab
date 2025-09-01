use jetcrab::api::config::{EngineConfig, OptimizationLevel, SecurityLevel, MemoryConfig};
use std::time::Duration;

#[test]
fn test_default_config() {
    let config = EngineConfig::default();
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_builder() {
    let config = EngineConfig::new()
        .with_optimization(OptimizationLevel::Aggressive)
        .with_strict_mode(true)
        .with_timeout(Duration::from_secs(60));

    assert_eq!(config.optimization_level, OptimizationLevel::Aggressive);
    assert!(config.strict_mode);
    assert_eq!(config.timeout, Some(Duration::from_secs(60)));
}

#[test]
fn test_memory_config_validation() {
    let mut memory_config = MemoryConfig::default();
    memory_config.initial_heap_size = 200 * 1024 * 1024; // 200MB
    memory_config.max_heap_size = 100 * 1024 * 1024; // 100MB

    let config = EngineConfig::new().with_memory_config(memory_config);
    assert!(config.validate().is_err());
}

#[test]
fn test_production_ready() {
    let production_config = EngineConfig::new()
        .with_security_level(SecurityLevel::Strict)
        .with_debugging(false)
        .with_profiling(false);

    assert!(production_config.is_production_ready());
}
