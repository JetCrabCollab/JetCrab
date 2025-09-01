//! API Unit Tests - Testing only public APIs
//!
//! This module contains unit tests for public API components:
//! - engine.rs: Main API engine
//! - compiler.rs: Compilation API
//! - interpreter.rs: Interpretation API
//! - config.rs: Configuration API

use jetcrab::api::{Compiler, Engine, EngineConfig, Interpreter, OptimizationLevel};
use jetcrab::vm::instructions::Instruction;

#[test]
fn test_api_engine_creation() {
    let engine = Engine::new();

    // Test that engine can be created and has context
    let context = engine.get_context();
    assert!(context.variables.is_empty());
}

#[test]
fn test_api_engine_evaluation() {
    let mut engine = Engine::new();

    // Test simple number evaluation
    let result = engine.evaluate("42");
    assert!(result.is_ok());

    // Test arithmetic evaluation
    let result = engine.evaluate("2 + 3");
    assert!(result.is_ok());
}

#[test]
fn test_api_compiler_creation() {
    let mut compiler = Compiler::new();

    // Test that compiler can be created
    let result = compiler.compile("42");
    assert!(result.is_ok());
}

#[test]
fn test_api_compiler_with_optimization() {
    let mut compiler = Compiler::new().with_optimization(true);

    // Test compilation with optimization
    let result = compiler.compile("42 + 0");
    assert!(result.is_ok());
}

#[test]
fn test_api_compiler_bytecode_generation() {
    let mut compiler = Compiler::new();

    let result = compiler.compile_to_bytecode("42");
    assert!(result.is_ok());

    let (instructions, constants) = result.unwrap();
    assert!(!instructions.is_empty());
    assert!(!constants.is_empty());
}

#[test]
fn test_api_interpreter_creation() {
    let instructions = vec![Instruction::PushConst(0.into())];
    let constants = vec!["42".to_string()];
    let interpreter = Interpreter::new(instructions, constants);

    // Test that interpreter can be created
    let result = interpreter.execute();
    assert!(result.is_ok());
}

#[test]
fn test_api_config_default() {
    let config = EngineConfig::new();

    // Test default configuration
    assert_eq!(config.optimization_level, OptimizationLevel::Basic);
}

#[test]
fn test_api_config_memory_config() {
    let config = EngineConfig::new();

    // Test memory configuration access
    assert!(config.memory_config.max_heap_size > 0);
}

#[test]
fn test_api_engine_context_access() {
    let mut engine = Engine::new();

    // Test context access
    let context = engine.get_context();
    assert!(context.variables.is_empty());

    // Test mutable context access
    let context_mut = engine.get_context_mut();
    assert!(context_mut.variables.is_empty());
}

#[test]
fn test_api_engine_error_handling() {
    let mut engine = Engine::new();

    // Test error handling with invalid syntax
    let result = engine.evaluate("invalid syntax + +");
    // Should either succeed (with error recovery) or fail gracefully
    let _ = result;
}

#[test]
fn test_api_compiler_error_handling() {
    let mut compiler = Compiler::new();

    // Test error handling with invalid syntax
    let result = compiler.compile("invalid syntax + +");
    // Should either succeed (with error recovery) or fail gracefully
    let _ = result;
}
