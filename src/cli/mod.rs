//! # CLI Module
//!
//! Command-line interface for JetCrab Runtime.

pub mod framework;
pub mod options;
pub mod commands;

pub use options::Cli;

use crate::runtime::JetCrabRuntime;
use crate::easter_egg::{should_trigger_easter_egg, should_trigger_easter_egg_for_command, show_walking_jetcrab};

impl Cli {
    /// Execute the CLI command
    pub async fn execute(
        &self,
        runtime: &mut JetCrabRuntime,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Handle help manually if needed (clap does it mostly, but --help flag is custom handled in node)
        // With disable_help_flag=true, we should probably print help here if self.main.help is true
        if self.main.help {
            // We can use clap to print help, but since we parsed it into our struct, 
            // and we disabled the flag, we might need to regenerate the help message or just print our custom one.
            // For now, let's assume clap handles it if we don't completely override execution flow before parsing.
            // Wait, if disable_help_flag=true, clap parses '--help' as a value or ignores it?
            // Actually, if we defined `help` field in `options.rs`, it parses it into that boolean.
            // So we must print help manually.
            // Since we don't have the Command object here easily without rebuilding it, 
            // we can just print the help template or let clap do it if we didn't disable it.
            // The user wanted Node behavior. Node prints help and exits.
            // We can print the help string we constructed or use Cli::command().print_help().
            // But `Cli` is the struct. `Cli::command()` is available via `CommandFactory`.
            use clap::CommandFactory;
            Cli::command().print_help()?;
            return Ok(());
        }

        if self.main.version {
            println!("v{}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        
        if self.main.eval.is_some() || self.main.print.is_some() {
             let code = self.main.eval.as_ref().or(self.main.print.as_ref()).unwrap();
             commands::eval::execute(runtime, code).await?;
             return Ok(());
        }

        // REPL (Interactive or no args)
        if self.main.interactive || (self.script.is_none() && self.main.eval.is_none()) {
             commands::repl::execute(runtime).await?;
             return Ok(());
        }
        
        // Modules/Script
        if let Some(script) = &self.script {
            // Easter egg check
            if script == "crab" {
                 commands::crab::execute();
                 return Ok(());
            }
            if script == "test" {
                 commands::test_cmd::execute(runtime).await?;
                 return Ok(());
            }
            if script == "fmt" {
                 commands::fmt_cmd::execute(&self.script_args)?;
                 return Ok(());
            }
            if script == "lint" {
                 commands::lint_cmd::execute(&self.script_args)?;
                 return Ok(());
            }

            commands::run::execute(runtime, script, &self.script_args).await?;
        }

        Ok(())
    }
}
