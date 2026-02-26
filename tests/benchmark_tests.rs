//! # Benchmark Tests
//!
//! Performance benchmarks for JetCrab runtime following Rust testing best practices.
//! These tests measure execution time and performance characteristics.

use jetcrab::runtime::JetCrabEngine;
use std::time::Instant;

/// Benchmark simple arithmetic operations
#[tokio::test]
async fn benchmark_arithmetic_operations() {
    let mut engine = JetCrabEngine::new();
    let start = std::time::Instant::now();

    for i in 0..1000 {
        let code = format!("{} + {} * {}", i, i + 1, i + 2);
        let result = engine.evaluate(&code).await;
        assert!(result.is_ok());
    }

    let duration = start.elapsed();
    println!("Arithmetic operations benchmark: {:?}", duration);
    assert!(duration.as_millis() < 5000);
}

/// Benchmark string operations
#[tokio::test]
async fn benchmark_string_operations() {
    let mut engine = JetCrabEngine::new();
    let start = std::time::Instant::now();

    for i in 0..1000 {
        let code = format!("'Hello' + ' ' + 'World' + '{}'", i);
        let result = engine.evaluate(&code).await;
        assert!(result.is_ok());
    }

    let duration = start.elapsed();
    println!("String operations benchmark: {:?}", duration);
    assert!(duration.as_millis() < 5000);
}

/// Benchmark variable assignments
#[tokio::test]
async fn benchmark_variable_assignments() {
    let mut engine = JetCrabEngine::new();
    let start = std::time::Instant::now();

    for i in 0..1000 {
        let code = format!("let var{} = {}; var{}", i, i, i);
        let result = engine.evaluate(&code).await;
        assert!(result.is_ok());
    }

    let duration = start.elapsed();
    println!("Variable assignments benchmark: {:?}", duration);
    assert!(duration.as_millis() < 5000);
}

/// Benchmark function calls
#[tokio::test]
async fn benchmark_function_calls() {
    let mut engine = JetCrabEngine::new();

    let setup_code = r#"
        function add(a, b) {
            return a + b;
        }
    "#;

    let result = engine.evaluate(setup_code).await;
    assert!(result.is_ok());

    let start = std::time::Instant::now();

    for i in 0..1000 {
        let code = format!("add({}, {})", i, i + 1);
        let result = engine.evaluate(&code).await;
        assert!(result.is_ok());
    }

    let duration = start.elapsed();
    println!("Function calls benchmark: {:?}", duration);
    assert!(duration.as_millis() < 5000);
}

/// Benchmark object property access
#[tokio::test]
async fn benchmark_object_property_access() {
    let mut engine = JetCrabEngine::new();

    let setup_code = r#"
        let testObj = {};
        for (let i = 0; i < 100; i++) {
            testObj[`prop${i}`] = i;
        }
    "#;

    let result = engine.evaluate(setup_code).await;
    assert!(result.is_ok());

    let start = std::time::Instant::now();

    for i in 0..1000 {
        let prop_index = i % 100;
        let code = format!("testObj.prop{}", prop_index);
        let result = engine.evaluate(&code).await;
        assert!(result.is_ok());
    }

    let duration = start.elapsed();
    println!("Object property access benchmark: {:?}", duration);
    assert!(duration.as_millis() < 5000);
}

/// Benchmark array operations
#[tokio::test]
async fn benchmark_array_operations() {
    let mut engine = JetCrabEngine::new();

    let setup_code = r#"
        let testArray = [];
        for (let i = 0; i < 1000; i++) {
            testArray.push(i);
        }
    "#;

    let result = engine.evaluate(setup_code).await;
    assert!(result.is_ok());

    let start = std::time::Instant::now();

    for i in 0..1000 {
        let code = format!("testArray[{}]", i % 1000);
        let result = engine.evaluate(&code).await;
        assert!(result.is_ok());
    }

    let duration = start.elapsed();
    println!("Array operations benchmark: {:?}", duration);
    assert!(duration.as_millis() < 5000);
}

