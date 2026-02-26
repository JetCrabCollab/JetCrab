//! # REPL (Read-Eval-Print Loop)
//!
//! Interactive REPL for JetCrab Runtime.

use crate::runtime::engine::JetCrabEngine;
use std::io::{self, Write};
use tracing::info;

/// Interactive REPL for JetCrab
pub struct Repl {
    engine: *mut JetCrabEngine,
    history: Vec<String>,
    running: bool,
}

impl Repl {
    /// Create a new REPL instance
    pub fn new(engine: &mut JetCrabEngine) -> Self {
        Self {
            engine: engine as *mut JetCrabEngine,
            history: Vec::new(),
            running: false,
        }
    }

    /// Start the REPL
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting JetCrab REPL");

        self.running = true;
        self.show_welcome();

        while self.running {
            if let Err(e) = self.read_eval_print_loop().await {
                eprintln!("Error: {}", e);
            }
        }

        info!("REPL session ended");
        Ok(())
    }

    /// Stop the REPL
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Main REPL loop
    async fn read_eval_print_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let input = self.read_input()?;

        if input.trim().is_empty() {
            return Ok(());
        }

        if self.handle_special_commands(&input).await? {
            return Ok(());
        }

        self.history.push(input.clone());

        self.evaluate_and_print(&input).await?;

        Ok(())
    }

    /// Read input from user
    fn read_input(&self) -> Result<String, Box<dyn std::error::Error>> {
        print!("jetcrab> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_string())
    }

    /// Handle special REPL commands
    async fn handle_special_commands(
        &mut self,
        input: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let input = input.trim();

        match input {
            ".exit" | ".quit" => {
                println!("Goodbye!");
                self.stop();
                return Ok(true);
            }
            ".help" => {
                self.show_help();
                return Ok(true);
            }
            ".clear" => {
                self.clear_screen();
                return Ok(true);
            }
            ".history" => {
                self.show_history();
                return Ok(true);
            }
            ".version" => {
                self.show_version();
                return Ok(true);
            }
            ".stats" => {
                self.show_stats().await?;
                return Ok(true);
            }
            _ => {
                if self.is_multi_line_input(input) {
                    return Ok(false); // Continue reading
                }
            }
        }

        Ok(false)
    }

    /// Evaluate JavaScript code and print result
    async fn evaluate_and_print(&mut self, input: &str) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let engine = &mut *self.engine;

            match engine.evaluate_to_string(input).await {
                Ok(result) => {
                    if result != "undefined" && !result.is_empty() {
                        println!("{}", result);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Check if input is part of multi-line statement
    fn is_multi_line_input(&self, input: &str) -> bool {
        let open_braces = input.matches('{').count();
        let close_braces = input.matches('}').count();
        let open_parens = input.matches('(').count();
        let close_parens = input.matches(')').count();
        let open_brackets = input.matches('[').count();
        let close_brackets = input.matches(']').count();

        open_braces != close_braces
            || open_parens != close_parens
            || open_brackets != close_brackets
            || input.ends_with('\\')
    }

    /// Show welcome message
    fn show_welcome(&self) {
        println!("🦀 JetCrab Runtime v0.4.0");
        println!("Powered by Chitin (WASM) Engine");
        println!("Type .help for commands, .exit to quit");
        println!();
    }

    /// Show help information
    fn show_help(&self) {
        println!("JetCrab REPL Commands:");
        println!("  .help     - Show this help message");
        println!("  .exit     - Exit the REPL");
        println!("  .quit     - Exit the REPL");
        println!("  .clear    - Clear the screen");
        println!("  .history  - Show command history");
        println!("  .version  - Show version information");
        println!("  .stats    - Show engine statistics");
        println!();
        println!("JavaScript expressions are evaluated directly.");
        println!("Multi-line statements are supported.");
    }

    /// Clear the screen
    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
    }

    /// Show command history
    fn show_history(&self) {
        if self.history.is_empty() {
            println!("No commands in history");
            return;
        }

        println!("Command History:");
        for (i, cmd) in self.history.iter().enumerate() {
            println!("  {}: {}", i + 1, cmd);
        }
    }

    /// Show version information
    fn show_version(&self) {
        println!("JetCrab Runtime v0.4.0");
        println!("Built with Rust");
        println!("JavaScript Engine: Chitin (WASM)");
    }

    /// Show engine statistics
    async fn show_stats(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let engine = &mut *self.engine;
            let stats = engine.get_stats();

            println!("Engine Statistics:");
            println!("  Globals: {}", stats.globals_count);
            println!("  Memory Usage: {} bytes", stats.memory_usage);
            println!("  Executions: {}", stats.execution_count);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repl_creation() {
        let mut engine = JetCrabEngine::new();
        let repl = Repl::new(&mut engine);

        assert!(!repl.running);
        assert!(repl.history.is_empty());
    }

    #[test]
    fn test_multi_line_detection() {
        let mut engine = JetCrabEngine::new();
        let repl = Repl::new(&mut engine);

        assert!(repl.is_multi_line_input("function test() {"));
        assert!(repl.is_multi_line_input("if (true) {"));
        assert!(!repl.is_multi_line_input("2 + 3"));
        assert!(!repl.is_multi_line_input("console.log('hello')"));
    }

    #[test]
    fn test_special_commands() {
        let mut engine = JetCrabEngine::new();
        let mut repl = Repl::new(&mut engine);

        assert_eq!(".exit", ".exit");
        assert_eq!(".help", ".help");
    }
}
