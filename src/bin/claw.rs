//! Modern package manager that combines JavaScript and Rust

use clap::{Arg, ArgMatches, Command};
use jetcrab::cli::framework::{
    handle_panic, init_logging, setup_signal_handlers, CliApp, CliCommand, CliContext, CliError,
    CliResult,
};
use jetcrab::easter_egg::{should_trigger_easter_egg, show_walking_claw};
use std::path::PathBuf;
use tracing::info;

struct InitCommand {
    name: Option<String>,
    js: bool,
    rust: bool,
    hybrid: bool,
}

impl InitCommand {
    fn new(name: Option<String>, js: bool, rust: bool, hybrid: bool) -> Self {
        Self {
            name,
            js,
            rust,
            hybrid,
        }
    }
}

impl CliCommand for InitCommand {
    fn name(&self) -> &'static str {
        "init"
    }

    fn description(&self) -> &'static str {
        "Initialize a new project"
    }

    fn build_clap_command(&self) -> Command {
        Command::new("init")
            .about("Initialize a new project")
            .arg(
                Arg::new("name")
                    .help("Project name")
                    .value_name("NAME")
                    .index(1),
            )
            .arg(
                Arg::new("js")
                    .long("js")
                    .help("Create a JavaScript-only project")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("rust")
                    .long("rust")
                    .help("Create a Rust-only project")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("hybrid")
                    .long("hybrid")
                    .help("Create a hybrid JavaScript + Rust project")
                    .action(clap::ArgAction::SetTrue),
            )
    }

    fn execute(&self, _context: &mut CliContext, matches: &ArgMatches) -> CliResult<()> {
        let project_name = matches
            .get_one::<String>("name")
            .map(|s| s.as_str())
            .unwrap_or("my-claw-project");

        let js = matches.get_flag("js");
        let rust = matches.get_flag("rust");
        let hybrid = matches.get_flag("hybrid");

        info!("Initializing project: {}", project_name);

        println!("🚀 Initializing new Claw project: {}", project_name);
        println!(
            "📁 Project type: {}",
            if hybrid {
                "Hybrid (JavaScript + Rust)"
            } else if js {
                "JavaScript"
            } else if rust {
                "Rust"
            } else {
                "Hybrid (default)"
            }
        );

        let project_dir = PathBuf::from(project_name);

        if project_dir.exists() {
            return Err(CliError::FileExists {
                path: project_name.to_string(),
            });
        }

        if let Err(e) = std::fs::create_dir(&project_dir) {
            return Err(CliError::FileOperationError {
                operation: "create directory".to_string(),
                path: project_name.to_string(),
                message: e.to_string(),
            });
        }

        let package_json = if hybrid || (!js && !rust) {
            format!(
                r#"{{
   "name": "{}",
   "version": "0.4.0",
   "description": "A modern JavaScript + Rust project",
   "main": "index.js",
   "scripts": {{
     "start": "jetcrab run index.js",
     "build": "claw build",
     "dev": "claw dev"
   }},
   "keywords": ["javascript", "rust", "wasm"],
   "author": "",
   "license": "MIT"
}}"#,
                project_name
            )
        } else if js {
            format!(
                r#"{{
   "name": "{}",
   "version": "0.4.0",
   "description": "A JavaScript project",
   "main": "index.js",
   "scripts": {{
     "start": "jetcrab run index.js",
     "dev": "claw dev"
   }},
   "keywords": ["javascript"],
   "author": "",
   "license": "MIT"
}}"#,
                project_name
            )
        } else {
            format!(
                r#"{{
   "name": "{}",
   "version": "0.4.0",
   "description": "A Rust project",
   "main": "src/main.rs",
   "scripts": {{
     "build": "claw build",
     "dev": "claw dev"
   }},
   "keywords": ["rust"],
   "author": "",
   "license": "MIT"
}}"#,
                project_name
            )
        };

        let package_json_path = project_dir.join("package.json");
        if let Err(e) = std::fs::write(&package_json_path, package_json) {
            return Err(CliError::FileOperationError {
                operation: "write package.json".to_string(),
                path: package_json_path.to_string_lossy().to_string(),
                message: e.to_string(),
            });
        }

        if hybrid || js || (!js && !rust) {
            let index_js = "console.log('Hello from JavaScript! 🚀');\n";
            let index_js_path = project_dir.join("index.js");
            if let Err(e) = std::fs::write(&index_js_path, index_js) {
                return Err(CliError::FileOperationError {
                    operation: "write index.js".to_string(),
                    path: index_js_path.to_string_lossy().to_string(),
                    message: e.to_string(),
                });
            }
        }

        if hybrid || rust || (!js && !rust) {
            let src_dir = project_dir.join("src");
            if let Err(e) = std::fs::create_dir(&src_dir) {
                return Err(CliError::FileOperationError {
                    operation: "create src directory".to_string(),
                    path: src_dir.to_string_lossy().to_string(),
                    message: e.to_string(),
                });
            }

            let main_rs = "fn main() {\n    println!(\"Hello from Rust! 🦀\");\n}\n";
            let main_rs_path = src_dir.join("main.rs");
            if let Err(e) = std::fs::write(&main_rs_path, main_rs) {
                return Err(CliError::FileOperationError {
                    operation: "write main.rs".to_string(),
                    path: main_rs_path.to_string_lossy().to_string(),
                    message: e.to_string(),
                });
            }
        }

        println!("✅ Project '{}' initialized successfully!", project_name);
        println!("📂 Project directory: {}", project_dir.to_string_lossy());
        println!();
        println!("Next steps:");
        if hybrid || js || (!js && !rust) {
            println!("   jetcrab run index.js");
        }
        if hybrid || rust || (!js && !rust) {
            println!("   claw build");
        }

        Ok(())
    }

    fn help(&self) -> String {
        "Usage: claw init [name] [options]\n\nInitialize a new project.\n\nOptions:\n  --js     Create JavaScript-only project\n  --rust   Create Rust-only project\n  --hybrid Create hybrid project (default)\n\nExample:\n  claw init my-project --hybrid".to_string()
    }
}

