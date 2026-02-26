//! # Graceful Degradation System
//!
//! This module provides graceful degradation functionality when API registration fails.

use crate::runtime::apis::{ApiConfig, ApiError, ApiPlugin, ApiResult, HealthStatus};
use chitin::boa_engine::Context;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, warn};

/// Graceful degradation manager
pub struct GracefulDegradationManager {
    /// Configuration
    config: ApiConfig,
    /// Failed APIs and their fallback strategies
    failed_apis: Arc<RwLock<HashMap<String, FallbackStrategy>>>,
    /// Critical APIs that must be available
    critical_apis: HashSet<String>,
    /// Fallback implementations
    fallbacks: Arc<RwLock<HashMap<String, Box<dyn ApiPlugin>>>>,
}

/// Fallback strategy for failed APIs
#[derive(Debug, Clone)]
pub enum FallbackStrategy {
    /// Use a mock implementation
    Mock,
    /// Use a simplified implementation
    Simplified,
    /// Use an alternative API
    Alternative(String),
    /// Disable the API completely
    Disable,
    /// Retry with exponential backoff
    Retry {
        max_attempts: u32,
        base_delay_ms: u64,
    },
}

/// Degradation level
#[derive(Debug, Clone, PartialEq)]
pub enum DegradationLevel {
    /// No degradation - all APIs working
    None,
    /// Minor degradation - some non-critical APIs failed
    Minor,
    /// Major degradation - some critical APIs failed
    Major,
    /// Critical degradation - core functionality compromised
    Critical,
}

