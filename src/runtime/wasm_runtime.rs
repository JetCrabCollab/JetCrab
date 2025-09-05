//! # WebAssembly Runtime Integration
//!
//! This module provides integration between JetCrab's JavaScript runtime
//! and WebAssembly modules compiled from Rust code.

use crate::runtime::engine::JetCrabEngine;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;
use tracing::{debug, error, info, warn};
use wasmtime::*;

/// WebAssembly runtime for executing Rust-compiled modules
pub struct WasmRuntime {
    engine: Engine,
    #[allow(dead_code)]
    store: Store<()>,
    modules: HashMap<String, Module>,
    temp_dir: Option<TempDir>,
    js_engine: *mut JetCrabEngine,
}

/// A compiled WebAssembly module with its exported functions
pub struct WasmModule {
    pub name: String,
    pub path: PathBuf,
    pub wasm_path: PathBuf,
    pub functions: HashMap<String, WasmFunction>,
}

/// Information about a WebAssembly function
pub struct WasmFunction {
    pub name: String,
    pub signature: WasmSignature,
    pub is_exported: bool,
}

/// WebAssembly function signature
pub struct WasmSignature {
    pub parameters: Vec<WasmType>,
    pub return_type: Option<WasmType>,
}

/// WebAssembly types
#[derive(Debug, Clone)]
pub enum WasmType {
    I32,
    I64,
    F32,
    F64,
}

impl WasmRuntime {
    /// Create a new WebAssembly runtime
    pub fn new(js_engine: &mut JetCrabEngine) -> Result<Self> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.wasm_multi_memory(true);
        config.wasm_memory64(true);

        let engine = Engine::new(&config)?;
        let store = Store::new(&engine, ());

