//! # API Error Types
//!
//! This module defines custom error types for the JetCrab API system.

use std::fmt;
use thiserror::Error;

/// A simple error type that implements StdError
#[derive(Debug)]
pub struct SimpleError {
    message: String,
}

impl SimpleError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SimpleError {}

/// Errors that can occur in the API system
#[derive(Error, Debug)]
pub enum ApiError {
    /// Failed to register an API
    #[error("Failed to register API '{api}': {source}")]
    RegistrationFailed {
        /// Name of the API that failed to register
        api: String,
        /// The underlying error
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// API dependency is missing
    #[error("API dependency missing: '{dependency}' (required by '{api}')")]
    MissingDependency {
        /// Name of the missing dependency
        dependency: String,
        /// Name of the API that requires the dependency
        api: String,
    },

    /// Context initialization failed
    #[error("Context initialization failed: {source}")]
    ContextError {
        /// The underlying context error
        source: boa_engine::JsError,
    },

    /// Plugin not found
    #[error("Plugin not found: '{name}'")]
    PluginNotFound {
        /// Name of the plugin that was not found
        name: String,
    },

    /// Plugin already registered
    #[error("Plugin already registered: '{name}'")]
    PluginAlreadyRegistered {
        /// Name of the plugin that is already registered
        name: String,
    },

    /// Plugin factory not found
    #[error("Plugin factory not found: '{name}'")]
    FactoryNotFound {
        /// Name of the factory that was not found
        name: String,
    },

    /// Circular dependency detected
    #[error("Circular dependency detected in plugin chain: {chain}")]
    CircularDependency {
        /// The dependency chain that contains the circular reference
        chain: String,
    },

    /// Resource limit exceeded
    #[error("Resource limit exceeded: {resource_type} (limit: {limit}, current: {current})")]
    ResourceLimitExceeded {
        /// Type of resource that exceeded the limit
        resource_type: String,
        /// The limit that was exceeded
        limit: u64,
        /// Current usage
        current: u64,
    },

    /// Plugin health check failed
    #[error("Plugin health check failed: '{name}' - {reason}")]
    HealthCheckFailed {
        /// Name of the plugin that failed health check
        name: String,
        /// Reason for the health check failure
        reason: String,
    },

    /// Plugin cleanup failed
    #[error("Plugin cleanup failed: '{name}' - {source}")]
    CleanupFailed {
        /// Name of the plugin that failed to cleanup
        name: String,
        /// The underlying error
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Configuration error
    #[error("Configuration error: {message}")]
    ConfigurationError {
        /// Error message describing the configuration issue
        message: String,
    },

    /// Plugin version mismatch
    #[error("Plugin version mismatch: '{name}' (expected: {expected}, found: {found})")]
    VersionMismatch {
        /// Name of the plugin with version mismatch
        name: String,
        /// Expected version
        expected: String,
        /// Found version
        found: String,
    },

    /// Plugin initialization timeout
    #[error("Plugin initialization timeout: '{name}' (timeout: {timeout_ms}ms)")]
    InitializationTimeout {
        /// Name of the plugin that timed out
        name: String,
        /// Timeout duration in milliseconds
        timeout_ms: u64,
    },

    /// Batch registration failed
    #[error("Batch registration failed: {failed_count}/{total_count} plugins failed")]
    BatchRegistrationFailed {
        /// Number of plugins that failed to register
        failed_count: usize,
        /// Total number of plugins in the batch
        total_count: usize,
    },

    /// Plugin dependency resolution failed
    #[error("Plugin dependency resolution failed: '{name}' - {reason}")]
    DependencyResolutionFailed {
        /// Name of the plugin with dependency issues
        name: String,
        /// Reason for the resolution failure
        reason: String,
    },
}

/// Result type for API operations
pub type ApiResult<T> = Result<T, ApiError>;

/// Helper trait for converting errors to ApiError
pub trait ToApiError<T> {
    /// Convert the error to ApiError with context
    fn with_api_context(self, api: &str) -> ApiResult<T>;
}

impl<T, E> ToApiError<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn with_api_context(self, api: &str) -> ApiResult<T> {
        self.map_err(|e| ApiError::RegistrationFailed {
            api: api.to_string(),
            source: Box::new(e),
        })
    }
}

/// Helper functions for creating common API errors
impl ApiError {
    /// Create a registration failed error
    pub fn registration_failed(
        api: &str,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::RegistrationFailed {
            api: api.to_string(),
            source: Box::new(source),
        }
    }

    /// Create a missing dependency error
    pub fn missing_dependency(dependency: &str, api: &str) -> Self {
        Self::MissingDependency {
            dependency: dependency.to_string(),
            api: api.to_string(),
        }
    }

    /// Create a plugin not found error
    pub fn plugin_not_found(name: &str) -> Self {
        Self::PluginNotFound {
            name: name.to_string(),
        }
    }

    /// Create a plugin already registered error
    pub fn plugin_already_registered(name: &str) -> Self {
        Self::PluginAlreadyRegistered {
            name: name.to_string(),
        }
    }

    /// Create a factory not found error
    pub fn factory_not_found(name: &str) -> Self {
        Self::FactoryNotFound {
            name: name.to_string(),
        }
    }

    /// Create a circular dependency error
    pub fn circular_dependency(chain: &[String]) -> Self {
        Self::CircularDependency {
            chain: chain.join(" -> "),
        }
    }

    /// Create a resource limit exceeded error
    pub fn resource_limit_exceeded(resource_type: &str, limit: u64, current: u64) -> Self {
        Self::ResourceLimitExceeded {
            resource_type: resource_type.to_string(),
            limit,
            current,
        }
    }

    /// Create a health check failed error
    pub fn health_check_failed(name: &str, reason: &str) -> Self {
        Self::HealthCheckFailed {
            name: name.to_string(),
            reason: reason.to_string(),
        }
    }

    /// Create a cleanup failed error
    pub fn cleanup_failed(
        name: &str,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::CleanupFailed {
            name: name.to_string(),
            source: Box::new(source),
        }
    }

    /// Create a configuration error
    pub fn configuration_error(message: &str) -> Self {
        Self::ConfigurationError {
            message: message.to_string(),
        }
    }

    /// Create a version mismatch error
    pub fn version_mismatch(name: &str, expected: &str, found: &str) -> Self {
        Self::VersionMismatch {
            name: name.to_string(),
            expected: expected.to_string(),
            found: found.to_string(),
        }
    }

    /// Create an initialization timeout error
    pub fn initialization_timeout(name: &str, timeout_ms: u64) -> Self {
        Self::InitializationTimeout {
            name: name.to_string(),
            timeout_ms,
        }
    }

    /// Create a batch registration failed error
    pub fn batch_registration_failed(failed_count: usize, total_count: usize) -> Self {
        Self::BatchRegistrationFailed {
            failed_count,
            total_count,
        }
    }

    /// Create a dependency resolution failed error
    pub fn dependency_resolution_failed(name: &str, reason: &str) -> Self {
        Self::DependencyResolutionFailed {
            name: name.to_string(),
            reason: reason.to_string(),
        }
    }
}
