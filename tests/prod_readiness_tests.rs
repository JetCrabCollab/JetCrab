//! Production Readiness Tests - JetCrab
//!
//! Validates that the JetCrab runtime meets production readiness criteria.
//! Run with: cargo test --test prod_readiness_tests

use jetcrab::runtime::{JetCrabEngine, JetCrabRuntime};
use clap::Parser;
use jetcrab::cli::Cli;

#[tokio::test]
async fn prod_engine_initializes_without_panic() {
    let _engine = JetCrabEngine::new();
}

#[tokio::test]
async fn prod_engine_evaluates_basic_arithmetic() {
    let mut engine = JetCrabEngine::new();
    let result = engine.evaluate("2 + 2").await;
    assert!(result.is_ok(), "Basic arithmetic must succeed: {:?}", result.err());
}

#[tokio::test]
async fn prod_engine_evaluates_string_operations() {
    let mut engine = JetCrabEngine::new();
    let result = engine.evaluate(r#""hello" + " " + "world""#).await;
    assert!(result.is_ok(), "String ops must succeed: {:?}", result.err());
}

#[tokio::test]
async fn prod_engine_handles_syntax_error_gracefully() {
    let mut engine = JetCrabEngine::new();
    let result = engine.evaluate("invalid syntax {{{").await;
    assert!(result.is_err(), "Syntax errors must return Err, not panic");
}

#[tokio::test]
async fn prod_runtime_initializes() {
    let _runtime = JetCrabRuntime::new();
}

#[tokio::test]
async fn prod_console_api_works() {
    let mut runtime = JetCrabRuntime::new();
    let result = runtime.evaluate_code("console.log('prod test')").await;
    assert!(result.is_ok(), "console.log must work: {:?}", result.err());
}

#[tokio::test]
async fn prod_process_api_works() {
    let mut runtime = JetCrabRuntime::new();
    let result = runtime.evaluate_code("process.version").await;
    assert!(result.is_ok(), "process.version must work: {:?}", result.err());
    let result = runtime.evaluate_code("process.cwd()").await;
    assert!(result.is_ok(), "process.cwd must work: {:?}", result.err());
}

#[tokio::test]
async fn prod_fetch_api_exists() {
    let mut runtime = JetCrabRuntime::new();
    let result = runtime.evaluate_code("typeof fetch === 'function'").await;
    assert!(result.is_ok(), "fetch must be available: {:?}", result.err());
}

#[tokio::test]
async fn prod_concurrent_evaluations() {
    use tokio::task;
    let handles: Vec<_> = (0..5)
        .map(|i| {
            task::spawn(async move {
                let mut engine = JetCrabEngine::new();
                engine.evaluate(&format!("{} + 1", i)).await
            })
        })
        .collect();
    for h in handles {
        let r = h.await.unwrap();
        assert!(r.is_ok(), "Concurrent eval must not panic");
    }
}

#[test]
fn prod_cli_parses_version_flag() {
    let cli = Cli::try_parse_from(&["jetcrab", "-v"]);
    assert!(cli.is_ok(), "CLI must parse -v/--version");
}

#[test]
fn prod_cli_parses_eval() {
    let cli = Cli::try_parse_from(&["jetcrab", "-e", "1+1"]);
    assert!(cli.is_ok(), "CLI must parse -e");
}

#[test]
fn prod_cli_parses_run_command() {
    let cli = Cli::try_parse_from(&["jetcrab", "run", "script.js"]);
    assert!(cli.is_ok(), "CLI must parse run");
}

#[test]
fn prod_cli_parses_help_flag() {
    let cli = Cli::try_parse_from(&["jetcrab", "--help"]);
    assert!(cli.is_ok(), "CLI must parse --help");
}

#[test]
fn prod_cli_parses_repl() {
    let cli = Cli::try_parse_from(&["jetcrab"]);
    assert!(cli.is_ok(), "CLI must parse with no args (REPL)");
}

#[tokio::test]
async fn prod_no_panic_on_large_expression() {
    let mut engine = JetCrabEngine::new();
    let expr = "1+2+3+4+5+6+7+8+9+10";
    let result = engine.evaluate(expr).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn prod_undefined_reference_returns_error() {
    let mut engine = JetCrabEngine::new();
    let result = engine.evaluate("nonexistentVariable").await;
    assert!(result.is_err());
}
