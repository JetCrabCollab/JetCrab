use jetcrab::api::error::ApiError;
use jetcrab::ast::Position;

#[test]
fn test_compilation_error_creation() {
    let error = ApiError::CompilationError {
        message: "Syntax error".to_string(),
        position: Some(Position::new(10, 5)),
    };
    assert!(matches!(error, ApiError::CompilationError { .. }));
}

#[test]
fn test_compilation_error_without_position() {
    let error = ApiError::CompilationError {
        message: "Syntax error".to_string(),
        position: None,
    };
    assert!(matches!(error, ApiError::CompilationError { .. }));
}

#[test]
fn test_execution_error_creation() {
    let error = ApiError::ExecutionError {
        message: "Runtime error".to_string(),
        position: Some(Position::new(15, 10)),
    };
    assert!(matches!(error, ApiError::ExecutionError { .. }));
}

#[test]
fn test_execution_error_without_position() {
    let error = ApiError::ExecutionError {
        message: "Runtime error".to_string(),
        position: None,
    };
    assert!(matches!(error, ApiError::ExecutionError { .. }));
}

#[test]
fn test_invalid_input_error_creation() {
    let error = ApiError::InvalidInput {
        message: "Invalid syntax".to_string(),
        input: "invalid code".to_string(),
        position: Some(Position::new(20, 3)),
    };
    assert!(matches!(error, ApiError::InvalidInput { .. }));
}

#[test]
fn test_invalid_input_error_without_position() {
    let error = ApiError::InvalidInput {
        message: "Invalid syntax".to_string(),
        input: "invalid code".to_string(),
        position: None,
    };
    assert!(matches!(error, ApiError::InvalidInput { .. }));
}

#[test]
fn test_engine_error_creation() {
    let error = ApiError::EngineError {
        message: "Engine failure".to_string(),
        position: Some(Position::new(5, 1)),
    };
    assert!(matches!(error, ApiError::EngineError { .. }));
}

#[test]
fn test_engine_error_without_position() {
    let error = ApiError::EngineError {
        message: "Engine failure".to_string(),
        position: None,
    };
    assert!(matches!(error, ApiError::EngineError { .. }));
}

#[test]
fn test_interpreter_error_creation() {
    let error = ApiError::InterpreterError {
        message: "Interpretation failed".to_string(),
        position: Some(Position::new(25, 8)),
    };
    assert!(matches!(error, ApiError::InterpreterError { .. }));
}

#[test]
fn test_interpreter_error_without_position() {
    let error = ApiError::InterpreterError {
        message: "Interpretation failed".to_string(),
        position: None,
    };
    assert!(matches!(error, ApiError::InterpreterError { .. }));
}

#[test]
fn test_configuration_error_creation() {
    let error = ApiError::ConfigurationError {
        message: "Invalid config".to_string(),
        position: Some(Position::new(1, 1)),
    };
    assert!(matches!(error, ApiError::ConfigurationError { .. }));
}

#[test]
fn test_configuration_error_without_position() {
    let error = ApiError::ConfigurationError {
        message: "Invalid config".to_string(),
        position: None,
    };
    assert!(matches!(error, ApiError::ConfigurationError { .. }));
}

#[test]
fn test_resource_error_creation() {
    let error = ApiError::ResourceError {
        resource: "file.js".to_string(),
        message: "File not found".to_string(),
        position: Some(Position::new(30, 12)),
    };
    assert!(matches!(error, ApiError::ResourceError { .. }));
}

#[test]
fn test_resource_error_without_position() {
    let error = ApiError::ResourceError {
        resource: "file.js".to_string(),
        message: "File not found".to_string(),
        position: None,
    };
    assert!(matches!(error, ApiError::ResourceError { .. }));
}

#[test]
fn test_timeout_error_creation() {
    let error = ApiError::TimeoutError {
        operation: "compilation".to_string(),
        timeout_ms: 5000,
        position: Some(Position::new(40, 20)),
    };
    assert!(matches!(error, ApiError::TimeoutError { .. }));
}

#[test]
fn test_timeout_error_without_position() {
    let error = ApiError::TimeoutError {
        operation: "compilation".to_string(),
        timeout_ms: 5000,
        position: None,
    };
    assert!(matches!(error, ApiError::TimeoutError { .. }));
}

#[test]
fn test_compilation_error_display_with_position() {
    let error = ApiError::CompilationError {
        message: "Syntax error".to_string(),
        position: Some(Position::new(10, 5)),
    };
    let display = format!("{}", error);
    assert!(display.contains("Compilation error: Syntax error"));
    assert!(display.contains("at line 10, column 5"));
}

#[test]
fn test_compilation_error_display_without_position() {
    let error = ApiError::CompilationError {
        message: "Syntax error".to_string(),
        position: None,
    };
    let display = format!("{}", error);
    assert_eq!(display, "Compilation error: Syntax error");
}

#[test]
fn test_execution_error_display_with_position() {
    let error = ApiError::ExecutionError {
        message: "Runtime error".to_string(),
        position: Some(Position::new(15, 10)),
    };
    let display = format!("{}", error);
    assert!(display.contains("Execution error: Runtime error"));
    assert!(display.contains("at line 15, column 10"));
}

#[test]
fn test_execution_error_display_without_position() {
    let error = ApiError::ExecutionError {
        message: "Runtime error".to_string(),
        position: None,
    };
    let display = format!("{}", error);
    assert_eq!(display, "Execution error: Runtime error");
}