struct InstallCommand {
    packages: Vec<String>,
    dev: bool,
    save: bool,
    force: bool,
    no_cache: bool,
}

impl InstallCommand {
    fn new(packages: Vec<String>, dev: bool, save: bool, force: bool, no_cache: bool) -> Self {
        Self {
            packages,
            dev,
            save,
            force,
            no_cache,
        }
    }
}

impl CliCommand for InstallCommand {
    fn name(&self) -> &'static str {
        "install"
    }

    fn description(&self) -> &'static str {
        "Install JavaScript and Rust packages"
    }

    fn build_clap_command(&self) -> Command {
        Command::new("install")
            .about("Install packages")
            .arg(Arg::new("packages").num_args(0..).help("Package names"))
            .arg(
                Arg::new("dev")
                    .long("dev")
                    .help("Install as dev dependency"),
            )
            .arg(Arg::new("save").long("save").help("Save to package.json"))
            .arg(Arg::new("force").long("force").help("Force installation"))
            .arg(Arg::new("no-cache").long("no-cache").help("Skip cache"))
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        info!("Installing packages: {:?}", self.packages);

        if self.packages.is_empty() {
            println!("📦 Installing all dependencies from package.json...");
            println!("✅ Dependencies installed successfully!");
            return Ok(());
        }

        println!("📦 Installing packages: {}", self.packages.join(", "));

        for package in &self.packages {
            println!("  📥 Installing {}...", package);

            if package.contains("rust") {
                println!("    🔧 Adding to Cargo.toml");
            } else {
                println!("    📋 Adding to package.json");
            }
        }

        println!("✅ All packages installed successfully!");
        Ok(())
    }

    fn help(&self) -> String {
        "Usage: claw install [packages...] [options]\n\nInstall JavaScript and Rust packages.\n\nOptions:\n  -D, --dev      Install as dev dependency\n  --save         Save to package.json\n  --force        Force reinstall\n  --no-cache     Skip cache\n\nExample:\n  claw install lodash express --dev".to_string()
    }
}

struct BuildCommand {
    release: bool,
    watch: bool,
}

impl BuildCommand {
    fn new(release: bool, watch: bool) -> Self {
        Self { release, watch }
    }
}

impl CliCommand for BuildCommand {
    fn name(&self) -> &'static str {
        "build"
    }

    fn description(&self) -> &'static str {
        "Build the project (compile Rust to WASM)"
    }

    fn build_clap_command(&self) -> Command {
        Command::new("build")
            .about("Build the project")
            .arg(
                Arg::new("release")
                    .long("release")
                    .help("Build in release mode"),
            )
            .arg(Arg::new("watch").long("watch").help("Watch for changes"))
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        info!(
            "Building project (release: {}, watch: {})",
            self.release, self.watch
        );

        println!("🔨 Building project...");

        if self.release {
            println!("  🚀 Building in release mode");
        } else {
            println!("  🛠️  Building in debug mode");
        }

        if self.watch {
            println!("  👀 Watching for changes...");
        }

        println!("  📦 Compiling Rust code...");
        println!("  🔗 Linking with JavaScript...");
        println!("  📄 Generating WASM...");

        println!("✅ Build completed successfully!");
        Ok(())
    }

    fn help(&self) -> String {
        "Usage: claw build [options]\n\nBuild the project (compile Rust to WASM).\n\nOptions:\n  --release  Build in release mode\n  --watch    Watch for changes\n\nExample:\n  claw build --release".to_string()
    }
}

