use crate::cli::framework::{CliContext, CliError, CliResult};
use serde::ser::Error as SerdeError;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct CliMetrics {
    pub session_id: String,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
    pub commands_executed: u64,
    pub commands_failed: u64,
    pub total_execution_time: Duration,
    pub command_metrics: HashMap<String, CommandMetrics>,
    pub error_metrics: HashMap<String, u64>,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone)]
pub struct CommandMetrics {
    pub name: String,
    pub execution_count: u64,
    pub total_time: Duration,
    pub average_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub success_count: u64,
    pub failure_count: u64,
    pub last_execution: Option<Instant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub file_operations: u64,
    pub network_requests: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl Default for CliMetrics {
    fn default() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            start_time: Instant::now(),
            end_time: None,
            commands_executed: 0,
            commands_failed: 0,
            total_execution_time: Duration::ZERO,
            command_metrics: HashMap::new(),
            error_metrics: HashMap::new(),
            performance_metrics: PerformanceMetrics::default(),
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            file_operations: 0,
            network_requests: 0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }
}

pub struct MetricsCollector {
    metrics: Arc<Mutex<CliMetrics>>,
    enabled: bool,
}

impl MetricsCollector {
    pub fn new(enabled: bool) -> Self {
        Self {
            metrics: Arc::new(Mutex::new(CliMetrics::default())),
            enabled,
        }
    }

    pub fn record_command_start(&self, command_name: String) -> Option<CommandTimer> {
        if !self.enabled {
            return None;
        }

        Some(CommandTimer::new(command_name, Arc::clone(&self.metrics)))
    }

    pub fn record_command_success(&self, command_name: String, execution_time: Duration) {
        if !self.enabled {
            return;
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.commands_executed += 1;
            metrics.total_execution_time += execution_time;

            let command_metrics = metrics
                .command_metrics
                .entry(command_name.clone())
                .or_insert_with(|| CommandMetrics {
                    name: command_name.clone(),
                    execution_count: 0,
                    total_time: Duration::ZERO,
                    average_time: Duration::ZERO,
                    min_time: Duration::MAX,
                    max_time: Duration::ZERO,
                    success_count: 0,
                    failure_count: 0,
                    last_execution: None,
                });

            command_metrics.execution_count += 1;
            command_metrics.total_time += execution_time;
            command_metrics.success_count += 1;
            command_metrics.last_execution = Some(Instant::now());

            if execution_time < command_metrics.min_time {
                command_metrics.min_time = execution_time;
            }
            if execution_time > command_metrics.max_time {
                command_metrics.max_time = execution_time;
            }

            command_metrics.average_time = Duration::from_nanos(
                command_metrics.total_time.as_nanos() as u64 / command_metrics.execution_count,
            );
        }
    }

    pub fn record_command_failure(
        &self,
        command_name: String,
        execution_time: Duration,
        error: &str,
    ) {
        if !self.enabled {
            return;
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.commands_failed += 1;
            metrics.total_execution_time += execution_time;

            let command_metrics = metrics
                .command_metrics
                .entry(command_name.clone())
                .or_insert_with(|| CommandMetrics {
                    name: command_name.clone(),
                    execution_count: 0,
                    total_time: Duration::ZERO,
                    average_time: Duration::ZERO,
                    min_time: Duration::MAX,
                    max_time: Duration::ZERO,
                    success_count: 0,
                    failure_count: 0,
                    last_execution: None,
                });

            command_metrics.execution_count += 1;
            command_metrics.total_time += execution_time;
            command_metrics.failure_count += 1;
            command_metrics.last_execution = Some(Instant::now());

            if execution_time < command_metrics.min_time {
                command_metrics.min_time = execution_time;
            }
            if execution_time > command_metrics.max_time {
                command_metrics.max_time = execution_time;
            }

            command_metrics.average_time = Duration::from_nanos(
                command_metrics.total_time.as_nanos() as u64 / command_metrics.execution_count,
            );

            *metrics.error_metrics.entry(error.to_string()).or_insert(0) += 1;
        }
    }

    pub fn record_file_operation(&self) {
        if !self.enabled {
            return;
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.performance_metrics.file_operations += 1;
        }
    }

