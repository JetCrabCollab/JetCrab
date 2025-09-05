use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct HelpSection {
    pub title: String,
    pub content: String,
    pub subsections: Vec<HelpSection>,
}

impl HelpSection {
    pub fn new(title: String, content: String) -> Self {
        Self {
            title,
            content,
            subsections: Vec::new(),
        }
    }

    pub fn with_subsection(mut self, subsection: HelpSection) -> Self {
        self.subsections.push(subsection);
        self
    }

    pub fn format(&self, width: usize) -> String {
        let mut output = String::new();
        self.format_recursive(&mut output, 0, width);
        output
    }

    fn format_recursive(&self, output: &mut String, indent: usize, width: usize) {
        let indent_str = " ".repeat(indent);

        if !self.title.is_empty() {
            output.push_str(&format!("{}{}\n", indent_str, self.title));
            output.push_str(&format!("{}{}\n", indent_str, "=".repeat(self.title.len())));
        }

        if !self.content.is_empty() {
            let wrapped_content = self.wrap_text(&self.content, width - indent);
            for line in wrapped_content {
                output.push_str(&format!("{}{}\n", indent_str, line));
            }
            output.push('\n');
        }

        for subsection in &self.subsections {
            subsection.format_recursive(output, indent + 2, width);
        }
    }

    fn wrap_text(&self, text: &str, width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.len() + word.len() + 1 <= width {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            } else {
                if !current_line.is_empty() {
                    lines.push(current_line);
                }
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }
}

#[derive(Debug, Clone)]
pub struct CommandHelp {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub examples: Vec<Example>,
    pub options: Vec<OptionHelp>,
    pub arguments: Vec<ArgumentHelp>,
    pub subcommands: Vec<CommandHelp>,
    pub see_also: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Example {
    pub description: String,
    pub command: String,
    pub explanation: Option<String>,
}

#[derive(Debug, Clone)]
pub struct OptionHelp {
    pub name: String,
    pub short: Option<char>,
    pub long: Option<String>,
    pub description: String,
    pub default_value: Option<String>,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub struct ArgumentHelp {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub multiple: bool,
    pub default_value: Option<String>,
}

impl CommandHelp {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            usage: String::new(),
            examples: Vec::new(),
            options: Vec::new(),
            arguments: Vec::new(),
            subcommands: Vec::new(),
            see_also: Vec::new(),
        }
    }

    pub fn with_usage(mut self, usage: String) -> Self {
        self.usage = usage;
        self
    }

    pub fn with_example(mut self, example: Example) -> Self {
        self.examples.push(example);
        self
    }

    pub fn with_option(mut self, option: OptionHelp) -> Self {
        self.options.push(option);
        self
    }

    pub fn with_argument(mut self, argument: ArgumentHelp) -> Self {
        self.arguments.push(argument);
        self
    }

    pub fn with_subcommand(mut self, subcommand: CommandHelp) -> Self {
        self.subcommands.push(subcommand);
        self
    }

    pub fn with_see_also(mut self, command: String) -> Self {
        self.see_also.push(command);
        self
    }

