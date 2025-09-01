//! Unit tests for VM runtime errors module
//!
//! This module tests the RuntimeError enum and all its associated methods,
//! including error creation, display formatting, and helper functions.

use jetcrab::vm::error::VmError;
use jetcrab::vm::runtime::errors::{helpers, RuntimeError};
use std::error::Error;

#[cfg(test)]
mod runtime_error_tests {
    use super::*;

    // Test basic error creation and variants
    #[test]
    fn test_type_error_creation() {
        let error = RuntimeError::TypeError {
            message: "TypeError: Cannot perform 'add' on type 'string', expected 'number'"
                .to_string(),
            operation: "add".to_string(),
            expected_type: "number".to_string(),
            actual_type: "string".to_string(),
        };

        assert!(matches!(error, RuntimeError::TypeError { .. }));
        if let RuntimeError::TypeError {
            message,
            operation,
            expected_type,
            actual_type,
        } = error
        {
            assert_eq!(
                message,
                "TypeError: Cannot perform 'add' on type 'string', expected 'number'"
            );
            assert_eq!(operation, "add");
            assert_eq!(expected_type, "number");
            assert_eq!(actual_type, "string");
        }
    }

    #[test]
    fn test_reference_error_creation() {
        let error = RuntimeError::ReferenceError {
            message: "ReferenceError: 'x' is not defined in scope 'global'".to_string(),
            identifier: "x".to_string(),
            scope: "global".to_string(),
        };

        assert!(matches!(error, RuntimeError::ReferenceError { .. }));
        if let RuntimeError::ReferenceError {
            message,
            identifier,
            scope,
        } = error
        {
            assert_eq!(
                message,
                "ReferenceError: 'x' is not defined in scope 'global'"
            );
            assert_eq!(identifier, "x");
            assert_eq!(scope, "global");
        }
    }

    #[test]
    fn test_range_error_creation() {
        let error = RuntimeError::RangeError {
            message: "RangeError: Value '5' is out of range [0, 3]".to_string(),
            value: "5".to_string(),
            min: Some("0".to_string()),
            max: Some("3".to_string()),
        };

        assert!(matches!(error, RuntimeError::RangeError { .. }));
        if let RuntimeError::RangeError {
            message,
            value,
            min,
            max,
        } = error
        {
            assert_eq!(message, "RangeError: Value '5' is out of range [0, 3]");
            assert_eq!(value, "5");
            assert_eq!(min, Some("0".to_string()));
            assert_eq!(max, Some("3".to_string()));
        }
    }

    #[test]
    fn test_syntax_error_creation() {
        let error = RuntimeError::SyntaxError {
            message: "SyntaxError: Unexpected token '}' at line 5, column 10 (token: '}')"
                .to_string(),
            line: Some(5),
            column: Some(10),
            token: Some("}".to_string()),
        };

        assert!(matches!(error, RuntimeError::SyntaxError { .. }));
        if let RuntimeError::SyntaxError {
            message,
            line,
            column,
            token,
        } = error
        {
            assert_eq!(
                message,
                "SyntaxError: Unexpected token '}' at line 5, column 10 (token: '}')"
            );
            assert_eq!(line, Some(5));
            assert_eq!(column, Some(10));
            assert_eq!(token, Some("}".to_string()));
        }
    }

    #[test]
    fn test_generic_error_creation() {
        let error = RuntimeError::Error {
            message: "Error: Something went wrong (caused by: network timeout)".to_string(),
            cause: Some("network timeout".to_string()),
        };

        assert!(matches!(error, RuntimeError::Error { .. }));
        if let RuntimeError::Error { message, cause } = error {
            assert_eq!(
                message,
                "Error: Something went wrong (caused by: network timeout)"
            );
            assert_eq!(cause, Some("network timeout".to_string()));
        }
    }

