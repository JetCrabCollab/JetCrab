use std::path::Path;
use std::process::Command;

pub fn execute(files: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if files.is_empty() {
        let npm_cmd = if cfg!(windows) { "npm.cmd" } else { "npm" };
        let status = Command::new(npm_cmd)
            .args(["exec", "--", "prettier", "--write", "."])
            .status()?;
        if !status.success() {
            eprintln!("Prettier not found or failed. Install with: npm install -D prettier");
            return Err("fmt failed".into());
        }
        return Ok(());
    }
    let npm_cmd = if cfg!(windows) { "npm.cmd" } else { "npm" };
    let mut cmd = Command::new(npm_cmd);
    cmd.arg("exec").arg("--").arg("prettier").arg("--write");
    for f in files {
        if Path::new(f).exists() {
            cmd.arg(f);
        }
    }
    let status = cmd.status()?;
    if !status.success() {
        eprintln!("Prettier not found or failed. Install with: npm install -D prettier");
        return Err("fmt failed".into());
    }
    Ok(())
}
