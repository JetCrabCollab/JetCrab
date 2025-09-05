//! # Batch API Registry
//!
//! This module provides batch registration functionality for better performance.

use crate::runtime::apis::{ApiConfig, ApiError, ApiPlugin, ApiRegistry, ApiResult};
use boa_engine::Context;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};

/// Batch registry for efficient API registration
pub struct BatchApiRegistry {
    /// Underlying registry
    registry: Arc<Mutex<ApiRegistry>>,
    /// Configuration
    config: ApiConfig,
    /// Batch processing queue
    queue: Arc<Mutex<VecDeque<BatchOperation>>>,
    /// Processing statistics
    stats: Arc<Mutex<BatchStats>>,
    /// Maximum batch size
    max_batch_size: usize,
    /// Batch timeout
    batch_timeout: Duration,
}

/// Batch operation types
pub enum BatchOperation {
    /// Register a single API
    Register {
        name: String,
        plugin: Box<dyn ApiPlugin>,
    },
    /// Register multiple APIs
    RegisterBatch {
        names: Vec<String>,
        plugins: Vec<Box<dyn ApiPlugin>>,
    },
    /// Register APIs by category
    RegisterCategory {
        category: String,
        names: Vec<String>,
    },
}

/// Statistics for batch operations
#[derive(Debug, Default)]
pub struct BatchStats {
    /// Total batches processed
    pub total_batches: usize,
    /// Total APIs registered in batches
    pub total_apis_registered: usize,
    /// Total batch processing time
    pub total_processing_time: Duration,
    /// Average batch size
    pub average_batch_size: f64,
    /// Average processing time per batch
    pub average_batch_time: Duration,
    /// Number of failed batches
    pub failed_batches: usize,
    /// Last batch time
    pub last_batch_time: Option<Instant>,
}

impl BatchApiRegistry {
    /// Create a new batch registry
    pub fn new(config: ApiConfig) -> Self {
        Self {
            registry: Arc::new(Mutex::new(ApiRegistry::new())),
            config,
            queue: Arc::new(Mutex::new(VecDeque::new())),
            stats: Arc::new(Mutex::new(BatchStats::default())),
            max_batch_size: 10,
            batch_timeout: Duration::from_millis(100),
        }
    }

    /// Set maximum batch size
    pub fn set_max_batch_size(&mut self, size: usize) {
        self.max_batch_size = size;
    }

    /// Set batch timeout
    pub fn set_batch_timeout(&mut self, timeout: Duration) {
        self.batch_timeout = timeout;
    }

    /// Queue a single API for batch registration
    pub fn queue_api(&self, name: &str, plugin: Box<dyn ApiPlugin>) -> ApiResult<()> {
        let operation = BatchOperation::Register {
            name: name.to_string(),
            plugin,
        };

        self.queue_operation(operation)
    }

    /// Queue multiple APIs for batch registration
    pub fn queue_apis(
        &self,
        names: Vec<String>,
        plugins: Vec<Box<dyn ApiPlugin>>,
    ) -> ApiResult<()> {
        if names.len() != plugins.len() {
            return Err(ApiError::configuration_error(
                "Number of names and plugins must match",
            ));
        }

        let operation = BatchOperation::RegisterBatch { names, plugins };
        self.queue_operation(operation)
    }

    /// Queue APIs by category for batch registration
    pub fn queue_category(&self, category: &str, names: Vec<String>) -> ApiResult<()> {
        let operation = BatchOperation::RegisterCategory {
            category: category.to_string(),
            names,
        };

        self.queue_operation(operation)
    }