    // Test constructor methods
    #[test]
    fn test_type_error_constructor() {
        let error = RuntimeError::type_error("add", "number", "string");

        assert!(matches!(error, RuntimeError::TypeError { .. }));
        if let RuntimeError::TypeError {
            message,
            operation,
            expected_type,
            actual_type,
        } = error
        {
            assert_eq!(
                message,
                "TypeError: Cannot perform 'add' on type 'string', expected 'number'"
            );
            assert_eq!(operation, "add");
            assert_eq!(expected_type, "number");
            assert_eq!(actual_type, "string");
        }
    }

    #[test]
    fn test_reference_error_constructor() {
        let error = RuntimeError::reference_error("x", "global");

        assert!(matches!(error, RuntimeError::ReferenceError { .. }));
        if let RuntimeError::ReferenceError {
            message,
            identifier,
            scope,
        } = error
        {
            assert_eq!(
                message,
                "ReferenceError: 'x' is not defined in scope 'global'"
            );
            assert_eq!(identifier, "x");
            assert_eq!(scope, "global");
        }
    }

    #[test]
    fn test_range_error_constructor_with_min_max() {
        let error = RuntimeError::range_error("5", Some("0"), Some("3"));

        assert!(matches!(error, RuntimeError::RangeError { .. }));
        if let RuntimeError::RangeError {
            message,
            value,
            min,
            max,
        } = error
        {
            assert_eq!(message, "RangeError: Value '5' is out of range [0, 3]");
            assert_eq!(value, "5");
            assert_eq!(min, Some("0".to_string()));
            assert_eq!(max, Some("3".to_string()));
        }
    }

    #[test]
    fn test_range_error_constructor_with_min_only() {
        let error = RuntimeError::range_error("5", Some("10"), None);

        assert!(matches!(error, RuntimeError::RangeError { .. }));
        if let RuntimeError::RangeError {
            message,
            value,
            min,
            max,
        } = error
        {
            assert_eq!(message, "RangeError: Value '5' is below minimum '10'");
            assert_eq!(value, "5");
            assert_eq!(min, Some("10".to_string()));
            assert_eq!(max, None);
        }
    }

    #[test]
    fn test_range_error_constructor_with_max_only() {
        let error = RuntimeError::range_error("5", None, Some("3"));

        assert!(matches!(error, RuntimeError::RangeError { .. }));
        if let RuntimeError::RangeError {
            message,
            value,
            min,
            max,
        } = error
        {
            assert_eq!(message, "RangeError: Value '5' is above maximum '3'");
            assert_eq!(value, "5");
            assert_eq!(min, None);
            assert_eq!(max, Some("3".to_string()));
        }
    }

    #[test]
    fn test_range_error_constructor_without_bounds() {
        let error = RuntimeError::range_error("5", None, None);

        assert!(matches!(error, RuntimeError::RangeError { .. }));
        if let RuntimeError::RangeError {
            message,
            value,
            min,
            max,
        } = error
        {
            assert_eq!(message, "RangeError: Value '5' is out of valid range");
            assert_eq!(value, "5");
            assert_eq!(min, None);
            assert_eq!(max, None);
        }
    }

    #[test]
    fn test_syntax_error_constructor_with_all_info() {
        let error = RuntimeError::syntax_error("Unexpected token", Some(5), Some(10), Some("}"));

        assert!(matches!(error, RuntimeError::SyntaxError { .. }));
        if let RuntimeError::SyntaxError {
            message,
            line,
            column,
            token,
        } = error
        {
            assert_eq!(
                message,
                "SyntaxError: Unexpected token at line 5, column 10 (token: '}')"
            );
            assert_eq!(line, Some(5));
            assert_eq!(column, Some(10));
            assert_eq!(token, Some("}".to_string()));
        }
    }

    #[test]
    fn test_syntax_error_constructor_with_line_column() {
        let error = RuntimeError::syntax_error("Unexpected token", Some(5), Some(10), None);

        assert!(matches!(error, RuntimeError::SyntaxError { .. }));
        if let RuntimeError::SyntaxError {
            message,
            line,
            column,
            token,
        } = error
        {
            assert_eq!(
                message,
                "SyntaxError: Unexpected token at line 5, column 10"
            );
            assert_eq!(line, Some(5));
            assert_eq!(column, Some(10));
            assert_eq!(token, None);
        }
    }

