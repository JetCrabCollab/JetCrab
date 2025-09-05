//! # API Metrics System
//!
//! This module provides comprehensive metrics collection and monitoring for JetCrab APIs.

use crate::runtime::apis::{
    ApiError, ApiMetrics, ApiPlugin, ApiResult, HealthStatus, ResourceUsage,
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

/// Metrics collector for APIs
pub struct ApiMetricsCollector {
    /// Metrics storage
    metrics: Arc<RwLock<HashMap<String, ApiMetrics>>>,
    /// Performance metrics
    performance: Arc<RwLock<PerformanceMetrics>>,
    /// Health metrics
    health: Arc<RwLock<HealthMetrics>>,
    /// Resource metrics
    resources: Arc<RwLock<ResourceMetrics>>,
    /// Metrics configuration
    config: MetricsConfig,
}

/// Performance metrics
#[derive(Debug, Default)]
pub struct PerformanceMetrics {
    /// Total API calls
    pub total_calls: u64,
    /// Successful API calls
    pub successful_calls: u64,
    /// Failed API calls
    pub failed_calls: u64,
    /// Average response time
    pub average_response_time: Duration,
    /// Total response time
    pub total_response_time: Duration,
    /// Peak response time
    pub peak_response_time: Duration,
    /// Calls per second
    pub calls_per_second: f64,
    /// Last call time
    pub last_call_time: Option<Instant>,
}

/// Health metrics
#[derive(Debug, Default)]
pub struct HealthMetrics {
    /// Total health checks
    pub total_health_checks: u64,
    /// Healthy checks
    pub healthy_checks: u64,
    /// Degraded checks
    pub degraded_checks: u64,
    /// Unhealthy checks
    pub unhealthy_checks: u64,
    /// Unknown checks
    pub unknown_checks: u64,
    /// Current health status
    pub current_status: HealthStatus,
    /// Last health check time
    pub last_health_check: Option<Instant>,
    /// Health check history
    pub health_history: Vec<HealthSnapshot>,
}

/// Resource metrics
#[derive(Debug, Default)]
pub struct ResourceMetrics {
    /// Peak memory usage
    pub peak_memory_mb: u64,
    /// Current memory usage
    pub current_memory_mb: u64,
    /// Peak file descriptors
    pub peak_file_descriptors: u32,
    /// Current file descriptors
    pub current_file_descriptors: u32,
    /// Peak network connections
    pub peak_network_connections: u32,
    /// Current network connections
    pub current_network_connections: u32,
    /// Peak CPU usage
    pub peak_cpu_percentage: u8,
    /// Current CPU usage
    pub current_cpu_percentage: u8,
    /// Resource usage history
    pub resource_history: Vec<ResourceSnapshot>,
}

/// Health snapshot
#[derive(Debug, Clone)]
pub struct HealthSnapshot {
    /// Timestamp
    pub timestamp: Instant,
    /// Health status
    pub status: HealthStatus,
    /// API name
    pub api_name: String,
}

/// Resource snapshot
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
}

/// Metrics configuration
#[derive(Debug, Clone)]
pub struct MetricsConfig {
    /// Whether to collect performance metrics
    pub collect_performance: bool,
    /// Whether to collect health metrics
    pub collect_health: bool,
    /// Whether to collect resource metrics
    pub collect_resources: bool,
    /// Metrics retention period
    pub retention_period: Duration,
    /// Maximum number of history entries
    pub max_history_entries: usize,
    /// Metrics collection interval
    pub collection_interval: Duration,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            collect_performance: true,
            collect_health: true,
            collect_resources: true,
            retention_period: Duration::from_secs(3600), // 1 hour
            max_history_entries: 1000,
            collection_interval: Duration::from_secs(60), // 1 minute
        }
    }
}

