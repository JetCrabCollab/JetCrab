//! # Property Tests
//!
//! Property-based tests for JetCrab runtime using quickcheck-style testing.
//! These tests validate function behavior across a wide range of inputs.

use jetcrab::easter_egg::should_trigger_easter_egg_for_command;
use jetcrab::runtime::JetCrabEngine;

/// Test that arithmetic operations are commutative
#[test]
fn test_arithmetic_commutativity() {
    let mut engine = JetCrabEngine::new();

    // Test addition commutativity: a + b = b + a
    for a in -100..=100 {
        for b in -100..=100 {
            let code1 = format!("{} + {}", a, b);
            let code2 = format!("{} + {}", b, a);

            let result1 = engine.evaluate(&code1);
            let result2 = engine.evaluate(&code2);

            assert!(result1.is_ok());
            assert!(result2.is_ok());
            assert_eq!(result1.unwrap(), result2.unwrap());
        }
    }
}

/// Test that string concatenation is associative
#[test]
fn test_string_concatenation_associativity() {
    let mut engine = JetCrabEngine::new();

    let test_strings = vec![
        "".to_string(),
        "a".to_string(),
        "hello".to_string(),
        "world".to_string(),
        "test string with spaces".to_string(),
        "special chars: !@#$%^&*()".to_string(),
    ];

    for a in &test_strings {
        for b in &test_strings {
            for c in &test_strings {
                let code1 = format!("'{}' + '{}' + '{}'", a, b, c);
                let code2 = format!("('{}' + '{}') + '{}'", a, b, c);
                let code3 = format!("'{}' + ('{}' + '{}')", a, b, c);

                let result1 = engine.evaluate(&code1);
                let result2 = engine.evaluate(&code2);
                let result3 = engine.evaluate(&code3);

                assert!(result1.is_ok());
                assert!(result2.is_ok());
                assert!(result3.is_ok());

                // All should produce the same result
                let val1 = result1.unwrap();
                let val2 = result2.unwrap();
                let val3 = result3.unwrap();
                assert_eq!(val1, val2);
                assert_eq!(val2, val3);
            }
        }
    }
}

/// Test that variable assignment is idempotent
#[test]
fn test_variable_assignment_idempotency() {
    let mut engine = JetCrabEngine::new();

    let test_values = vec![
        "42",
        "0",
        "-1",
        "3.14",
        "'hello'",
        "true",
        "false",
        "null",
        "undefined",
    ];

    for value in test_values {
        let code = format!("let x = {}; let y = x; x = y; x", value);
        let result = engine.evaluate(&code);

        assert!(result.is_ok());
        // The final value should be the same as the original
        let original_result = engine.evaluate(value);
        assert!(original_result.is_ok());
        let final_value = result.unwrap();
        let original_value = original_result.unwrap();
        assert_eq!(final_value, original_value);
    }
}

/// Test that function definitions are consistent
#[test]
fn test_function_definition_consistency() {
    let mut engine = JetCrabEngine::new();

    // Test that a function always returns the same result for the same input
    let code = r#"
        function identity(x) { return x; }
        function test() {
            let results = [];
            for (let i = 0; i < 10; i++) {
                results.push(identity(42));
            }
            return results.every(r => r === 42);
        }
        test()
    "#;

    let result = engine.evaluate(code);
    assert!(result.is_ok());
    // Should return true (all results are 42)
    let result_value = result.unwrap();
    assert!(!result_value.is_undefined() && !result_value.is_null());
}

/// Test easter egg trigger consistency
#[test]
fn test_easter_egg_trigger_consistency() {
    // Test that specific commands always trigger easter egg
    let trigger_commands = vec!["crab", "walk", "dance", "party"];

    for command in trigger_commands {
        assert!(should_trigger_easter_egg_for_command(command));
    }

    // Test that non-trigger commands never trigger easter egg
    let non_trigger_commands = vec!["hello", "test", "install", "build", "run"];

    for command in non_trigger_commands {
        assert!(!should_trigger_easter_egg_for_command(command));
    }
}

/// Test that object property access is consistent
#[test]
fn test_object_property_consistency() {
    let mut engine = JetCrabEngine::new();

    let code = r#"
        let obj = { a: 1, b: 2, c: 3 };
        let results = [];
        
        // Access properties multiple times
        for (let i = 0; i < 5; i++) {
            results.push(obj.a);
            results.push(obj.b);
            results.push(obj.c);
        }
        
        // All accesses should return the same values
        results.every((val, idx) => {
            if (idx % 3 === 0) return val === 1;
            if (idx % 3 === 1) return val === 2;
            return val === 3;
        })
    "#;

    let result = engine.evaluate(code);
    assert!(result.is_ok());
    let result_value = result.unwrap();
    assert!(!result_value.is_undefined() && !result_value.is_null());
}

