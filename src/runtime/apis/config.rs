//! # API Configuration
//!
//! This module provides configuration management for JetCrab APIs.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

/// Configuration for the API system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// APIs that are enabled by default
    pub enabled_apis: Vec<String>,

    /// APIs that are explicitly disabled
    pub disabled_apis: Vec<String>,

    /// Experimental APIs that require explicit enabling
    pub experimental_apis: Vec<String>,

    /// API-specific configuration
    pub api_settings: HashMap<String, ApiSettings>,

    /// Global API timeout in milliseconds
    pub api_timeout_ms: Option<u64>,

    /// Maximum number of concurrent API registrations
    pub max_concurrent_registrations: Option<usize>,

    /// Whether to enable lazy loading
    pub enable_lazy_loading: bool,

    /// Whether to enable graceful degradation
    pub enable_graceful_degradation: bool,

    /// Resource limits
    pub resource_limits: ResourceLimits,

    /// Logging configuration
    pub logging: LoggingConfig,
}

/// Settings for individual APIs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSettings {
    /// Whether this API is enabled
    pub enabled: bool,

    /// API-specific timeout in milliseconds
    pub timeout_ms: Option<u64>,

    /// Custom configuration for this API
    pub custom_config: HashMap<String, serde_json::Value>,

    /// Dependencies for this API
    pub dependencies: Vec<String>,

    /// Whether this API is critical
    pub critical: bool,
}

/// Resource limits for the API system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum memory usage in MB
    pub max_memory_mb: u64,

    /// Maximum number of file descriptors
    pub max_file_descriptors: u32,

    /// Maximum number of network connections
    pub max_network_connections: u32,

    /// Maximum CPU usage percentage
    pub max_cpu_percentage: u8,
}

/// Logging configuration for APIs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Whether to log API registrations
    pub log_registrations: bool,

    /// Whether to log API usage
    pub log_usage: bool,

    /// Whether to log API errors
    pub log_errors: bool,

    /// Whether to log performance metrics
    pub log_metrics: bool,

    /// Log level for API operations
    pub log_level: String,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            enabled_apis: vec![
                "console".to_string(),
                "process".to_string(),
                "events".to_string(),
                "timers".to_string(),
                "util".to_string(),
            ],
            disabled_apis: vec![],
            experimental_apis: vec![
                "worker_threads".to_string(),
                "vm".to_string(),
                "cluster".to_string(),
            ],
            api_settings: HashMap::new(),
            api_timeout_ms: Some(5000),
            max_concurrent_registrations: Some(10),
            enable_lazy_loading: true,
            enable_graceful_degradation: true,
            resource_limits: ResourceLimits::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 512,
            max_file_descriptors: 1000,
            max_network_connections: 100,
            max_cpu_percentage: 80,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            log_registrations: true,
            log_usage: false,
            log_errors: true,
            log_metrics: false,
            log_level: "info".to_string(),
        }
    }
}