    #[test]
    fn test_syntax_error_constructor_with_line_only() {
        let error = RuntimeError::syntax_error("Unexpected token", Some(5), None, None);

        assert!(matches!(error, RuntimeError::SyntaxError { .. }));
        if let RuntimeError::SyntaxError {
            message,
            line,
            column,
            token,
        } = error
        {
            assert_eq!(message, "SyntaxError: Unexpected token at line 5");
            assert_eq!(line, Some(5));
            assert_eq!(column, None);
            assert_eq!(token, None);
        }
    }

    #[test]
    fn test_syntax_error_constructor_with_message_only() {
        let error = RuntimeError::syntax_error("Unexpected token", None, None, None);

        assert!(matches!(error, RuntimeError::SyntaxError { .. }));
        if let RuntimeError::SyntaxError {
            message,
            line,
            column,
            token,
        } = error
        {
            assert_eq!(message, "SyntaxError: Unexpected token");
            assert_eq!(line, None);
            assert_eq!(column, None);
            assert_eq!(token, None);
        }
    }

    #[test]
    fn test_generic_error_constructor_with_cause() {
        let error = RuntimeError::error("Something went wrong", Some("network timeout"));

        assert!(matches!(error, RuntimeError::Error { .. }));
        if let RuntimeError::Error { message, cause } = error {
            assert_eq!(
                message,
                "Error: Something went wrong (caused by: network timeout)"
            );
            assert_eq!(cause, Some("network timeout".to_string()));
        }
    }

    #[test]
    fn test_generic_error_constructor_without_cause() {
        let error = RuntimeError::error("Something went wrong", None);

        assert!(matches!(error, RuntimeError::Error { .. }));
        if let RuntimeError::Error { message, cause } = error {
            assert_eq!(message, "Error: Something went wrong");
            assert_eq!(cause, None);
        }
    }

    // Test message method
    #[test]
    fn test_message_type_error() {
        let error = RuntimeError::type_error("add", "number", "string");
        assert_eq!(
            error.message(),
            "TypeError: Cannot perform 'add' on type 'string', expected 'number'"
        );
    }

    #[test]
    fn test_message_reference_error() {
        let error = RuntimeError::reference_error("x", "global");
        assert_eq!(
            error.message(),
            "ReferenceError: 'x' is not defined in scope 'global'"
        );
    }

    #[test]
    fn test_message_range_error() {
        let error = RuntimeError::range_error("5", Some("0"), Some("3"));
        assert_eq!(
            error.message(),
            "RangeError: Value '5' is out of range [0, 3]"
        );
    }

    #[test]
    fn test_message_syntax_error() {
        let error = RuntimeError::syntax_error("Unexpected token", Some(5), Some(10), Some("}"));
        assert_eq!(
            error.message(),
            "SyntaxError: Unexpected token at line 5, column 10 (token: '}')"
        );
    }

    #[test]
    fn test_message_generic_error() {
        let error = RuntimeError::error("Something went wrong", Some("network timeout"));
        assert_eq!(
            error.message(),
            "Error: Something went wrong (caused by: network timeout)"
        );
    }

    // Test error_type method
    #[test]
    fn test_error_type_type_error() {
        let error = RuntimeError::type_error("add", "number", "string");
        assert_eq!(error.error_type(), "TypeError");
    }

    #[test]
    fn test_error_type_reference_error() {
        let error = RuntimeError::reference_error("x", "global");
        assert_eq!(error.error_type(), "ReferenceError");
    }

    #[test]
    fn test_error_type_range_error() {
        let error = RuntimeError::range_error("5", Some("0"), Some("3"));
        assert_eq!(error.error_type(), "RangeError");
    }

    #[test]
    fn test_error_type_syntax_error() {
        let error = RuntimeError::syntax_error("Unexpected token", Some(5), Some(10), Some("}"));
        assert_eq!(error.error_type(), "SyntaxError");
    }

    #[test]
    fn test_error_type_generic_error() {
        let error = RuntimeError::error("Something went wrong", Some("network timeout"));
        assert_eq!(error.error_type(), "Error");
    }

