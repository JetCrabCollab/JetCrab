use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum CliError {
    #[error("Invalid argument: {argument} - {reason}")]
    InvalidArgument { argument: String, reason: String },

    #[error("Missing required argument: {argument}")]
    MissingArgument { argument: String },

    #[error("File not found: {path}")]
    FileNotFound { path: String },

    #[error("Permission denied: {operation} on {resource}")]
    PermissionDenied { operation: String, resource: String },

    #[error("Configuration error: {message}")]
    ConfigError { message: String },

    #[error("Network error: {message}")]
    NetworkError { message: String },

    #[error("Validation error: {field} - {reason}")]
    ValidationError { field: String, reason: String },

    #[error("Command execution failed: {command} - {reason}")]
    CommandExecutionFailed { command: String, reason: String },

    #[error("Timeout error: {operation} timed out after {timeout_ms}ms")]
    TimeoutError { operation: String, timeout_ms: u64 },

    #[error("Internal error: {message}")]
    InternalError { message: String },

    #[error("User cancelled operation")]
    UserCancelled,

    #[error("Unknown error: {message}")]
    Unknown { message: String },

    #[error("File already exists: {path}")]
    FileExists { path: String },

    #[error("File operation error: {operation} on {path} - {message}")]
    FileOperationError {
        operation: String,
        path: String,
        message: String,
    },

    #[error("Execution error: {command} - {message}")]
    ExecutionError { command: String, message: String },
}

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            CliError::InvalidArgument { .. } => 2,
            CliError::MissingArgument { .. } => 2,
            CliError::FileNotFound { .. } => 2,
            CliError::PermissionDenied { .. } => 13,
            CliError::ConfigError { .. } => 78,
            CliError::NetworkError { .. } => 6,
            CliError::ValidationError { .. } => 2,
            CliError::CommandExecutionFailed { .. } => 1,
            CliError::TimeoutError { .. } => 124,
            CliError::InternalError { .. } => 70,
            CliError::UserCancelled => 130,
            CliError::Unknown { .. } => 1,
            CliError::FileExists { .. } => 2,
            CliError::FileOperationError { .. } => 2,
            CliError::ExecutionError { .. } => 1,
        }
    }

    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            CliError::InternalError { .. } | CliError::Unknown { .. }
        )
    }

    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            CliError::NetworkError { .. } | CliError::TimeoutError { .. } | CliError::UserCancelled
        )
    }
}

pub type CliResult<T> = Result<T, CliError>;

impl From<Box<dyn std::error::Error>> for CliError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        CliError::InternalError {
            message: error.to_string(),
        }
    }
}

impl From<ctrlc::Error> for CliError {
    fn from(error: ctrlc::Error) -> Self {
        CliError::InternalError {
            message: format!("Signal handler error: {}", error),
        }
    }
}

pub trait ToCliError<T> {
    fn to_cli_error(self, context: &str) -> CliResult<T>;
}

impl<T, E> ToCliError<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn to_cli_error(self, context: &str) -> CliResult<T> {
        self.map_err(|e| CliError::InternalError {
            message: format!("{}: {}", context, e),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    pub command: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub additional_info: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    pub fn new(command: String) -> Self {
        Self {
            command,
            timestamp: chrono::Utc::now(),
            user_id: None,
            session_id: None,
            additional_info: std::collections::HashMap::new(),
        }
    }

    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_session_id(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn add_info(mut self, key: String, value: String) -> Self {
        self.additional_info.insert(key, value);
        self
    }
}
