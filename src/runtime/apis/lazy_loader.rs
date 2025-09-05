//! # Lazy API Loader
//!
//! This module provides lazy loading functionality for JetCrab APIs.

use crate::runtime::apis::{ApiConfig, ApiError, ApiPlugin, ApiRegistry, ApiResult};
use boa_engine::Context;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};

/// Lazy loader for APIs
pub struct LazyApiLoader {
    /// Registry for managing loaded APIs
    registry: Arc<RwLock<ApiRegistry>>,
    /// Configuration for the loader
    config: ApiConfig,
    /// Loading statistics
    stats: Arc<RwLock<LoadingStats>>,
    /// Loading timeouts
    timeouts: HashMap<String, Duration>,
}

/// Statistics for API loading
#[derive(Debug, Default)]
pub struct LoadingStats {
    /// Total number of APIs loaded
    pub total_loaded: usize,
    /// Number of successful loads
    pub successful_loads: usize,
    /// Number of failed loads
    pub failed_loads: usize,
    /// Total loading time
    pub total_loading_time: Duration,
    /// Average loading time per API
    pub average_loading_time: Duration,
    /// Last load time
    pub last_load_time: Option<Instant>,
}

impl LazyApiLoader {
    /// Create a new lazy API loader
    pub fn new(config: ApiConfig) -> Self {
        let mut timeouts = HashMap::new();

        timeouts.insert("core".to_string(), Duration::from_millis(1000));
        timeouts.insert("networking".to_string(), Duration::from_millis(2000));
        timeouts.insert("filesystem".to_string(), Duration::from_millis(1500));
        timeouts.insert("crypto".to_string(), Duration::from_millis(3000));
        timeouts.insert("system".to_string(), Duration::from_millis(2000));
        timeouts.insert("utility".to_string(), Duration::from_millis(1000));
        timeouts.insert("experimental".to_string(), Duration::from_millis(5000));

        Self {
            registry: Arc::new(RwLock::new(ApiRegistry::new())),
            config,
            stats: Arc::new(RwLock::new(LoadingStats::default())),
            timeouts,
        }
    }

    /// Register a plugin factory for lazy loading
    pub fn register_factory<F>(&self, name: &str, factory: F) -> ApiResult<()>
    where
        F: Fn() -> Box<dyn ApiPlugin> + Send + Sync + 'static,
    {
        let mut registry = self
            .registry
            .write()
            .map_err(|_| ApiError::configuration_error("Failed to acquire registry lock"))?;

        registry.register_factory(name, factory);
        debug!("Registered lazy loading factory for: {}", name);
        Ok(())
    }

    /// Load an API by name with timeout
    pub fn load_api(&self, name: &str, context: &mut Context) -> ApiResult<()> {
        let start_time = Instant::now();

        if !self.config.is_api_enabled(name) {
            return Err(ApiError::configuration_error(&format!(
                "API '{}' is disabled in configuration",
                name
            )));
        }

        {
            let registry = self
                .registry
                .read()
                .map_err(|_| ApiError::configuration_error("Failed to acquire registry lock"))?;

            if registry.is_loaded(name) {
                debug!("API '{}' is already loaded", name);
                return Ok(());
            }
        }

        let timeout = self.get_timeout_for_api(name);

        let load_result = self.load_with_timeout(name, context, timeout);

        self.update_stats(name, start_time, load_result.is_ok());

        load_result
    }

    /// Load multiple APIs in parallel
    pub fn load_apis_parallel(&self, names: &[&str], context: &mut Context) -> Vec<ApiResult<()>> {
        let mut results = Vec::new();

        for name in names {
            let result = self.load_api(name, context);
            results.push(result);
        }

        info!("Loaded {} APIs in parallel", names.len());
        results
    }

    /// Load APIs by category
    pub fn load_category(&self, category: &str, context: &mut Context) -> ApiResult<()> {
        let api_names = self.get_apis_for_category(category);

        if api_names.is_empty() {
            warn!("No APIs found for category: {}", category);
            return Ok(());
        }

        info!(
            "Loading {} APIs for category: {}",
            api_names.len(),
            category
        );

        let mut errors = Vec::new();
        for name in &api_names {
            if let Err(e) = self.load_api(name, context) {
                errors.push(format!("Failed to load {}: {}", name, e));
            }
        }

        if !errors.is_empty() {
            return Err(ApiError::batch_registration_failed(
                errors.len(),
                api_names.len(),
            ));
        }

        Ok(())
    }

    /// Load all enabled APIs
    pub fn load_all_enabled(&self, context: &mut Context) -> ApiResult<()> {
        let enabled_apis = self.config.get_enabled_apis();

        info!("Loading {} enabled APIs", enabled_apis.len());

        let mut errors = Vec::new();
        for name in &enabled_apis {
            if let Err(e) = self.load_api(name, context) {
                if self.config.enable_graceful_degradation {
                    warn!("Failed to load API '{}': {}", name, e);
                    errors.push(format!("{}: {}", name, e));
                } else {
                    return Err(e);
                }
            }
        }

        if !errors.is_empty() && !self.config.enable_graceful_degradation {
            return Err(ApiError::batch_registration_failed(
                errors.len(),
                enabled_apis.len(),
            ));
        }

        if !errors.is_empty() {
            warn!("Loaded APIs with {} errors: {:?}", errors.len(), errors);
        }

        Ok(())
    }

