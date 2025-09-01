use jetcrab::api::error::ApiError;

#[test]
fn test_api_error_creation() {
    let error = ApiError::ExecutionError {
        message: "Execution failed".to_string(),
        position: Some((10, 5)),
    };
    
    match error {
        ApiError::ExecutionError { message, position } => {
            assert_eq!(message, "Execution failed");
            assert_eq!(position, Some((10, 5)));
        }
        _ => panic!("Expected ExecutionError error"),
    }
}

#[test]
fn test_api_error_variants() {
    let execution_error = ApiError::ExecutionError {
        message: "Execution failed".to_string(),
        position: Some((1, 1)),
    };
    
    let compilation_error = ApiError::CompilationError {
        message: "Compilation failed".to_string(),
        position: Some((2, 1)),
    };
    
    let runtime_error = ApiError::RuntimeError {
        message: "Runtime error".to_string(),
        position: None,
    };
    
    match execution_error {
        ApiError::ExecutionError { message, .. } => {
            assert_eq!(message, "Execution failed");
        }
        _ => panic!("Expected ExecutionError error"),
    }
    
    match compilation_error {
        ApiError::CompilationError { message, .. } => {
            assert_eq!(message, "Compilation failed");
        }
        _ => panic!("Expected CompilationError error"),
    }
    
    match runtime_error {
        ApiError::RuntimeError { message, .. } => {
            assert_eq!(message, "Runtime error");
        }
        _ => panic!("Expected RuntimeError error"),
    }
}

#[test]
fn test_api_error_without_position() {
    let error = ApiError::RuntimeError {
        message: "No position error".to_string(),
        position: None,
    };
    
    match error {
        ApiError::RuntimeError { message, position } => {
            assert_eq!(message, "No position error");
            assert!(position.is_none());
        }
        _ => panic!("Expected RuntimeError error"),
    }
}

#[test]
fn test_api_error_display() {
    let error = ApiError::CompilationError {
        message: "Compilation failed".to_string(),
        position: Some((5, 10)),
    };
    
    let display_str = format!("{}", error);
    assert!(display_str.contains("CompilationError"));
    assert!(display_str.contains("Compilation failed"));
    assert!(display_str.contains("5:10"));
}

#[test]
fn test_api_error_debug() {
    let error = ApiError::ExecutionError {
        message: "Debug error".to_string(),
        position: Some((8, 12)),
    };
    
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("ExecutionError"));
    assert!(debug_str.contains("Debug error"));
    assert!(debug_str.contains("8:12"));
}
