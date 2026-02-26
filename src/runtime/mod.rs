//! # JetCrab Runtime Core
//!
//! The core runtime implementation using the WASM-based JetCrab Engine.
//!
pub mod engine;
pub mod module_loader;

pub use chitin::EngineConfig;
pub use engine::JetCrabEngine;
pub use module_loader::ModuleLoader;

use std::path::Path;
use tracing::{error, info};

pub struct JetCrabRuntime {
    pub engine: JetCrabEngine,
}

impl JetCrabRuntime {
    /// Create a new JetCrab Runtime
    pub fn new() -> Self {
        Self::with_config(EngineConfig::default())
    }

    /// Create a new JetCrab Runtime with custom configuration
    pub fn with_config(config: EngineConfig) -> Self {
        let engine = JetCrabEngine::with_config(config);
        // let apis = BuiltinAPIs::new(); // Disabled

        let mut runtime = Self { engine };

        // Initialize engine (load WASM)
        if let Err(e) = runtime.engine.init() {
            error!("Failed to initialize engine: {}", e);
        }

        // runtime.register_apis().unwrap(); // Disabled

        runtime
    }

    /// Initialize and register built-in APIs
    // fn register_apis(&mut self) -> Result<(), String> {
    //     self.apis.register(&mut self.engine.get_context()) // get_context no longer exists
    //         .map_err(|e| format!("Failed to register APIs: {:?}", e))
    // }

    /// Run a JavaScript file (uses module bundler when require/import detected)
    pub async fn run_file(&mut self, path: &Path, _args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let source = tokio::fs::read_to_string(path).await?;
        let use_bundle = source.contains("require(") || source.contains("import ");
        let to_eval = if use_bundle {
            let mut loader = ModuleLoader::new();
            loader.bundle_sync(path).unwrap_or(source)
        } else {
            source
        };
        self.evaluate_code(&to_eval).await?;
        Ok(())
    }

    /// Evaluate JavaScript code string
    pub async fn evaluate_code(&mut self, source: &str) -> Result<(), Box<dyn std::error::Error>> {
        match self.engine.evaluate(source).await {
            Ok(result) => {
                // If the result string is not empty and not "undefined", print it (REPL style)
                // But for `evaluate_code`, maybe we just execute?
                // The new evaluate returns String result.
                if !result.is_empty() && result != "undefined" {
                     println!("{}", result);
                }
                Ok(())
            },
            Err(e) => {
                error!("Uncaught Exception: {}", e);
                Err(e.into())
            }
        }
    }

    /// Start the interactive REPL
    pub async fn start_repl(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use crate::cli::commands::repl; // Use the repl command logic
        repl::execute(self).await
    }

    pub fn show_version(&self) {
        println!("JetCrab Runtime v{}", env!("CARGO_PKG_VERSION"));
        println!("Engine: Chitin (WASM/QuickJS Mode)");
    }
}

impl Default for JetCrabRuntime {
    fn default() -> Self {
        Self::new()
    }
}
