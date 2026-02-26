use crate::runtime::JetCrabRuntime;
use tracing::{info, error};

pub async fn execute(runtime: &mut JetCrabRuntime, code: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Evaluating JavaScript code: {}", code);
    
    // Check for easter eggs in code content
    use crate::easter_egg::{should_trigger_easter_egg, should_trigger_easter_egg_for_command, show_walking_jetcrab};
    
    if should_trigger_easter_egg_for_command(code) {
        show_walking_jetcrab();
    } else if should_trigger_easter_egg() {
        show_walking_jetcrab();
    }

    // Execute on the EXISTING runtime
    if let Err(e) = runtime.evaluate_code(code).await {
        error!("Failed to evaluate code: {}", e);
        return Err(e);
    }

    info!("Code executed successfully");
    Ok(())
}
