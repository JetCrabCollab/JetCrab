use crate::cli::framework::config::CliConfig;
use crate::cli::framework::error::{CliError, CliResult, ErrorContext};
use crate::cli::framework::logging::{init_logging, LoggingConfig};
use crate::cli::framework::validation::InputValidator;
use clap::{Arg, ArgMatches, Command, Parser, Subcommand};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};

pub trait CliCommand: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn execute(&self, context: &mut CliContext, matches: &ArgMatches) -> CliResult<()>;
    fn build_clap_command(&self) -> Command;
    fn help(&self) -> String;
}

pub struct CliContext {
    pub config: CliConfig,
    pub validator: InputValidator,
    pub start_time: Instant,
    pub session_id: String,
    pub user_id: Option<String>,
    pub working_directory: PathBuf,
    pub verbose: bool,
    pub debug: bool,
}

impl CliContext {
    pub fn new(config: CliConfig, verbose: bool, debug: bool) -> Self {
        Self {
            config,
            validator: InputValidator::new(),
            start_time: Instant::now(),
            session_id: uuid::Uuid::new_v4().to_string(),
            user_id: None,
            working_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            verbose,
            debug,
        }
    }

    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_working_directory(mut self, dir: PathBuf) -> Self {
        self.working_directory = dir;
        self
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn create_error_context(&self, command: String) -> ErrorContext {
        ErrorContext::new(command)
            .with_session_id(self.session_id.clone())
            .with_user_id(
                self.user_id
                    .clone()
                    .unwrap_or_else(|| "anonymous".to_string()),
            )
            .add_info(
                "working_directory".to_string(),
                self.working_directory.to_string_lossy().to_string(),
            )
            .add_info("verbose".to_string(), self.verbose.to_string())
            .add_info("debug".to_string(), self.debug.to_string())
    }
}

pub struct CliApp {
    name: String,
    version: String,
    description: String,
    commands: Vec<Box<dyn CliCommand>>,
    config: CliConfig,
    clap_app: Command,
}

impl CliApp {
    pub fn new(name: String, version: String, description: String) -> Self {
        let clap_app = Command::new("app")
            .version("0.4.0")
            .about("CLI Application")
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .help("Enable verbose output")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("debug")
                    .long("debug")
                    .help("Enable debug output")
                    .action(clap::ArgAction::SetTrue),
            );

        Self {
            name,
            version,
            description,
            commands: Vec::new(),
            config: CliConfig::default(),
            clap_app,
        }
    }

    pub fn with_config(mut self, config: CliConfig) -> Self {
        self.config = config;
        self
    }

    pub fn add_command(mut self, command: Box<dyn CliCommand>) -> Self {
        let clap_command = command.build_clap_command();
        self.clap_app = self.clap_app.subcommand(clap_command);
        self.commands.push(command);
        self
    }

    pub fn build_clap_app(mut self) -> Self {
        let mut clap_app = Command::new("jetcrab")
            .version("0.4.0")
            .about("Modern JavaScript runtime (Chitin WASM) + package tools")
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .help("Enable verbose output")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("debug")
                    .long("debug")
                    .help("Enable debug output")
                    .action(clap::ArgAction::SetTrue),
            );

        for command in &self.commands {
            let clap_command = command.build_clap_command();
            clap_app = clap_app.subcommand(clap_command);
        }