impl ApiConfig {
    /// Create a new API configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Load configuration from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: ApiConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(enabled) = std::env::var("JETCRAB_ENABLED_APIS") {
            config.enabled_apis = enabled.split(',').map(|s| s.trim().to_string()).collect();
        }

        if let Ok(disabled) = std::env::var("JETCRAB_DISABLED_APIS") {
            config.disabled_apis = disabled.split(',').map(|s| s.trim().to_string()).collect();
        }

        if let Ok(experimental) = std::env::var("JETCRAB_EXPERIMENTAL_APIS") {
            config.experimental_apis = experimental
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        if let Ok(timeout) = std::env::var("JETCRAB_API_TIMEOUT_MS") {
            if let Ok(timeout_ms) = timeout.parse::<u64>() {
                config.api_timeout_ms = Some(timeout_ms);
            }
        }

        if let Ok(lazy_loading) = std::env::var("JETCRAB_ENABLE_LAZY_LOADING") {
            config.enable_lazy_loading = lazy_loading.parse().unwrap_or(true);
        }

        if let Ok(graceful) = std::env::var("JETCRAB_ENABLE_GRACEFUL_DEGRADATION") {
            config.enable_graceful_degradation = graceful.parse().unwrap_or(true);
        }

        config
    }

    /// Check if an API is enabled
    pub fn is_api_enabled(&self, api_name: &str) -> bool {
        if self.disabled_apis.contains(&api_name.to_string()) {
            return false;
        }

        if self.enabled_apis.contains(&api_name.to_string()) {
            return true;
        }

        if let Some(settings) = self.api_settings.get(api_name) {
            return settings.enabled;
        }

        matches!(
            api_name,
            "console" | "process" | "events" | "timers" | "util"
        )
    }

    /// Check if an API is experimental
    pub fn is_api_experimental(&self, api_name: &str) -> bool {
        self.experimental_apis.contains(&api_name.to_string())
    }

    /// Get timeout for an API
    pub fn get_api_timeout(&self, api_name: &str) -> Duration {
        if let Some(settings) = self.api_settings.get(api_name) {
            if let Some(timeout_ms) = settings.timeout_ms {
                return Duration::from_millis(timeout_ms);
            }
        }

        if let Some(timeout_ms) = self.api_timeout_ms {
            Duration::from_millis(timeout_ms)
        } else {
            Duration::from_secs(5) // Default 5 seconds
        }
    }

    /// Get settings for an API
    pub fn get_api_settings(&self, api_name: &str) -> ApiSettings {
        self.api_settings.get(api_name).cloned().unwrap_or_default()
    }

    /// Set settings for an API
    pub fn set_api_settings(&mut self, api_name: String, settings: ApiSettings) {
        self.api_settings.insert(api_name, settings);
    }

    /// Enable an API
    pub fn enable_api(&mut self, api_name: &str) {
        self.disabled_apis.retain(|name| name != api_name);
        if !self.enabled_apis.contains(&api_name.to_string()) {
            self.enabled_apis.push(api_name.to_string());
        }
    }

    /// Disable an API
    pub fn disable_api(&mut self, api_name: &str) {
        self.enabled_apis.retain(|name| name != api_name);
        if !self.disabled_apis.contains(&api_name.to_string()) {
            self.disabled_apis.push(api_name.to_string());
        }
    }

    /// Add an experimental API
    pub fn add_experimental_api(&mut self, api_name: &str) {
        if !self.experimental_apis.contains(&api_name.to_string()) {
            self.experimental_apis.push(api_name.to_string());
        }
    }

    /// Remove an experimental API
    pub fn remove_experimental_api(&mut self, api_name: &str) {
        self.experimental_apis.retain(|name| name != api_name);
    }

    /// Get all enabled APIs
    pub fn get_enabled_apis(&self) -> Vec<String> {
        self.enabled_apis.clone()
    }

    /// Get all disabled APIs
    pub fn get_disabled_apis(&self) -> Vec<String> {
        self.disabled_apis.clone()
    }

    /// Get all experimental APIs
    pub fn get_experimental_apis(&self) -> Vec<String> {
        self.experimental_apis.clone()
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for api in &self.enabled_apis {
            if self.disabled_apis.contains(api) {
                errors.push(format!("API '{}' is both enabled and disabled", api));
            }
        }

        if let Some(timeout) = self.api_timeout_ms {
            if timeout == 0 {
                errors.push("API timeout cannot be 0".to_string());
            }
        }

        if self.resource_limits.max_memory_mb == 0 {
            errors.push("Maximum memory limit cannot be 0".to_string());
        }

        if self.resource_limits.max_cpu_percentage > 100 {
            errors.push("Maximum CPU percentage cannot exceed 100".to_string());
        }

        if let Some(limit) = self.max_concurrent_registrations {
            if limit == 0 {
                errors.push("Maximum concurrent registrations cannot be 0".to_string());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Default for ApiSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout_ms: None,
            custom_config: HashMap::new(),
            dependencies: Vec::new(),
            critical: false,
        }
    }
}
