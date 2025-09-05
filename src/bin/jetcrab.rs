//! # JetCrab Runtime
//!
//! A modern JavaScript runtime in Rust, powered by Boa engine.
//!
//! This is the main entry point for the JetCrab CLI.

use clap::Parser;
use jetcrab::cli::framework::{
    handle_panic, init_logging, setup_signal_handlers, CliApp, CliCommand, CliContext, CliError,
    CliResult,
};
use jetcrab::cli::Cli;
use jetcrab::easter_egg::show_walking_claw;
use jetcrab::runtime::JetCrabRuntime;
use std::path::PathBuf;
use std::process;
use tracing::{error, info};

struct EvalCommand {
    code: String,
}

impl EvalCommand {
    fn new(code: String) -> Self {
        Self { code }
    }
}

impl CliCommand for EvalCommand {
    fn name(&self) -> &'static str {
        "eval"
    }

    fn description(&self) -> &'static str {
        "Evaluate JavaScript code directly"
    }

    fn execute(&self, _context: &mut CliContext, matches: &clap::ArgMatches) -> CliResult<()> {
        let code = matches.get_one::<String>("code").unwrap();
        info!("Evaluating JavaScript code: {}", code);

        let code = code.clone();
        let result = std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let mut runtime = JetCrabRuntime::new();
            match rt.block_on(runtime.evaluate_code(&code)) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        })
        .join();

        match result {
            Ok(Ok(_)) => {
                info!("Code executed successfully");
                Ok(())
            }
            Ok(Err(e)) => {
                error!("Failed to evaluate code: {}", e);
                Err(CliError::ExecutionError {
                    command: "eval".to_string(),
                    message: format!("Failed to evaluate JavaScript code: {}", e),
                })
            }
            Err(_) => {
                error!("Thread panicked while evaluating code");
                Err(CliError::ExecutionError {
                    command: "eval".to_string(),
                    message: "Thread panicked while evaluating JavaScript code".to_string(),
                })
            }
        }
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("eval")
            .about("Evaluate JavaScript code")
            .arg(
                clap::Arg::new("code")
                    .help("JavaScript code to evaluate")
                    .required(true)
                    .index(1),
            )
    }

    fn help(&self) -> String {
        "Usage: jetcrab eval <code>\n\nEvaluate JavaScript code directly.\n\nExample:\n  jetcrab eval \"console.log('Hello, World!')\"".to_string()
    }
}

struct RunCommand {
    file_path: PathBuf,
}

impl RunCommand {
    fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }
}

impl CliCommand for RunCommand {
    fn name(&self) -> &'static str {
        "run"
    }

    fn description(&self) -> &'static str {
        "Run a JavaScript file"
    }

    fn execute(&self, _context: &mut CliContext, matches: &clap::ArgMatches) -> CliResult<()> {
        let file_path = matches.get_one::<String>("file").unwrap();
        let file_path = std::path::PathBuf::from(file_path);

        info!("Running JavaScript file: {:?}", file_path);

        if !file_path.exists() {
            return Err(CliError::FileNotFound {
                path: file_path.to_string_lossy().to_string(),
            });
        }

        let file = file_path.clone();
        let result = std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let mut runtime = JetCrabRuntime::new();
            match rt.block_on(runtime.run_file(&file, &[])) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        })
        .join();

        match result {
            Ok(Ok(_)) => {
                info!("File executed successfully");
                Ok(())
            }
            Ok(Err(e)) => {
                error!("Failed to run file: {}", e);
                Err(CliError::ExecutionError {
                    command: "run".to_string(),
                    message: format!("Failed to run JavaScript file: {}", e),
                })
            }
            Err(_) => {
                error!("Thread panicked while running file");
                Err(CliError::ExecutionError {
                    command: "run".to_string(),
                    message: "Thread panicked while running JavaScript file".to_string(),
                })
            }
        }
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("run")
            .about("Run a JavaScript file")
            .arg(
                clap::Arg::new("file")
                    .help("JavaScript file to run")
                    .required(true)
                    .index(1),
            )
    }

    fn help(&self) -> String {
        "Usage: jetcrab run <file>\n\nRun a JavaScript file.\n\nExample:\n  jetcrab run script.js"
            .to_string()
    }
}

