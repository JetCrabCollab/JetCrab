//! # Health Check System
//!
//! This module provides comprehensive health checking functionality for JetCrab APIs.

use crate::runtime::apis::{ApiError, ApiPlugin, ApiResult, HealthStatus, ResourceUsage};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};

/// Health checker for APIs
pub struct ApiHealthChecker {
    /// Health check results
    health_results: Arc<RwLock<HashMap<String, HealthCheckResult>>>,
    /// Health check configuration
    config: HealthCheckConfig,
    /// Health check history
    history: Arc<RwLock<Vec<HealthCheckHistory>>>,
    /// Health check intervals
    intervals: Arc<RwLock<HashMap<String, Duration>>>,
}

/// Health check result
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    /// API name
    pub api_name: String,
    /// Health status
    pub status: HealthStatus,
    /// Check timestamp
    pub timestamp: Instant,
    /// Response time
    pub response_time: Duration,
    /// Error message if any
    pub error_message: Option<String>,
    /// Additional details
    pub details: HashMap<String, String>,
}

/// Health check configuration
#[derive(Debug, Clone)]
pub struct HealthCheckConfig {
    /// Default health check interval
    pub default_interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Maximum number of consecutive failures before marking as unhealthy
    pub max_consecutive_failures: u32,
    /// Whether to enable automatic health checks
    pub enable_automatic_checks: bool,
    /// Whether to enable health check history
    pub enable_history: bool,
    /// Maximum history entries
    pub max_history_entries: usize,
}

/// Health check history entry
#[derive(Debug, Clone)]
pub struct HealthCheckHistory {
    /// API name
    pub api_name: String,
    /// Health status
    pub status: HealthStatus,
    /// Check timestamp
    pub timestamp: Instant,
    /// Response time
    pub response_time: Duration,
    /// Error message if any
    pub error_message: Option<String>,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            default_interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            max_consecutive_failures: 3,
            enable_automatic_checks: true,
            enable_history: true,
            max_history_entries: 1000,
        }
    }
}

