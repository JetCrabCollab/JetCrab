//! # Resource Manager
//!
//! This module provides resource management and cleanup mechanisms for JetCrab APIs.

use crate::runtime::apis::{ApiError, ApiPlugin, ApiResult, ResourceUsage};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};

/// Resource manager for API resources
pub struct ResourceManager {
    /// Current resource usage by API
    resource_usage: Arc<RwLock<HashMap<String, ResourceUsage>>>,
    /// Resource limits
    limits: ResourceLimits,
    /// Cleanup handlers
    cleanup_handlers: Arc<
        RwLock<
            HashMap<String, Box<dyn Fn() -> Result<(), Box<dyn std::error::Error>> + Send + Sync>>,
        >,
    >,
    /// Resource monitoring
    monitoring: Arc<RwLock<ResourceMonitoring>>,
}

/// Resource limits
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum memory usage in MB
    pub max_memory_mb: u64,
    /// Maximum number of file descriptors
    pub max_file_descriptors: u32,
    /// Maximum number of network connections
    pub max_network_connections: u32,
    /// Maximum CPU usage percentage
    pub max_cpu_percentage: u8,
    /// Maximum number of APIs
    pub max_apis: usize,
}

/// Resource monitoring data
#[derive(Debug, Default)]
pub struct ResourceMonitoring {
    /// Total memory usage
    pub total_memory_mb: u64,
    /// Total file descriptors
    pub total_file_descriptors: u32,
    /// Total network connections
    pub total_network_connections: u32,
    /// Total CPU usage
    pub total_cpu_percentage: u8,
    /// Number of active APIs
    pub active_apis: usize,
    /// Last update time
    pub last_update: Option<Instant>,
    /// Resource usage history
    pub usage_history: Vec<ResourceSnapshot>,
}

/// Resource usage snapshot
#[derive(Debug, Clone)]
pub struct ResourceSnapshot {
    /// Timestamp
    pub timestamp: Instant,
    /// Memory usage
    pub memory_mb: u64,
    /// File descriptors
    pub file_descriptors: u32,
    /// Network connections
    pub network_connections: u32,
    /// CPU usage
    pub cpu_percentage: u8,
    /// Number of APIs
    pub api_count: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 512,
            max_file_descriptors: 1000,
            max_network_connections: 100,
            max_cpu_percentage: 80,
            max_apis: 50,
        }
    }
}

impl ResourceManager {
    /// Create a new resource manager
    pub fn new(limits: ResourceLimits) -> Self {
        Self {
            resource_usage: Arc::new(RwLock::new(HashMap::new())),
            limits,
            cleanup_handlers: Arc::new(RwLock::new(HashMap::new())),
            monitoring: Arc::new(RwLock::new(ResourceMonitoring::default())),
        }
    }

    /// Register resource usage for an API
    pub fn register_usage(&self, api_name: &str, usage: ResourceUsage) -> ApiResult<()> {
        let mut resource_usage = self
            .resource_usage
            .write()
            .map_err(|_| ApiError::configuration_error("Failed to acquire resource_usage lock"))?;

        resource_usage.insert(api_name.to_string(), usage.clone());

        self.update_monitoring()?;

        self.check_limits()?;

        debug!(
            "Registered resource usage for API '{}': {:?}",
            api_name, usage
        );
        Ok(())
    }

    /// Update resource usage for an API
    pub fn update_usage(&self, api_name: &str, usage: ResourceUsage) -> ApiResult<()> {
        self.register_usage(api_name, usage)
    }

    /// Get resource usage for an API
    pub fn get_usage(&self, api_name: &str) -> Option<ResourceUsage> {
        let resource_usage = self.resource_usage.read().ok()?;
        resource_usage.get(api_name).cloned()
    }

    /// Get total resource usage
    pub fn get_total_usage(&self) -> ResourceUsage {
        let resource_usage = self
            .resource_usage
            .read()
            .unwrap_or_else(|_| panic!("Failed to acquire resource_usage lock"));

        let mut total = ResourceUsage::default();

        for usage in resource_usage.values() {
            total.memory_mb += usage.memory_mb;
            total.file_descriptors += usage.file_descriptors;
            total.network_connections += usage.network_connections;
            total.cpu_percentage = total.cpu_percentage.saturating_add(usage.cpu_percentage);
        }

        total
    }

    /// Register a cleanup handler for an API
    pub fn register_cleanup_handler<F>(&self, api_name: &str, handler: F) -> ApiResult<()>
    where
        F: Fn() -> Result<(), Box<dyn std::error::Error>> + Send + Sync + 'static,
    {
        let mut cleanup_handlers = self.cleanup_handlers.write().map_err(|_| {
            ApiError::configuration_error("Failed to acquire cleanup_handlers lock")
        })?;

        cleanup_handlers.insert(api_name.to_string(), Box::new(handler));
        debug!("Registered cleanup handler for API: {}", api_name);
        Ok(())
    }

    /// Cleanup resources for an API
    pub fn cleanup_api(&self, api_name: &str) -> ApiResult<()> {
        {
            let cleanup_handlers = self.cleanup_handlers.read().map_err(|_| {
                ApiError::configuration_error("Failed to acquire cleanup_handlers lock")
            })?;

            if let Some(handler) = cleanup_handlers.get(api_name) {
                handler().map_err(|e| {
                    ApiError::cleanup_failed(
                        api_name,
                        crate::runtime::apis::error::SimpleError::new(format!("{}", e)),
                    )
                })?;
                debug!("Executed cleanup handler for API: {}", api_name);
            }
        }

        {
            let mut resource_usage = self.resource_usage.write().map_err(|_| {
                ApiError::configuration_error("Failed to acquire resource_usage lock")
            })?;

            resource_usage.remove(api_name);
        }

        {
            let mut cleanup_handlers = self.cleanup_handlers.write().map_err(|_| {
                ApiError::configuration_error("Failed to acquire cleanup_handlers lock")
            })?;

            cleanup_handlers.remove(api_name);
        }

        self.update_monitoring()?;

        info!("Cleaned up resources for API: {}", api_name);
        Ok(())
    }

