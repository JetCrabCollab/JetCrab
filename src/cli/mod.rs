//! # CLI Module
//!
//! Command-line interface for JetCrab Runtime.

pub mod framework;

use crate::easter_egg::{
    should_trigger_easter_egg, should_trigger_easter_egg_for_command, show_walking_jetcrab,
};
use crate::runtime::JetCrabRuntime;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// JetCrab - A modern JavaScript runtime in Rust
#[derive(Parser)]
#[command(name = "jetcrab")]
#[command(about = "A modern JavaScript runtime in Rust")]
#[command(version = "0.4.0")]
pub struct Cli {
    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a JavaScript file or load a Rust module
    Run {
        /// File to execute (.js for JavaScript or .rs for Rust modules)
        file: PathBuf,
        /// Additional arguments to pass to the script
        #[arg(short, long)]
        args: Vec<String>,
    },
    /// Start interactive REPL
    Repl,
    /// Evaluate JavaScript code directly
    Eval {
        /// JavaScript code to evaluate
        code: String,
    },
    /// Run tests
    Test {
        /// Test pattern to match
        pattern: Option<String>,
        /// Test directory
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,
    },
    /// Format JavaScript code
    Fmt {
        /// Files or directories to format
        files: Vec<PathBuf>,
        /// Check formatting without making changes
        #[arg(long)]
        check: bool,
    },
    /// Lint JavaScript code
    Lint {
        /// Files or directories to lint
        files: Vec<PathBuf>,
    },
    /// Bundle JavaScript modules
    Bundle {
        /// Entry point file
        entry: PathBuf,
        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Show version information
    Version,
    /// 🦀 Easter egg - show walking crab animation
    Crab,
}

impl Cli {
    /// Execute the CLI command
    pub async fn execute(
        &self,
        runtime: &mut JetCrabRuntime,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            Commands::Run { file, args } => {
                runtime.run_file(file, args).await?;
            }
            Commands::Repl => {
                runtime.start_repl().await?;
            }
            Commands::Eval { code } => {
                if should_trigger_easter_egg_for_command(&code) {
                    show_walking_jetcrab();
                } else if should_trigger_easter_egg() {
                    show_walking_jetcrab();
                }
                runtime.evaluate_code(code).await?;
            }
            Commands::Test { pattern, dir } => {
                runtime.run_tests(pattern.as_deref(), dir).await?;
            }
            Commands::Fmt { files, check } => {
                runtime.format_code(files, *check).await?;
            }
            Commands::Lint { files } => {
                runtime.lint_code(files).await?;
            }
            Commands::Bundle { entry, output } => {
                runtime.bundle_modules(entry, output.as_deref()).await?;
            }
            Commands::Version => {
                runtime.show_version();
            }
            Commands::Crab => {
                show_walking_jetcrab();
            }
        }
        Ok(())
    }
}