    pub fn format(&self, width: usize) -> String {
        let mut output = String::new();

        output.push_str(&format!("{}\n", self.name));
        output.push_str(&format!("{}\n", "=".repeat(self.name.len())));
        output.push_str(&format!("{}\n\n", self.description));

        if !self.usage.is_empty() {
            output.push_str("USAGE:\n");
            output.push_str(&format!("    {}\n\n", self.usage));
        }

        if !self.arguments.is_empty() {
            output.push_str("ARGUMENTS:\n");
            for arg in &self.arguments {
                let required = if arg.required { " (required)" } else { "" };
                let multiple = if arg.multiple { "..." } else { "" };
                let default = if let Some(default) = &arg.default_value {
                    format!(" (default: {})", default)
                } else {
                    String::new()
                };

                output.push_str(&format!(
                    "    {}{}{}{}\n",
                    arg.name, multiple, required, default
                ));
                output.push_str(&format!("        {}\n", arg.description));
            }
            output.push('\n');
        }

        if !self.options.is_empty() {
            output.push_str("OPTIONS:\n");
            for option in &self.options {
                let mut option_str = String::new();

                if let Some(short) = option.short {
                    option_str.push_str(&format!("-{}, ", short));
                }

                if let Some(long) = &option.long {
                    option_str.push_str(&format!("--{}", long));
                }

                let required = if option.required { " (required)" } else { "" };
                let default = if let Some(default) = &option.default_value {
                    format!(" (default: {})", default)
                } else {
                    String::new()
                };

                output.push_str(&format!("    {}{}{}\n", option_str, required, default));
                output.push_str(&format!("        {}\n", option.description));
            }
            output.push('\n');
        }

        if !self.subcommands.is_empty() {
            output.push_str("SUBCOMMANDS:\n");
            for subcommand in &self.subcommands {
                output.push_str(&format!(
                    "    {:<20} {}\n",
                    subcommand.name, subcommand.description
                ));
            }
            output.push('\n');
        }

        if !self.examples.is_empty() {
            output.push_str("EXAMPLES:\n");
            for (i, example) in self.examples.iter().enumerate() {
                output.push_str(&format!("    {}. {}\n", i + 1, example.description));
                output.push_str(&format!("       {}\n", example.command));
                if let Some(explanation) = &example.explanation {
                    output.push_str(&format!("       {}\n", explanation));
                }
                output.push('\n');
            }
        }

        if !self.see_also.is_empty() {
            output.push_str("SEE ALSO:\n");
            for command in &self.see_also {
                output.push_str(&format!("    {}\n", command));
            }
        }

        output
    }
}

pub struct HelpSystem {
    commands: HashMap<String, CommandHelp>,
    global_help: Option<HelpSection>,
}

impl HelpSystem {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            global_help: None,
        }
    }

    pub fn add_command(&mut self, help: CommandHelp) {
        self.commands.insert(help.name.clone(), help);
    }

    pub fn set_global_help(&mut self, help: HelpSection) {
        self.global_help = Some(help);
    }

    pub fn get_command_help(&self, command_name: &str) -> Option<&CommandHelp> {
        self.commands.get(command_name)
    }

    pub fn list_commands(&self) -> Vec<&CommandHelp> {
        self.commands.values().collect()
    }

    pub fn search_commands(&self, query: &str) -> Vec<&CommandHelp> {
        self.commands
            .values()
            .filter(|help| {
                help.name.contains(query)
                    || help
                        .description
                        .to_lowercase()
                        .contains(&query.to_lowercase())
            })
            .collect()
    }

    pub fn format_global_help(&self, width: usize) -> String {
        if let Some(global_help) = &self.global_help {
            global_help.format(width)
        } else {
            String::new()
        }
    }

    pub fn format_command_help(&self, command_name: &str, width: usize) -> Option<String> {
        self.get_command_help(command_name)
            .map(|help| help.format(width))
    }

    pub fn format_all_commands(&self, width: usize) -> String {
        let mut output = String::new();

        output.push_str("AVAILABLE COMMANDS:\n");
        output.push_str("==================\n\n");

        let mut commands: Vec<_> = self.commands.values().collect();
        commands.sort_by(|a, b| a.name.cmp(&b.name));

        for command in commands {
            output.push_str(&format!("{:<20} {}\n", command.name, command.description));
        }

        output
    }
}

pub struct InteractiveHelp {
    help_system: HelpSystem,
    history: Vec<String>,
    current_command: Option<String>,
}