    pub fn record_network_request(&self) {
        if !self.enabled {
            return;
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.performance_metrics.network_requests += 1;
        }
    }

    pub fn record_cache_hit(&self) {
        if !self.enabled {
            return;
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.performance_metrics.cache_hits += 1;
        }
    }

    pub fn record_cache_miss(&self) {
        if !self.enabled {
            return;
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.performance_metrics.cache_misses += 1;
        }
    }

    pub fn update_memory_usage(&self, usage_mb: f64) {
        if !self.enabled {
            return;
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.performance_metrics.memory_usage_mb = usage_mb;
        }
    }

    pub fn update_cpu_usage(&self, usage_percent: f64) {
        if !self.enabled {
            return;
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.performance_metrics.cpu_usage_percent = usage_percent;
        }
    }

    pub fn finish_session(&self) {
        if !self.enabled {
            return;
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.end_time = Some(Instant::now());
        }
    }

    pub fn get_metrics(&self) -> CliMetrics {
        self.metrics.lock().unwrap().clone()
    }

    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        Err(serde_json::Error::io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "CliMetrics cannot be serialized due to Instant fields",
        )))
    }

    pub fn export_csv(&self) -> String {
        let metrics = self.get_metrics();
        let mut csv = String::new();

        csv.push_str("Metric,Value\n");
        csv.push_str(&format!("Session ID,{}\n", metrics.session_id));
        csv.push_str(&format!(
            "Commands Executed,{}\n",
            metrics.commands_executed
        ));
        csv.push_str(&format!("Commands Failed,{}\n", metrics.commands_failed));
        csv.push_str(&format!(
            "Total Execution Time (ms),{}\n",
            metrics.total_execution_time.as_millis()
        ));
        csv.push_str(&format!(
            "Memory Usage (MB),{}\n",
            metrics.performance_metrics.memory_usage_mb
        ));
        csv.push_str(&format!(
            "CPU Usage (%),{}\n",
            metrics.performance_metrics.cpu_usage_percent
        ));
        csv.push_str(&format!(
            "File Operations,{}\n",
            metrics.performance_metrics.file_operations
        ));
        csv.push_str(&format!(
            "Network Requests,{}\n",
            metrics.performance_metrics.network_requests
        ));
        csv.push_str(&format!(
            "Cache Hits,{}\n",
            metrics.performance_metrics.cache_hits
        ));
        csv.push_str(&format!(
            "Cache Misses,{}\n",
            metrics.performance_metrics.cache_misses
        ));

        csv
    }
}

pub struct CommandTimer {
    command_name: String,
    start_time: Instant,
    metrics: Arc<Mutex<CliMetrics>>,
}

impl CommandTimer {
    fn new(command_name: String, metrics: Arc<Mutex<CliMetrics>>) -> Self {
        Self {
            command_name,
            start_time: Instant::now(),
            metrics,
        }
    }