    // Test into_vm_error method
    #[test]
    fn test_into_vm_error_type_error() {
        let error = RuntimeError::type_error("add", "number", "string");
        let vm_error = error.into_vm_error();

        assert!(matches!(vm_error, VmError::RuntimeError { .. }));
        if let VmError::RuntimeError { message, position } = vm_error {
            assert_eq!(
                message,
                "TypeError: Cannot perform 'add' on type 'string', expected 'number'"
            );
            assert_eq!(position, None);
        }
    }

    #[test]
    fn test_into_vm_error_reference_error() {
        let error = RuntimeError::reference_error("x", "global");
        let vm_error = error.into_vm_error();

        assert!(matches!(vm_error, VmError::RuntimeError { .. }));
        if let VmError::RuntimeError { message, position } = vm_error {
            assert_eq!(
                message,
                "ReferenceError: 'x' is not defined in scope 'global'"
            );
            assert_eq!(position, None);
        }
    }

    // Test Display implementation
    #[test]
    fn test_display_type_error() {
        let error = RuntimeError::type_error("add", "number", "string");
        assert_eq!(
            format!("{}", error),
            "TypeError: Cannot perform 'add' on type 'string', expected 'number'"
        );
    }

    #[test]
    fn test_display_reference_error() {
        let error = RuntimeError::reference_error("x", "global");
        assert_eq!(
            format!("{}", error),
            "ReferenceError: 'x' is not defined in scope 'global'"
        );
    }

    #[test]
    fn test_display_range_error() {
        let error = RuntimeError::range_error("5", Some("0"), Some("3"));
        assert_eq!(
            format!("{}", error),
            "RangeError: Value '5' is out of range [0, 3]"
        );
    }

    #[test]
    fn test_display_syntax_error() {
        let error = RuntimeError::syntax_error("Unexpected token", Some(5), Some(10), Some("}"));
        assert_eq!(
            format!("{}", error),
            "SyntaxError: Unexpected token at line 5, column 10 (token: '}')"
        );
    }

    #[test]
    fn test_display_generic_error() {
        let error = RuntimeError::error("Something went wrong", Some("network timeout"));
        assert_eq!(
            format!("{}", error),
            "Error: Something went wrong (caused by: network timeout)"
        );
    }

    // Test Error trait implementation
    #[test]
    fn test_error_trait_implementation() {
        let error = RuntimeError::type_error("add", "number", "string");
        assert!(error.source().is_none());
    }

    // Test From trait implementation
    #[test]
    fn test_from_runtime_error_to_vm_error() {
        let runtime_error = RuntimeError::type_error("add", "number", "string");
        let vm_error: VmError = runtime_error.into();

        assert!(matches!(vm_error, VmError::RuntimeError { .. }));
        if let VmError::RuntimeError { message, position } = vm_error {
            assert_eq!(
                message,
                "TypeError: Cannot perform 'add' on type 'string', expected 'number'"
            );
            assert_eq!(position, None);
        }
    }

    // Test Clone trait
    #[test]
    fn test_clone_type_error() {
        let error = RuntimeError::type_error("add", "number", "string");
        let cloned_error = error.clone();

        assert!(matches!(cloned_error, RuntimeError::TypeError { .. }));
        if let RuntimeError::TypeError {
            message,
            operation,
            expected_type,
            actual_type,
        } = cloned_error
        {
            assert_eq!(
                message,
                "TypeError: Cannot perform 'add' on type 'string', expected 'number'"
            );
            assert_eq!(operation, "add");
            assert_eq!(expected_type, "number");
            assert_eq!(actual_type, "string");
        }
    }

    #[test]
    fn test_clone_reference_error() {
        let error = RuntimeError::reference_error("x", "global");
        let cloned_error = error.clone();

        assert!(matches!(cloned_error, RuntimeError::ReferenceError { .. }));
        if let RuntimeError::ReferenceError {
            message,
            identifier,
            scope,
        } = cloned_error
        {
            assert_eq!(
                message,
                "ReferenceError: 'x' is not defined in scope 'global'"
            );
            assert_eq!(identifier, "x");
            assert_eq!(scope, "global");
        }
    }

