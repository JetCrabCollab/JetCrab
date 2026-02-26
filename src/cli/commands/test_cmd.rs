use crate::runtime::JetCrabRuntime;
use std::path::Path;
use std::process::Command;

pub async fn execute(_runtime: &mut JetCrabRuntime) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new("package.json").exists() {
        let npm_cmd = if cfg!(windows) { "npm.cmd" } else { "npm" };
        let status = Command::new(npm_cmd).arg("test").status()?;
        if !status.success() {
            return Err("npm test failed".into());
        }
        return Ok(());
    }
    if Path::new("Cargo.toml").exists() {
        let status = Command::new("cargo").arg("test").status()?;
        if !status.success() {
            return Err("cargo test failed".into());
        }
        return Ok(());
    }
    eprintln!("No package.json or Cargo.toml found. Run from a project directory or use 'cpm init'.");
    Err("No test configuration".into())
}