impl ApiMetricsCollector {
    /// Create a new metrics collector
    pub fn new(config: MetricsConfig) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            performance: Arc::new(RwLock::new(PerformanceMetrics::default())),
            health: Arc::new(RwLock::new(HealthMetrics::default())),
            resources: Arc::new(RwLock::new(ResourceMetrics::default())),
            config,
        }
    }

    /// Record API call metrics
    pub fn record_api_call(
        &self,
        api_name: &str,
        success: bool,
        response_time: Duration,
    ) -> ApiResult<()> {
        if !self.config.collect_performance {
            return Ok(());
        }

        {
            let mut metrics = self
                .metrics
                .write()
                .map_err(|_| ApiError::configuration_error("Failed to acquire metrics lock"))?;

            let api_metrics = metrics
                .entry(api_name.to_string())
                .or_insert_with(ApiMetrics::default);
            api_metrics.usage_count += 1;
            api_metrics.last_used = Some(Instant::now());

            if success {
            } else {
                api_metrics.error_count += 1;
            }
        }

        {
            let mut performance = self
                .performance
                .write()
                .map_err(|_| ApiError::configuration_error("Failed to acquire performance lock"))?;

            performance.total_calls += 1;
            performance.last_call_time = Some(Instant::now());

            if success {
                performance.successful_calls += 1;
            } else {
                performance.failed_calls += 1;
            }

            performance.total_response_time += response_time;

            if response_time > performance.peak_response_time {
                performance.peak_response_time = response_time;
            }

            if performance.total_calls > 0 {
                performance.average_response_time =
                    performance.total_response_time / performance.total_calls as u32;
            }

            if let Some(last_call) = performance.last_call_time {
                let elapsed = last_call.elapsed();
                if elapsed.as_secs() > 0 {
                    performance.calls_per_second =
                        performance.total_calls as f64 / elapsed.as_secs() as f64;
                }
            }
        }

        debug!(
            "Recorded API call metrics for '{}': success={}, time={:?}",
            api_name, success, response_time
        );
        Ok(())
    }

    /// Record health check metrics
    pub fn record_health_check(&self, api_name: &str, status: HealthStatus) -> ApiResult<()> {
        if !self.config.collect_health {
            return Ok(());
        }

        {
            let mut metrics = self
                .metrics
                .write()
                .map_err(|_| ApiError::configuration_error("Failed to acquire metrics lock"))?;

            let api_metrics = metrics
                .entry(api_name.to_string())
                .or_insert_with(ApiMetrics::default);
        }

        {
            let mut health = self
                .health
                .write()
                .map_err(|_| ApiError::configuration_error("Failed to acquire health lock"))?;

            health.total_health_checks += 1;
            health.last_health_check = Some(Instant::now());
            health.current_status = status.clone();

            match status {
                HealthStatus::Healthy => health.healthy_checks += 1,
                HealthStatus::Degraded => health.degraded_checks += 1,
                HealthStatus::Unhealthy => health.unhealthy_checks += 1,
                HealthStatus::Unknown => health.unknown_checks += 1,
            }

            let snapshot = HealthSnapshot {
                timestamp: Instant::now(),
                status: status.clone(),
                api_name: api_name.to_string(),
            };

            health.health_history.push(snapshot);

            if health.health_history.len() > self.config.max_history_entries {
                health.health_history.remove(0);
            }
        }

        debug!(
            "Recorded health check metrics for '{}': {:?}",
            api_name, status
        );
        Ok(())
    }

    /// Record resource usage metrics
    pub fn record_resource_usage(&self, api_name: &str, usage: ResourceUsage) -> ApiResult<()> {
        if !self.config.collect_resources {
            return Ok(());
        }

        {
            let mut metrics = self
                .metrics
                .write()
                .map_err(|_| ApiError::configuration_error("Failed to acquire metrics lock"))?;

            let api_metrics = metrics
                .entry(api_name.to_string())
                .or_insert_with(ApiMetrics::default);
            api_metrics.resource_usage = usage.clone();
        }

        {
            let mut resources = self
                .resources
                .write()
                .map_err(|_| ApiError::configuration_error("Failed to acquire resources lock"))?;

            resources.current_memory_mb = usage.memory_mb;
            resources.current_file_descriptors = usage.file_descriptors;
            resources.current_network_connections = usage.network_connections;
            resources.current_cpu_percentage = usage.cpu_percentage;

            if usage.memory_mb > resources.peak_memory_mb {
                resources.peak_memory_mb = usage.memory_mb;
            }
            if usage.file_descriptors > resources.peak_file_descriptors {
                resources.peak_file_descriptors = usage.file_descriptors;
            }
            if usage.network_connections > resources.peak_network_connections {
                resources.peak_network_connections = usage.network_connections;
            }
            if usage.cpu_percentage > resources.peak_cpu_percentage {
                resources.peak_cpu_percentage = usage.cpu_percentage;
            }

            let snapshot = ResourceSnapshot {
                timestamp: Instant::now(),
                memory_mb: usage.memory_mb,
                file_descriptors: usage.file_descriptors,
                network_connections: usage.network_connections,
                cpu_percentage: usage.cpu_percentage,
            };

            resources.resource_history.push(snapshot);

            if resources.resource_history.len() > self.config.max_history_entries {
                resources.resource_history.remove(0);
            }
        }

        debug!(
            "Recorded resource usage metrics for '{}': {:?}",
            api_name, usage
        );
        Ok(())
    }

    /// Get metrics for a specific API
    pub fn get_api_metrics(&self, api_name: &str) -> Option<ApiMetrics> {
        let metrics = self.metrics.read().ok()?;
        metrics.get(api_name).cloned()
    }

    /// Get all API metrics
    pub fn get_all_metrics(&self) -> HashMap<String, ApiMetrics> {
        self.metrics
            .read()
            .unwrap_or_else(|_| panic!("Failed to acquire metrics lock"))
            .clone()
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.performance
            .read()
            .unwrap_or_else(|_| panic!("Failed to acquire performance lock"))
            .clone()
    }

    /// Get health metrics
    pub fn get_health_metrics(&self) -> HealthMetrics {
        self.health
            .read()
            .unwrap_or_else(|_| panic!("Failed to acquire health lock"))
            .clone()
    }

    /// Get resource metrics
    pub fn get_resource_metrics(&self) -> ResourceMetrics {
        self.resources
            .read()
            .unwrap_or_else(|_| panic!("Failed to acquire resources lock"))
            .clone()
    }

    /// Get comprehensive metrics report
    pub fn get_metrics_report(&self) -> MetricsReport {
        let performance = self.get_performance_metrics();
        let health = self.get_health_metrics();
        let resources = self.get_resource_metrics();
        let api_metrics = self.get_all_metrics();

        MetricsReport {
            timestamp: Instant::now(),
            performance,
            health,
            resources,
            api_metrics: api_metrics.clone(),
            total_apis: api_metrics.len(),
        }
    }

    /// Clear all metrics
    pub fn clear_metrics(&self) -> ApiResult<()> {
        {
            let mut metrics = self
                .metrics
                .write()
                .map_err(|_| ApiError::configuration_error("Failed to acquire metrics lock"))?;
            metrics.clear();
        }

        {
            let mut performance = self
                .performance
                .write()
                .map_err(|_| ApiError::configuration_error("Failed to acquire performance lock"))?;
            *performance = PerformanceMetrics::default();
        }

        {
            let mut health = self
                .health
                .write()
                .map_err(|_| ApiError::configuration_error("Failed to acquire health lock"))?;
            *health = HealthMetrics::default();
        }

        {
            let mut resources = self
                .resources
                .write()
                .map_err(|_| ApiError::configuration_error("Failed to acquire resources lock"))?;
            *resources = ResourceMetrics::default();
        }

        info!("Cleared all metrics");
        Ok(())
    }

    /// Update metrics configuration
    pub fn update_config(&mut self, config: MetricsConfig) {
        self.config = config;
        debug!("Updated metrics configuration: {:?}", self.config);
    }
}