#[test]
fn test_invalid_input_error_display_with_position() {
    let error = ApiError::InvalidInput {
        message: "Invalid syntax".to_string(),
        input: "invalid code".to_string(),
        position: Some(Position::new(20, 3)),
    };
    let display = format!("{}", error);
    assert!(display.contains("Invalid input 'invalid code': Invalid syntax"));
    assert!(display.contains("at line 20, column 3"));
}

#[test]
fn test_invalid_input_error_display_without_position() {
    let error = ApiError::InvalidInput {
        message: "Invalid syntax".to_string(),
        input: "invalid code".to_string(),
        position: None,
    };
    let display = format!("{}", error);
    assert_eq!(display, "Invalid input 'invalid code': Invalid syntax");
}

#[test]
fn test_engine_error_display_with_position() {
    let error = ApiError::EngineError {
        message: "Engine failure".to_string(),
        position: Some(Position::new(5, 1)),
    };
    let display = format!("{}", error);
    assert!(display.contains("Engine error: Engine failure"));
    assert!(display.contains("at line 5, column 1"));
}

#[test]
fn test_engine_error_display_without_position() {
    let error = ApiError::EngineError {
        message: "Engine failure".to_string(),
        position: None,
    };
    let display = format!("{}", error);
    assert_eq!(display, "Engine error: Engine failure");
}

#[test]
fn test_interpreter_error_display_with_position() {
    let error = ApiError::InterpreterError {
        message: "Interpretation failed".to_string(),
        position: Some(Position::new(25, 8)),
    };
    let display = format!("{}", error);
    assert!(display.contains("Interpreter error: Interpretation failed"));
    assert!(display.contains("at line 25, column 8"));
}

#[test]
fn test_interpreter_error_display_without_position() {
    let error = ApiError::InterpreterError {
        message: "Interpretation failed".to_string(),
        position: None,
    };
    let display = format!("{}", error);
    assert_eq!(display, "Interpreter error: Interpretation failed");
}

#[test]
fn test_configuration_error_display_with_position() {
    let error = ApiError::ConfigurationError {
        message: "Invalid config".to_string(),
        position: Some(Position::new(1, 1)),
    };
    let display = format!("{}", error);
    assert!(display.contains("Configuration error: Invalid config"));
    assert!(display.contains("at line 1, column 1"));
}

#[test]
fn test_configuration_error_display_without_position() {
    let error = ApiError::ConfigurationError {
        message: "Invalid config".to_string(),
        position: None,
    };
    let display = format!("{}", error);
    assert_eq!(display, "Configuration error: Invalid config");
}

#[test]
fn test_resource_error_display_with_position() {
    let error = ApiError::ResourceError {
        resource: "file.js".to_string(),
        message: "File not found".to_string(),
        position: Some(Position::new(30, 12)),
    };
    let display = format!("{}", error);
    assert!(display.contains("Resource 'file.js' error: File not found"));
    assert!(display.contains("at line 30, column 12"));
}

#[test]
fn test_resource_error_display_without_position() {
    let error = ApiError::ResourceError {
        resource: "file.js".to_string(),
        message: "File not found".to_string(),
        position: None,
    };
    let display = format!("{}", error);
    assert_eq!(display, "Resource 'file.js' error: File not found");
}

#[test]
fn test_timeout_error_display_with_position() {
    let error = ApiError::TimeoutError {
        operation: "compilation".to_string(),
        timeout_ms: 5000,
        position: Some(Position::new(40, 20)),
    };
    let display = format!("{}", error);
    assert!(display.contains("Operation 'compilation' timed out after 5000ms"));
    assert!(display.contains("at line 40, column 20"));
}

#[test]
fn test_timeout_error_display_without_position() {
    let error = ApiError::TimeoutError {
        operation: "compilation".to_string(),
        timeout_ms: 5000,
        position: None,
    };
    let display = format!("{}", error);
    assert_eq!(display, "Operation 'compilation' timed out after 5000ms");
}

#[test]
fn test_api_error_clone() {
    let error = ApiError::CompilationError {
        message: "Test error".to_string(),
        position: Some(Position::new(1, 1)),
    };
    let cloned = error.clone();
    assert!(matches!(cloned, ApiError::CompilationError { .. }));
}

#[test]
fn test_api_error_debug() {
    let error = ApiError::ExecutionError {
        message: "Test error".to_string(),
        position: None,
    };
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("ExecutionError"));
}

#[test]
fn test_api_error_serialization() {
    let error = ApiError::InvalidInput {
        message: "Test error".to_string(),
        input: "test input".to_string(),
        position: Some(Position::new(1, 1)),
    };
    let serialized = serde_json::to_string(&error).unwrap();
    assert!(serialized.contains("InvalidInput"));
    assert!(serialized.contains("Test error"));
    assert!(serialized.contains("test input"));
}

#[test]
fn test_api_error_deserialization() {
    let json = r#"{"CompilationError":{"message":"Test error","position":{"line":1,"column":1}}}"#;
    let error: ApiError = serde_json::from_str(json).unwrap();
    assert!(matches!(error, ApiError::CompilationError { .. }));
}

#[test]
fn test_api_error_implements_error_trait() {
    let error = ApiError::EngineError {
        message: "Test error".to_string(),
        position: None,
    };
    let error_ref: &dyn std::error::Error = &error;
    assert!(error_ref.source().is_none());
}