struct ReplCommand;

impl CliCommand for ReplCommand {
    fn name(&self) -> &'static str {
        "repl"
    }

    fn description(&self) -> &'static str {
        "Start interactive REPL"
    }

    fn execute(&self, _context: &mut CliContext, _matches: &clap::ArgMatches) -> CliResult<()> {
        info!("Starting interactive REPL");

        println!("🦀 JetCrab REPL v0.4.0");
        println!("Type 'exit' or 'quit' to exit, 'help' for help");

        loop {
            use std::io::{self, Write};

            print!("jetcrab> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
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
                            let input = input.to_string();
                            let result = std::thread::spawn(move || {
                                let rt = tokio::runtime::Runtime::new().unwrap();
                                let mut runtime = JetCrabRuntime::new();
                                match rt.block_on(runtime.evaluate_code(&input)) {
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(e.to_string()),
                                }
                            })
                            .join();

                            match result {
                                Ok(Ok(_)) => {
                                }
                                Ok(Err(e)) => {
                                    error!("Error: {}", e);
                                }
                                Err(_) => {
                                    error!("Thread panicked while evaluating code");
                                }
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

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("repl").about("Start an interactive JavaScript REPL")
    }

    fn help(&self) -> String {
        "Usage: jetcrab repl\n\nStart an interactive JavaScript REPL.\n\nExample:\n  jetcrab repl"
            .to_string()
    }
}

struct CrabCommand;

impl CliCommand for CrabCommand {
    fn name(&self) -> &'static str {
        "crab"
    }

    fn description(&self) -> &'static str {
        "🦀 Easter egg - show walking crab animation"
    }

    fn execute(&self, _context: &mut CliContext, _matches: &clap::ArgMatches) -> CliResult<()> {
        show_walking_claw();
        Ok(())
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("crab").about("🦀 Show a walking crab animation (easter egg)")
    }

    fn help(&self) -> String {
        "Usage: jetcrab crab\n\n🦀 Show a walking crab animation (easter egg).\n\nExample:\n  jetcrab crab".to_string()
    }
}

struct JetCrabCommand {
    cli: Cli,
}

impl JetCrabCommand {
    fn new(cli: Cli) -> Self {
        Self { cli }
    }
}

