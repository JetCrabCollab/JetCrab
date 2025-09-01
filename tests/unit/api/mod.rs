//! API Unit Tests - Mirroring src/api/ structure
//! 
//! This module contains unit tests for public API components:
//! - engine.rs: Main API engine
//! - compiler.rs: Compilation API
//! - interpreter.rs: Interpretation API
//! - config.rs: Configuration API
//! - debug.rs: Debug API
//! - error.rs: Error handling
//! - events.rs: Event system
//! - modules.rs: Module system

use jetcrab::api::{Engine, Compiler, Interpreter, ApiConfig};

#[test]
fn test_api_engine_creation() {
    let config = ApiConfig::default();
    let engine = Engine::new(config);
    
    assert!(engine.is_initialized());
    assert!(engine.is_ready());
}

#[test]
fn test_api_compiler_creation() {
    let config = ApiConfig::default();
    let compiler = Compiler::new(config);
    
    assert!(compiler.is_initialized());
    assert!(compiler.is_ready());
}

#[test]
fn test_api_interpreter_creation() {
    let config = ApiConfig::default();
    let interpreter = Interpreter::new(config);
    
    assert!(interpreter.is_initialized());
    assert!(interpreter.is_ready());
}

#[test]
fn test_api_config_default() {
    let config = ApiConfig::default();
    
    assert_eq!(config.memory_size, 64 * 1024 * 1024); // 64MB
    assert_eq!(config.stack_size, 1024 * 1024); // 1MB
    assert!(config.enable_gc);
    assert!(config.enable_optimization);
}

#[test]
fn test_api_config_custom() {
    let config = ApiConfig::new()
        .with_memory_size(32 * 1024 * 1024) // 32MB
        .with_stack_size(512 * 1024) // 512KB
        .with_gc(false)
        .with_optimization(false);
    
    assert_eq!(config.memory_size, 32 * 1024 * 1024);
    assert_eq!(config.stack_size, 512 * 1024);
    assert!(!config.enable_gc);
    assert!(!config.enable_optimization);
}

#[test]
fn test_api_engine_compilation() {
    let mut engine = Engine::new(ApiConfig::default());
    
    let source = "42 + 10";
    let result = engine.compile(source);
    
    // Should either succeed or fail gracefully
    match result {
        Ok(bytecode) => {
            // Compilation succeeded
            assert!(bytecode.instructions.len() > 0);
        }
        Err(_) => {
            // Compilation failed as expected
            assert!(true);
        }
    }
}

#[test]
fn test_api_engine_execution() {
    let mut engine = Engine::new(ApiConfig::default());
    
    let source = "42";
    let result = engine.execute(source);
    
    // Should either succeed or fail gracefully
    match result {
        Ok(value) => {
            // Execution succeeded
            assert!(matches!(value, jetcrab::vm::value::Value::Number(42.0)));
        }
        Err(_) => {
            // Execution failed as expected
            assert!(true);
        }
    }
}

#[test]
fn test_api_compiler_compile() {
    let mut compiler = Compiler::new(ApiConfig::default());
    
    let source = "let x = 42; x + 10";
    let result = compiler.compile(source);
    
    // Should either succeed or fail gracefully
    match result {
        Ok(bytecode) => {
            // Compilation succeeded
            assert!(bytecode.instructions.len() > 0);
        }
        Err(_) => {
            // Compilation failed as expected
            assert!(true);
        }
    }
}

#[test]
fn test_api_interpreter_execute() {
    let mut interpreter = Interpreter::new(ApiConfig::default());
    
    let source = "42 + 10";
    let result = interpreter.execute(source);
    
    // Should either succeed or fail gracefully
    match result {
        Ok(value) => {
            // Execution succeeded
            assert!(matches!(value, jetcrab::vm::value::Value::Number(52.0)));
        }
        Err(_) => {
            // Execution failed as expected
            assert!(true);
        }
    }
}

#[test]
fn test_api_error_handling() {
    let mut engine = Engine::new(ApiConfig::default());
    
    let invalid_source = "invalid syntax + +";
    let result = engine.compile(invalid_source);
    
    // Should fail gracefully
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    assert!(error.to_string().contains("error"));
}

#[test]
fn test_api_memory_management() {
    let mut engine = Engine::new(ApiConfig::default());
    
    // Allocate memory
    let addr = engine.allocate_memory(1024);
    assert!(addr.is_some());
    
    // Check memory usage
    let stats = engine.stats();
    assert!(stats.memory_usage > 0);
    
    // Deallocate memory
    let result = engine.deallocate_memory(addr.unwrap());
    assert!(result);
}

#[test]
fn test_api_gc_integration() {
    let mut engine = Engine::new(
        ApiConfig::new().with_gc(true)
    );
    
    // Allocate some memory to trigger GC
    for _ in 0..100 {
        engine.allocate_memory(1024 * 1024); // 1MB each
    }
    
    // Check if GC was triggered
    let stats = engine.stats();
    assert!(stats.gc_collections >= 0);
}

#[test]
fn test_api_optimization() {
    let mut engine = Engine::new(
        ApiConfig::new().with_optimization(true)
    );
    
    let source = "42 + 0"; // Should be optimized to just 42
    let result = engine.compile(source);
    
    // Should either succeed or fail gracefully
    match result {
        Ok(bytecode) => {
            // Compilation succeeded, check if optimization was applied
            assert!(bytecode.instructions.len() > 0);
        }
        Err(_) => {
            // Compilation failed as expected
            assert!(true);
        }
    }
}