        self.clap_app = clap_app;
        self
    }

    pub fn run(&self, args: Vec<String>) -> CliResult<()> {
        let matches = match self.clap_app.clone().try_get_matches_from(args) {
            Ok(matches) => matches,
            Err(e) => {
                eprintln!("{}", e);
                return Ok(()); // clap handles help/version automatically
            }
        };

        let verbose = matches.get_flag("verbose");
        let debug = matches.get_flag("debug");

        init_logging(verbose, debug)?;

        let mut context = CliContext::new(self.config.clone(), verbose, debug);

        info!("Starting {} v{}", self.name, self.version);
        debug!("Session ID: {}", context.session_id);

        if let Some((command_name, sub_matches)) = matches.subcommand() {
            if let Some(command) = self.commands.iter().find(|cmd| cmd.name() == command_name) {
                debug!("Executing command: {}", command.name());

                match command.execute(&mut context, sub_matches) {
                    Ok(_) => {
                        info!(
                            "Command '{}' completed successfully in {:?}",
                            command_name,
                            context.elapsed()
                        );
                        Ok(())
                    }
                    Err(e) => {
                        error!("Command '{}' failed: {:?}", command_name, e);
                        Err(e)
                    }
                }
            } else {
                error!("Unknown command: {}", command_name);
                Err(CliError::InvalidArgument {
                    argument: command_name.to_string(),
                    reason: format!(
                        "Unknown command '{}'. Use --help to see available commands.",
                        command_name
                    ),
                })
            }
        } else {
            self.show_help();
            Ok(())
        }
    }

    fn show_help(&self) {
        println!("{} v{}", self.name, self.version);
        println!("{}", self.description);
        println!();
        println!("USAGE:");
        println!("    {} <COMMAND>", self.name);
        println!();
        println!("COMMANDS:");

        for command in &self.commands {
            println!("    {:<20} {}", command.name(), command.description());
        }

        println!();
        println!("OPTIONS:");
        println!("    -h, --help       Print help information");
        println!("    -V, --version    Print version information");
        println!("    -v, --verbose    Enable verbose output");
        println!("    --debug          Enable debug output");
        println!();
        println!("For more information about a specific command, run:");
        println!("    {} <COMMAND> --help", self.name);
    }

    fn show_version(&self) {
        println!("{} v{}", self.name, self.version);
    }
}

pub struct CommandExecutor {
    context: CliContext,
}

impl CommandExecutor {
    pub fn new(context: CliContext) -> Self {
        Self { context }
    }

    pub fn execute_with_timeout<F, T>(
        &mut self,
        operation: F,
        timeout: Duration,
        operation_name: &str,
    ) -> CliResult<T>
    where
        F: FnOnce() -> CliResult<T>,
    {
        let start = Instant::now();

        debug!(
            "Starting operation '{}' with timeout {:?}",
            operation_name, timeout
        );

        let result = operation();

        let elapsed = start.elapsed();
        debug!("Operation '{}' completed in {:?}", operation_name, elapsed);

        if elapsed > timeout {
            warn!(
                "Operation '{}' took longer than expected timeout",
                operation_name
            );
        }

        result
    }

    pub fn execute_with_retry<F, T>(
        &mut self,
        operation: F,
        max_retries: u32,
        retry_delay: Duration,
        operation_name: &str,
    ) -> CliResult<T>
    where
        F: Fn() -> CliResult<T>,
    {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            debug!(
                "Executing operation '{}' (attempt {}/{})",
                operation_name,
                attempt + 1,
                max_retries + 1
            );

            match operation() {
                Ok(result) => {
                    if attempt > 0 {
                        info!(
                            "Operation '{}' succeeded on attempt {}",
                            operation_name,
                            attempt + 1
                        );
                    }
                    return Ok(result);
                }
                Err(e) => {
                    last_error = Some(e);

                    if attempt < max_retries {
                        warn!(
                            "Operation '{}' failed on attempt {}, retrying in {:?}",
                            operation_name,
                            attempt + 1,
                            retry_delay
                        );
                        std::thread::sleep(retry_delay);
                    }
                }
            }
        }

        error!(
            "Operation '{}' failed after {} attempts",
            operation_name,
            max_retries + 1
        );
        Err(last_error.unwrap())
    }

    pub fn get_context(&self) -> &CliContext {
        &self.context
    }

    pub fn get_context_mut(&mut self) -> &mut CliContext {
        &mut self.context
    }
}

pub fn setup_signal_handlers() -> CliResult<()> {
    ctrlc::set_handler(|| {
        warn!("Received interrupt signal, shutting down gracefully...");
        std::process::exit(130);
    })?;

    Ok(())
}

pub fn handle_panic() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        error!("Panic occurred: {:?}", panic_info);
        hook(panic_info);
    }));
}
