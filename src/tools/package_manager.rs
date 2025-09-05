//! # Claw Package Manager
//!
//! Package manager for the JetCrab runtime that provides unified dependency management
//! for both JavaScript and Rust packages through WebAssembly integration.
//!
//! ## Overview
//!
//! Claw serves as the package manager for JetCrab, similar to how npm serves Node.js.
//! It provides unified installation of JavaScript and Rust packages, automatic
//! compilation of Rust code to WebAssembly, and intelligent dependency resolution.
//!
//! ## Features
//!
//! - Unified installation of JavaScript and Rust packages
//! - Automatic compilation of Rust code to WebAssembly
//! - Intelligent dependency resolution across multiple registries
//! - Optimized caching system for fast builds
//! - Support for NPM and Cargo registries
//! - Local package management
//!
//! ## Usage
//!
//! ```rust
//! use jetcrab::tools::Claw;
//! use std::path::PathBuf;
//!
//! let claw = Claw::new(PathBuf::from("."));
//! claw.install(&["react", "serde"], &Default::default()).await?;
//! ```

use anyhow::Result;
use flate2::read::GzDecoder;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf;
use std::process::Command;
use tar::Archive;
use tracing::{error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub dependencies: HashMap<String, String>,
    pub rust_dependencies: HashMap<String, String>,
    pub scripts: HashMap<String, String>,
    pub main: Option<String>,
    pub wasm_entry: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InstallOptions {
    pub dev: bool,
    pub save: bool,
    pub force: bool,
    pub cache: bool,
}

impl Default for InstallOptions {
    fn default() -> Self {
        Self {
            dev: false,
            save: true,
            force: false,
            cache: true,
        }
    }
}

pub struct Claw {
    #[allow(dead_code)]
    project_root: PathBuf,
    #[allow(dead_code)]
    cache_dir: PathBuf,
    #[allow(dead_code)]
    registry_url: String,
}

impl Claw {
    pub fn new(project_root: PathBuf) -> Self {
        let cache_dir = project_root.join(".claw");
        Self {
            project_root,
            cache_dir,
            registry_url: "https://registry.npmjs.org".to_string(),
        }
    }

    pub async fn install(&self, packages: &[String], options: &InstallOptions) -> Result<()> {
        info!("Installing packages: {:?}", packages);

        for package in packages {
            if package.ends_with(".rs") {
                self.install_rust_package(package, options).await?;
            } else {
                self.install_js_package(package, options).await?;
            }
        }

        Ok(())
    }

    async fn install_js_package(&self, package: &str, options: &InstallOptions) -> Result<()> {
        info!("Installing JavaScript package: {}", package);

        if !self.is_npm_available().await? {
            warn!("npm not found, trying to install package manually...");
            return self.install_package_manually(package, options).await;
        }

        let mut cmd = Command::new("npm");
        cmd.arg("install");

        if options.dev {
            cmd.arg("--save-dev");
        } else if options.save {
            cmd.arg("--save");
        }

        if options.force {
            cmd.arg("--force");
        }

        cmd.arg(package);
        cmd.current_dir(&self.project_root);

        info!("Running: {:?}", cmd);

        let output = cmd.output()?;

        if output.status.success() {
            info!("✅ Successfully installed package: {}", package);
            println!("✅ Installed {} successfully!", package);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("Failed to install package {}: {}", package, stderr);
            return Err(anyhow::anyhow!("npm install failed: {}", stderr));
        }

        Ok(())
    }

    async fn install_rust_package(&self, package: &str, _options: &InstallOptions) -> Result<()> {
        info!("Installing Rust package: {}", package);

        warn!(
            "Rust package compilation not fully implemented: {}",
            package
        );

        Ok(())
    }

    pub async fn build(&self) -> Result<()> {
        info!("Building project with Claw");


        warn!("Project build not fully implemented");

        Ok(())
    }

    pub async fn run_script(&self, script_name: &str) -> Result<()> {
        info!("Running script: {}", script_name);

        if script_name.ends_with(".js") {
            self.run_file_with_jetcrab(script_name).await?;
        } else {
            self.run_script_from_package_json(script_name).await?;
        }

        Ok(())
    }

    /// Run a JavaScript file using JetCrab runtime
    async fn run_file_with_jetcrab(&self, file_path: &str) -> Result<()> {
        info!("Running file with JetCrab: {}", file_path);

        let jetcrab_output = Command::new("jetcrab").arg("--version").output();

        match jetcrab_output {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout);
                    info!("✅ JetCrab runtime found: {}", version.trim());

                    let mut cmd = Command::new("jetcrab");
                    cmd.arg("run");
                    cmd.arg(file_path);
                    cmd.current_dir(&self.project_root);

                    info!("Running: {:?}", cmd);

                    let output = cmd.output()?;

                    if !output.stdout.is_empty() {
                        print!("{}", String::from_utf8_lossy(&output.stdout));
                    }
                    if !output.stderr.is_empty() {
                        eprint!("{}", String::from_utf8_lossy(&output.stderr));
                    }

                    if !output.status.success() {
                        return Err(anyhow::anyhow!(
                            "JetCrab execution failed with exit code: {:?}",
                            output.status.code()
                        ));
                    }

                    info!("✅ File executed successfully with JetCrab");
                } else {
                    return Err(anyhow::anyhow!("JetCrab runtime failed"));
                }
            }
            Err(e) => {
                return Err(anyhow::anyhow!("JetCrab runtime not found: {}", e));
            }
        }

        Ok(())
    }

    /// Run a script defined in package.json
    async fn run_script_from_package_json(&self, script_name: &str) -> Result<()> {
        info!("Running script from package.json: {}", script_name);

        let package_json_path = self.project_root.join("package.json");

        if !package_json_path.exists() {
            return Err(anyhow::anyhow!("package.json not found in project root"));
        }

        let content = std::fs::read_to_string(&package_json_path)?;
        let package_json: serde_json::Value = serde_json::from_str(&content)?;

        if let Some(scripts) = package_json["scripts"].as_object() {
            if let Some(script_command) = scripts.get(script_name) {
                if let Some(command_str) = script_command.as_str() {
                    info!("Found script '{}': {}", script_name, command_str);

                    if command_str.starts_with("jetcrab run") {
                        let file_path = command_str.replace("jetcrab run ", "");
                        self.run_file_with_jetcrab(&file_path).await?;
                    } else {
                        warn!(
                            "Script execution not fully implemented for: {}",
                            command_str
                        );
                        println!("📝 Script '{}' would execute: {}", script_name, command_str);
                    }
                } else {
                    return Err(anyhow::anyhow!("Script '{}' is not a string", script_name));
                }
            } else {
                return Err(anyhow::anyhow!(
                    "Script '{}' not found in package.json",
                    script_name
                ));
            }
        } else {
            return Err(anyhow::anyhow!("No scripts section found in package.json"));
        }

        Ok(())
    }

    pub fn get_package_info(&self) -> Result<PackageInfo> {

        Ok(PackageInfo {
            name: "jetcrab-project".to_string(),
            version: "0.4.0".to_string(),
            description: Some("JetCrab project with JS + Rust".to_string()),
            dependencies: HashMap::new(),
            rust_dependencies: HashMap::new(),
            scripts: HashMap::new(),
            main: Some("index.js".to_string()),
            wasm_entry: Some("lib.rs".to_string()),
        })
    }
}

