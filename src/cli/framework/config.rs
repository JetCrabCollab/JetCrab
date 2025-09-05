use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    pub general: GeneralConfig,
    pub logging: LoggingConfig,
    pub network: NetworkConfig,
    pub security: SecurityConfig,
    pub performance: PerformanceConfig,
    pub features: FeatureConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub default_timeout_ms: u64,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub auto_update_check: bool,
    pub telemetry_enabled: bool,
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub output: String,
    pub file_path: Option<String>,
    pub max_file_size_mb: u64,
    pub max_files: u32,
    pub include_timestamp: bool,
    pub include_target: bool,
    pub color: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub timeout_ms: u64,
    pub max_redirects: u32,
    pub user_agent: String,
    pub proxy: Option<String>,
    pub verify_ssl: bool,
    pub connection_pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub allow_insecure: bool,
    pub max_file_size_mb: u64,
    pub allowed_extensions: Vec<String>,
    pub blocked_extensions: Vec<String>,
    pub sandbox_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub max_concurrent_operations: u32,
    pub cache_size_mb: u64,
    pub cache_ttl_seconds: u64,
    pub enable_compression: bool,
    pub chunk_size_kb: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureConfig {
    pub experimental_features: Vec<String>,
    pub disabled_features: Vec<String>,
    pub feature_flags: HashMap<String, bool>,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            logging: LoggingConfig::default(),
            network: NetworkConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
            features: FeatureConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            default_timeout_ms: 30000,
            max_retries: 3,
            retry_delay_ms: 1000,
            auto_update_check: true,
            telemetry_enabled: false,
            user_agent: "JetCrab/0.4.0".to_string(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "compact".to_string(),
            output: "stderr".to_string(),
            file_path: None,
            max_file_size_mb: 100,
            max_files: 5,
            include_timestamp: true,
            include_target: false,
            color: true,
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 30000,
            max_redirects: 5,
            user_agent: "JetCrab/0.4.0".to_string(),
            proxy: None,
            verify_ssl: true,
            connection_pool_size: 10,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            allow_insecure: false,
            max_file_size_mb: 100,
            allowed_extensions: vec![
                "js".to_string(),
                "ts".to_string(),
                "json".to_string(),
                "toml".to_string(),
                "yaml".to_string(),
                "yml".to_string(),
            ],
            blocked_extensions: vec![
                "exe".to_string(),
                "bat".to_string(),
                "cmd".to_string(),
                "sh".to_string(),
                "ps1".to_string(),
            ],
            sandbox_mode: true,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_concurrent_operations: 10,
            cache_size_mb: 50,
            cache_ttl_seconds: 3600,
            enable_compression: true,
            chunk_size_kb: 64,
        }
    }
}

impl Default for FeatureConfig {
    fn default() -> Self {
        Self {
            experimental_features: vec![],
            disabled_features: vec![],
            feature_flags: HashMap::new(),
        }
    }
}

pub struct ConfigManager {
    config: CliConfig,
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new(config_path: Option<PathBuf>) -> Self {
        let path = config_path.unwrap_or_else(|| {
            dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("jetcrab")
                .join("config.toml")
        });

        Self {
            config: CliConfig::default(),
            config_path: path,
        }
    }

    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config_path.exists() {
            let content = std::fs::read_to_string(&self.config_path)?;
            self.config = toml::from_str(&content)?;
        }
        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(&self.config)?;
        std::fs::write(&self.config_path, content)?;
        Ok(())
    }

    pub fn get(&self) -> &CliConfig {
        &self.config
    }

    pub fn get_mut(&mut self) -> &mut CliConfig {
        &mut self.config
    }

    pub fn update_from_env(&mut self) {
        if let Ok(timeout) = std::env::var("JETCRAB_TIMEOUT_MS") {
            if let Ok(value) = timeout.parse::<u64>() {
                self.config.general.default_timeout_ms = value;
                self.config.network.timeout_ms = value;
            }
        }

        if let Ok(level) = std::env::var("JETCRAB_LOG_LEVEL") {
            self.config.logging.level = level;
        }

        if let Ok(format) = std::env::var("JETCRAB_LOG_FORMAT") {
            self.config.logging.format = format;
        }

        if let Ok(telemetry) = std::env::var("JETCRAB_TELEMETRY") {
            self.config.general.telemetry_enabled = telemetry.parse().unwrap_or(false);
        }

        if let Ok(proxy) = std::env::var("JETCRAB_PROXY") {
            self.config.network.proxy = Some(proxy);
        }

        if let Ok(verify_ssl) = std::env::var("JETCRAB_VERIFY_SSL") {
            self.config.network.verify_ssl = verify_ssl.parse().unwrap_or(true);
        }
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.config.general.default_timeout_ms == 0 {
            errors.push("default_timeout_ms must be greater than 0".to_string());
        }

        if self.config.network.timeout_ms == 0 {
            errors.push("network.timeout_ms must be greater than 0".to_string());
        }

        if self.config.performance.max_concurrent_operations == 0 {
            errors.push("max_concurrent_operations must be greater than 0".to_string());
        }

        if self.config.security.max_file_size_mb == 0 {
            errors.push("max_file_size_mb must be greater than 0".to_string());
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }

    pub fn reset_to_defaults(&mut self) {
        self.config = CliConfig::default();
    }

    pub fn get_config_path(&self) -> &Path {
        &self.config_path
    }
}

pub fn load_config(config_path: Option<PathBuf>) -> Result<CliConfig, Box<dyn std::error::Error>> {
    let mut manager = ConfigManager::new(config_path);
    manager.load()?;
    manager.update_from_env();
    if let Err(errors) = manager.validate() {
        return Err(format!("Configuration validation failed: {}", errors.join(", ")).into());
    }
    Ok(manager.config)
}
