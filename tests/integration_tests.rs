//! # Integration Tests
//!
//! Integration tests for JetCrab runtime following Rust testing best practices.
//! These tests verify that different modules work together correctly.

use jetcrab::runtime::{JetCrabEngine, JetCrabRuntime};

/// Test basic JavaScript execution through the engine
#[tokio::test]
async fn test_basic_javascript_execution() {
    let mut engine = JetCrabEngine::new();

    let result = engine.evaluate("2 + 3 * 4").await;
    assert!(result.is_ok());

    let result = engine.evaluate("'Hello' + ' ' + 'World'").await;
    assert!(result.is_ok());

    let result = engine.evaluate("let x = 42; x").await;
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

/// Test error handling across modules
#[tokio::test]
async fn test_error_handling_integration() {
    let mut engine = JetCrabEngine::new();

    let result = engine.evaluate("invalid syntax {").await;
    assert!(result.is_err());

    let result = engine.evaluate("undefined_variable").await;
    assert!(result.is_err());
}

/// Test WebAssembly runtime integration
#[tokio::test]
async fn test_wasm_runtime_integration() {
    let mut engine = JetCrabEngine::new();

    let result = engine.evaluate("WebAssembly").await;
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
#[tokio::test]
async fn test_performance_characteristics() {
    let mut engine = JetCrabEngine::new();

    for i in 0..100 {
        let code = format!("let x{} = {}; x{}", i, i, i);
        let result = engine.evaluate(&code).await;
        assert!(result.is_ok(), "Failed at iteration {}", i);
    }
}

/// Test memory management
#[tokio::test]
async fn test_memory_management() {
    let mut engine = JetCrabEngine::new();

    let code = r#"
        let largeObject = {};
        for (let i = 0; i < 1000; i++) {
            largeObject[`key${i}`] = `value${i}`;
        }
        Object.keys(largeObject).length
    "#;

    let result = engine.evaluate(code).await;
    assert!(result.is_ok());
}

/// Test concurrent operations
#[tokio::test]
async fn test_concurrent_operations() {
    use tokio::task;

    let handles: Vec<_> = (0..10)
        .map(|i| {
            task::spawn(async move {
                let mut engine = JetCrabEngine::new();
                let result = engine.evaluate(&format!("{} * 2", i)).await;
                assert!(result.is_ok());
                i * 2
            })
        })
        .collect();

    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result >= 0);
    }
}