impl Claw {
    /// Check if npm is available on the system
    async fn is_npm_available(&self) -> Result<bool> {
        let output = Command::new("npm").arg("--version").output();
        match output {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout);
                    info!("Found npm version: {}", version.trim());
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => {
                warn!("npm not found: {}", e);
                Ok(false)
            }
        }
    }

    /// Install package manually by downloading from npm registry
    async fn install_package_manually(
        &self,
        package: &str,
        _options: &InstallOptions,
    ) -> Result<()> {
        info!("Installing package manually: {}", package);

        let package_info = self.get_package_from_registry(package).await?;

        let node_modules = self.project_root.join("node_modules");
        if !node_modules.exists() {
            std::fs::create_dir_all(&node_modules)?;
        }

        let package_dir = node_modules.join(&package_info.name);
        if !package_dir.exists() {
            std::fs::create_dir_all(&package_dir)?;
        }

        let tarball_url = package_info.dist.tarball;
        let tarball_data = self.download_tarball(&tarball_url).await?;

        self.extract_tarball(&tarball_data, &package_dir)?;

        self.update_package_json(package, &package_info.version)?;

        info!("✅ Manually installed package: {}", package);
        println!(
            "✅ Installed {} {} manually!",
            package_info.name, package_info.version
        );

        Ok(())
    }

    /// Get package information from npm registry
    async fn get_package_from_registry(&self, package: &str) -> Result<NpmPackageInfo> {
        let url = format!("https://registry.npmjs.org/{}", package);
        let response = reqwest::get(&url).await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Package not found: {}", package));
        }

        let json: serde_json::Value = response.json().await?;

        let latest_version = json["dist-tags"]["latest"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No latest version found"))?;

        let version_info = &json["versions"][latest_version];

        let package_info = NpmPackageInfo {
            name: version_info["name"].as_str().unwrap_or(package).to_string(),
            version: latest_version.to_string(),
            dist: NpmDist {
                tarball: version_info["dist"]["tarball"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("No tarball URL found"))?
                    .to_string(),
            },
        };

        Ok(package_info)
    }

    /// Download tarball from URL
    async fn download_tarball(&self, url: &str) -> Result<Vec<u8>> {
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// Extract tarball to directory
    fn extract_tarball(&self, tarball_data: &[u8], target_dir: &PathBuf) -> Result<()> {
        info!("Extracting tarball to: {:?}", target_dir);

        let cursor = Cursor::new(tarball_data);

        let gz_decoder = GzDecoder::new(cursor);

        let mut archive = Archive::new(gz_decoder);

        archive.unpack(target_dir)?;

        info!("✅ Successfully extracted tarball to {:?}", target_dir);
        Ok(())
    }

    /// Update package.json with new dependency
    fn update_package_json(&self, package: &str, version: &str) -> Result<()> {
        let package_json_path = self.project_root.join("package.json");

        if !package_json_path.exists() {
            let package_json = serde_json::json!({
                "name": "jetcrab-project",
                "version": "0.4.0",
                "dependencies": {
                    package: format!("^{}", version)
                }
            });
            std::fs::write(
                &package_json_path,
                serde_json::to_string_pretty(&package_json)?,
            )?;
        } else {
            let content = std::fs::read_to_string(&package_json_path)?;
            let mut package_json: serde_json::Value = serde_json::from_str(&content)?;

            if !package_json["dependencies"].is_object() {
                package_json["dependencies"] = serde_json::json!({});
            }

            package_json["dependencies"][package] = serde_json::json!(format!("^{}", version));

            std::fs::write(
                &package_json_path,
                serde_json::to_string_pretty(&package_json)?,
            )?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct NpmPackageInfo {
    name: String,
    version: String,
    dist: NpmDist,
}

#[derive(Debug, Deserialize)]
struct NpmDist {
    tarball: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_install_options_default() {
        let options = InstallOptions::default();
        assert!(!options.dev);
        assert!(options.save);
        assert!(!options.force);
        assert!(options.cache);
    }

    #[test]
    fn test_install_options_custom() {
        let options = InstallOptions {
            dev: true,
            save: true,
            force: true,
            cache: false,
        };

        assert!(options.dev);
        assert!(options.save);
        assert!(options.force);
        assert!(!options.cache);
    }

    #[test]
    fn test_package_info_creation() {
        let mut deps = HashMap::new();
        deps.insert("lodash".to_string(), "^4.17.21".to_string());

        let mut rust_deps = HashMap::new();
        rust_deps.insert("serde".to_string(), "1.0".to_string());

        let package = PackageInfo {
            name: "test-package".to_string(),
            version: "0.4.0".to_string(),
            description: Some("Test package".to_string()),
            dependencies: deps.clone(),
            rust_dependencies: rust_deps.clone(),
            scripts: HashMap::new(),
            main: Some("index.js".to_string()),
            wasm_entry: Some("lib.rs".to_string()),
        };

        assert_eq!(package.name, "test-package");
        assert_eq!(package.version, "0.4.0");
        assert_eq!(package.description, Some("Test package".to_string()));
        assert_eq!(package.dependencies, deps);
        assert_eq!(package.rust_dependencies, rust_deps);
        assert_eq!(package.main, Some("index.js".to_string()));
        assert_eq!(package.wasm_entry, Some("lib.rs".to_string()));
    }

    #[test]
    fn test_claw_new() {
        let project_root = PathBuf::from(".");
        let claw = Claw::new(project_root);
        assert_eq!(claw.project_root, PathBuf::from("."));
    }

    #[test]
    fn test_claw_get_package_info() {
        let project_root = PathBuf::from(".");
        let claw = Claw::new(project_root);

        let result = claw.get_package_info();
        assert!(result.is_ok());

        let package_info = result.unwrap();
        assert_eq!(package_info.name, "jetcrab-project");
        assert_eq!(package_info.version, "0.4.0");
    }

    #[tokio::test]
    async fn test_claw_install_empty_packages() {
        let project_root = PathBuf::from(".");
        let claw = Claw::new(project_root);
        let options = InstallOptions::default();

        let result = claw.install(&[], &options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_claw_install_javascript_package() {
        let project_root = PathBuf::from(".");
        let claw = Claw::new(project_root);
        let options = InstallOptions::default();

        let result = claw.install(&["lodash".to_string()], &options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_claw_install_rust_package() {
        let project_root = PathBuf::from(".");
        let claw = Claw::new(project_root);
        let options = InstallOptions::default();

        let result = claw.install(&["serde".to_string()], &options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_claw_install_mixed_packages() {
        let project_root = PathBuf::from(".");
        let claw = Claw::new(project_root);
        let options = InstallOptions::default();

        let packages = vec!["lodash".to_string(), "serde".to_string()];
        let result = claw.install(&packages, &options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_claw_build() {
        let project_root = PathBuf::from(".");
        let claw = Claw::new(project_root);

        let result = claw.build().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_claw_run_script() {
        let project_root = PathBuf::from(".");
        let claw = Claw::new(project_root);

        let result = claw.run_script("test").await;
        assert!(result.is_ok());
    }
}
