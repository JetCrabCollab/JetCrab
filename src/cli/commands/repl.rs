use crate::runtime::JetCrabRuntime;
use tracing::{info, error};
use std::io::{self, Write};

pub async fn execute(runtime: &mut JetCrabRuntime) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting interactive REPL");

    println!("🦀 JetCrab REPL v0.4.0");
    println!("Type 'exit' or 'quit' to exit, 'help' for help");

    // Use a simple buffer for input
    // TODO: Switch to rustyline for history/arrow keys later
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        print!("jetcrab> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match handle.read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                match input {
                    "exit" | "quit" => {
                        println!("Goodbye! 🦀");
                        break;
                    }
                    "help" => {
                        println!("Available commands:");
                        println!("  exit, quit - Exit the REPL");
                        println!("  help - Show this help");
                        println!("  Any JavaScript code will be evaluated");
                    }
                    _ => {
                        // Execute on the EXISTING runtime to preserve state (variables, etc.)
                        if let Err(e) = runtime.evaluate_code(input).await {
                            // Error is already logged in evaluate_code usually, 
                            // but we can print it nicely here too if needed.
                            // The runtime.evaluate_code currently logs "Uncaught Exception".
                            // distinct from runtime internal error.
                            // We just continue the loop.
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to read input: {}", e);
                break;
            }
        }
    }
    Ok(())
}