    /// Cleanup all resources
    pub fn cleanup_all(&self) -> ApiResult<()> {
        let mut errors = Vec::new();

        let api_names: Vec<String> = {
            let resource_usage = self.resource_usage.read().map_err(|_| {
                ApiError::configuration_error("Failed to acquire resource_usage lock")
            })?;
            resource_usage.keys().cloned().collect()
        };

        for api_name in api_names {
            if let Err(e) = self.cleanup_api(&api_name) {
                errors.push(format!("Failed to cleanup {}: {}", api_name, e));
            }
        }

        if !errors.is_empty() {
            warn!(
                "Resource cleanup completed with {} errors: {:?}",
                errors.len(),
                errors
            );
        } else {
            info!("Successfully cleaned up all resources");
        }

        Ok(())
    }

    /// Check if resource limits are exceeded
    pub fn check_limits(&self) -> ApiResult<()> {
        let total_usage = self.get_total_usage();
        let mut violations = Vec::new();

        if total_usage.memory_mb > self.limits.max_memory_mb {
            violations.push(format!(
                "Memory limit exceeded: {} MB > {} MB",
                total_usage.memory_mb, self.limits.max_memory_mb
            ));
        }

        if total_usage.file_descriptors > self.limits.max_file_descriptors {
            violations.push(format!(
                "File descriptor limit exceeded: {} > {}",
                total_usage.file_descriptors, self.limits.max_file_descriptors
            ));
        }

        if total_usage.network_connections > self.limits.max_network_connections {
            violations.push(format!(
                "Network connection limit exceeded: {} > {}",
                total_usage.network_connections, self.limits.max_network_connections
            ));
        }

        if total_usage.cpu_percentage > self.limits.max_cpu_percentage {
            violations.push(format!(
                "CPU usage limit exceeded: {}% > {}%",
                total_usage.cpu_percentage, self.limits.max_cpu_percentage
            ));
        }

        let api_count = {
            let resource_usage = self.resource_usage.read().map_err(|_| {
                ApiError::configuration_error("Failed to acquire resource_usage lock")
            })?;
            resource_usage.len()
        };

        if api_count > self.limits.max_apis {
            violations.push(format!(
                "API count limit exceeded: {} > {}",
                api_count, self.limits.max_apis
            ));
        }

        if !violations.is_empty() {
            return Err(ApiError::resource_limit_exceeded(
                "Multiple resources",
                violations.len() as u64,
                violations.len() as u64,
            ));
        }

        Ok(())
    }

    /// Get resource monitoring data
    pub fn get_monitoring(&self) -> ResourceMonitoring {
        self.monitoring
            .read()
            .unwrap_or_else(|_| panic!("Failed to acquire monitoring lock"))
            .clone()
    }

    /// Get resource limits
    pub fn get_limits(&self) -> ResourceLimits {
        self.limits.clone()
    }

    /// Update resource limits
    pub fn update_limits(&mut self, limits: ResourceLimits) {
        self.limits = limits;
        debug!("Updated resource limits: {:?}", self.limits);
    }

    /// Get resource usage history
    pub fn get_usage_history(&self) -> Vec<ResourceSnapshot> {
        let monitoring = self
            .monitoring
            .read()
            .unwrap_or_else(|_| panic!("Failed to acquire monitoring lock"));
        monitoring.usage_history.clone()
    }

    /// Clear usage history
    pub fn clear_usage_history(&self) -> ApiResult<()> {
        let mut monitoring = self
            .monitoring
            .write()
            .map_err(|_| ApiError::configuration_error("Failed to acquire monitoring lock"))?;

        monitoring.usage_history.clear();
        debug!("Cleared resource usage history");
        Ok(())
    }

    /// Update monitoring data
    fn update_monitoring(&self) -> ApiResult<()> {
        let total_usage = self.get_total_usage();
        let api_count = {
            let resource_usage = self.resource_usage.read().map_err(|_| {
                ApiError::configuration_error("Failed to acquire resource_usage lock")
            })?;
            resource_usage.len()
        };

        let mut monitoring = self
            .monitoring
            .write()
            .map_err(|_| ApiError::configuration_error("Failed to acquire monitoring lock"))?;

        monitoring.total_memory_mb = total_usage.memory_mb;
        monitoring.total_file_descriptors = total_usage.file_descriptors;
        monitoring.total_network_connections = total_usage.network_connections;
        monitoring.total_cpu_percentage = total_usage.cpu_percentage;
        monitoring.active_apis = api_count;
        monitoring.last_update = Some(Instant::now());

        let snapshot = ResourceSnapshot {
            timestamp: Instant::now(),
            memory_mb: total_usage.memory_mb,
            file_descriptors: total_usage.file_descriptors,
            network_connections: total_usage.network_connections,
            cpu_percentage: total_usage.cpu_percentage,
            api_count,
        };

        monitoring.usage_history.push(snapshot);

        if monitoring.usage_history.len() > 100 {
            monitoring.usage_history.remove(0);
        }

        Ok(())
    }
}

impl Clone for ResourceMonitoring {
    fn clone(&self) -> Self {
        Self {
            total_memory_mb: self.total_memory_mb,
            total_file_descriptors: self.total_file_descriptors,
            total_network_connections: self.total_network_connections,
            total_cpu_percentage: self.total_cpu_percentage,
            active_apis: self.active_apis,
            last_update: self.last_update,
            usage_history: self.usage_history.clone(),
        }
    }
}
