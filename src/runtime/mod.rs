//! # JetCrab Runtime Core
//!
//! The core runtime implementation using Boa as the JavaScript engine backend.

use boa_engine::JsValue;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{error, info};

pub mod apis;
pub mod async_runtime;
pub mod engine;
pub mod module_loader;
pub mod repl;
pub mod wasm_runtime;

pub use apis::BuiltinAPIs;
pub use async_runtime::AsyncRuntime;
pub use engine::JetCrabEngine;
pub use module_loader::ModuleLoader;
pub use repl::Repl;
pub use wasm_runtime::WasmRuntime;

/// Main JetCrab Runtime
pub struct JetCrabRuntime {
    engine: JetCrabEngine,
    module_loader: ModuleLoader,
    wasm_runtime: Option<WasmRuntime>,
    async_runtime: AsyncRuntime,
    #[allow(dead_code)]
    builtin_apis: BuiltinAPIs,
    #[allow(dead_code)]
    working_directory: PathBuf,
}

impl JetCrabRuntime {
    /// Create a new JetCrab Runtime instance
    pub fn new() -> Self {
        let mut engine = JetCrabEngine::new();
        let module_loader = ModuleLoader::new();

        let async_runtime = match AsyncRuntime::new() {
            Ok(runtime) => {
                info!("AsyncRuntime initialized successfully");
                runtime
            }
            Err(e) => {
                error!("Failed to initialize AsyncRuntime: {}", e);
                panic!("Cannot continue without async runtime");
            }
        };

        info!("Initializing WasmRuntime...");
        let wasm_runtime = match WasmRuntime::new(&mut engine) {
            Ok(runtime) => {
                info!("WasmRuntime initialized successfully");
                Some(runtime)
            }
            Err(e) => {
                error!("Failed to initialize WasmRuntime: {}", e);
                None
            }
        };

        let mut builtin_apis = BuiltinAPIs::new();
        let working_directory = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

        info!("Setting up built-in APIs...");
        if let Err(e) = builtin_apis.setup(&mut engine) {
            error!("Failed to setup built-in APIs: {}", e);
        } else {
            info!("Built-in APIs setup completed successfully");
        }

        Self {
            engine,
            module_loader,
            wasm_runtime,
            async_runtime,
            builtin_apis,
            working_directory,
        }
    }

    /// Run a JavaScript file or load a Rust module as WebAssembly
    pub async fn run_file(
        &mut self,
        file: &Path,
        args: &[String],
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Running file: {:?}", file);

        if file.extension().map_or(false, |ext| ext == "rs") {
            return self.load_rust_module_as_wasm(file).await;
        }

        let content = fs::read_to_string(file).await?;

        let _ = self
            .engine
            .set_global("process", self.create_process_object(args));

        let _result = self.engine.evaluate_to_string(&content)?;

        info!("Execution completed successfully");
        Ok(())
    }

    /// Load a Rust module as WebAssembly
    async fn load_rust_module_as_wasm(
        &mut self,
        file: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Loading Rust module as WebAssembly: {:?}", file);

        if let Some(ref mut wasm_runtime) = self.wasm_runtime {
            info!("WasmRuntime is available, attempting to load module");
            match wasm_runtime.load_rust_module(file).await {
                Ok(_module) => {
                    info!("Rust module loaded as WebAssembly successfully");
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to load Rust module as WebAssembly: {}", e);
                    Err(e.into())
                }
            }
        } else {
            error!("WebAssembly runtime not available - this should not happen");
            Err("WebAssembly runtime not available".into())
        }
    }

    /// Start interactive REPL
    pub async fn start_repl(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting REPL");

        let mut repl = Repl::new(&mut self.engine);
        repl.start().await?;

        Ok(())
    }

    /// Evaluate JavaScript code directly
    pub async fn evaluate_code(&mut self, code: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("Evaluating code: {}", code);

        let result = self.engine.evaluate(code)?;
        println!("{:?}", result);

        Ok(())
    }

    /// Run tests
    pub async fn run_tests(
        &mut self,
        _pattern: Option<&str>,
        _dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Test runner not implemented yet");
        Ok(())
    }

    /// Format JavaScript code
    pub async fn format_code(
        &mut self,
        _files: &[PathBuf],
        _check: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Formatter not implemented yet");
        Ok(())
    }

    /// Lint JavaScript code
    pub async fn lint_code(
        &mut self,
        _files: &[PathBuf],
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Linter not implemented yet");
        Ok(())
    }

    /// Bundle JavaScript modules
    pub async fn bundle_modules(
        &mut self,
        entry: &Path,
        output: Option<&Path>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("Bundling modules from entry: {:?}", entry);

        let bundled_code = self.module_loader.bundle(entry).await?;

        let default_output = PathBuf::from("bundle.js");
        let output_path = output.unwrap_or(&default_output);
        fs::write(output_path, bundled_code).await?;

        println!("Bundle created: {:?}", output_path);
        Ok(())
    }

    /// Show version information
    pub fn show_version(&self) {
        println!("JetCrab Runtime v0.4.0");
        println!("Powered by Boa Engine");
        println!("Built with Rust");
    }

    /// Get reference to the async runtime
    pub fn get_async_runtime(&self) -> &AsyncRuntime {
        &self.async_runtime
    }

    fn create_process_object(&self, _args: &[String]) -> JsValue {
        JsValue::undefined()
    }
}

impl Default for JetCrabRuntime {
    fn default() -> Self {
        Self::new()
    }
}