impl InteractiveHelp {
    pub fn new(help_system: HelpSystem) -> Self {
        Self {
            help_system,
            history: Vec::new(),
            current_command: None,
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Welcome to the interactive help system!");
        println!("Type 'help' for available commands, 'quit' to exit.");

        loop {
            print!("help> ");
            std::io::Write::flush(&mut std::io::stdout())?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            self.history.push(input.to_string());

            match input {
                "quit" | "exit" => break,
                "help" => self.show_help(),
                "list" => self.list_commands(),
                "history" => self.show_history(),
                "clear" => self.clear_screen(),
                _ => {
                    if input.starts_with("help ") {
                        let command = &input[5..];
                        self.show_command_help(command);
                    } else if input.starts_with("search ") {
                        let query = &input[7..];
                        self.search_commands(query);
                    } else {
                        println!(
                            "Unknown command: {}. Type 'help' for available commands.",
                            input
                        );
                    }
                }
            }
        }

        Ok(())
    }

    fn show_help(&self) {
        println!("Available commands:");
        println!("  help [command]  - Show help for a specific command");
        println!("  list            - List all available commands");
        println!("  search <query>  - Search for commands");
        println!("  history         - Show command history");
        println!("  clear           - Clear the screen");
        println!("  quit/exit       - Exit the help system");
    }

    fn list_commands(&self) {
        println!("{}", self.help_system.format_all_commands(80));
    }

    fn show_command_help(&self, command_name: &str) {
        if let Some(help) = self.help_system.format_command_help(command_name, 80) {
            println!("{}", help);
        } else {
            println!("Command '{}' not found.", command_name);
        }
    }

    fn search_commands(&self, query: &str) {
        let results = self.help_system.search_commands(query);
        if results.is_empty() {
            println!("No commands found matching '{}'.", query);
        } else {
            println!("Commands matching '{}':", query);
            for command in results {
                println!("  {:<20} {}", command.name, command.description);
            }
        }
    }

    fn show_history(&self) {
        if self.history.is_empty() {
            println!("No commands in history.");
        } else {
            println!("Command history:");
            for (i, command) in self.history.iter().enumerate() {
                println!("  {}: {}", i + 1, command);
            }
        }
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }
}

pub fn create_jetcrab_help_system() -> HelpSystem {
    let mut help_system = HelpSystem::new();

    let global_help = HelpSection::new(
        "JetCrab - A modern JavaScript runtime in Rust".to_string(),
        "JetCrab is a modern JavaScript runtime built with Rust, powered by the Boa engine. It provides a fast, secure, and extensible environment for running JavaScript applications with native Rust integration.".to_string(),
    )
    .with_subsection(HelpSection::new(
        "Features".to_string(),
        "• Fast JavaScript execution with Boa engine\n• Native Rust integration\n• Comprehensive Node.js API compatibility\n• WebAssembly support\n• Built-in package management with Claw\n• Development tools and hot reloading".to_string(),
    ))
    .with_subsection(HelpSection::new(
        "Getting Started".to_string(),
        "1. Install JetCrab from source or package manager\n2. Create a new project with 'claw init'\n3. Write your JavaScript code\n4. Run with 'jetcrab run your-file.js'".to_string(),
    ));

    help_system.set_global_help(global_help);

    let run_help = CommandHelp::new(
        "run".to_string(),
        "Run a JavaScript file or load a Rust module".to_string(),
    )
    .with_usage("jetcrab run <file> [args...]".to_string())
    .with_argument(ArgumentHelp {
        name: "file".to_string(),
        description: "JavaScript file (.js) or Rust module (.rs) to execute".to_string(),
        required: true,
        multiple: false,
        default_value: None,
    })
    .with_argument(ArgumentHelp {
        name: "args".to_string(),
        description: "Additional arguments to pass to the script".to_string(),
        required: false,
        multiple: true,
        default_value: None,
    })
    .with_example(Example {
        description: "Run a simple JavaScript file".to_string(),
        command: "jetcrab run hello.js".to_string(),
        explanation: Some("Executes the hello.js file in the JetCrab runtime".to_string()),
    })
    .with_example(Example {
        description: "Run with arguments".to_string(),
        command: "jetcrab run app.js --port 3000 --debug".to_string(),
        explanation: Some("Runs app.js and passes --port 3000 --debug as arguments".to_string()),
    });

    help_system.add_command(run_help);

    let repl_help = CommandHelp::new(
        "repl".to_string(),
        "Start interactive REPL (Read-Eval-Print Loop)".to_string(),
    )
    .with_usage("jetcrab repl".to_string())
    .with_example(Example {
        description: "Start interactive session".to_string(),
        command: "jetcrab repl".to_string(),
        explanation: Some(
            "Opens an interactive JavaScript console where you can type and execute code"
                .to_string(),
        ),
    });

    help_system.add_command(repl_help);

    let install_help = CommandHelp::new(
        "install".to_string(),
        "Install JavaScript and Rust packages".to_string(),
    )
    .with_usage("claw install [packages...] [options]".to_string())
    .with_argument(ArgumentHelp {
        name: "packages".to_string(),
        description: "Package names to install".to_string(),
        required: false,
        multiple: true,
        default_value: None,
    })
    .with_option(OptionHelp {
        name: "dev".to_string(),
        short: Some('D'),
        long: Some("dev".to_string()),
        description: "Install as development dependency".to_string(),
        default_value: None,
        required: false,
    })
    .with_option(OptionHelp {
        name: "save".to_string(),
        short: None,
        long: Some("save".to_string()),
        description: "Save to package.json".to_string(),
        default_value: None,
        required: false,
    })
    .with_example(Example {
        description: "Install a package".to_string(),
        command: "claw install express".to_string(),
        explanation: Some("Installs the express package from npm".to_string()),
    })
    .with_example(Example {
        description: "Install as dev dependency".to_string(),
        command: "claw install --dev jest".to_string(),
        explanation: Some("Installs jest as a development dependency".to_string()),
    });

    help_system.add_command(install_help);

    help_system
}