impl ApiHealthChecker {
    /// Create a new health checker
    pub fn new(config: HealthCheckConfig) -> Self {
        Self {
            health_results: Arc::new(RwLock::new(HashMap::new())),
            config,
            history: Arc::new(RwLock::new(Vec::new())),
            intervals: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Perform health check for an API
    pub fn check_api_health(
        &self,
        api_name: &str,
        plugin: &dyn ApiPlugin,
    ) -> ApiResult<HealthCheckResult> {
        let start_time = Instant::now();

        debug!("Performing health check for API: {}", api_name);

        let status = plugin.health_check();
        let response_time = start_time.elapsed();

        let result = HealthCheckResult {
            api_name: api_name.to_string(),
            status: status.clone(),
            timestamp: start_time,
            response_time,
            error_message: None,
            details: HashMap::new(),
        };

        {
            let mut health_results = self.health_results.write().map_err(|_| {
                ApiError::configuration_error("Failed to acquire health_results lock")
            })?;

            health_results.insert(api_name.to_string(), result.clone());
        }

        if self.config.enable_history {
            self.add_to_history(&result)?;
        }

        debug!(
            "Health check completed for '{}': {:?} in {:?}",
            api_name, status, response_time
        );
        Ok(result)
    }

    /// Perform health check for multiple APIs
    pub fn check_multiple_apis(
        &self,
        apis: &HashMap<String, &dyn ApiPlugin>,
    ) -> ApiResult<Vec<HealthCheckResult>> {
        let mut results = Vec::new();
        let mut errors = Vec::new();

        for (api_name, plugin) in apis {
            match self.check_api_health(api_name, *plugin) {
                Ok(result) => results.push(result),
                Err(e) => {
                    let error_msg = format!("Failed to check health for '{}': {}", api_name, e);
                    errors.push(error_msg.clone());
                    error!("{}", error_msg);
                }
            }
        }

        if !errors.is_empty() && !self.config.enable_automatic_checks {
            return Err(ApiError::health_check_failed(
                "multiple_apis",
                &format!("{} health checks failed: {:?}", errors.len(), errors),
            ));
        }

        info!("Completed health checks for {} APIs", apis.len());
        Ok(results)
    }

    /// Get health status for an API
    pub fn get_api_health(&self, api_name: &str) -> Option<HealthCheckResult> {
        let health_results = self.health_results.read().ok()?;
        health_results.get(api_name).cloned()
    }

    /// Get all health results
    pub fn get_all_health_results(&self) -> HashMap<String, HealthCheckResult> {
        self.health_results
            .read()
            .unwrap_or_else(|_| panic!("Failed to acquire health_results lock"))
            .clone()
    }

    /// Get overall system health
    pub fn get_system_health(&self) -> SystemHealth {
        let health_results = self
            .health_results
            .read()
            .unwrap_or_else(|_| panic!("Failed to acquire health_results lock"));

        let mut healthy_count = 0;
        let mut degraded_count = 0;
        let mut unhealthy_count = 0;
        let mut unknown_count = 0;
        let mut total_apis = health_results.len();

        for result in health_results.values() {
            match result.status {
                HealthStatus::Healthy => healthy_count += 1,
                HealthStatus::Degraded => degraded_count += 1,
                HealthStatus::Unhealthy => unhealthy_count += 1,
                HealthStatus::Unknown => unknown_count += 1,
            }
        }

        let overall_status = if unhealthy_count > 0 {
            HealthStatus::Unhealthy
        } else if degraded_count > 0 {
            HealthStatus::Degraded
        } else if unknown_count > 0 {
            HealthStatus::Unknown
        } else {
            HealthStatus::Healthy
        };

        SystemHealth {
            overall_status,
            total_apis,
            healthy_count,
            degraded_count,
            unhealthy_count,
            unknown_count,
            health_percentage: if total_apis > 0 {
                (healthy_count as f64 / total_apis as f64) * 100.0
            } else {
                0.0
            },
        }
    }

    /// Get health check history
    pub fn get_health_history(&self) -> Vec<HealthCheckHistory> {
        self.history
            .read()
            .unwrap_or_else(|_| panic!("Failed to acquire history lock"))
            .clone()
    }

    /// Get health check history for a specific API
    pub fn get_api_health_history(&self, api_name: &str) -> Vec<HealthCheckHistory> {
        let history = self
            .history
            .read()
            .unwrap_or_else(|_| panic!("Failed to acquire history lock"));

        history
            .iter()
            .filter(|entry| entry.api_name == api_name)
            .cloned()
            .collect()
    }

    /// Set health check interval for an API
    pub fn set_api_interval(&self, api_name: &str, interval: Duration) -> ApiResult<()> {
        let mut intervals = self
            .intervals
            .write()
            .map_err(|_| ApiError::configuration_error("Failed to acquire intervals lock"))?;

        intervals.insert(api_name.to_string(), interval);
        debug!(
            "Set health check interval for '{}': {:?}",
            api_name, interval
        );
        Ok(())
    }

    /// Get health check interval for an API
    pub fn get_api_interval(&self, api_name: &str) -> Duration {
        let intervals = self
            .intervals
            .read()
            .unwrap_or_else(|_| panic!("Failed to acquire intervals lock"));

        intervals
            .get(api_name)
            .cloned()
            .unwrap_or(self.config.default_interval)
    }

    /// Clear health check history
    pub fn clear_history(&self) -> ApiResult<()> {
        let mut history = self
            .history
            .write()
            .map_err(|_| ApiError::configuration_error("Failed to acquire history lock"))?;

        history.clear();
        debug!("Cleared health check history");
        Ok(())
    }

    /// Update health check configuration
    pub fn update_config(&mut self, config: HealthCheckConfig) {
        self.config = config;
        debug!("Updated health check configuration: {:?}", self.config);
    }

    /// Get health check configuration
    pub fn get_config(&self) -> HealthCheckConfig {
        self.config.clone()
    }

    /// Check if an API needs a health check
    pub fn needs_health_check(&self, api_name: &str) -> bool {
        let health_results = self
            .health_results
            .read()
            .unwrap_or_else(|_| panic!("Failed to acquire health_results lock"));

        if let Some(result) = health_results.get(api_name) {
            let interval = self.get_api_interval(api_name);
            result.timestamp.elapsed() >= interval
        } else {
            true // Never checked before
        }
    }

    /// Get APIs that need health checks
    pub fn get_apis_needing_checks(&self, apis: &HashMap<String, &dyn ApiPlugin>) -> Vec<String> {
        apis.keys()
            .filter(|api_name| self.needs_health_check(api_name))
            .cloned()
            .collect()
    }

    /// Add result to history
    fn add_to_history(&self, result: &HealthCheckResult) -> ApiResult<()> {
        let mut history = self
            .history
            .write()
            .map_err(|_| ApiError::configuration_error("Failed to acquire history lock"))?;

        let history_entry = HealthCheckHistory {
            api_name: result.api_name.clone(),
            status: result.status.clone(),
            timestamp: result.timestamp,
            response_time: result.response_time,
            error_message: result.error_message.clone(),
        };

        history.push(history_entry);

        if history.len() > self.config.max_history_entries {
            history.remove(0);
        }

        Ok(())
    }
}

/// System health overview
#[derive(Debug, Clone)]
pub struct SystemHealth {
    /// Overall system health status
    pub overall_status: HealthStatus,
    /// Total number of APIs
    pub total_apis: usize,
    /// Number of healthy APIs
    pub healthy_count: usize,
    /// Number of degraded APIs
    pub degraded_count: usize,
    /// Number of unhealthy APIs
    pub unhealthy_count: usize,
    /// Number of APIs with unknown status
    pub unknown_count: usize,
    /// Health percentage (healthy / total)
    pub health_percentage: f64,
}
