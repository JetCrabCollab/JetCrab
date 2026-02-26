use crate::runtime::JetCrabRuntime;
use std::path::PathBuf;
use tracing::{info, error};

pub async fn execute(runtime: &mut JetCrabRuntime, file: &str, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = PathBuf::from(file);
    info!("Running JavaScript file: {:?}", file_path);

    if !file_path.exists() {
        return Err(format!("File not found: {}", file).into());
    }

    // Execute on the EXISTING runtime
    // Was: thread::spawn with new runtime
    // Now: direct async call
    if let Err(e) = runtime.run_file(&file_path, args).await {
        error!("Failed to run file: {}", e);
        return Err(e); // run_file error is already boxed error
    }

    info!("File executed successfully");
    Ok(())
}