impl GracefulDegradationManager {
    /// Create a new graceful degradation manager
    pub fn new(config: ApiConfig) -> Self {
        let mut critical_apis = HashSet::new();
        critical_apis.insert("console".to_string());
        critical_apis.insert("process".to_string());
        critical_apis.insert("events".to_string());

        Self {
            config,
            failed_apis: Arc::new(RwLock::new(HashMap::new())),
            critical_apis,
            fallbacks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a fallback implementation for an API
    pub fn register_fallback(&self, api_name: &str, fallback: Box<dyn ApiPlugin>) -> ApiResult<()> {
        let mut fallbacks = self
            .fallbacks
            .write()
            .map_err(|_| ApiError::configuration_error("Failed to acquire fallbacks lock"))?;

        fallbacks.insert(api_name.to_string(), fallback);
        debug!("Registered fallback for API: {}", api_name);
        Ok(())
    }

    /// Set fallback strategy for an API
    pub fn set_fallback_strategy(
        &self,
        api_name: &str,
        strategy: FallbackStrategy,
    ) -> ApiResult<()> {
        let mut failed_apis = self
            .failed_apis
            .write()
            .map_err(|_| ApiError::configuration_error("Failed to acquire failed_apis lock"))?;

        failed_apis.insert(api_name.to_string(), strategy.clone());
        debug!("Set fallback strategy for API {}: {:?}", api_name, strategy);
        Ok(())
    }

    /// Handle API registration failure
    pub fn handle_registration_failure(
        &self,
        api_name: &str,
        error: ApiError,
        context: &mut Context,
    ) -> ApiResult<()> {
        warn!("API registration failed for '{}': {}", api_name, error);

        let is_critical = self.critical_apis.contains(api_name);

        if is_critical {
            error!("Critical API '{}' failed to register", api_name);
            return self.handle_critical_failure(api_name, error, context);
        }

        self.handle_non_critical_failure(api_name, error, context)
    }

    /// Get current degradation level
    pub fn get_degradation_level(&self) -> DegradationLevel {
        let failed_apis = self
            .failed_apis
            .read()
            .unwrap_or_else(|_| panic!("Failed to acquire failed_apis lock"));

        let mut critical_failures = 0;
        let mut non_critical_failures = 0;

        for (api_name, _) in failed_apis.iter() {
            if self.critical_apis.contains(api_name) {
                critical_failures += 1;
            } else {
                non_critical_failures += 1;
            }
        }

        match (critical_failures, non_critical_failures) {
            (0, 0) => DegradationLevel::None,
            (0, _) => DegradationLevel::Minor,
            (1..=2, _) => DegradationLevel::Major,
            _ => DegradationLevel::Critical,
        }
    }

    /// Get list of failed APIs
    pub fn get_failed_apis(&self) -> Vec<String> {
        let failed_apis = self
            .failed_apis
            .read()
            .unwrap_or_else(|_| panic!("Failed to acquire failed_apis lock"));

        failed_apis.keys().cloned().collect()
    }

    /// Get fallback strategy for an API
    pub fn get_fallback_strategy(&self, api_name: &str) -> Option<FallbackStrategy> {
        let failed_apis = self
            .failed_apis
            .read()
            .unwrap_or_else(|_| panic!("Failed to acquire failed_apis lock"));

        failed_apis.get(api_name).cloned()
    }

    /// Check if graceful degradation is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enable_graceful_degradation
    }

    /// Add a critical API
    pub fn add_critical_api(&mut self, api_name: &str) {
        self.critical_apis.insert(api_name.to_string());
        debug!("Added critical API: {}", api_name);
    }

    /// Remove a critical API
    pub fn remove_critical_api(&mut self, api_name: &str) {
        self.critical_apis.remove(api_name);
        debug!("Removed critical API: {}", api_name);
    }

    /// Get health status considering degradation
    pub fn get_health_status(&self) -> HealthStatus {
        match self.get_degradation_level() {
            DegradationLevel::None => HealthStatus::Healthy,
            DegradationLevel::Minor => HealthStatus::Healthy,
            DegradationLevel::Major => HealthStatus::Degraded,
            DegradationLevel::Critical => HealthStatus::Unhealthy,
        }
    }

    /// Handle critical API failure
    fn handle_critical_failure(
        &self,
        api_name: &str,
        error: ApiError,
        context: &mut Context,
    ) -> ApiResult<()> {
        if self.get_fallback_implementation(api_name) {
            info!(
                "Using fallback implementation for critical API: {}",
                api_name
            );
            return self.create_minimal_console(context);
        }

        match api_name {
            "console" => self.create_minimal_console(context),
            "process" => self.create_minimal_process(context),
            "events" => self.create_minimal_events(context),
            _ => Err(ApiError::registration_failed(
                api_name,
                crate::runtime::apis::error::SimpleError::new(format!(
                    "Critical API '{}' failed and no fallback available",
                    api_name
                )),
            )),
        }
    }

    /// Handle non-critical API failure
    fn handle_non_critical_failure(
        &self,
        api_name: &str,
        error: ApiError,
        context: &mut Context,
    ) -> ApiResult<()> {
        if !self.config.enable_graceful_degradation {
            return Err(error);
        }

        let strategy = FallbackStrategy::Mock;
        self.set_fallback_strategy(api_name, strategy.clone())?;

        match strategy {
            FallbackStrategy::Mock => {
                info!("Using mock implementation for API: {}", api_name);
                self.create_mock_implementation(api_name, context)
            }
            FallbackStrategy::Simplified => {
                info!("Using simplified implementation for API: {}", api_name);
                self.create_simplified_implementation(api_name, context)
            }
            FallbackStrategy::Alternative(alt_name) => {
                info!("Using alternative API '{}' for '{}'", alt_name, api_name);
                self.use_alternative_api(api_name, &alt_name, context)
            }
            FallbackStrategy::Disable => {
                info!("Disabling API: {}", api_name);
                Ok(())
            }
            FallbackStrategy::Retry {
                max_attempts,
                base_delay_ms,
            } => {
                warn!("Retry strategy not implemented for API: {}", api_name);
                Err(error)
            }
        }
    }

    /// Get fallback implementation for an API
    fn get_fallback_implementation(&self, api_name: &str) -> bool {
        if let Ok(fallbacks) = self.fallbacks.read() {
            fallbacks.contains_key(api_name)
        } else {
            false
        }
    }

    /// Create minimal console implementation
    fn create_minimal_console(&self, context: &mut Context) -> ApiResult<()> {
        let console_code = r#"
            globalThis.console = {
                log: function(...args) { 
                },
                error: function(...args) { 
                },
                warn: function(...args) { 
                },
                info: function(...args) { 
                }
            };
        "#;

        context
            .eval(chitin::boa_engine::Source::from_bytes(console_code))
            .map_err(|e| {
                ApiError::registration_failed(
                    "console (minimal)",
                    crate::runtime::apis::error::SimpleError::new(format!("{}", e)),
                )
            })?;

        info!("Created minimal console implementation");
        Ok(())
    }

    /// Create minimal process implementation
    fn create_minimal_process(&self, context: &mut Context) -> ApiResult<()> {
        let process_code = r#"
            globalThis.process = {
                argv: [],
                env: {},
                version: "v18.0.0",
                cwd: function() { return "."; },
                exit: function(code) { 
                }
            };
        "#;

        context
            .eval(chitin::boa_engine::Source::from_bytes(process_code))
            .map_err(|e| {
                ApiError::registration_failed(
                    "process (minimal)",
                    crate::runtime::apis::error::SimpleError::new(format!("{}", e)),
                )
            })?;

        info!("Created minimal process implementation");
        Ok(())
    }

    /// Create minimal events implementation
    fn create_minimal_events(&self, context: &mut Context) -> ApiResult<()> {
        let events_code = r#"
            globalThis.EventEmitter = function() {
                this.listeners = {};
            };
            globalThis.EventEmitter.prototype.on = function(event, listener) {
                if (!this.listeners[event]) this.listeners[event] = [];
                this.listeners[event].push(listener);
            };
            globalThis.EventEmitter.prototype.emit = function(event, ...args) {
                if (this.listeners[event]) {
                    this.listeners[event].forEach(listener => listener(...args));
                }
            };
        "#;

        context
            .eval(chitin::boa_engine::Source::from_bytes(events_code))
            .map_err(|e| {
                ApiError::registration_failed(
                    "events (minimal)",
                    crate::runtime::apis::error::SimpleError::new(format!("{}", e)),
                )
            })?;

        info!("Created minimal events implementation");
        Ok(())
    }

    /// Create mock implementation for an API
    fn create_mock_implementation(&self, api_name: &str, context: &mut Context) -> ApiResult<()> {
        let mock_code = format!(
            r#"
            globalThis.{} = {{
                mock: true,
                version: "mock-0.4.0"
            }};
        "#,
            api_name
        );

        context
            .eval(chitin::boa_engine::Source::from_bytes(mock_code.as_bytes()))
            .map_err(|e| {
                ApiError::registration_failed(
                    &format!("{} (mock)", api_name),
                    crate::runtime::apis::error::SimpleError::new(format!("{}", e)),
                )
            })?;

        debug!("Created mock implementation for API: {}", api_name);
        Ok(())
    }

    /// Create simplified implementation for an API
    fn create_simplified_implementation(
        &self,
        api_name: &str,
        context: &mut Context,
    ) -> ApiResult<()> {
        self.create_mock_implementation(api_name, context)
    }

    /// Use alternative API
    fn use_alternative_api(
        &self,
        api_name: &str,
        alt_name: &str,
        context: &mut Context,
    ) -> ApiResult<()> {
        let alt_code = format!(
            r#"
            globalThis.{} = globalThis.{} || {{}};
        "#,
            api_name, alt_name
        );

        context
            .eval(chitin::boa_engine::Source::from_bytes(alt_code.as_bytes()))
            .map_err(|e| {
                ApiError::registration_failed(
                    &format!("{} (alternative)", api_name),
                    crate::runtime::apis::error::SimpleError::new(format!("{}", e)),
                )
            })?;

        debug!("Used alternative API '{}' for '{}'", alt_name, api_name);
        Ok(())
    }
}
