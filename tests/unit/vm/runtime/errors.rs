//! Runtime Errors Unit Tests
//! 
//! Tests for JavaScript runtime errors

use jetcrab::vm::runtime::errors::{RuntimeError, helpers};

#[test]
fn test_type_error_creation() {
    let error = RuntimeError::type_error("addition", "number", "string");
    assert!(error.message().contains("TypeError"));
    assert!(error.message().contains("addition"));
    assert!(error.message().contains("string"));
    assert!(error.message().contains("number"));
}

#[test]
fn test_reference_error_creation() {
    let error = RuntimeError::reference_error("undefinedVar", "global");
    assert!(error.message().contains("ReferenceError"));
    assert!(error.message().contains("undefinedVar"));
    assert!(error.message().contains("global"));
}

#[test]
fn test_range_error_creation() {
    let error = RuntimeError::range_error("1000", Some("0"), Some("100"));
    assert!(error.message().contains("RangeError"));
    assert!(error.message().contains("1000"));
    assert!(error.message().contains("0"));
    assert!(error.message().contains("100"));
}

#[test]
fn test_syntax_error_creation() {
    let error = RuntimeError::syntax_error("Unexpected token", Some(1), Some(5), Some(";"));
    assert!(error.message().contains("SyntaxError"));
    assert!(error.message().contains("Unexpected token"));
    assert!(error.message().contains("line 1"));
    assert!(error.message().contains("column 5"));
}

#[test]
fn test_generic_error_creation() {
    let error = RuntimeError::error("Something went wrong", Some("Internal error"));
    assert!(error.message().contains("Error"));
    assert!(error.message().contains("Something went wrong"));
    assert!(error.message().contains("Internal error"));
}

#[test]
fn test_error_type_names() {
    let type_error = RuntimeError::type_error("test", "number", "string");
    let ref_error = RuntimeError::reference_error("test", "global");
    let range_error = RuntimeError::range_error("100", Some("0"), Some("50"));
    let syntax_error = RuntimeError::syntax_error("test", None, None, None);
    let generic_error = RuntimeError::error("test", None);

    assert_eq!(type_error.error_type(), "TypeError");
    assert_eq!(ref_error.error_type(), "ReferenceError");
    assert_eq!(range_error.error_type(), "RangeError");
    assert_eq!(syntax_error.error_type(), "SyntaxError");
    assert_eq!(generic_error.error_type(), "Error");
}

#[test]
fn test_helper_functions() {
    let undefined_error = helpers::undefined_operation("addition");
    let null_error = helpers::null_operation("multiplication");
    let primitive_error = helpers::primitive_operation("property_access", "number");
    let var_error = helpers::undefined_variable("myVar");
    let prop_error = helpers::undefined_property("myObject", "myProperty");

    assert!(undefined_error.message().contains("undefined"));
    assert!(null_error.message().contains("null"));
    assert!(primitive_error.message().contains("number"));
    assert!(var_error.message().contains("myVar"));
    assert!(prop_error.message().contains("myProperty"));
}

#[test]
fn test_array_errors() {
    let bounds_error = helpers::array_index_out_of_bounds(10, 5);
    let length_error = helpers::invalid_array_length(4294967296);

    assert!(bounds_error.message().contains("10"));
    assert!(bounds_error.message().contains("4"));
    assert!(length_error.message().contains("4294967296"));
    assert!(length_error.message().contains("2^32-1"));
}

#[test]
fn test_error_conversion() {
    let runtime_error = RuntimeError::type_error("test", "number", "string");
    let vm_error = runtime_error.into_vm_error();
    
    match vm_error {
        jetcrab::vm::error::VmError::RuntimeError(_) => assert!(true),
        _ => assert!(false, "Expected RuntimeError variant"),
    }
}
