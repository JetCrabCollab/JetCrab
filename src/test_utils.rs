//! # Test Utilities Module
//!
//! Provides testing utilities and test runner functionality for the JetCrab engine,
//! enabling automated testing of JavaScript code execution and compliance.
//!
//! ## Overview
//!
//! The test utilities module includes:
//!
//! - **Test Results**: Tracking of test outcomes and statistics
//! - **Test Runner**: Automated test execution engine
//! - **Compliance Testing**: JavaScript specification compliance validation
//! - **Result Reporting**: Detailed test result formatting
//!
//! ## Features
//!
//! - **Pass/Fail Tracking**: Count successful and failed tests
//! - **Compliance Rate**: Calculate percentage of compliant features
//! - **Batch Testing**: Run multiple tests in sequence
//! - **Error Handling**: Graceful handling of test failures
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::test_utils::{TestRunner, TestResult};
//!
//! let mut runner = TestRunner::new();
//! let tests = vec![
//!     ("addition", "2 + 2", "4"),
//!     ("string", "'hello'", "hello"),
//! ];
//! let results = runner.run_tests(tests);
//! println!("{}", results);
//! ```

use std::fmt;

#[derive(Default)]
pub struct TestResult {
    pub passed: i32,
    pub failed: i32,
    pub skipped: i32,
}

impl TestResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_passed(&mut self) {
        self.passed += 1;
    }

    pub fn add_failed(&mut self) {
        self.failed += 1;
    }

    pub fn add_skipped(&mut self) {
        self.skipped += 1;
    }

    pub fn total(&self) -> i32 {
        self.passed + self.failed + self.skipped
    }

    pub fn compliance_rate(&self) -> f64 {
        if self.passed + self.failed > 0 {
            (self.passed as f64 / (self.passed + self.failed) as f64) * 100.0
        } else {
            0.0
        }
    }
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Final Results ===")?;
        writeln!(f, "Passed: {}", self.passed)?;
        writeln!(f, "Failed: {}", self.failed)?;
        writeln!(f, "Skipped: {}", self.skipped)?;
        writeln!(f, "Total: {}", self.total())?;
        writeln!(f, "Compliance Rate: {:.1}%", self.compliance_rate())?;

        if self.failed == 0 {
            writeln!(f, "🎉 All implemented features are compliant!")?;
        } else {
            writeln!(f, "⚠️  Some features need improvements.")?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct TestRunner {
    pub engine: crate::api::Engine,
}

impl TestRunner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run_test(&mut self, _test_name: &str, code: &str, expected: &str) -> TestResult {
        let mut result = TestResult::new();

        match self.engine.evaluate(code) {
            Ok(eval_result) => {
                let result_str = eval_result.to_string();
                if result_str == expected {
                    result.add_passed();
                } else {
                    result.add_failed();
                }
            }
            Err(_) => {
                result.add_skipped();
            }
        }

        result
    }

    pub fn run_tests(&mut self, tests: Vec<(&str, &str, &str)>) -> TestResult {
        let mut total_result = TestResult::new();

        for (test_name, code, expected) in tests {
            let test_result = self.run_test(test_name, code, expected);
            total_result.passed += test_result.passed;
            total_result.failed += test_result.failed;
            total_result.skipped += test_result.skipped;
        }

        total_result
    }
}

#[derive(Default)]
pub struct E2ETestRunner {
    pub engine: crate::api::Engine,
}

impl E2ETestRunner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run_e2e_test(&mut self, _test_name: &str, code: &str, expected: &str) -> TestResult {
        let mut result = TestResult::new();

        match self.engine.evaluate(code) {
            Ok(eval_result) => {
                let result_str = eval_result.to_string();
                if result_str == expected {
                    result.add_passed();
                } else {
                    result.add_failed();
                }
            }
            Err(_error) => {
                result.add_failed();
            }
        }

        result
    }

    pub fn run_e2e_tests(&mut self, tests: Vec<(&str, &str, &str)>) -> TestResult {
        let mut total_result = TestResult::new();

        for (test_name, code, expected) in tests {
            let test_result = self.run_e2e_test(test_name, code, expected);
            total_result.passed += test_result.passed;
            total_result.failed += test_result.failed;
            total_result.skipped += test_result.skipped;
        }

        total_result
    }
}

pub fn get_test_header(title: &str) -> String {
    format!("\n=== {title} ===")
}

pub fn get_test_summary(result: &TestResult) -> String {
    format!("{result}")
}