    /// Check if an API is loaded
    pub fn is_loaded(&self, name: &str) -> bool {
        self.registry
            .read()
            .map(|registry| registry.is_loaded(name))
            .unwrap_or(false)
    }

    /// Get loading statistics
    pub fn get_stats(&self) -> LoadingStats {
        self.stats
            .read()
            .unwrap_or_else(|_| panic!("Failed to read stats"))
            .clone()
    }

    /// Get registry reference
    pub fn get_registry(&self) -> Arc<RwLock<ApiRegistry>> {
        self.registry.clone()
    }

    /// Set timeout for a specific API
    pub fn set_api_timeout(&mut self, name: &str, timeout: Duration) {
        self.timeouts.insert(name.to_string(), timeout);
    }

    /// Get timeout for an API
    fn get_timeout_for_api(&self, name: &str) -> Duration {
        if let Some(timeout) = self.timeouts.get(name) {
            return *timeout;
        }

        let category = self.get_category_for_api(name);
        if let Some(timeout) = self.timeouts.get(&category) {
            return *timeout;
        }

        self.config.get_api_timeout(name)
    }

    /// Get category for an API
    fn get_category_for_api(&self, name: &str) -> String {
        match name {
            "console" | "process" | "events" => "core".to_string(),
            "fetch" | "http" | "https" | "net" | "tls" => "networking".to_string(),
            "fs" | "path" | "buffer" => "filesystem".to_string(),
            "crypto" => "crypto".to_string(),
            "os" | "child_process" | "cluster" | "dgram" => "system".to_string(),
            "util" | "url" | "querystring" | "timers" | "stream" | "readline" | "repl"
            | "assert" | "perf_hooks" => "utility".to_string(),
            "worker_threads" | "vm" | "zlib" => "experimental".to_string(),
            _ => "utility".to_string(),
        }
    }

    /// Get APIs for a category
    fn get_apis_for_category(&self, category: &str) -> Vec<String> {
        match category {
            "core" => vec![
                "console".to_string(),
                "process".to_string(),
                "events".to_string(),
            ],
            "networking" => vec![
                "fetch".to_string(),
                "http".to_string(),
                "https".to_string(),
                "net".to_string(),
                "tls".to_string(),
            ],
            "filesystem" => vec!["fs".to_string(), "path".to_string(), "buffer".to_string()],
            "crypto" => vec!["crypto".to_string()],
            "system" => vec![
                "os".to_string(),
                "child_process".to_string(),
                "cluster".to_string(),
                "dgram".to_string(),
            ],
            "utility" => vec![
                "util".to_string(),
                "url".to_string(),
                "querystring".to_string(),
                "timers".to_string(),
                "stream".to_string(),
                "readline".to_string(),
                "repl".to_string(),
                "assert".to_string(),
                "perf_hooks".to_string(),
            ],
            "experimental" => vec![
                "worker_threads".to_string(),
                "vm".to_string(),
                "zlib".to_string(),
            ],
            _ => vec![],
        }
    }

    /// Load API with timeout
    fn load_with_timeout(
        &self,
        name: &str,
        context: &mut Context,
        timeout: Duration,
    ) -> ApiResult<()> {
        let start_time = Instant::now();

        {
            let mut registry = self
                .registry
                .write()
                .map_err(|_| ApiError::configuration_error("Failed to acquire registry lock"))?;

            registry.load_plugin(name).map_err(|e| {
                ApiError::registration_failed(
                    name,
                    crate::runtime::apis::error::SimpleError::new(format!("{}", e)),
                )
            })?;

            registry.register_in_context(name, context).map_err(|e| {
                ApiError::registration_failed(
                    name,
                    crate::runtime::apis::error::SimpleError::new(format!("{}", e)),
                )
            })?;
        }

        let load_time = start_time.elapsed();

        if load_time > timeout {
            warn!(
                "API '{}' loaded but exceeded timeout: {:?} > {:?}",
                name, load_time, timeout
            );
        }

        debug!("Successfully loaded API '{}' in {:?}", name, load_time);
        Ok(())
    }

    /// Update loading statistics
    fn update_stats(&self, name: &str, start_time: Instant, success: bool) {
        if let Ok(mut stats) = self.stats.write() {
            stats.total_loaded += 1;
            stats.last_load_time = Some(start_time);

            let load_time = start_time.elapsed();
            stats.total_loading_time += load_time;

            if success {
                stats.successful_loads += 1;
            } else {
                stats.failed_loads += 1;
            }

            if stats.total_loaded > 0 {
                stats.average_loading_time = stats.total_loading_time / stats.total_loaded as u32;
            }

            debug!(
                "Updated loading stats for '{}': success={}, time={:?}",
                name, success, load_time
            );
        }
    }
}

impl Clone for LoadingStats {
    fn clone(&self) -> Self {
        Self {
            total_loaded: self.total_loaded,
            successful_loads: self.successful_loads,
            failed_loads: self.failed_loads,
            total_loading_time: self.total_loading_time,
            average_loading_time: self.average_loading_time,
            last_load_time: self.last_load_time,
        }
    }
}