    // Test Debug trait
    #[test]
    fn test_debug_formatting() {
        let error = RuntimeError::type_error("add", "number", "string");
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("TypeError"));
        assert!(debug_str.contains("add"));
        assert!(debug_str.contains("number"));
        assert!(debug_str.contains("string"));
    }

    // Test PartialEq and Eq traits
    #[test]
    fn test_partial_eq_same_errors() {
        let error1 = RuntimeError::type_error("add", "number", "string");
        let error2 = RuntimeError::type_error("add", "number", "string");
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_partial_eq_different_errors() {
        let error1 = RuntimeError::type_error("add", "number", "string");
        let error2 = RuntimeError::type_error("subtract", "number", "string");
        assert_ne!(error1, error2);
    }

    #[test]
    fn test_partial_eq_different_types() {
        let error1 = RuntimeError::type_error("add", "number", "string");
        let error2 = RuntimeError::reference_error("x", "global");
        assert_ne!(error1, error2);
    }

    // Test helper functions
    #[test]
    fn test_undefined_operation_helper() {
        let error = helpers::undefined_operation("add");

        assert!(matches!(error, RuntimeError::TypeError { .. }));
        if let RuntimeError::TypeError {
            message,
            operation,
            expected_type,
            actual_type,
        } = error
        {
            assert_eq!(
                message,
                "TypeError: Cannot perform 'add' on type 'undefined', expected 'defined value'"
            );
            assert_eq!(operation, "add");
            assert_eq!(expected_type, "defined value");
            assert_eq!(actual_type, "undefined");
        }
    }

    #[test]
    fn test_null_operation_helper() {
        let error = helpers::null_operation("add");

        assert!(matches!(error, RuntimeError::TypeError { .. }));
        if let RuntimeError::TypeError {
            message,
            operation,
            expected_type,
            actual_type,
        } = error
        {
            assert_eq!(
                message,
                "TypeError: Cannot perform 'add' on type 'null', expected 'defined value'"
            );
            assert_eq!(operation, "add");
            assert_eq!(expected_type, "defined value");
            assert_eq!(actual_type, "null");
        }
    }

    #[test]
    fn test_primitive_operation_helper() {
        let error = helpers::primitive_operation("add", "string");

        assert!(matches!(error, RuntimeError::TypeError { .. }));
        if let RuntimeError::TypeError {
            message,
            operation,
            expected_type,
            actual_type,
        } = error
        {
            assert_eq!(
                message,
                "TypeError: Cannot perform 'add' on type 'string', expected 'object'"
            );
            assert_eq!(operation, "add");
            assert_eq!(expected_type, "object");
            assert_eq!(actual_type, "string");
        }
    }

    #[test]
    fn test_undefined_variable_helper() {
        let error = helpers::undefined_variable("x");

        assert!(matches!(error, RuntimeError::ReferenceError { .. }));
        if let RuntimeError::ReferenceError {
            message,
            identifier,
            scope,
        } = error
        {
            assert_eq!(
                message,
                "ReferenceError: 'x' is not defined in scope 'global'"
            );
            assert_eq!(identifier, "x");
            assert_eq!(scope, "global");
        }
    }

    #[test]
    fn test_undefined_property_helper() {
        let error = helpers::undefined_property("obj", "prop");

        assert!(matches!(error, RuntimeError::ReferenceError { .. }));
        if let RuntimeError::ReferenceError {
            message,
            identifier,
            scope,
        } = error
        {
            assert_eq!(
                message,
                "ReferenceError: 'prop' is not defined in scope 'object 'obj''"
            );
            assert_eq!(identifier, "prop");
            assert_eq!(scope, "object 'obj'");
        }
    }

    #[test]
    fn test_array_index_out_of_bounds_helper() {
        let error = helpers::array_index_out_of_bounds(5, 3);

        assert!(matches!(error, RuntimeError::RangeError { .. }));
        if let RuntimeError::RangeError {
            message,
            value,
            min,
            max,
        } = error
        {
            assert_eq!(message, "RangeError: Value '5' is out of range [0, 2]");
            assert_eq!(value, "5");
            assert_eq!(min, Some("0".to_string()));
            assert_eq!(max, Some("2".to_string()));
        }
    }

    #[test]
    fn test_invalid_array_length_helper() {
        let error = helpers::invalid_array_length(5);

        assert!(matches!(error, RuntimeError::RangeError { .. }));
        if let RuntimeError::RangeError {
            message,
            value,
            min,
            max,
        } = error
        {
            assert_eq!(message, "RangeError: Value '5' is out of range [0, 2^32-1]");
            assert_eq!(value, "5");
            assert_eq!(min, Some("0".to_string()));
            assert_eq!(max, Some("2^32-1".to_string()));
        }
    }

    // Test error matching patterns
    #[test]
    fn test_error_matching_patterns() {
        let errors = vec![
            RuntimeError::type_error("add", "number", "string"),
            RuntimeError::reference_error("x", "global"),
            RuntimeError::range_error("5", Some("0"), Some("3")),
            RuntimeError::syntax_error("Unexpected token", Some(5), Some(10), Some("}")),
            RuntimeError::error("Something went wrong", Some("network timeout")),
        ];

        for error in errors {
            match error {
                RuntimeError::TypeError { .. } => assert!(true),
                RuntimeError::ReferenceError { .. } => assert!(true),
                RuntimeError::RangeError { .. } => assert!(true),
                RuntimeError::SyntaxError { .. } => assert!(true),
                RuntimeError::Error { .. } => assert!(true),
            }
        }
    }

    // Test comprehensive error scenarios
    #[test]
    fn test_comprehensive_error_scenarios() {
        // Test all error types with various parameters
        let type_error = RuntimeError::type_error("multiply", "number", "boolean");
        assert_eq!(type_error.error_type(), "TypeError");
        assert!(type_error.message().contains("multiply"));
        assert!(type_error.message().contains("boolean"));
        assert!(type_error.message().contains("number"));

        let reference_error = RuntimeError::reference_error("undefinedVar", "function scope");
        assert_eq!(reference_error.error_type(), "ReferenceError");
        assert!(reference_error.message().contains("undefinedVar"));
        assert!(reference_error.message().contains("function scope"));

        let range_error = RuntimeError::range_error("100", Some("0"), Some("50"));
        assert_eq!(range_error.error_type(), "RangeError");
        assert!(range_error.message().contains("100"));
        assert!(range_error.message().contains("0"));
        assert!(range_error.message().contains("50"));

        let syntax_error = RuntimeError::syntax_error("Missing semicolon", Some(10), Some(5), None);
        assert_eq!(syntax_error.error_type(), "SyntaxError");
        assert!(syntax_error.message().contains("Missing semicolon"));
        assert!(syntax_error.message().contains("line 10"));
        assert!(syntax_error.message().contains("column 5"));

        let generic_error = RuntimeError::error("Network failure", Some("connection timeout"));
        assert_eq!(generic_error.error_type(), "Error");
        assert!(generic_error.message().contains("Network failure"));
        assert!(generic_error.message().contains("connection timeout"));
    }

    // Test edge cases
    #[test]
    fn test_edge_cases() {
        // Test with empty strings
        let error = RuntimeError::type_error("", "", "");
        assert!(error.message().contains("Cannot perform '' on type ''"));

        // Test with very long strings
        let long_string = "a".repeat(1000);
        let error = RuntimeError::reference_error(&long_string, "global");
        assert!(error.message().contains(&long_string));

        // Test with maximum values
        let error =
            RuntimeError::syntax_error("Error", Some(u32::MAX), Some(u32::MAX), Some("token"));
        assert!(error.message().contains(&u32::MAX.to_string()));

        // Test with zero values
        let error = RuntimeError::range_error("0", Some("0"), Some("0"));
        assert_eq!(
            error.message(),
            "RangeError: Value '0' is out of range [0, 0]"
        );
    }
}