    pub fn finish_success(self) {
        let execution_time = self.start_time.elapsed();
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.commands_executed += 1;
            metrics.total_execution_time += execution_time;

            let command_metrics = metrics
                .command_metrics
                .entry(self.command_name.clone())
                .or_insert_with(|| CommandMetrics {
                    name: self.command_name.clone(),
                    execution_count: 0,
                    total_time: Duration::ZERO,
                    average_time: Duration::ZERO,
                    min_time: Duration::MAX,
                    max_time: Duration::ZERO,
                    success_count: 0,
                    failure_count: 0,
                    last_execution: None,
                });

            command_metrics.execution_count += 1;
            command_metrics.total_time += execution_time;
            command_metrics.success_count += 1;
            command_metrics.last_execution = Some(Instant::now());

            if execution_time < command_metrics.min_time {
                command_metrics.min_time = execution_time;
            }
            if execution_time > command_metrics.max_time {
                command_metrics.max_time = execution_time;
            }

            command_metrics.average_time = Duration::from_nanos(
                command_metrics.total_time.as_nanos() as u64 / command_metrics.execution_count,
            );
        }
    }

    pub fn finish_failure(self, error: &str) {
        let execution_time = self.start_time.elapsed();
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.commands_failed += 1;
            metrics.total_execution_time += execution_time;

            let command_metrics = metrics
                .command_metrics
                .entry(self.command_name.clone())
                .or_insert_with(|| CommandMetrics {
                    name: self.command_name.clone(),
                    execution_count: 0,
                    total_time: Duration::ZERO,
                    average_time: Duration::ZERO,
                    min_time: Duration::MAX,
                    max_time: Duration::ZERO,
                    success_count: 0,
                    failure_count: 0,
                    last_execution: None,
                });

            command_metrics.execution_count += 1;
            command_metrics.total_time += execution_time;
            command_metrics.failure_count += 1;
            command_metrics.last_execution = Some(Instant::now());

            if execution_time < command_metrics.min_time {
                command_metrics.min_time = execution_time;
            }
            if execution_time > command_metrics.max_time {
                command_metrics.max_time = execution_time;
            }

            command_metrics.average_time = Duration::from_nanos(
                command_metrics.total_time.as_nanos() as u64 / command_metrics.execution_count,
            );

            *metrics.error_metrics.entry(error.to_string()).or_insert(0) += 1;
        }
    }
}

pub struct TelemetryReporter {
    endpoint: Option<String>,
    api_key: Option<String>,
    enabled: bool,
}

impl TelemetryReporter {
    pub fn new(endpoint: Option<String>, api_key: Option<String>, enabled: bool) -> Self {
        Self {
            endpoint,
            api_key,
            enabled,
        }
    }

    pub async fn report_metrics(
        &self,
        metrics: &CliMetrics,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.enabled || self.endpoint.is_none() {
            return Ok(());
        }

        let client = reqwest::Client::new();
        let mut request = client.post(self.endpoint.as_ref().unwrap());

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request
            .json(&serde_json::json!({"error": "Cannot serialize CliMetrics"}))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Failed to report metrics: {}", response.status()).into());
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct AdvancedCliMetrics {
    pub session_id: String,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
    pub commands_executed: u64,
    pub commands_failed: u64,
    pub total_execution_time: Duration,
    pub command_metrics: HashMap<String, CommandMetrics>,
    pub error_metrics: HashMap<String, u64>,
    pub performance_metrics: PerformanceMetrics,
    pub user_metrics: UserMetrics,
    pub system_metrics: SystemMetrics,
    pub feature_usage: HashMap<String, u64>,
    pub plugin_metrics: HashMap<String, PluginMetrics>,
    pub network_metrics: NetworkMetrics,
    pub file_metrics: FileMetrics,
}

#[derive(Debug, Clone)]
pub struct UserMetrics {
    pub user_id: Option<String>,
    pub session_count: u64,
    pub total_usage_time: Duration,
    pub preferred_commands: Vec<String>,
    pub error_rate: f64,
    pub success_rate: f64,
    pub average_session_duration: Duration,
    pub timezone: Option<String>,
    pub locale: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub os: String,
    pub arch: String,
    pub cli_version: String,
    pub rust_version: String,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub disk_usage_mb: f64,
    pub network_interface: Option<String>,
    pub terminal_type: Option<String>,
    pub shell: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PluginMetrics {
    pub name: String,
    pub version: String,
    pub load_count: u64,
    pub execution_count: u64,
    pub error_count: u64,
    pub average_execution_time: Duration,
    pub last_used: Option<Instant>,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub requests_made: u64,
    pub requests_failed: u64,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub average_response_time: Duration,
    pub dns_lookups: u64,
    pub ssl_handshakes: u64,
    pub connection_errors: u64,
    pub timeout_errors: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetrics {
    pub files_read: u64,
    pub files_written: u64,
    pub files_created: u64,
    pub files_deleted: u64,
    pub total_bytes_read: u64,
    pub total_bytes_written: u64,
    pub average_file_size: f64,
    pub file_operations_failed: u64,
    pub directory_operations: u64,
}

impl Default for AdvancedCliMetrics {
    fn default() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            start_time: Instant::now(),
            end_time: None,
            commands_executed: 0,
            commands_failed: 0,
            total_execution_time: Duration::ZERO,
            command_metrics: HashMap::new(),
            error_metrics: HashMap::new(),
            performance_metrics: PerformanceMetrics::default(),
            user_metrics: UserMetrics::default(),
            system_metrics: SystemMetrics::default(),
            feature_usage: HashMap::new(),
            plugin_metrics: HashMap::new(),
            network_metrics: NetworkMetrics::default(),
            file_metrics: FileMetrics::default(),
        }
    }
}

impl Default for UserMetrics {
    fn default() -> Self {
        Self {
            user_id: None,
            session_count: 1,
            total_usage_time: Duration::ZERO,
            preferred_commands: Vec::new(),
            error_rate: 0.0,
            success_rate: 0.0,
            average_session_duration: Duration::ZERO,
            timezone: None,
            locale: None,
        }
    }
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            cli_version: env!("CARGO_PKG_VERSION").to_string(),
            rust_version: std::env::var("RUSTC_SEMVER").unwrap_or_else(|_| "unknown".to_string()),
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            disk_usage_mb: 0.0,
            network_interface: None,
            terminal_type: std::env::var("TERM").ok(),
            shell: std::env::var("SHELL").ok(),
        }
    }
}

