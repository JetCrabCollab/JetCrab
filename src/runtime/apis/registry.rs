//! # API Registry
//!
//! This module provides the registry system for managing JetCrab APIs.

use crate::runtime::apis::plugin::{ApiMetrics, ApiPlugin, HealthStatus, ResourceUsage};
use chitin::boa_engine::{Context, JsResult};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::Instant;
use tracing::{debug, error, info, warn};

/// Registry for managing API plugins
pub struct ApiRegistry {
    /// Registered plugins
    plugins: HashMap<String, Arc<dyn ApiPlugin>>,
    /// Plugin factories for lazy loading
    factories: HashMap<String, Box<dyn Fn() -> Box<dyn ApiPlugin> + Send + Sync>>,
    /// Currently loaded plugins
    loaded: HashSet<String>,
    /// Plugin metrics
    metrics: Arc<RwLock<HashMap<String, ApiMetrics>>>,
    /// Plugin health status
    health_status: Arc<RwLock<HashMap<String, HealthStatus>>>,
}

impl ApiRegistry {
    /// Create a new API registry
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            factories: HashMap::new(),
            loaded: HashSet::new(),
            metrics: Arc::new(RwLock::new(HashMap::new())),
            health_status: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a plugin factory for lazy loading
    pub fn register_factory<F>(&mut self, name: &str, factory: F)
    where
        F: Fn() -> Box<dyn ApiPlugin> + Send + Sync + 'static,
    {
        self.factories.insert(name.to_string(), Box::new(factory));
        debug!("Registered factory for API: {}", name);
    }

    /// Register a plugin directly
    pub fn register_plugin(&mut self, plugin: Box<dyn ApiPlugin>) {
        let name = plugin.name().to_string();
        self.plugins.insert(name.clone(), Arc::from(plugin));
        self.loaded.insert(name.clone());
        debug!("Registered plugin: {}", name);
    }

    /// Load a plugin by name (lazy loading)
    pub fn load_plugin(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.loaded.contains(name) {
            return Ok(());
        }

        let factory = self
            .factories
            .get(name)
            .ok_or_else(|| format!("No factory found for API: {}", name))?;

        let plugin = factory();
        let plugin_name = plugin.name().to_string();

        self.plugins.insert(plugin_name.clone(), Arc::from(plugin));
        self.loaded.insert(plugin_name.clone());

        info!("Loaded plugin: {}", plugin_name);
        Ok(())
    }

    /// Register a plugin in the JavaScript context
    pub fn register_in_context(&self, name: &str, context: &mut Context) -> JsResult<()> {
        let plugin = self.plugins.get(name).ok_or_else(|| {
            chitin::boa_engine::JsNativeError::error().with_message(format!("Plugin not found: {}", name))
        })?;

        let start_time = Instant::now();

        match plugin.register(context) {
            Ok(()) => {
                let registration_time = start_time.elapsed();
                self.update_metrics(name, |metrics| {
                    metrics.registration_time = registration_time;
                });
                debug!("Successfully registered API: {}", name);
                Ok(())
            }
            Err(e) => {
                self.update_metrics(name, |metrics| {
                    metrics.error_count += 1;
                });
                error!("Failed to register API {}: {:?}", name, e);
                Err(e)
            }
        }
    }

    /// Register multiple APIs in batch
    pub fn register_batch(&self, names: &[&str], context: &mut Context) -> Vec<String> {
        let mut errors = Vec::new();

        for name in names {
            if let Err(e) = self.register_in_context(name, context) {
                errors.push(format!("Failed to register {}: {:?}", name, e));
            }
        }

        if errors.is_empty() {
            info!("Successfully registered {} APIs in batch", names.len());
        } else {
            warn!("Batch registration completed with {} errors", errors.len());
        }

        errors
    }

    /// Register all available plugins with graceful degradation
    pub fn register_all_with_fallback(&self, context: &mut Context) -> Vec<String> {
        let mut errors = Vec::new();
        let mut registered_count = 0;

        for name in self.loaded.iter() {
            match self.register_in_context(name, context) {
                Ok(()) => {
                    registered_count += 1;
                }
                Err(e) => {
                    let error_msg = format!("Failed to register {}: {:?}", name, e);
                    errors.push(error_msg.clone());
                    error!("{}", error_msg);
                }
            }
        }

        info!(
            "Registered {}/{} APIs successfully",
            registered_count,
            self.loaded.len()
        );
        errors
    }

    /// Get a plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<&Arc<dyn ApiPlugin>> {
        self.plugins.get(name)
    }

    /// Check if a plugin is loaded
    pub fn is_loaded(&self, name: &str) -> bool {
        self.loaded.contains(name)
    }

    /// Get all loaded plugin names
    pub fn loaded_plugins(&self) -> Vec<String> {
        self.loaded.iter().cloned().collect()
    }

    /// Get all available plugin names (including unloaded)
    pub fn available_plugins(&self) -> Vec<String> {
        let mut names: Vec<String> = self.plugins.keys().cloned().collect();
        names.extend(self.factories.keys().cloned());
        names.sort();
        names
    }

    /// Update metrics for a plugin
    fn update_metrics<F>(&self, name: &str, updater: F)
    where
        F: FnOnce(&mut ApiMetrics),
    {
        if let Ok(mut metrics_map) = self.metrics.write() {
            let metrics = metrics_map
                .entry(name.to_string())
                .or_insert_with(ApiMetrics::default);
            updater(metrics);
        }
    }

    /// Get metrics for a specific plugin
    pub fn get_metrics(&self, name: &str) -> Option<ApiMetrics> {
        self.metrics.read().ok()?.get(name).cloned()
    }

    /// Get all plugin metrics
    pub fn get_all_metrics(&self) -> HashMap<String, ApiMetrics> {
        self.metrics
            .read()
            .unwrap_or_else(|_| panic!("Failed to read metrics"))
            .clone()
    }

    /// Perform health check on all plugins
    pub fn health_check_all(&self) -> HashMap<String, HealthStatus> {
        let mut health_map = HashMap::new();

        for (name, plugin) in &self.plugins {
            let status = plugin.health_check();
            health_map.insert(name.clone(), status);
        }

        if let Ok(mut health_status) = self.health_status.write() {
            *health_status = health_map.clone();
        }

        health_map
    }

    /// Get health status for a specific plugin
    pub fn get_health_status(&self, name: &str) -> Option<HealthStatus> {
        self.health_status.read().ok()?.get(name).cloned()
    }

    /// Cleanup all plugins
    pub fn cleanup_all(&self) -> Vec<String> {
        let mut errors = Vec::new();

        for (name, plugin) in &self.plugins {
            if let Err(e) = plugin.cleanup() {
                let error_msg = format!("Failed to cleanup {}: {}", name, e);
                errors.push(error_msg);
            }
        }

        if errors.is_empty() {
            info!("Successfully cleaned up all plugins");
        } else {
            warn!("Plugin cleanup completed with {} errors", errors.len());
        }

        errors
    }

    /// Get total resource usage across all plugins
    pub fn get_total_resource_usage(&self) -> ResourceUsage {
        let mut total = ResourceUsage::default();

        for plugin in self.plugins.values() {
            let usage = plugin.resource_usage();
            total.memory_mb += usage.memory_mb;
            total.file_descriptors += usage.file_descriptors;
            total.network_connections += usage.network_connections;
            total.cpu_percentage = total.cpu_percentage.saturating_add(usage.cpu_percentage);
        }

        total
    }
}

impl Default for ApiRegistry {
    fn default() -> Self {
        Self::new()
    }
}
