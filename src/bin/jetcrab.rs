//! # JetCrab Runtime
//!
//! A modern JavaScript runtime in Rust, powered by WASM bridge.
//!
//! This is the main entry point for the JetCrab CLI.

use clap::Parser;
use jetcrab::cli::framework::{init_logging, setup_signal_handlers};
use jetcrab::cli::Cli;
use jetcrab::easter_egg::{should_trigger_easter_egg, show_walking_jetcrab};
use jetcrab::runtime::JetCrabRuntime;
use tracing::warn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup signal handlers (Ctrl+C)
    if let Err(e) = setup_signal_handlers() {
        warn!("Failed to setup signal handlers: {}", e);
    }

    // Parse CLI args
    let cli = Cli::parse();
    
    // Initialize Logging
    // We can use debug options to control logging level if needed
    if let Err(e) = init_logging(false, false) {
        warn!("Failed to initialize logging: {}", e);
    }

    // Easter egg check
    if should_trigger_easter_egg() {
        show_walking_jetcrab();
    }

    // Configure Engine based on CLI
    use chitin::EngineConfig;
    
    // Default config for now, but wired via Builder for Clean Code
    // In the future, we map `cli.experimental` and `cli.permissions` here.
    let config = EngineConfig::builder()
        // Example: .strict_mode(cli.main.strict) 
        // Example: .memory_limit(cli.main.memory_limit)
        .build();

    // Initialize Runtime (WASM Mode)
    let mut runtime = JetCrabRuntime::with_config(config);

    // Execute CLI command
    if let Err(e) = cli.execute(&mut runtime).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
