use std::path::Path;
use std::process::Command;

pub fn execute(files: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let npm_cmd = if cfg!(windows) { "npm.cmd" } else { "npm" };
    let mut cmd = Command::new(npm_cmd);
    cmd.arg("exec").arg("--").arg("eslint");
    if files.is_empty() {
        cmd.arg(".");
    } else {
        for f in files {
            if Path::new(f).exists() {
                cmd.arg(f);
            }
        }
    }
    let status = cmd.status()?;
    if !status.success() {
        return Err("lint reported issues or eslint not found. Install with: npm install -D eslint".into());
    }
    Ok(())
}