impl CliCommand for JetCrabCommand {
    fn name(&self) -> &'static str {
        "jetcrab"
    }

    fn description(&self) -> &'static str {
        "A modern JavaScript runtime in Rust"
    }

    fn execute(&self, _context: &mut CliContext, _matches: &clap::ArgMatches) -> CliResult<()> {
        Ok(())
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::Command::new("jetcrab")
            .about("JetCrab - A modern JavaScript runtime in Rust")
            .subcommand_required(true)
            .arg_required_else_help(true)
    }

    fn help(&self) -> String {
        "JetCrab - A modern JavaScript runtime in Rust\n\n\
        USAGE:\n\
            jetcrab <COMMAND>\n\n\
        COMMANDS:\n\
            run     Run a JavaScript file or load a Rust module\n\
            repl    Start interactive REPL\n\
            eval    Evaluate JavaScript code directly\n\
            test    Run tests\n\
            fmt     Format JavaScript code\n\
            lint    Lint JavaScript code\n\
            bundle  Bundle JavaScript modules\n\
            version Show version information\n\
            crab    Show walking crab animation\n\n\
        OPTIONS:\n\
            -h, --help       Print help information\n\
            -V, --version    Print version information\n\
            -v, --verbose    Enable verbose output\n\
            --debug          Enable debug output"
            .to_string()
    }
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    handle_panic();

    if let Err(e) = setup_signal_handlers() {
        eprintln!("Warning: Failed to setup signal handlers: {}", e);
    }

    let cli = Cli::parse();

    if let Err(e) = init_logging(cli.verbose, false) {
        eprintln!("Warning: Failed to initialize logging: {}", e);
    }

    let mut app = CliApp::new(
        "jetcrab".to_string(),
        "0.4.0".to_string(),
        "A modern JavaScript runtime in Rust".to_string(),
    );

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        app.run(args)?;
        return Ok(());
    }

    if args.contains(&"--version".to_string()) || args.contains(&"-V".to_string()) {
        app.run(args)?;
        return Ok(());
    }

    let command_name = &args[0];
    let command_args = &args[1..];

    let command: Box<dyn CliCommand> = match command_name.as_str() {
        "eval" => {
            if command_args.is_empty() {
                return Err(CliError::InvalidArgument {
                    argument: "code".to_string(),
                    reason: "Code argument is required for eval command".to_string(),
                });
            }
            Box::new(EvalCommand::new(command_args.join(" ")))
        }
        "run" => {
            if command_args.is_empty() {
                return Err(CliError::InvalidArgument {
                    argument: "file".to_string(),
                    reason: "File path is required for run command".to_string(),
                });
            }
            Box::new(RunCommand::new(PathBuf::from(&command_args[0])))
        }
        "repl" => Box::new(ReplCommand),
        "crab" => Box::new(CrabCommand),
        _ => {
            return Err(CliError::InvalidArgument {
                argument: command_name.clone(),
                reason: format!(
                    "Unknown command '{}'. Use --help to see available commands.",
                    command_name
                ),
            });
        }
    };

    let cli = Cli::parse();

    if cli.verbose {
        init_logging(true, false)?;
    }

    match cli.command {
        jetcrab::cli::Commands::Run { file, args: _ } => {
            let file_path = file.clone();
            let result = std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let mut runtime = JetCrabRuntime::new();
                match rt.block_on(runtime.run_file(&file_path, &[])) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string()),
                }
            })
            .join();

            match result {
                Ok(Ok(_)) => {
                    info!("File executed successfully");
                    Ok(())
                }
                Ok(Err(e)) => {
                    error!("Failed to run file: {}", e);
                    Err(CliError::ExecutionError {
                        command: "run".to_string(),
                        message: format!("Failed to run JavaScript file: {}", e),
                    })
                }
                Err(_) => {
                    error!("Thread panicked while running file");
                    Err(CliError::ExecutionError {
                        command: "run".to_string(),
                        message: "Thread panicked while running JavaScript file".to_string(),
                    })
                }
            }
        }
        jetcrab::cli::Commands::Eval { code } => {
            let code_str = code.clone();
            let result = std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let mut runtime = JetCrabRuntime::new();
                match rt.block_on(runtime.evaluate_code(&code_str)) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string()),
                }
            })
            .join();

            match result {
                Ok(Ok(_)) => {
                    info!("Code executed successfully");
                    Ok(())
                }
                Ok(Err(e)) => {
                    error!("Failed to evaluate code: {}", e);
                    Err(CliError::ExecutionError {
                        command: "eval".to_string(),
                        message: format!("Failed to evaluate JavaScript code: {}", e),
                    })
                }
                Err(_) => {
                    error!("Thread panicked while evaluating code");
                    Err(CliError::ExecutionError {
                        command: "eval".to_string(),
                        message: "Thread panicked while evaluating JavaScript code".to_string(),
                    })
                }
            }
        }
        jetcrab::cli::Commands::Repl => {
            info!("Starting interactive REPL");

            println!("🦀 JetCrab REPL v0.4.0");
            println!("Type 'exit' or 'quit' to exit, 'help' for help");

            loop {
                use std::io::{self, Write};

                print!("jetcrab> ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        let input = input.trim();

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
                                let input = input.to_string();
                                let result = std::thread::spawn(move || {
                                    let rt = tokio::runtime::Runtime::new().unwrap();
                                    let mut runtime = JetCrabRuntime::new();
                                    match rt.block_on(runtime.evaluate_code(&input)) {
                                        Ok(_) => Ok(()),
                                        Err(e) => Err(e.to_string()),
                                    }
                                })
                                .join();

                                match result {
                                    Ok(Ok(_)) => {}
                                    Ok(Err(e)) => {
                                        error!("Error: {}", e);
                                    }
                                    Err(_) => {
                                        error!("Thread panicked while evaluating code");
                                    }
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
        jetcrab::cli::Commands::Crab => {
            show_walking_claw();
            Ok(())
        }
        _ => {
            error!("Command not implemented yet");
            Err(CliError::InvalidArgument {
                argument: "command".to_string(),
                reason: "Command not implemented yet".to_string(),
            })
        }
    }
}
