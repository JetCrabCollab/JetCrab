//! # API Plugin System
//!
//! This module defines the plugin architecture for JetCrab APIs.

use chitin::boa_engine::{Context, JsResult};
use std::time::Duration;

/// Resource usage information for an API
#[derive(Debug, Default, Clone)]
pub struct ResourceUsage {
    /// Memory usage in megabytes
    pub memory_mb: u64,
    /// Number of file descriptors used
    pub file_descriptors: u32,
    /// Number of network connections
    pub network_connections: u32,
    /// CPU usage percentage (0-100)
    pub cpu_percentage: u8,
}

/// API metrics for monitoring
#[derive(Debug, Default, Clone)]
pub struct ApiMetrics {
    /// Time taken to register the API
    pub registration_time: Duration,
    /// Number of times the API was used
    pub usage_count: u64,
    /// Number of errors encountered
    pub error_count: u64,
    /// Last time the API was used
    pub last_used: Option<std::time::Instant>,
    /// Current resource usage
    pub resource_usage: ResourceUsage,
}

/// Health status of an API
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    /// API is healthy and functioning normally
    Healthy,
    /// API has minor issues but is still functional
    Degraded,
    /// API is not functioning properly
    Unhealthy,
    /// API status is unknown
    Unknown,
}

impl Default for HealthStatus {
    fn default() -> Self {
        HealthStatus::Unknown
    }
}

/// Trait that all JetCrab APIs must implement
pub trait ApiPlugin: Send + Sync {
    /// Get the name of this API
    fn name(&self) -> &'static str;

    /// Register this API in the given JavaScript context
    fn register(&self, context: &mut Context) -> JsResult<()>;

    /// Get the names of APIs this plugin depends on
    fn dependencies(&self) -> Vec<&'static str> {
        vec![]
    }

    /// Cleanup resources when the API is being unloaded
    fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Get current resource usage information
    fn resource_usage(&self) -> ResourceUsage {
        ResourceUsage::default()
    }

    /// Check the health status of this API
    fn health_check(&self) -> HealthStatus {
        HealthStatus::Unknown
    }

    /// Get metrics for this API
    fn metrics(&self) -> ApiMetrics {
        ApiMetrics::default()
    }

    /// Whether this API is critical for the runtime to function
    fn is_critical(&self) -> bool {
        false
    }

    /// Whether this API should be loaded by default
    fn is_enabled_by_default(&self) -> bool {
        true
    }

    /// Get the version of this API
    fn version(&self) -> &'static str {
        "0.4.0"
    }

    /// Get a description of what this API provides
    fn description(&self) -> &'static str {
        "No description available"
    }
}
