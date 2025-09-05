use crate::cli::framework::{
    validate_required_arg, CliCommand, CliContext, CliError, CliResult, InputValidator,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub dependencies: Vec<String>,
    pub commands: Vec<CommandMetadata>,
    pub config_schema: Option<serde_json::Value>,
    pub min_cli_version: Option<String>,
    pub max_cli_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandMetadata {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub examples: Vec<String>,
    pub options: Vec<OptionMetadata>,
    pub arguments: Vec<ArgumentMetadata>,
    pub category: Option<String>,
    pub hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionMetadata {
    pub name: String,
    pub short: Option<char>,
    pub long: Option<String>,
    pub description: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub value_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgumentMetadata {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub multiple: bool,
    pub default_value: Option<String>,
    pub value_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub metadata: PluginMetadata,
    pub path: PathBuf,
    pub loaded_at: Instant,
    pub is_enabled: bool,
    pub error_count: u32,
    pub last_error: Option<String>,
}

pub trait PluginCommand: CliCommand {
    fn plugin_name(&self) -> &'static str;
    fn plugin_version(&self) -> &'static str;
    fn plugin_metadata(&self) -> &PluginMetadata;
}

pub struct PluginManager {
    plugins: Arc<Mutex<HashMap<String, PluginInfo>>>,
    commands: Arc<Mutex<HashMap<String, Box<dyn PluginCommand>>>>,
    plugin_directories: Vec<PathBuf>,
    validator: InputValidator,
    auto_reload: bool,
    reload_interval: Duration,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(Mutex::new(HashMap::new())),
            commands: Arc::new(Mutex::new(HashMap::new())),
            plugin_directories: vec![
                dirs::config_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("jetcrab")
                    .join("plugins"),
                PathBuf::from("./plugins"),
                PathBuf::from("./.jetcrab/plugins"),
            ],
            validator: InputValidator::new(),
            auto_reload: false,
            reload_interval: Duration::from_secs(30),
        }
    }

    pub fn with_plugin_directory(mut self, directory: PathBuf) -> Self {
        self.plugin_directories.push(directory);
        self
    }

    pub fn with_auto_reload(mut self, enabled: bool) -> Self {
        self.auto_reload = enabled;
        self
    }

    pub fn with_reload_interval(mut self, interval: Duration) -> Self {
        self.reload_interval = interval;
        self
    }

    pub fn load_plugins(&self) -> CliResult<()> {
        info!(
            "Loading plugins from directories: {:?}",
            self.plugin_directories
        );

        for directory in &self.plugin_directories {
            if directory.exists() {
                self.load_plugins_from_directory(directory)?;
            } else {
                debug!("Plugin directory does not exist: {:?}", directory);
            }
        }

        let plugins = self.plugins.lock().unwrap();
        info!("Loaded {} plugins", plugins.len());

        Ok(())
    }

    fn load_plugins_from_directory(&self, directory: &Path) -> CliResult<()> {
        debug!("Scanning plugin directory: {:?}", directory);

        let entries = std::fs::read_dir(directory).map_err(|e| CliError::InternalError {
            message: format!("Failed to read plugin directory {:?}: {}", directory, e),
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| CliError::InternalError {
                message: format!("Failed to read directory entry: {}", e),
            })?;

            let path = entry.path();

            if path.is_dir() {
                self.load_plugin_from_directory(&path)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                self.load_plugin_from_manifest(&path)?;
            }
        }

        Ok(())
    }

    fn load_plugin_from_directory(&self, directory: &Path) -> CliResult<()> {
        let manifest_path = directory.join("plugin.toml");

        if manifest_path.exists() {
            self.load_plugin_from_manifest(&manifest_path)?;
        } else {
            warn!("No plugin.toml found in directory: {:?}", directory);
        }

        Ok(())
    }

    fn load_plugin_from_manifest(&self, manifest_path: &Path) -> CliResult<()> {
        debug!("Loading plugin from manifest: {:?}", manifest_path);

        let content =
            std::fs::read_to_string(manifest_path).map_err(|e| CliError::InternalError {
                message: format!("Failed to read plugin manifest {:?}: {}", manifest_path, e),
            })?;

        let metadata: PluginMetadata =
            toml::from_str(&content).map_err(|e| CliError::InternalError {
                message: format!("Failed to parse plugin manifest {:?}: {}", manifest_path, e),
            })?;

        self.validate_plugin_metadata(&metadata)?;

        let plugin_info = PluginInfo {
            metadata: metadata.clone(),
            path: manifest_path.parent().unwrap().to_path_buf(),
            loaded_at: Instant::now(),
            is_enabled: true,
            error_count: 0,
            last_error: None,
        };

        let mut plugins = self.plugins.lock().unwrap();
        plugins.insert(metadata.name.clone(), plugin_info);

        info!("Loaded plugin: {} v{}", metadata.name, metadata.version);

        Ok(())
    }

    fn validate_plugin_metadata(&self, metadata: &PluginMetadata) -> CliResult<()> {
        if metadata.name.is_empty() {
            return Err(CliError::ValidationError {
                field: "name".to_string(),
                reason: "Plugin name cannot be empty".to_string(),
            });
        }

        if metadata.version.is_empty() {
            return Err(CliError::ValidationError {
                field: "version".to_string(),
                reason: "Plugin version cannot be empty".to_string(),
            });
        }

        if metadata.description.is_empty() {
            return Err(CliError::ValidationError {
                field: "description".to_string(),
                reason: "Plugin description cannot be empty".to_string(),
            });
        }

        for command in &metadata.commands {
            if command.name.is_empty() {
                return Err(CliError::ValidationError {
                    field: "command.name".to_string(),
                    reason: "Command name cannot be empty".to_string(),
                });
            }

            if command.description.is_empty() {
                return Err(CliError::ValidationError {
                    field: "command.description".to_string(),
                    reason: "Command description cannot be empty".to_string(),
                });
            }
        }

        Ok(())
    }

    pub fn register_command(&self, command: Box<dyn PluginCommand>) -> CliResult<()> {
        let command_name = command.name().to_string();
        let plugin_name = command.plugin_name().to_string();

        debug!(
            "Registering command '{}' from plugin '{}'",
            command_name, plugin_name
        );

        let mut commands = self.commands.lock().unwrap();

        if commands.contains_key(&command_name) {
            return Err(CliError::ValidationError {
                field: "command_name".to_string(),
                reason: format!("Command '{}' is already registered", command_name),
            });
        }

        commands.insert(command_name.clone(), command);

        info!(
            "Registered command '{}' from plugin '{}'",
            command_name, plugin_name
        );
        Ok(())
    }

    pub fn unregister_command(&self, command_name: &str) -> CliResult<()> {
        debug!("Unregistering command: {}", command_name);

        let mut commands = self.commands.lock().unwrap();

        if commands.remove(command_name).is_some() {
            info!("Unregistered command: {}", command_name);
            Ok(())
        } else {
            Err(CliError::ValidationError {
                field: "command_name".to_string(),
                reason: format!("Command '{}' is not registered", command_name),
            })
        }
    }

    pub fn get_command(&self, command_name: &str) -> Option<Box<dyn CliCommand>> {
        let commands = self.commands.lock().unwrap();
        commands
            .get(command_name)
            .map(|cmd| todo!("Need to implement command cloning or use Arc<dyn CliCommand>"))
    }

    pub fn list_commands(&self) -> Vec<String> {
        let commands = self.commands.lock().unwrap();
        commands.keys().cloned().collect()
    }

    pub fn list_plugins(&self) -> Vec<PluginInfo> {
        let plugins = self.plugins.lock().unwrap();
        plugins.values().cloned().collect()
    }

    pub fn get_plugin(&self, plugin_name: &str) -> Option<PluginInfo> {
        let plugins = self.plugins.lock().unwrap();
        plugins.get(plugin_name).cloned()
    }

    pub fn enable_plugin(&self, plugin_name: &str) -> CliResult<()> {
        let mut plugins = self.plugins.lock().unwrap();

        if let Some(plugin_info) = plugins.get_mut(plugin_name) {
            plugin_info.is_enabled = true;
            info!("Enabled plugin: {}", plugin_name);
            Ok(())
        } else {
            Err(CliError::ValidationError {
                field: "plugin_name".to_string(),
                reason: format!("Plugin '{}' not found", plugin_name),
            })
        }
    }

    pub fn disable_plugin(&self, plugin_name: &str) -> CliResult<()> {
        let mut plugins = self.plugins.lock().unwrap();

        if let Some(plugin_info) = plugins.get_mut(plugin_name) {
            plugin_info.is_enabled = false;
            info!("Disabled plugin: {}", plugin_name);
            Ok(())
        } else {
            Err(CliError::ValidationError {
                field: "plugin_name".to_string(),
                reason: format!("Plugin '{}' not found", plugin_name),
            })
        }
    }

    pub fn reload_plugin(&self, plugin_name: &str) -> CliResult<()> {
        debug!("Reloading plugin: {}", plugin_name);

        let plugin_info = {
            let plugins = self.plugins.lock().unwrap();
            plugins.get(plugin_name).cloned()
        };

        if let Some(plugin_info) = plugin_info {
            let mut commands = self.commands.lock().unwrap();
            commands.retain(|_, cmd| cmd.plugin_name() != plugin_name);
            drop(commands);

            self.load_plugin_from_manifest(&plugin_info.path.join("plugin.toml"))?;

            info!("Reloaded plugin: {}", plugin_name);
            Ok(())
        } else {
            Err(CliError::ValidationError {
                field: "plugin_name".to_string(),
                reason: format!("Plugin '{}' not found", plugin_name),
            })
        }
    }

    pub fn reload_all_plugins(&self) -> CliResult<()> {
        info!("Reloading all plugins");

        let plugin_names: Vec<String> = {
            let plugins = self.plugins.lock().unwrap();
            plugins.keys().cloned().collect()
        };

        for plugin_name in plugin_names {
            if let Err(e) = self.reload_plugin(&plugin_name) {
                error!("Failed to reload plugin '{}': {:?}", plugin_name, e);
            }
        }

        Ok(())
    }

    pub fn start_auto_reload(&self) -> CliResult<()> {
        if !self.auto_reload {
            return Ok(());
        }

        let plugins = Arc::clone(&self.plugins);
        let plugin_directories = self.plugin_directories.clone();
        let interval = self.reload_interval;

        std::thread::spawn(move || loop {
            std::thread::sleep(interval);

            debug!("Auto-reloading plugins");
        });

        info!("Started auto-reload for plugins");
        Ok(())
    }

    pub fn validate_plugin_dependencies(&self, plugin_name: &str) -> CliResult<()> {
        let plugin_info = {
            let plugins = self.plugins.lock().unwrap();
            plugins.get(plugin_name).cloned()
        };

        if let Some(plugin_info) = plugin_info {
            for dependency in &plugin_info.metadata.dependencies {
                let plugins = self.plugins.lock().unwrap();
                if !plugins.contains_key(dependency) {
                    return Err(CliError::ValidationError {
                        field: "dependencies".to_string(),
                        reason: format!("Required dependency '{}' not found", dependency),
                    });
                }
            }
            Ok(())
        } else {
            Err(CliError::ValidationError {
                field: "plugin_name".to_string(),
                reason: format!("Plugin '{}' not found", plugin_name),
            })
        }
    }

    pub fn get_plugin_commands(&self, plugin_name: &str) -> Vec<CommandMetadata> {
        let plugins = self.plugins.lock().unwrap();

        if let Some(plugin_info) = plugins.get(plugin_name) {
            plugin_info.metadata.commands.clone()
        } else {
            Vec::new()
        }
    }

    pub fn search_plugins(&self, query: &str) -> Vec<PluginInfo> {
        let plugins = self.plugins.lock().unwrap();

        plugins
            .values()
            .filter(|plugin| {
                plugin.metadata.name.contains(query)
                    || plugin
                        .metadata
                        .description
                        .to_lowercase()
                        .contains(&query.to_lowercase())
                    || plugin
                        .metadata
                        .author
                        .as_ref()
                        .map_or(false, |author| author.contains(query))
            })
            .cloned()
            .collect()
    }

    pub fn get_plugin_stats(&self) -> PluginStats {
        let plugins = self.plugins.lock().unwrap();
        let commands = self.commands.lock().unwrap();

        let total_plugins = plugins.len();
        let enabled_plugins = plugins.values().filter(|p| p.is_enabled).count();
        let total_commands = commands.len();
        let plugins_with_errors = plugins.values().filter(|p| p.error_count > 0).count();

        PluginStats {
            total_plugins,
            enabled_plugins,
            disabled_plugins: total_plugins - enabled_plugins,
            total_commands,
            plugins_with_errors,
            average_commands_per_plugin: if total_plugins > 0 {
                total_commands as f64 / total_plugins as f64
            } else {
                0.0
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct PluginStats {
    pub total_plugins: usize,
    pub enabled_plugins: usize,
    pub disabled_plugins: usize,
    pub total_commands: usize,
    pub plugins_with_errors: usize,
    pub average_commands_per_plugin: f64,
}

pub struct PluginLoader {
    manager: Arc<PluginManager>,
}

impl PluginLoader {
    pub fn new(manager: Arc<PluginManager>) -> Self {
        Self { manager }
    }

    pub fn load_plugin_from_file(&self, file_path: &Path) -> CliResult<()> {
        debug!("Loading plugin from file: {:?}", file_path);

        if !file_path.exists() {
            return Err(CliError::FileNotFound {
                path: file_path.to_string_lossy().to_string(),
            });
        }

        if file_path.extension().and_then(|s| s.to_str()) == Some("toml") {
            self.manager.load_plugin_from_manifest(file_path)
        } else {
            Err(CliError::ValidationError {
                field: "file_path".to_string(),
                reason: "Plugin file must have .toml extension".to_string(),
            })
        }
    }

    pub fn load_plugin_from_url(&self, url: &str) -> CliResult<()> {
        debug!("Loading plugin from URL: {}", url);

        Err(CliError::ValidationError {
            field: "url".to_string(),
            reason: "URL plugin loading not yet implemented".to_string(),
        })
    }

    pub fn load_plugin_from_git(&self, repository: &str, branch: Option<&str>) -> CliResult<()> {
        debug!("Loading plugin from git repository: {}", repository);

        Err(CliError::ValidationError {
            field: "repository".to_string(),
            reason: "Git plugin loading not yet implemented".to_string(),
        })
    }
}

pub fn create_default_plugin_manager() -> Arc<PluginManager> {
    Arc::new(PluginManager::new())
}

pub fn create_plugin_manager_with_directories(directories: Vec<PathBuf>) -> Arc<PluginManager> {
    let mut manager = PluginManager::new();

    for directory in directories {
        manager = manager.with_plugin_directory(directory);
    }

    Arc::new(manager)
}