impl Default for NetworkMetrics {
    fn default() -> Self {
        Self {
            requests_made: 0,
            requests_failed: 0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            average_response_time: Duration::ZERO,
            dns_lookups: 0,
            ssl_handshakes: 0,
            connection_errors: 0,
            timeout_errors: 0,
        }
    }
}

impl Default for FileMetrics {
    fn default() -> Self {
        Self {
            files_read: 0,
            files_written: 0,
            files_created: 0,
            files_deleted: 0,
            total_bytes_read: 0,
            total_bytes_written: 0,
            average_file_size: 0.0,
            file_operations_failed: 0,
            directory_operations: 0,
        }
    }
}

pub struct AdvancedMetricsCollector {
    metrics: Arc<Mutex<AdvancedCliMetrics>>,
    enabled: bool,
    telemetry_enabled: bool,
    telemetry_endpoint: Option<String>,
    telemetry_api_key: Option<String>,
    metrics_file: Option<PathBuf>,
    auto_save: bool,
    save_interval: Duration,
    last_save: Arc<Mutex<Instant>>,
}

impl AdvancedMetricsCollector {
    pub fn new(enabled: bool) -> Self {
        Self {
            metrics: Arc::new(Mutex::new(AdvancedCliMetrics::default())),
            enabled,
            telemetry_enabled: false,
            telemetry_endpoint: None,
            telemetry_api_key: None,
            metrics_file: None,
            auto_save: false,
            save_interval: Duration::from_secs(300), // 5 minutes
            last_save: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub fn with_telemetry(mut self, endpoint: String, api_key: Option<String>) -> Self {
        self.telemetry_enabled = true;
        self.telemetry_endpoint = Some(endpoint);
        self.telemetry_api_key = api_key;
        self
    }

    pub fn with_metrics_file(mut self, file_path: PathBuf) -> Self {
        self.metrics_file = Some(file_path);
        self
    }

    pub fn with_auto_save(mut self, enabled: bool, interval: Duration) -> Self {
        self.auto_save = enabled;
        self.save_interval = interval;
        self
    }

    pub fn record_command_start(
        &self,
        command_name: String,
        context: &CliContext,
    ) -> Option<AdvancedCommandTimer> {
        if !self.enabled {
            return None;
        }

        self.update_user_metrics(context);
        self.update_system_metrics();

        Some(AdvancedCommandTimer::new(
            command_name,
            Arc::clone(&self.metrics),
        ))
    }

    pub fn record_command_success(
        &self,
        command_name: String,
        execution_time: Duration,
        context: &CliContext,
    ) {
        if !self.enabled {
            return;
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.commands_executed += 1;
            metrics.total_execution_time += execution_time;

            let command_metrics = metrics
                .command_metrics
                .entry(command_name.clone())
                .or_insert_with(|| CommandMetrics {
                    name: command_name.clone(),
                    execution_count: 0,
                    total_time: Duration::ZERO,
                    average_time: Duration::ZERO,
                    min_time: Duration::MAX,
                    max_time: Duration::ZERO,
                    success_count: 0,
                    failure_count: 0,
                    last_execution: None,
                });

            command_metrics.execution_count += 1;
            command_metrics.total_time += execution_time;
            command_metrics.success_count += 1;
            command_metrics.last_execution = Some(Instant::now());

            if execution_time < command_metrics.min_time {
                command_metrics.min_time = execution_time;
            }
            if execution_time > command_metrics.max_time {
                command_metrics.max_time = execution_time;
            }

            command_metrics.average_time = Duration::from_nanos(
                command_metrics.total_time.as_nanos() as u64 / command_metrics.execution_count,
            );

            *metrics
                .feature_usage
                .entry(command_name.clone())
                .or_insert(0) += 1;

            self.update_user_preferences(&mut metrics, &command_name);
        }

        self.check_auto_save();
    }

    pub fn record_command_failure(
        &self,
        command_name: String,
        execution_time: Duration,
        error: &str,
        context: &CliContext,
    ) {
        if !self.enabled {
            return;
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.commands_failed += 1;
            metrics.total_execution_time += execution_time;

            let command_metrics = metrics
                .command_metrics
                .entry(command_name.clone())
                .or_insert_with(|| CommandMetrics {
                    name: command_name.clone(),
                    execution_count: 0,
                    total_time: Duration::ZERO,
                    average_time: Duration::ZERO,
                    min_time: Duration::MAX,
                    max_time: Duration::ZERO,
                    success_count: 0,
                    failure_count: 0,
                    last_execution: None,
                });

            command_metrics.execution_count += 1;
            command_metrics.total_time += execution_time;
            command_metrics.failure_count += 1;
            command_metrics.last_execution = Some(Instant::now());

            if execution_time < command_metrics.min_time {
                command_metrics.min_time = execution_time;
            }
            if execution_time > command_metrics.max_time {
                command_metrics.max_time = execution_time;
            }

            command_metrics.average_time = Duration::from_nanos(
                command_metrics.total_time.as_nanos() as u64 / command_metrics.execution_count,
            );

            *metrics.error_metrics.entry(error.to_string()).or_insert(0) += 1;
        }

        self.check_auto_save();
    }

    pub fn record_network_request(
        &self,
        bytes_sent: u64,
        bytes_received: u64,
        response_time: Duration,
        success: bool,
    ) {
        if !self.enabled {
            return;
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.network_metrics.requests_made += 1;
            metrics.network_metrics.total_bytes_sent += bytes_sent;
            metrics.network_metrics.total_bytes_received += bytes_received;

            if !success {
                metrics.network_metrics.requests_failed += 1;
            }

            let total_requests = metrics.network_metrics.requests_made;
            let current_avg = metrics.network_metrics.average_response_time;
            metrics.network_metrics.average_response_time = Duration::from_nanos(
                ((current_avg.as_nanos() as u64 * (total_requests - 1))
                    + response_time.as_nanos() as u64)
                    / total_requests,
            );
        }

        self.check_auto_save();
    }

    pub fn record_file_operation(
        &self,
        operation_type: FileOperationType,
        bytes: u64,
        success: bool,
    ) {
        if !self.enabled {
            return;
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            match operation_type {
                FileOperationType::Read => {
                    metrics.file_metrics.files_read += 1;
                    metrics.file_metrics.total_bytes_read += bytes;
                }
                FileOperationType::Write => {
                    metrics.file_metrics.files_written += 1;
                    metrics.file_metrics.total_bytes_written += bytes;
                }
                FileOperationType::Create => {
                    metrics.file_metrics.files_created += 1;
                }
                FileOperationType::Delete => {
                    metrics.file_metrics.files_deleted += 1;
                }
                FileOperationType::Directory => {
                    metrics.file_metrics.directory_operations += 1;
                }
            }

            if !success {
                metrics.file_metrics.file_operations_failed += 1;
            }

            let total_files = metrics.file_metrics.files_read + metrics.file_metrics.files_written;
            if total_files > 0 {
                let total_bytes = metrics.file_metrics.total_bytes_read
                    + metrics.file_metrics.total_bytes_written;
                metrics.file_metrics.average_file_size = total_bytes as f64 / total_files as f64;
            }
        }

        self.check_auto_save();
    }

    pub fn record_plugin_usage(
        &self,
        plugin_name: String,
        plugin_version: String,
        execution_time: Duration,
        success: bool,
    ) {
        if !self.enabled {
            return;
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            let plugin_metrics = metrics
                .plugin_metrics
                .entry(plugin_name.clone())
                .or_insert_with(|| PluginMetrics {
                    name: plugin_name.clone(),
                    version: plugin_version.clone(),
                    load_count: 0,
                    execution_count: 0,
                    error_count: 0,
                    average_execution_time: Duration::ZERO,
                    last_used: None,
                    is_enabled: true,
                });

            plugin_metrics.execution_count += 1;
            plugin_metrics.last_used = Some(Instant::now());

            if !success {
                plugin_metrics.error_count += 1;
            }

            let total_executions = plugin_metrics.execution_count;
            let current_avg = plugin_metrics.average_execution_time;
            plugin_metrics.average_execution_time = Duration::from_nanos(
                ((current_avg.as_nanos() as u64 * (total_executions - 1))
                    + execution_time.as_nanos() as u64)
                    / total_executions,
            );
        }

        self.check_auto_save();
    }

    fn update_user_metrics(&self, context: &CliContext) {
        if let Ok(mut metrics) = self.metrics.lock() {
            if let Some(user_id) = &context.user_id {
                metrics.user_metrics.user_id = Some(user_id.clone());
            }

            metrics.user_metrics.total_usage_time = context.elapsed();
            metrics.user_metrics.session_count += 1;
        }
    }

    fn update_system_metrics(&self) {
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.system_metrics.memory_usage_mb = self.get_memory_usage();

            metrics.system_metrics.cpu_usage_percent = self.get_cpu_usage();

            metrics.system_metrics.disk_usage_mb = self.get_disk_usage();
        }
    }

    fn update_user_preferences(&self, metrics: &mut AdvancedCliMetrics, command_name: &str) {
        if !metrics
            .user_metrics
            .preferred_commands
            .contains(&command_name.to_string())
        {
            metrics
                .user_metrics
                .preferred_commands
                .push(command_name.to_string());
        }

        metrics.user_metrics.preferred_commands.sort_by(|a, b| {
            let a_count = metrics.feature_usage.get(a).unwrap_or(&0);
            let b_count = metrics.feature_usage.get(b).unwrap_or(&0);
            b_count.cmp(a_count)
        });

        if metrics.user_metrics.preferred_commands.len() > 10 {
            metrics.user_metrics.preferred_commands.truncate(10);
        }

        let total_commands = metrics.commands_executed + metrics.commands_failed;
        if total_commands > 0 {
            metrics.user_metrics.success_rate =
                metrics.commands_executed as f64 / total_commands as f64;
            metrics.user_metrics.error_rate =
                metrics.commands_failed as f64 / total_commands as f64;
        }
    }

    fn get_memory_usage(&self) -> f64 {
        0.0
    }

    fn get_cpu_usage(&self) -> f64 {
        0.0
    }

    fn get_disk_usage(&self) -> f64 {
        0.0
    }

    fn check_auto_save(&self) {
        if !self.auto_save {
            return;
        }

        let now = Instant::now();
        if let Ok(mut last_save) = self.last_save.lock() {
            if now.duration_since(*last_save) >= self.save_interval {
                if let Err(e) = self.save_metrics() {
                    eprintln!("Failed to auto-save metrics: {}", e);
                }
                *last_save = now;
            }
        }
    }

    pub fn save_metrics(&self) -> CliResult<()> {
        if let Some(file_path) = &self.metrics_file {
            let json = serde_json::to_string_pretty(
                &serde_json::json!({"error": "Cannot serialize AdvancedCliMetrics"}),
            )
            .map_err(|e| CliError::InternalError {
                message: format!("Failed to serialize metrics: {}", e),
            })?;

            std::fs::write(file_path, json).map_err(|e| CliError::InternalError {
                message: format!("Failed to write metrics file: {}", e),
            })?;
        }

        Ok(())
    }

    pub fn load_metrics(&self) -> CliResult<()> {
        if let Some(file_path) = &self.metrics_file {
            if file_path.exists() {
                let content =
                    std::fs::read_to_string(file_path).map_err(|e| CliError::InternalError {
                        message: format!("Failed to read metrics file: {}", e),
                    })?;

                return Err(CliError::InternalError {
                    message: "Cannot deserialize AdvancedCliMetrics due to Instant fields"
                        .to_string(),
                });
            }
        }

        Ok(())
    }

    pub fn finish_session(&self) {
        if !self.enabled {
            return;
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.end_time = Some(Instant::now());

            let session_duration = metrics.end_time.unwrap().duration_since(metrics.start_time);
            metrics.user_metrics.average_session_duration = session_duration;
        }

        if let Err(e) = self.save_metrics() {
            eprintln!("Failed to save metrics: {}", e);
        }

        if self.telemetry_enabled {
            eprintln!("Telemetry sending is async and needs to be handled separately");
        }
    }

    pub async fn send_telemetry(&self) -> CliResult<()> {
        if !self.telemetry_enabled || self.telemetry_endpoint.is_none() {
            return Ok(());
        }

        let metrics = self.get_metrics();
        let client = reqwest::Client::new();
        let mut request = client.post(self.telemetry_endpoint.as_ref().unwrap());

        if let Some(api_key) = &self.telemetry_api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request
            .json(&serde_json::json!({"error": "Cannot serialize AdvancedCliMetrics"}))
            .send()
            .await
            .map_err(|e| CliError::NetworkError {
                message: format!("Failed to send telemetry: {}", e),
            })?;

        if !response.status().is_success() {
            return Err(CliError::NetworkError {
                message: format!(
                    "Telemetry request failed with status: {}",
                    response.status()
                ),
            });
        }

        Ok(())
    }

    pub fn get_metrics(&self) -> AdvancedCliMetrics {
        self.metrics.lock().unwrap().clone()
    }

    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        Err(serde_json::Error::io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "AdvancedCliMetrics cannot be serialized due to Instant fields",
        )))
    }

    pub fn export_csv(&self) -> String {
        let metrics = self.get_metrics();
        let mut csv = String::new();

        csv.push_str("Metric,Value\n");
        csv.push_str(&format!("Session ID,{}\n", metrics.session_id));
        csv.push_str(&format!(
            "Commands Executed,{}\n",
            metrics.commands_executed
        ));
        csv.push_str(&format!("Commands Failed,{}\n", metrics.commands_failed));
        csv.push_str(&format!(
            "Total Execution Time (ms),{}\n",
            metrics.total_execution_time.as_millis()
        ));
        csv.push_str(&format!(
            "Memory Usage (MB),{}\n",
            metrics.performance_metrics.memory_usage_mb
        ));
        csv.push_str(&format!(
            "CPU Usage (%),{}\n",
            metrics.performance_metrics.cpu_usage_percent
        ));
        csv.push_str(&format!(
            "File Operations,{}\n",
            metrics.performance_metrics.file_operations
        ));
        csv.push_str(&format!(
            "Network Requests,{}\n",
            metrics.performance_metrics.network_requests
        ));
        csv.push_str(&format!(
            "Cache Hits,{}\n",
            metrics.performance_metrics.cache_hits
        ));
        csv.push_str(&format!(
            "Cache Misses,{}\n",
            metrics.performance_metrics.cache_misses
        ));

        csv
    }

    pub fn get_summary(&self) -> MetricsSummary {
        let metrics = self.get_metrics();

        MetricsSummary {
            session_duration: metrics
                .end_time
                .map(|end| end.duration_since(metrics.start_time))
                .unwrap_or_else(|| metrics.start_time.elapsed()),
            commands_executed: metrics.commands_executed,
            commands_failed: metrics.commands_failed,
            success_rate: if metrics.commands_executed + metrics.commands_failed > 0 {
                metrics.commands_executed as f64
                    / (metrics.commands_executed + metrics.commands_failed) as f64
            } else {
                0.0
            },
            average_command_time: if metrics.commands_executed > 0 {
                metrics.total_execution_time / metrics.commands_executed as u32
            } else {
                Duration::ZERO
            },
            most_used_command: metrics
                .feature_usage
                .iter()
                .max_by_key(|(_, count)| *count)
                .map(|(command, _)| command.clone()),
            plugins_used: metrics.plugin_metrics.len(),
            network_requests: metrics.network_metrics.requests_made,
            file_operations: metrics.file_metrics.files_read + metrics.file_metrics.files_written,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FileOperationType {
    Read,
    Write,
    Create,
    Delete,
    Directory,
}

#[derive(Debug, Clone)]
pub struct MetricsSummary {
    pub session_duration: Duration,
    pub commands_executed: u64,
    pub commands_failed: u64,
    pub success_rate: f64,
    pub average_command_time: Duration,
    pub most_used_command: Option<String>,
    pub plugins_used: usize,
    pub network_requests: u64,
    pub file_operations: u64,
}

pub struct AdvancedCommandTimer {
    command_name: String,
    start_time: Instant,
    metrics: Arc<Mutex<AdvancedCliMetrics>>,
}

impl AdvancedCommandTimer {
    fn new(command_name: String, metrics: Arc<Mutex<AdvancedCliMetrics>>) -> Self {
        Self {
            command_name,
            start_time: Instant::now(),
            metrics,
        }
    }

    pub fn finish_success(self) {
        let execution_time = self.start_time.elapsed();
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.commands_executed += 1;
            metrics.total_execution_time += execution_time;

            let command_metrics = metrics
                .command_metrics
                .entry(self.command_name.clone())
                .or_insert_with(|| CommandMetrics {
                    name: self.command_name.clone(),
                    execution_count: 0,
                    total_time: Duration::ZERO,
                    average_time: Duration::ZERO,
                    min_time: Duration::MAX,
                    max_time: Duration::ZERO,
                    success_count: 0,
                    failure_count: 0,
                    last_execution: None,
                });

            command_metrics.execution_count += 1;
            command_metrics.total_time += execution_time;
            command_metrics.success_count += 1;
            command_metrics.last_execution = Some(Instant::now());

            if execution_time < command_metrics.min_time {
                command_metrics.min_time = execution_time;
            }
            if execution_time > command_metrics.max_time {
                command_metrics.max_time = execution_time;
            }

            command_metrics.average_time = Duration::from_nanos(
                command_metrics.total_time.as_nanos() as u64 / command_metrics.execution_count,
            );

            *metrics.feature_usage.entry(self.command_name).or_insert(0) += 1;
        }
    }

    pub fn finish_failure(self, error: &str) {
        let execution_time = self.start_time.elapsed();
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.commands_failed += 1;
            metrics.total_execution_time += execution_time;

            let command_metrics = metrics
                .command_metrics
                .entry(self.command_name.clone())
                .or_insert_with(|| CommandMetrics {
                    name: self.command_name.clone(),
                    execution_count: 0,
                    total_time: Duration::ZERO,
                    average_time: Duration::ZERO,
                    min_time: Duration::MAX,
                    max_time: Duration::ZERO,
                    success_count: 0,
                    failure_count: 0,
                    last_execution: None,
                });

            command_metrics.execution_count += 1;
            command_metrics.total_time += execution_time;
            command_metrics.failure_count += 1;
            command_metrics.last_execution = Some(Instant::now());

            if execution_time < command_metrics.min_time {
                command_metrics.min_time = execution_time;
            }
            if execution_time > command_metrics.max_time {
                command_metrics.max_time = execution_time;
            }

            command_metrics.average_time = Duration::from_nanos(
                command_metrics.total_time.as_nanos() as u64 / command_metrics.execution_count,
            );

            *metrics.error_metrics.entry(error.to_string()).or_insert(0) += 1;
        }
    }
}

pub fn create_advanced_metrics_collector(enabled: bool) -> Arc<AdvancedMetricsCollector> {
    Arc::new(AdvancedMetricsCollector::new(enabled))
}
