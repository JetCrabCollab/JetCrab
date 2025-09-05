//! # Built-in APIs
//!
//! This module contains all the built-in APIs available in the JetCrab runtime.

pub mod direct_apis;
pub mod native_apis;

pub mod batch_registry;
pub mod config;
pub mod error;
pub mod graceful_degradation;
pub mod health_checker;
pub mod lazy_loader;
pub mod metrics;
pub mod plugin;
pub mod registry;
pub mod resource_manager;

pub mod core;
pub mod crypto;
pub mod experimental;
pub mod filesystem;
pub mod networking;
pub mod system;
pub mod utility;

pub use batch_registry::{BatchApiRegistry, BatchOperation, BatchStats};
pub use config::ApiConfig;
pub use error::{ApiError, ApiResult, ToApiError};
pub use graceful_degradation::{DegradationLevel, FallbackStrategy, GracefulDegradationManager};
pub use health_checker::{
    ApiHealthChecker, HealthCheckConfig, HealthCheckHistory, HealthCheckResult, SystemHealth,
};
pub use lazy_loader::{LazyApiLoader, LoadingStats};
pub use metrics::{
    ApiMetricsCollector, HealthMetrics, MetricsConfig, MetricsReport, PerformanceMetrics,
    ResourceMetrics,
};
pub use plugin::{ApiMetrics, ApiPlugin, HealthStatus, ResourceUsage};
pub use registry::ApiRegistry;
pub use resource_manager::{ResourceLimits, ResourceManager, ResourceMonitoring, ResourceSnapshot};

use direct_apis::DirectAPIs;
use native_apis::NativeAPIs;
use tracing::debug;

/// Built-in APIs manager
pub struct BuiltinAPIs {
    available_apis: Vec<String>,
    #[allow(dead_code)]
    direct_apis: DirectAPIs,
    #[allow(dead_code)]
    native_apis: NativeAPIs,
}

impl BuiltinAPIs {
    pub fn new() -> Self {
        Self {
            available_apis: vec![
                "console".to_string(),
                "fs".to_string(),
                "os".to_string(),
                "http".to_string(),
                "path".to_string(),
                "process".to_string(),
                "require".to_string(),
                "child_process".to_string(),
                "crypto".to_string(),
                "url".to_string(),
                "util".to_string(),
                "events".to_string(),
                "stream".to_string(),
                "buffer".to_string(),
                "querystring".to_string(),
                "timers".to_string(),
            ],
            direct_apis: DirectAPIs::new(),
            native_apis: NativeAPIs::new(),
        }
    }

    /// Setup built-in APIs in the engine
    pub fn setup(
        &mut self,
        engine: &mut crate::runtime::engine::JetCrabEngine,
    ) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Setting up built-in APIs: {:?}", self.available_apis);

        self.native_apis.register(engine.get_context())?;
        debug!("Native APIs registered");

        self.direct_apis.register(engine.get_context())?;
        debug!("Direct APIs registered");

        debug!("All built-in APIs setup completed");
        Ok(())
    }
}

impl Default for BuiltinAPIs {
    fn default() -> Self {
        Self::new()
    }
}