    /// Process all queued operations
    pub fn process_batch(&self, context: &mut Context) -> ApiResult<BatchStats> {
        let start_time = Instant::now();
        let mut operations = self.drain_queue()?;

        if operations.is_empty() {
            debug!("No operations to process in batch");
            return Ok(self.get_stats());
        }

        info!("Processing batch of {} operations", operations.len());

        let mut batch_stats = BatchStats::default();
        let mut total_apis = 0;
        let mut errors = Vec::new();

        let mut register_ops = Vec::new();
        let mut batch_ops = Vec::new();
        let mut category_ops = Vec::new();

        for op in operations {
            match op {
                BatchOperation::Register { name, plugin } => {
                    register_ops.push((name, plugin));
                }
                BatchOperation::RegisterBatch { names, plugins } => {
                    batch_ops.push((names, plugins));
                }
                BatchOperation::RegisterCategory { category, names } => {
                    category_ops.push((category, names));
                }
            }
        }

        for (name, plugin) in register_ops {
            match self.register_single_api(&name, plugin, context) {
                Ok(()) => {
                    total_apis += 1;
                    debug!("Successfully registered API: {}", name);
                }
                Err(e) => {
                    errors.push(format!("Failed to register {}: {}", name, e));
                }
            }
        }

        for (names, plugins) in batch_ops {
            match self.register_batch_apis(names.clone(), plugins, context) {
                Ok(count) => {
                    total_apis += count;
                    debug!("Successfully registered batch of {} APIs", count);
                }
                Err(e) => {
                    errors.push(format!("Failed to register batch {:?}: {}", names, e));
                }
            }
        }

        for (category, names) in category_ops {
            match self.register_category_apis(&category, names.clone(), context) {
                Ok(count) => {
                    total_apis += count;
                    debug!(
                        "Successfully registered {} APIs for category: {}",
                        count, category
                    );
                }
                Err(e) => {
                    errors.push(format!("Failed to register category {}: {}", category, e));
                }
            }
        }

        let processing_time = start_time.elapsed();

        batch_stats.total_batches = 1;
        batch_stats.total_apis_registered = total_apis;
        batch_stats.total_processing_time = processing_time;
        batch_stats.average_batch_size = total_apis as f64;
        batch_stats.average_batch_time = processing_time;
        batch_stats.last_batch_time = Some(start_time);

        if !errors.is_empty() {
            batch_stats.failed_batches = 1;
            warn!(
                "Batch processing completed with {} errors: {:?}",
                errors.len(),
                errors
            );
        }

        self.update_stats(batch_stats.clone());

        info!(
            "Batch processing completed: {} APIs registered in {:?}",
            total_apis, processing_time
        );

        if !errors.is_empty() && !self.config.enable_graceful_degradation {
            return Err(ApiError::batch_registration_failed(
                errors.len(),
                total_apis + errors.len(),
            ));
        }

        Ok(batch_stats)
    }

