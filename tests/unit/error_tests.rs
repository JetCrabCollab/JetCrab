//! Unit tests for error types

use jetcrab::api::error::ApiError;
use jetcrab::ast::error::AstError;
use jetcrab::parser::error::ParserError;
use jetcrab::semantic::error::SemanticError;
use jetcrab::vm::compiler::error::BytecodeError;
use jetcrab::vm::error::VmError;

#[test]
fn test_api_error_creation() {
    let error = ApiError::CompilationError {
        message: "test error".to_string(),
        position: None,
    };
    assert!(matches!(error, ApiError::CompilationError { .. }));
}

#[test]
fn test_ast_error_creation() {
    let error = AstError::SerializationError {
        message: "test error".to_string(),
        position: None,
    };
    assert!(matches!(error, AstError::SerializationError { .. }));
}

#[test]
fn test_parser_error_creation() {
    use jetcrab::ast::Position;
    let error = ParserError::UnexpectedToken {
        token: "test".to_string(),
        position: Position::new(1, 1),
        expected: Some("expected".to_string()),
    };
    assert!(matches!(error, ParserError::UnexpectedToken { .. }));
}

#[test]
fn test_semantic_error_creation() {
    let error = SemanticError {
        message: "test error".to_string(),
        position: None,
    };
    assert_eq!(error.message, "test error");
}

#[test]
fn test_vm_error_creation() {
    let error = VmError::StackUnderflow {
        message: "test".to_string(),
        position: None,
    };
    assert!(matches!(error, VmError::StackUnderflow { .. }));
}

#[test]
fn test_bytecode_error_creation() {
    let error = BytecodeError::InvalidInstruction {
        instruction: "test".to_string(),
        message: "test message".to_string(),
        position: None,
    };
    assert!(matches!(error, BytecodeError::InvalidInstruction { .. }));
}
