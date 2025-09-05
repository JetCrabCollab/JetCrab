//! # Integration Tests
//!
//! Integration tests for JetCrab runtime following Rust testing best practices.
//! These tests verify that different modules work together correctly.

use jetcrab::runtime::{JetCrabEngine, JetCrabRuntime};
use jetcrab::tools::Claw;
use std::path::PathBuf;

/// Test basic JavaScript execution through the engine
#[test]
fn test_basic_javascript_execution() {
    let mut engine = JetCrabEngine::new();

    // Test simple arithmetic
    let result = engine.evaluate("2 + 3 * 4");
    assert!(result.is_ok());

    // Test string operations
    let result = engine.evaluate("'Hello' + ' ' + 'World'");
    assert!(result.is_ok());

    // Test variable declarations
    let result = engine.evaluate("let x = 42; x");
    assert!(result.is_ok());
}

/// Test console API integration
#[tokio::test]
async fn test_console_api_integration() {
    let mut runtime = JetCrabRuntime::new();

    // Test console.log
    let result = runtime
        .evaluate_code("console.log('Hello, JetCrab!')")
        .await;
    assert!(result.is_ok());

    // Test console.error
    let result = runtime.evaluate_code("console.error('Test error')").await;
    assert!(result.is_ok());

    // Test console.warn
    let result = runtime.evaluate_code("console.warn('Test warning')").await;
    assert!(result.is_ok());
}

/// Test process API integration
#[tokio::test]
async fn test_process_api_integration() {
    let mut runtime = JetCrabRuntime::new();

    // Test process.version
    let result = runtime.evaluate_code("process.version").await;
    assert!(result.is_ok());

    // Test process.cwd
    let result = runtime.evaluate_code("process.cwd()").await;
    assert!(result.is_ok());

    // Test process.argv
    let result = runtime.evaluate_code("process.argv").await;
    assert!(result.is_ok());
}

/// Test fetch API integration
#[tokio::test]
async fn test_fetch_api_integration() {
    let mut runtime = JetCrabRuntime::new();

    // Test fetch function exists
    let result = runtime.evaluate_code("typeof fetch").await;
    assert!(result.is_ok());

    // Test fetch returns a promise
    let result = runtime.evaluate_code("fetch('https://example.com')").await;
    assert!(result.is_ok());
}

/// Test Claw package manager integration
#[test]
fn test_claw_package_manager_integration() {
    let project_root = PathBuf::from(".");
    let claw = Claw::new(project_root);

    // Test package info retrieval
    let result = claw.get_package_info();
    assert!(result.is_ok());

    let package_info = result.unwrap();
    assert_eq!(package_info.name, "jetcrab-project");
    assert_eq!(package_info.version, "0.4.0");
}

/// Test error handling across modules
#[test]
fn test_error_handling_integration() {
    let mut engine = JetCrabEngine::new();

    // Test syntax error handling
    let result = engine.evaluate("invalid syntax {");
    assert!(result.is_err());

    // Test runtime error handling
    let result = engine.evaluate("undefined_variable");
    assert!(result.is_err());
}

/// Test WebAssembly runtime integration
#[test]
fn test_wasm_runtime_integration() {
    let mut engine = JetCrabEngine::new();

    // Test WASM runtime initialization
    // This should not panic
    let result = engine.evaluate("WebAssembly");
    // Should handle WebAssembly object gracefully
    assert!(result.is_ok());
}

/// Test CLI integration
#[test]
fn test_cli_integration() {
    use clap::Parser;
    use jetcrab::cli::Cli;

    // Test CLI parsing
    let cli = Cli::try_parse_from(&["jetcrab", "version"]);
    assert!(cli.is_ok());

    let cli = Cli::try_parse_from(&["jetcrab", "eval", "console.log('test')"]);
    assert!(cli.is_ok());

    let cli = Cli::try_parse_from(&["jetcrab", "crab"]);
    assert!(cli.is_ok());
}

/// Test end-to-end workflow
#[tokio::test]
async fn test_end_to_end_workflow() {
    let mut runtime = JetCrabRuntime::new();

    // Test complete workflow: evaluate code with APIs
    let code = r#"
        console.log("Starting workflow test");
        console.log("Version:", process.version);
        console.log("Current directory:", process.cwd());
        
        // Test fetch operation
        fetch("https://example.com");
        
        console.log("Workflow test completed");
    "#;

    let result = runtime.evaluate_code(code).await;
    assert!(result.is_ok());
}

/// Test performance characteristics
#[test]
fn test_performance_characteristics() {
    let mut engine = JetCrabEngine::new();

    // Test rapid successive evaluations
    for i in 0..100 {
        let code = format!("let x{} = {}; x{}", i, i, i);
        let result = engine.evaluate(&code);
        assert!(result.is_ok(), "Failed at iteration {}", i);
    }
}

/// Test memory management
#[test]
fn test_memory_management() {
    let mut engine = JetCrabEngine::new();

    // Test large object creation
    let code = r#"
        let largeObject = {};
        for (let i = 0; i < 1000; i++) {
            largeObject[`key${i}`] = `value${i}`;
        }
        Object.keys(largeObject).length
    "#;

    let result = engine.evaluate(code);
    assert!(result.is_ok());
}

/// Test concurrent operations
#[test]
fn test_concurrent_operations() {
    use std::thread;

    // Test spawning multiple threads
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let mut engine = JetCrabEngine::new();
            let result = engine.evaluate(&format!("{} * 2", i));
            assert!(result.is_ok());
            i * 2
        });
        handles.push(handle);
    }

    // All threads should complete successfully
    for handle in handles {
        let result = handle.join().unwrap();
        assert!(result >= 0);
    }
}
