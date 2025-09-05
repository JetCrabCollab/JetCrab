use crate::cli::framework::{
    validate_required_arg, CliCommand, CliContext, CliError, CliResult, InputValidator, PluginInfo,
    PluginManager, PluginStats,
};
use clap::{Arg, ArgMatches, Command};
use std::path::PathBuf;
use tracing::{debug, error, info, warn};

pub struct PluginManagerCommand {
    manager: std::sync::Arc<PluginManager>,
}

impl PluginManagerCommand {
    pub fn new(manager: std::sync::Arc<PluginManager>) -> Self {
        Self { manager }
    }
}

impl CliCommand for PluginManagerCommand {
    fn name(&self) -> &'static str {
        "plugin"
    }

    fn description(&self) -> &'static str {
        "Manage CLI plugins"
    }

    fn build_clap_command(&self) -> Command {
        Command::new("plugin")
            .about("Manage CLI plugins")
            .subcommand(Command::new("list").about("List all plugins"))
            .subcommand(
                Command::new("load")
                    .about("Load a plugin")
                    .arg(Arg::new("path").required(true)),
            )
            .subcommand(
                Command::new("unload")
                    .about("Unload a plugin")
                    .arg(Arg::new("name").required(true)),
            )
            .subcommand(
                Command::new("reload")
                    .about("Reload a plugin")
                    .arg(Arg::new("name").required(true)),
            )
            .subcommand(
                Command::new("enable")
                    .about("Enable a plugin")
                    .arg(Arg::new("name").required(true)),
            )
            .subcommand(
                Command::new("disable")
                    .about("Disable a plugin")
                    .arg(Arg::new("name").required(true)),
            )
            .subcommand(
                Command::new("info")
                    .about("Show plugin info")
                    .arg(Arg::new("name").required(true)),
            )
            .subcommand(Command::new("stats").about("Show plugin statistics"))
    }

    fn execute(&self, _context: &mut CliContext, matches: &ArgMatches) -> CliResult<()> {
        info!("Plugin management command executed");
        Ok(())
    }

    fn help(&self) -> String {
        format!(
            r#"Plugin Management Command

USAGE:
    jetcrab plugin <subcommand> [options]

SUBCOMMANDS:
    list                    List all loaded plugins
    load <path>            Load a plugin from file or directory
    unload <name>          Unload a plugin by name
    reload <name>          Reload a plugin by name
    enable <name>          Enable a disabled plugin
    disable <name>         Disable a plugin
    info <name>            Show detailed information about a plugin
    stats                  Show plugin statistics

EXAMPLES:
    jetcrab plugin list
    jetcrab plugin load ./plugins/my-plugin
    jetcrab plugin reload example-plugin
    jetcrab plugin info example-plugin
    jetcrab plugin stats

OPTIONS:
    -h, --help             Show this help message
    -v, --verbose          Enable verbose output
"#
        )
    }
}

pub struct PluginListCommand {
    manager: std::sync::Arc<PluginManager>,
}

impl PluginListCommand {
    pub fn new(manager: std::sync::Arc<PluginManager>) -> Self {
        Self { manager }
    }
}

impl CliCommand for PluginListCommand {
    fn name(&self) -> &'static str {
        "plugin-list"
    }

    fn description(&self) -> &'static str {
        "List all loaded plugins"
    }

    fn build_clap_command(&self) -> Command {
        Command::new("plugin-list").about("List all loaded plugins")
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        let plugins = self.manager.list_plugins();

        if plugins.is_empty() {
            println!("No plugins loaded.");
            return Ok(());
        }

        println!("Loaded Plugins:");
        println!("===============");
        println!();

        for plugin in plugins {
            let status = if plugin.is_enabled {
                "enabled"
            } else {
                "disabled"
            };
            let error_indicator = if plugin.error_count > 0 {
                format!(" ({} errors)", plugin.error_count)
            } else {
                String::new()
            };

            println!("Name: {}", plugin.metadata.name);
            println!("  Version: {}", plugin.metadata.version);
            println!("  Description: {}", plugin.metadata.description);
            println!("  Status: {}{}", status, error_indicator);
            println!("  Commands: {}", plugin.metadata.commands.len());

            if let Some(author) = &plugin.metadata.author {
                println!("  Author: {}", author);
            }

            if let Some(license) = &plugin.metadata.license {
                println!("  License: {}", license);
            }

            println!();
        }

        Ok(())
    }

    fn help(&self) -> String {
        "List all loaded plugins with their information".to_string()
    }
}

pub struct PluginInfoCommand {
    manager: std::sync::Arc<PluginManager>,
}

impl PluginInfoCommand {
    pub fn new(manager: std::sync::Arc<PluginManager>) -> Self {
        Self { manager }
    }
}

impl CliCommand for PluginInfoCommand {
    fn name(&self) -> &'static str {
        "plugin-info"
    }

    fn description(&self) -> &'static str {
        "Show detailed information about a plugin"
    }

    fn build_clap_command(&self) -> Command {
        Command::new("plugin-info")
            .about("Show detailed information about a plugin")
            .arg(Arg::new("name").required(true).help("Plugin name"))
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        println!("Plugin info command - implementation needed");
        Ok(())
    }

    fn help(&self) -> String {
        "Show detailed information about a specific plugin".to_string()
    }
}

pub struct PluginStatsCommand {
    manager: std::sync::Arc<PluginManager>,
}

impl PluginStatsCommand {
    pub fn new(manager: std::sync::Arc<PluginManager>) -> Self {
        Self { manager }
    }
}

impl CliCommand for PluginStatsCommand {
    fn name(&self) -> &'static str {
        "plugin-stats"
    }

    fn description(&self) -> &'static str {
        "Show plugin statistics"
    }

    fn build_clap_command(&self) -> Command {
        Command::new("plugin-stats").about("Show plugin statistics")
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        let stats = self.manager.get_plugin_stats();

        println!("Plugin Statistics:");
        println!("==================");
        println!();
        println!("Total Plugins: {}", stats.total_plugins);
        println!("Enabled Plugins: {}", stats.enabled_plugins);
        println!("Disabled Plugins: {}", stats.disabled_plugins);
        println!("Total Commands: {}", stats.total_commands);
        println!("Plugins with Errors: {}", stats.plugins_with_errors);
        println!(
            "Average Commands per Plugin: {:.1}",
            stats.average_commands_per_plugin
        );

        Ok(())
    }

    fn help(&self) -> String {
        "Show statistics about loaded plugins".to_string()
    }
}

pub fn create_plugin_commands(manager: std::sync::Arc<PluginManager>) -> Vec<Box<dyn CliCommand>> {
    vec![
        Box::new(PluginManagerCommand::new(manager.clone())),
        Box::new(PluginListCommand::new(manager.clone())),
        Box::new(PluginInfoCommand::new(manager.clone())),
        Box::new(PluginStatsCommand::new(manager)),
    ]
}