/// Comprehensive metrics report
#[derive(Debug)]
pub struct MetricsReport {
    /// Report timestamp
    pub timestamp: Instant,
    /// Performance metrics
    pub performance: PerformanceMetrics,
    /// Health metrics
    pub health: HealthMetrics,
    /// Resource metrics
    pub resources: ResourceMetrics,
    /// API-specific metrics
    pub api_metrics: HashMap<String, ApiMetrics>,
    /// Total number of APIs
    pub total_apis: usize,
}

impl Clone for PerformanceMetrics {
    fn clone(&self) -> Self {
        Self {
            total_calls: self.total_calls,
            successful_calls: self.successful_calls,
            failed_calls: self.failed_calls,
            average_response_time: self.average_response_time,
            total_response_time: self.total_response_time,
            peak_response_time: self.peak_response_time,
            calls_per_second: self.calls_per_second,
            last_call_time: self.last_call_time,
        }
    }
}

impl Clone for HealthMetrics {
    fn clone(&self) -> Self {
        Self {
            total_health_checks: self.total_health_checks,
            healthy_checks: self.healthy_checks,
            degraded_checks: self.degraded_checks,
            unhealthy_checks: self.unhealthy_checks,
            unknown_checks: self.unknown_checks,
            current_status: self.current_status.clone(),
            last_health_check: self.last_health_check,
            health_history: self.health_history.clone(),
        }
    }
}

impl Clone for ResourceMetrics {
    fn clone(&self) -> Self {
        Self {
            peak_memory_mb: self.peak_memory_mb,
            current_memory_mb: self.current_memory_mb,
            peak_file_descriptors: self.peak_file_descriptors,
            current_file_descriptors: self.current_file_descriptors,
            peak_network_connections: self.peak_network_connections,
            current_network_connections: self.current_network_connections,
            peak_cpu_percentage: self.peak_cpu_percentage,
            current_cpu_percentage: self.current_cpu_percentage,
            resource_history: self.resource_history.clone(),
        }
    }
}
