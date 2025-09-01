use jetcrab::api::compiler::Compiler;
use jetcrab::api::config::EngineConfig;

#[test]
fn test_compiler_creation() {
    let config = EngineConfig::new();
    let compiler = Compiler::new(config);
    
    assert!(compiler.is_initialized());
}

#[test]
fn test_compiler_with_custom_config() {
    let config = EngineConfig::new()
        .with_optimization(jetcrab::api::config::OptimizationLevel::Aggressive)
        .with_strict_mode(true);
    
    let compiler = Compiler::new(config);
    
    assert!(compiler.is_initialized());
    assert!(compiler.config().strict_mode);
}

#[test]
fn test_compiler_initialization() {
    let compiler = Compiler::default();
    
    assert!(compiler.is_initialized());
    assert!(compiler.is_ready());
}

#[test]
fn test_compiler_config_access() {
    let config = EngineConfig::new()
        .with_timeout(std::time::Duration::from_secs(30));
    
    let compiler = Compiler::new(config);
    
    let compiler_config = compiler.config();
    assert_eq!(compiler_config.timeout, Some(std::time::Duration::from_secs(30)));
}

#[test]
fn test_compiler_status() {
    let compiler = Compiler::default();
    
    assert!(compiler.is_initialized());
    assert!(compiler.is_ready());
    assert!(!compiler.is_shutdown());
}

#[test]
fn test_compiler_shutdown() {
    let mut compiler = Compiler::default();
    
    assert!(compiler.is_ready());
    
    compiler.shutdown();
    
    assert!(compiler.is_shutdown());
    assert!(!compiler.is_ready());
}