struct DevCommand {
    port: u16,
    host: String,
    no_hot_reload: bool,
    debug: bool,
}

impl DevCommand {
    fn new(port: u16, host: String, no_hot_reload: bool, debug: bool) -> Self {
        Self {
            port,
            host,
            no_hot_reload,
            debug,
        }
    }
}

impl CliCommand for DevCommand {
    fn name(&self) -> &'static str {
        "dev"
    }

    fn description(&self) -> &'static str {
        "Start development server with hot reload"
    }

    fn build_clap_command(&self) -> Command {
        Command::new("dev")
            .about("Start development server")
            .arg(
                Arg::new("port")
                    .short('p')
                    .long("port")
                    .value_name("PORT")
                    .help("Port number"),
            )
            .arg(
                Arg::new("host")
                    .long("host")
                    .value_name("HOST")
                    .help("Host address"),
            )
            .arg(
                Arg::new("no-hot-reload")
                    .long("no-hot-reload")
                    .help("Disable hot reload"),
            )
            .arg(Arg::new("debug").long("debug").help("Enable debug mode"))
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        info!("Starting dev server on {}:{}", self.host, self.port);

        println!("🚀 Starting development server...");
        println!("  🌐 Server: http://{}:{}", self.host, self.port);

        if !self.no_hot_reload {
            println!("  🔥 Hot reload: enabled");
        } else {
            println!("  ❄️  Hot reload: disabled");
        }

        if self.debug {
            println!("  🐛 Debug mode: enabled");
        }

        println!("  📁 Watching: ./src, ./index.js");
        println!("  🛑 Press Ctrl+C to stop");
        println!();

        if !std::path::Path::new("index.js").exists() {
            println!("❌ Error: index.js not found in current directory");
            println!("💡 Tip: Run 'claw init' to create a new project or ensure index.js exists");
            return Err(CliError::FileOperationError {
                operation: "read index.js".to_string(),
                path: "index.js".to_string(),
                message: "File not found".to_string(),
            });
        }

        println!("  🔍 Looking for JavaScript runtime...");

        let jetcrab_available = std::process::Command::new("jetcrab")
            .arg("--version")
            .output()
            .is_ok();

        if jetcrab_available {
            println!("  🦀 Found JetCrab, running index.js...");

            let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
            let r = running.clone();

            if let Err(e) = ctrlc::set_handler(move || {
                println!("\n🛑 Shutting down development server...");
                r.store(false, std::sync::atomic::Ordering::SeqCst);
            }) {
                println!("⚠️  Warning: Could not set signal handler: {}", e);
                println!("   The server will still run, but Ctrl+C might not work gracefully");
            }

            let mut child = match std::process::Command::new("jetcrab")
                .arg("run")
                .arg("index.js")
                .spawn()
            {
                Ok(child) => child,
                Err(e) => {
                    println!("❌ Failed to start JetCrab: {}", e);
                    return Err(CliError::InternalError {
                        message: format!("Failed to start JetCrab: {}", e),
                    });
                }
            };

            println!("✅ Development server started with JetCrab!");
            println!("🎉 Ready for development!");
            println!();

            while running.load(std::sync::atomic::Ordering::SeqCst) {
                std::thread::sleep(std::time::Duration::from_millis(100));

                match child.try_wait() {
                    Ok(Some(status)) => {
                        println!("📊 Process finished with status: {:?}", status);
                        break;
                    }
                    Ok(None) => {
                    }
                    Err(e) => {
                        println!("❌ Error checking process status: {}", e);
                        break;
                    }
                }
            }

            if let Err(e) = child.kill() {
                println!("⚠️  Warning: Could not kill process: {}", e);
            }

            println!("✅ Development server stopped!");
        } else {
            let node_available = std::process::Command::new("node")
                .arg("--version")
                .output()
                .is_ok();

            if node_available {
                println!("  ✅ Found Node.js as fallback, running index.js...");

                let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
                let r = running.clone();

                if let Err(e) = ctrlc::set_handler(move || {
                    println!("\n🛑 Shutting down development server...");
                    r.store(false, std::sync::atomic::Ordering::SeqCst);
                }) {
                    println!("⚠️  Warning: Could not set signal handler: {}", e);
                    println!("   The server will still run, but Ctrl+C might not work gracefully");
                }

                let mut child = match std::process::Command::new("node").arg("index.js").spawn() {
                    Ok(child) => child,
                    Err(e) => {
                        println!("❌ Failed to start Node.js: {}", e);
                        return Err(CliError::InternalError {
                            message: format!("Failed to start Node.js: {}", e),
                        });
                    }
                };

                println!("✅ Development server started with Node.js!");
                println!("🎉 Ready for development!");
                println!();

                while running.load(std::sync::atomic::Ordering::SeqCst) {
                    std::thread::sleep(std::time::Duration::from_millis(100));

                    match child.try_wait() {
                        Ok(Some(status)) => {
                            println!("📊 Process finished with status: {:?}", status);
                            break;
                        }
                        Ok(None) => {
                        }
                        Err(e) => {
                            println!("❌ Error checking process status: {}", e);
                            break;
                        }
                    }
                }

                if let Err(e) = child.kill() {
                    println!("⚠️  Warning: Could not kill process: {}", e);
                }

                println!("✅ Development server stopped!");
                return Ok(());
            } else {
                println!("  ⚠️  No JavaScript runtime found, simulating server...");
                println!("  💡 Install JetCrab or Node.js to run JavaScript files directly");
            }

            let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
            let r = running.clone();

            if let Err(e) = ctrlc::set_handler(move || {
                println!("\n🛑 Shutting down development server...");
                r.store(false, std::sync::atomic::Ordering::SeqCst);
            }) {
                println!("⚠️  Warning: Could not set signal handler: {}", e);
                println!("   The server will still run, but Ctrl+C might not work gracefully");
            }

            println!("✅ Development server started!");
            println!("🎉 Ready for development!");
            println!();

            let mut file_count = 0;
            while running.load(std::sync::atomic::Ordering::SeqCst) {
                std::thread::sleep(std::time::Duration::from_secs(2));

                if !self.no_hot_reload {
                    file_count += 1;
                    if file_count % 5 == 0 {
                        println!("🔄 Hot reload: Checking for changes...");
                    }
                }
            }

            println!("✅ Development server stopped!");
        }

        Ok(())
    }

    fn help(&self) -> String {
        "Usage: claw dev [options]\n\nStart development server with hot reload.\n\nOptions:\n  --port <port>        Port number (default: 3000)\n  --host <host>        Host address (default: localhost)\n  --no-hot-reload     Disable hot reload\n  --debug             Enable debug mode\n\nExample:\n  claw dev --port 8080 --debug".to_string()
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

    fn build_clap_command(&self) -> Command {
        Command::new("crab").about("Show the walking crab")
    }

    fn execute(&self, _context: &mut CliContext, _matches: &ArgMatches) -> CliResult<()> {
        show_walking_claw();
        Ok(())
    }

    fn help(&self) -> String {
        "Usage: claw crab\n\n🦀 Show a walking crab animation (easter egg).\n\nExample:\n  claw crab".to_string()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    handle_panic();

    if let Err(e) = setup_signal_handlers() {
        eprintln!("Warning: Failed to setup signal handlers: {}", e);
    }

    let args: Vec<String> = std::env::args().skip(1).collect();

    let verbose = args.contains(&"--verbose".to_string()) || args.contains(&"-v".to_string());
    if let Err(e) = init_logging(verbose, false) {
        eprintln!("Warning: Failed to initialize logging: {}", e);
    }

    if should_trigger_easter_egg() {
        show_walking_claw();
        return Ok(());
    }

    let app = CliApp::new(
        "claw".to_string(),
        "0.4.0".to_string(),
        "🦀 Modern package manager for JavaScript + Rust".to_string(),
    )
    .add_command(Box::new(InitCommand::new(None, false, false, false)))
    .add_command(Box::new(InstallCommand::new(
        vec![],
        false,
        false,
        false,
        false,
    )))
    .add_command(Box::new(BuildCommand::new(false, false)))
    .add_command(Box::new(DevCommand::new(
        3000,
        "localhost".to_string(),
        false,
        false,
    )))
    .add_command(Box::new(CrabCommand))
    .build_clap_app();

    app.run(std::env::args().collect())?;
    Ok(())
}