/// Benchmark complex expressions
#[tokio::test]
async fn benchmark_complex_expressions() {
    let mut engine = JetCrabEngine::new();
    let start = std::time::Instant::now();

    for i in 0..100 {
        let code = format!(
            "Math.sqrt({}) + Math.sin({}) * Math.cos({}) + ({}) * ({})",
            i, i, i, i, i + 1
        );
        let result = engine.evaluate(&code).await;
        assert!(result.is_ok());
    }

    let duration = start.elapsed();
    println!("Complex expressions benchmark: {:?}", duration);
    assert!(duration.as_millis() < 5000);
}

/// Benchmark engine initialization
#[test]
fn benchmark_engine_initialization() {
    let start = std::time::Instant::now();

    for _ in 0..100 {
        let _engine = JetCrabEngine::new();
    }

    let duration = start.elapsed();
    println!("Engine initialization benchmark: {:?}", duration);
    assert!(duration.as_millis() < 1000);
}

/// Benchmark memory usage with large objects
#[tokio::test]
async fn benchmark_memory_usage() {
    let mut engine = JetCrabEngine::new();
    let start = std::time::Instant::now();

    for i in 0..10 {
        let code = format!(
            r#"
            let largeObj{} = {{}};
            for (let j = 0; j < 1000; j++) {{
                largeObj{}[`key${{j}}`] = `value${{j}}_${{i}}`;
            }}
            Object.keys(largeObj{}).length
            "#,
            i, i, i
        );

        let result = engine.evaluate(&code).await;
        if result.is_err() {
            println!("Memory test failed as expected: {:?}", result.err());
        }
    }

    let duration = start.elapsed();
    println!("Memory usage benchmark: {:?}", duration);
    assert!(duration.as_millis() < 10000);
}

/// Benchmark concurrent evaluations
#[tokio::test]
async fn benchmark_concurrent_evaluations() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tokio::task;

    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    let start = std::time::Instant::now();

    for thread_id in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = task::spawn(async move {
            let mut engine = JetCrabEngine::new();

            for i in 0..100 {
                let code = format!("{} + {} * {}", thread_id, i, i + 1);
                let result = engine.evaluate(&code).await;
                if result.is_ok() {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let duration = start.elapsed();
    let total_evaluations = counter.load(Ordering::Relaxed);

    println!("Concurrent evaluations benchmark: {:?}", duration);
    println!("Total successful evaluations: {}", total_evaluations);

    assert_eq!(total_evaluations, 1000);
    assert!(duration.as_millis() < 10000);
}

/// Benchmark error handling performance
#[tokio::test]
async fn benchmark_error_handling() {
    let mut engine = JetCrabEngine::new();
    let start = std::time::Instant::now();

    for i in 0..1000 {
        let code = format!("undefined_variable_{}", i);
        let result = engine.evaluate(&code).await;
        assert!(result.is_err());
    }

    let duration = start.elapsed();
    println!("Error handling benchmark: {:?}", duration);
    assert!(duration.as_millis() < 5000);
}

/// Benchmark mixed workload
#[tokio::test]
async fn benchmark_mixed_workload() {
    let mut engine = JetCrabEngine::new();
    let start = std::time::Instant::now();

    for i in 0..1000 {
        let operations = vec![
            format!("{} + {}", i, i + 1),
            format!("'Hello' + '{}'", i),
            format!("let x{} = {}; x{}", i, i, i),
            format!("Math.sqrt({})", i),
            format!("[1, 2, 3, {}].length", i),
        ];

        for operation in operations {
            let result = engine.evaluate(&operation).await;
            assert!(result.is_ok());
        }
    }

    let duration = start.elapsed();
    println!("Mixed workload benchmark: {:?}", duration);
    assert!(duration.as_millis() < 15000);
}