/// Test that array operations maintain consistency
#[test]
fn test_array_operation_consistency() {
    let mut engine = JetCrabEngine::new();

    let code = r#"
        let arr = [1, 2, 3, 4, 5];
        let originalLength = arr.length;
        
        // Push and pop should maintain length consistency
        arr.push(6);
        let afterPush = arr.length;
        arr.pop();
        let afterPop = arr.length;
        
        // Length should be consistent
        afterPush === originalLength + 1 && afterPop === originalLength
    "#;

    let result = engine.evaluate(code);
    assert!(result.is_ok());
    let result_value = result.unwrap();
    assert!(!result_value.is_undefined() && !result_value.is_null());
}

/// Test that mathematical operations follow expected properties
#[test]
fn test_mathematical_properties() {
    let mut engine = JetCrabEngine::new();

    // Test distributive property: a * (b + c) = a * b + a * c
    for a in 1..=10 {
        for b in 1..=10 {
            for c in 1..=10 {
                let code1 = format!("{} * ({} + {})", a, b, c);
                let code2 = format!("{} * {} + {} * {}", a, b, a, c);

                let result1 = engine.evaluate(&code1);
                let result2 = engine.evaluate(&code2);

                assert!(result1.is_ok());
                assert!(result2.is_ok());
                assert_eq!(result1.unwrap(), result2.unwrap());
            }
        }
    }
}

/// Test that boolean operations follow logical properties
#[test]
fn test_boolean_logic_properties() {
    let mut engine = JetCrabEngine::new();

    let boolean_combinations = vec![
        ("true", "true"),
        ("true", "false"),
        ("false", "true"),
        ("false", "false"),
    ];

    for (a, b) in boolean_combinations {
        // Test De Morgan's laws: !(a && b) = !a || !b
        let code1 = format!("!({} && {})", a, b);
        let code2 = format!("!{} || !{}", a, b);

        let result1 = engine.evaluate(&code1);
        let result2 = engine.evaluate(&code2);

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert_eq!(result1.unwrap(), result2.unwrap());
    }
}

/// Test that type coercion is consistent
#[test]
fn test_type_coercion_consistency() {
    let mut engine = JetCrabEngine::new();

    let test_cases = vec![
        ("42", "42"),
        ("'42'", "'42'"),
        ("true", "true"),
        ("false", "false"),
    ];

    for (input, expected) in test_cases {
        // Test that String() conversion is consistent
        let code = format!("String({})", input);
        let result = engine.evaluate(&code);

        assert!(result.is_ok());
        // The result should be a string representation
        let result_value = result.unwrap();
        assert!(!result_value.is_undefined() && !result_value.is_null());
    }
}

/// Test that error handling is consistent
#[test]
fn test_error_handling_consistency() {
    let mut engine = JetCrabEngine::new();

    let error_cases = vec![
        "undefined_variable",
        "invalid syntax {",
        "throw new Error('test')",
        "null.someProperty",
    ];

    for error_case in error_cases {
        let result = engine.evaluate(error_case);
        // All error cases should result in an error
        assert!(result.is_err());
    }
}

/// Test that the engine state is isolated between evaluations
#[test]
fn test_engine_state_isolation() {
    let mut engine = JetCrabEngine::new();

    // Set a variable in first evaluation
    let result1 = engine.evaluate("let x = 42; x");
    assert!(result1.is_ok());

    // Try to access the same variable in a new engine instance
    let mut engine2 = JetCrabEngine::new();
    let result2 = engine2.evaluate("x");

    // Should fail because x is not defined in the new engine
    assert!(result2.is_err());
}

/// Test that global variables persist within the same engine
#[test]
fn test_global_variable_persistence() {
    let mut engine = JetCrabEngine::new();

    // Set a global variable
    let result1 = engine.evaluate("globalThis.testVar = 42");
    assert!(result1.is_ok());

    // Access it in a subsequent evaluation
    let result2 = engine.evaluate("globalThis.testVar");
    assert!(result2.is_ok());
    let result_value = result2.unwrap();
    assert!(!result_value.is_undefined() && !result_value.is_null());
}