        Ok(Self {
            engine,
            store,
            modules: HashMap::new(),
            temp_dir: None,
            js_engine: js_engine as *mut JetCrabEngine,
        })
    }

    /// Load and compile a Rust module to WebAssembly
    pub async fn load_rust_module(&mut self, path: &Path) -> Result<WasmModule> {
        info!("Loading Rust module as WebAssembly: {:?}", path);

        let module_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .context("Invalid module name")?
            .to_string();

        if self.modules.contains_key(&module_name) {
            debug!("WASM module {} already loaded", module_name);
            return self.create_wasm_module(&module_name, path);
        }

        let functions = self.parse_rust_functions(path).await?;

        let wasm_path = self.compile_rust_to_wasm(path, &module_name).await?;

        let wasm_bytes = std::fs::read(&wasm_path)?;
        let module = Module::new(&self.engine, &wasm_bytes)?;

        self.modules.insert(module_name.clone(), module);

        let wasm_module = WasmModule {
            name: module_name.clone(),
            path: path.to_path_buf(),
            wasm_path,
            functions,
        };

        self.register_wasm_functions_with_js(&wasm_module).await?;

        info!("WASM module {} loaded successfully", module_name);
        Ok(wasm_module)
    }

    /// Parse Rust file to extract exported functions
    async fn parse_rust_functions(&self, path: &Path) -> Result<HashMap<String, WasmFunction>> {
        let content = std::fs::read_to_string(path)?;
        let mut functions = HashMap::new();

        for line in content.lines() {
            let line = line.trim();

            if line.starts_with("#[export]") || line.starts_with("pub fn") {
                if let Some(func) = self.parse_function_definition(line, &content)? {
                    functions.insert(func.name.clone(), func);
                }
            }
        }

        debug!("Found {} exported functions in {:?}", functions.len(), path);
        Ok(functions)
    }

    /// Parse a single function definition
    fn parse_function_definition(
        &self,
        line: &str,
        _content: &str,
    ) -> Result<Option<WasmFunction>> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut name = None;

        for (i, part) in parts.iter().enumerate() {
            if part == &"fn" && i + 1 < parts.len() {
                let name_part = parts[i + 1];
                if let Some(open_paren) = name_part.find('(') {
                    name = Some(name_part[..open_paren].to_string());
                    break;
                }
            }
        }

        if let Some(name) = name {
            Ok(Some(WasmFunction {
                name,
                signature: WasmSignature {
                    parameters: vec![WasmType::I32],  // Simplified
                    return_type: Some(WasmType::I32), // Simplified
                },
                is_exported: true,
            }))
        } else {
            Ok(None)
        }
    }

    /// Compile Rust module to WebAssembly
    async fn compile_rust_to_wasm(&mut self, path: &Path, module_name: &str) -> Result<PathBuf> {
        info!("Compiling Rust module to WebAssembly: {}", module_name);

        if self.temp_dir.is_none() {
            self.temp_dir = Some(TempDir::new()?);
        }

        let temp_dir = self.temp_dir.as_ref().unwrap();
        let build_dir = temp_dir.path().join("wasm_modules");
        std::fs::create_dir_all(&build_dir)?;

        let cargo_toml = format!(
            r#"
[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"

[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-Os", "--enable-mutable-globals"]
"#,
            module_name
        );

        let module_dir = build_dir.join(module_name);
        std::fs::create_dir_all(&module_dir)?;

        std::fs::write(module_dir.join("Cargo.toml"), cargo_toml)?;

        let src_dir = module_dir.join("src");
        std::fs::create_dir_all(&src_dir)?;

        let mut rust_content = std::fs::read_to_string(path)?;
        rust_content = format!(
            "use wasm_bindgen::prelude::*;\n\n{}\n\n// WASM bindings\n{}",
            rust_content,
            self.generate_wasm_bindings(&rust_content)?
        );

        std::fs::write(src_dir.join("lib.rs"), rust_content)?;

        let output = Command::new("wasm-pack")
            .arg("build")
            .arg("--target")
            .arg("web")
            .arg("--out-dir")
            .arg("pkg")
            .arg("--release")
            .current_dir(&module_dir)
            .output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            error!("Failed to compile Rust to WASM: {}", error);
            return Err(anyhow::anyhow!("WASM compilation failed: {}", error));
        }

        let wasm_path = module_dir
            .join("pkg")
            .join(format!("{}_bg.wasm", module_name));
        if !wasm_path.exists() {
            return Err(anyhow::anyhow!(
                "Compiled WASM file not found: {:?}",
                wasm_path
            ));
        }

        info!("Rust module {} compiled to WASM successfully", module_name);
        Ok(wasm_path)
    }

    /// Generate WASM bindings for Rust functions
    fn generate_wasm_bindings(&self, rust_content: &str) -> Result<String> {
        let mut bindings = String::new();

        for line in rust_content.lines() {
            let line = line.trim();
            if line.starts_with("#[export]") || line.starts_with("pub fn") {
                if let Some(name) = self.extract_function_name(line) {
                    bindings.push_str(&format!(
                        "#[wasm_bindgen]\npub fn {}_wasm() -> i32 {{\n    // WASM binding for {}\n    42\n}}\n\n",
                        name, name
                    ));
                }
            }
        }

        Ok(bindings)
    }

    /// Extract function name from line
    fn extract_function_name(&self, line: &str) -> Option<String> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        for (i, part) in parts.iter().enumerate() {
            if part == &"fn" && i + 1 < parts.len() {
                let name_part = parts[i + 1];
                if let Some(open_paren) = name_part.find('(') {
                    return Some(name_part[..open_paren].to_string());
                }
            }
        }
        None
    }

    /// Register WebAssembly functions with JavaScript engine
    async fn register_wasm_functions_with_js(&self, module: &WasmModule) -> Result<()> {
        debug!(
            "Registering {} WASM functions from module {}",
            module.functions.len(),
            module.name
        );

        unsafe {
            let engine = &mut *self.js_engine;

            for (name, _func) in &module.functions {
                let function_name = format!("{}__{}", module.name, name);
                let name_clone = function_name.clone();

                engine
                    .add_function(&name_clone.clone(), move |_args, _context| {
                        warn!("WASM function call not fully implemented: {}", name_clone);
                        Ok(boa_engine::JsValue::from(42)) // Placeholder return
                    })
                    .map_err(|e| anyhow::anyhow!("Failed to add function: {}", e))?;
            }
        }

        Ok(())
    }

    /// Create WASM module info (for already loaded modules)
    fn create_wasm_module(&self, name: &str, path: &Path) -> Result<WasmModule> {
        Ok(WasmModule {
            name: name.to_string(),
            path: path.to_path_buf(),
            wasm_path: PathBuf::new(), // Placeholder
            functions: HashMap::new(), // Placeholder
        })
    }

    /// Execute a WebAssembly function
    pub fn execute_wasm_function(
        &mut self,
        module_name: &str,
        function_name: &str,
        _args: &[i32],
    ) -> Result<i32> {
        if let Some(_module) = self.modules.get(module_name) {
            warn!(
                "WASM function execution not fully implemented: {}::{}",
                module_name, function_name
            );
            Ok(42) // Placeholder return
        } else {
            Err(anyhow::anyhow!("Module {} not found", module_name))
        }
    }

    /// Get all loaded modules
    pub fn get_loaded_modules(&self) -> Vec<&str> {
        self.modules.keys().map(|k| k.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_runtime_creation() {
        let mut engine = JetCrabEngine::new();
        let runtime = WasmRuntime::new(&mut engine);
        assert!(runtime.is_ok());
    }

    #[test]
    fn test_function_name_extraction() {
        let mut engine = JetCrabEngine::new();
        let runtime = WasmRuntime::new(&mut engine).unwrap();
        let line = "pub fn calculate_fibonacci(n: u32) -> u32";
        let name = runtime.extract_function_name(line);
        assert_eq!(name, Some("calculate_fibonacci".to_string()));
    }
}