    /// Process operations with automatic batching
    pub fn process_with_auto_batching(&self, context: &mut Context) -> ApiResult<()> {
        let mut processed_any = false;

        loop {
            let queue_size = self.get_queue_size();
            if queue_size == 0 {
                break;
            }

            if queue_size >= self.max_batch_size {
                self.process_batch(context)?;
                processed_any = true;
            } else {
                if let Some(last_op_time) = self.get_last_operation_time() {
                    if last_op_time.elapsed() >= self.batch_timeout {
                        self.process_batch(context)?;
                        processed_any = true;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        if processed_any {
            info!("Auto-batching completed");
        }

        Ok(())
    }

    /// Get current statistics
    pub fn get_stats(&self) -> BatchStats {
        self.stats
            .lock()
            .unwrap_or_else(|_| panic!("Failed to acquire stats lock"))
            .clone()
    }

    /// Get queue size
    pub fn get_queue_size(&self) -> usize {
        self.queue
            .lock()
            .unwrap_or_else(|_| panic!("Failed to acquire queue lock"))
            .len()
    }

    /// Clear the queue
    pub fn clear_queue(&self) -> ApiResult<()> {
        let mut queue = self
            .queue
            .lock()
            .map_err(|_| ApiError::configuration_error("Failed to acquire queue lock"))?;

        let cleared_count = queue.len();
        queue.clear();

        debug!("Cleared {} operations from queue", cleared_count);
        Ok(())
    }

    /// Get registry reference
    pub fn get_registry(&self) -> Arc<Mutex<ApiRegistry>> {
        self.registry.clone()
    }

    /// Queue an operation
    fn queue_operation(&self, operation: BatchOperation) -> ApiResult<()> {
        let mut queue = self
            .queue
            .lock()
            .map_err(|_| ApiError::configuration_error("Failed to acquire queue lock"))?;

        queue.push_back(operation);
        debug!("Queued operation for batch processing");
        Ok(())
    }

    /// Drain the queue
    fn drain_queue(&self) -> ApiResult<Vec<BatchOperation>> {
        let mut queue = self
            .queue
            .lock()
            .map_err(|_| ApiError::configuration_error("Failed to acquire queue lock"))?;

        let operations: Vec<BatchOperation> = queue.drain(..).collect();
        Ok(operations)
    }

    /// Register a single API
    fn register_single_api(
        &self,
        name: &str,
        plugin: Box<dyn ApiPlugin>,
        context: &mut Context,
    ) -> ApiResult<()> {
        let mut registry = self
            .registry
            .lock()
            .map_err(|_| ApiError::configuration_error("Failed to acquire registry lock"))?;

        registry.register_plugin(plugin);
        registry.register_in_context(name, context).map_err(|e| {
            ApiError::registration_failed(
                name,
                crate::runtime::apis::error::SimpleError::new(format!("{}", e)),
            )
        })?;

        Ok(())
    }

    /// Register multiple APIs in a batch
    fn register_batch_apis(
        &self,
        names: Vec<String>,
        plugins: Vec<Box<dyn ApiPlugin>>,
        context: &mut Context,
    ) -> ApiResult<usize> {
        let mut registry = self
            .registry
            .lock()
            .map_err(|_| ApiError::configuration_error("Failed to acquire registry lock"))?;

        let mut registered_count = 0;

        for (name, plugin) in names.iter().zip(plugins.into_iter()) {
            registry.register_plugin(plugin);
        }

        for name in &names {
            match registry.register_in_context(name, context) {
                Ok(()) => {
                    registered_count += 1;
                    debug!("Successfully registered API in batch: {}", name);
                }
                Err(e) => {
                    error!("Failed to register API in batch {}: {:?}", name, e);
                    if !self.config.enable_graceful_degradation {
                        return Err(ApiError::registration_failed(
                            name,
                            crate::runtime::apis::error::SimpleError::new(format!("{}", e)),
                        ));
                    }
                }
            }
        }

        Ok(registered_count)
    }

    /// Register APIs by category
    fn register_category_apis(
        &self,
        category: &str,
        names: Vec<String>,
        context: &mut Context,
    ) -> ApiResult<usize> {
        let mut registry = self
            .registry
            .lock()
            .map_err(|_| ApiError::configuration_error("Failed to acquire registry lock"))?;

        let mut registered_count = 0;

        for name in &names {
            if !self.config.is_api_enabled(name) {
                debug!("Skipping disabled API: {}", name);
                continue;
            }

            match registry.register_in_context(name, context) {
                Ok(()) => {
                    registered_count += 1;
                    debug!(
                        "Successfully registered API for category {}: {}",
                        category, name
                    );
                }
                Err(e) => {
                    error!(
                        "Failed to register API for category {} {}: {:?}",
                        category, name, e
                    );
                    if !self.config.enable_graceful_degradation {
                        return Err(ApiError::registration_failed(
                            name,
                            crate::runtime::apis::error::SimpleError::new(format!("{}", e)),
                        ));
                    }
                }
            }
        }

        Ok(registered_count)
    }

    /// Get last operation time
    fn get_last_operation_time(&self) -> Option<Instant> {
        None
    }

    /// Update statistics
    fn update_stats(&self, new_stats: BatchStats) {
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_batches += new_stats.total_batches;
            stats.total_apis_registered += new_stats.total_apis_registered;
            stats.total_processing_time += new_stats.total_processing_time;
            stats.failed_batches += new_stats.failed_batches;
            stats.last_batch_time = new_stats.last_batch_time;

            if stats.total_batches > 0 {
                stats.average_batch_size =
                    stats.total_apis_registered as f64 / stats.total_batches as f64;
                stats.average_batch_time = stats.total_processing_time / stats.total_batches as u32;
            }
        }
    }
}

impl Clone for BatchStats {
    fn clone(&self) -> Self {
        Self {
            total_batches: self.total_batches,
            total_apis_registered: self.total_apis_registered,
            total_processing_time: self.total_processing_time,
            average_batch_size: self.average_batch_size,
            average_batch_time: self.average_batch_time,
            failed_batches: self.failed_batches,
            last_batch_time: self.last_batch_time,
        }
    }
}
