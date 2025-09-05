use crate::cli::framework::{CliCommand, CliContext, CliError, CliResult, PluginManager};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum CompletionType {
    File,
    Directory,
    Command,
    Argument,
    Custom(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub value: String,
    pub description: Option<String>,
    pub completion_type: CompletionType,
}

impl CompletionItem {
    pub fn new(value: String) -> Self {
        Self {
            value,
            description: None,
            completion_type: CompletionType::Custom(vec![]),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_type(mut self, completion_type: CompletionType) -> Self {
        self.completion_type = completion_type;
        self
    }
}

pub struct CompletionGenerator {
    commands: HashMap<String, Vec<CompletionItem>>,
    file_extensions: Vec<String>,
    directory_blacklist: Vec<String>,
}

impl Default for CompletionGenerator {
    fn default() -> Self {
        Self {
            commands: HashMap::new(),
            file_extensions: vec![
                "js".to_string(),
                "ts".to_string(),
                "json".to_string(),
                "toml".to_string(),
                "yaml".to_string(),
                "yml".to_string(),
                "rs".to_string(),
                "md".to_string(),
                "txt".to_string(),
            ],
            directory_blacklist: vec![
                "node_modules".to_string(),
                "target".to_string(),
                ".git".to_string(),
                ".svn".to_string(),
                ".hg".to_string(),
                "dist".to_string(),
                "build".to_string(),
                "out".to_string(),
            ],
        }
    }
}

impl CompletionGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_command_completions(&mut self, command: String, completions: Vec<CompletionItem>) {
        self.commands.insert(command, completions);
    }

    pub fn generate_completions(&self, command: &str, current_word: &str) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        if let Some(command_completions) = self.commands.get(command) {
            for completion in command_completions {
                if completion.value.starts_with(current_word) {
                    completions.push(completion.clone());
                }
            }
        }

        completions
    }

    pub fn generate_file_completions(&self, current_word: &str) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        let path = PathBuf::from(current_word);
        let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
        let prefix = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");

        if let Ok(entries) = std::fs::read_dir(parent) {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();

                if file_name_str.starts_with(prefix) {
                    let full_path = entry.path();
                    let completion_type = if full_path.is_dir() {
                        CompletionType::Directory
                    } else {
                        CompletionType::File
                    };

                    let description = if full_path.is_dir() {
                        Some("Directory".to_string())
                    } else if let Some(ext) = full_path.extension() {
                        Some(format!("File ({})", ext.to_string_lossy()))
                    } else {
                        Some("File".to_string())
                    };

                    completions.push(CompletionItem {
                        value: full_path.to_string_lossy().to_string(),
                        description,
                        completion_type,
                    });
                }
            }
        }

        completions.sort_by(|a, b| match (&a.completion_type, &b.completion_type) {
            (CompletionType::Directory, CompletionType::File) => std::cmp::Ordering::Less,
            (CompletionType::File, CompletionType::Directory) => std::cmp::Ordering::Greater,
            _ => a.value.cmp(&b.value),
        });

        completions
    }

    pub fn generate_directory_completions(&self, current_word: &str) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        let path = PathBuf::from(current_word);
        let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
        let prefix = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");

        if let Ok(entries) = std::fs::read_dir(parent) {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();

                if file_name_str.starts_with(prefix) && entry.path().is_dir() {
                    if !self
                        .directory_blacklist
                        .contains(&file_name_str.to_string())
                    {
                        completions.push(CompletionItem {
                            value: entry.path().to_string_lossy().to_string(),
                            description: Some("Directory".to_string()),
                            completion_type: CompletionType::Directory,
                        });
                    }
                }
            }
        }

        completions.sort_by(|a, b| a.value.cmp(&b.value));
        completions
    }

    pub fn generate_package_completions(&self, current_word: &str) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        if let Ok(package_json) = std::fs::read_to_string("package.json") {
            if let Ok(package) = serde_json::from_str::<serde_json::Value>(&package_json) {
                if let Some(dependencies) = package.get("dependencies").and_then(|d| d.as_object())
                {
                    for (name, _) in dependencies {
                        if name.starts_with(current_word) {
                            completions.push(CompletionItem {
                                value: name.clone(),
                                description: Some("Installed package".to_string()),
                                completion_type: CompletionType::Custom(vec![]),
                            });
                        }
                    }
                }

                if let Some(dev_dependencies) =
                    package.get("devDependencies").and_then(|d| d.as_object())
                {
                    for (name, _) in dev_dependencies {
                        if name.starts_with(current_word) {
                            completions.push(CompletionItem {
                                value: name.clone(),
                                description: Some("Dev dependency".to_string()),
                                completion_type: CompletionType::Custom(vec![]),
                            });
                        }
                    }
                }
            }
        }

        completions.sort_by(|a, b| a.value.cmp(&b.value));
        completions
    }
}

pub struct ShellCompletion {
    generator: CompletionGenerator,
}

impl ShellCompletion {
    pub fn new() -> Self {
        Self {
            generator: CompletionGenerator::new(),
        }
    }

    pub fn generate_bash_completion(&self, command: &str, current_word: &str) -> String {
        let completions = self.generator.generate_completions(command, current_word);
        let mut output = String::new();

        for completion in completions {
            if let Some(description) = &completion.description {
                output.push_str(&format!("{}:{}\n", completion.value, description));
            } else {
                output.push_str(&format!("{}\n", completion.value));
            }
        }

        output
    }

    pub fn generate_zsh_completion(&self, command: &str, current_word: &str) -> String {
        let completions = self.generator.generate_completions(command, current_word);
        let mut output = String::new();

        for completion in completions {
            if let Some(description) = &completion.description {
                output.push_str(&format!("{}[{}]\n", completion.value, description));
            } else {
                output.push_str(&format!("{}\n", completion.value));
            }
        }

        output
    }

    pub fn generate_fish_completion(&self, command: &str, current_word: &str) -> String {
        let completions = self.generator.generate_completions(command, current_word);
        let mut output = String::new();

        for completion in completions {
            if let Some(description) = &completion.description {
                output.push_str(&format!("{}\t{}\n", completion.value, description));
            } else {
                output.push_str(&format!("{}\n", completion.value));
            }
        }

        output
    }

    pub fn generate_powershell_completion(&self, command: &str, current_word: &str) -> String {
        let completions = self.generator.generate_completions(command, current_word);
        let mut output = String::new();

        for completion in completions {
            if let Some(description) = &completion.description {
                output.push_str(&format!("{} # {}\n", completion.value, description));
            } else {
                output.push_str(&format!("{}\n", completion.value));
            }
        }

        output
    }

    pub fn install_bash_completion(&self, app_name: &str) -> String {
        format!(
            r#"# Add to ~/.bashrc or ~/.bash_profile
complete -C "{} --complete" {}
"#,
            app_name, app_name
        )
    }

    pub fn install_zsh_completion(&self, app_name: &str) -> String {
        format!(
            r#"# Add to ~/.zshrc
autoload -U compinit && compinit
source <({} --complete zsh)
"#,
            app_name
        )
    }

    pub fn install_fish_completion(&self, app_name: &str) -> String {
        format!(
            r#"# Add to ~/.config/fish/completions/{}.fish
{} --complete fish
"#,
            app_name, app_name
        )
    }

    pub fn install_powershell_completion(&self, app_name: &str) -> String {
        format!(
            r#"# Add to PowerShell profile
Register-ArgumentCompleter -Native -CommandName {} -ScriptBlock {{
    param($commandName, $wordToComplete, $cursorPosition)
    {} --complete powershell | ForEach-Object {{ $_ }}
}}
"#,
            app_name, app_name
        )
    }
}

pub struct AdvancedCompletionSystem {
    generator: CompletionGenerator,
    plugin_manager: Option<Arc<PluginManager>>,
    command_cache: HashMap<String, Vec<CompletionItem>>,
    context_aware: bool,
    fuzzy_matching: bool,
    case_sensitive: bool,
}

impl AdvancedCompletionSystem {
    pub fn new() -> Self {
        Self {
            generator: CompletionGenerator::new(),
            plugin_manager: None,
            command_cache: HashMap::new(),
            context_aware: true,
            fuzzy_matching: true,
            case_sensitive: false,
        }
    }

    pub fn with_plugin_manager(mut self, manager: Arc<PluginManager>) -> Self {
        self.plugin_manager = Some(manager);
        self
    }

    pub fn with_context_aware(mut self, enabled: bool) -> Self {
        self.context_aware = enabled;
        self
    }

    pub fn with_fuzzy_matching(mut self, enabled: bool) -> Self {
        self.fuzzy_matching = enabled;
        self
    }

    pub fn with_case_sensitive(mut self, enabled: bool) -> Self {
        self.case_sensitive = enabled;
        self
    }

    pub fn generate_completions(
        &mut self,
        command: &str,
        current_word: &str,
        context: &CliContext,
    ) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        completions.extend(self.generator.generate_completions(command, current_word));

        if let Some(plugin_manager) = &self.plugin_manager {
            completions.extend(self.generate_plugin_completions(
                command,
                current_word,
                plugin_manager,
            ));
        }

        if self.context_aware {
            completions.extend(self.generate_context_completions(current_word, context));
        }

        if self.fuzzy_matching {
            completions = self.apply_fuzzy_matching(completions, current_word);
        }

        completions.sort_by(|a, b| match (&a.completion_type, &b.completion_type) {
            (CompletionType::Directory, CompletionType::File) => std::cmp::Ordering::Less,
            (CompletionType::File, CompletionType::Directory) => std::cmp::Ordering::Greater,
            _ => {
                if self.case_sensitive {
                    a.value.cmp(&b.value)
                } else {
                    a.value.to_lowercase().cmp(&b.value.to_lowercase())
                }
            }
        });

        completions.dedup_by(|a, b| a.value == b.value);
        completions
    }

    fn generate_plugin_completions(
        &self,
        command: &str,
        current_word: &str,
        plugin_manager: &PluginManager,
    ) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        let plugin_commands = plugin_manager.list_commands();
        for cmd_name in plugin_commands {
            if cmd_name.starts_with(current_word) {
                completions.push(
                    CompletionItem::new(cmd_name)
                        .with_description("Plugin command".to_string())
                        .with_type(CompletionType::Command),
                );
            }
        }

        let plugins = plugin_manager.list_plugins();
        for plugin in plugins {
            for cmd_metadata in plugin.metadata.commands {
                if cmd_metadata.name == command {
                    for option in cmd_metadata.options {
                        if let Some(long) = &option.long {
                            if long.starts_with(current_word) {
                                completions.push(
                                    CompletionItem::new(format!("--{}", long))
                                        .with_description(option.description.clone())
                                        .with_type(CompletionType::Argument),
                                );
                            }
                        }
                        if let Some(short) = option.short {
                            if format!("-{}", short).starts_with(current_word) {
                                completions.push(
                                    CompletionItem::new(format!("-{}", short))
                                        .with_description(option.description.clone())
                                        .with_type(CompletionType::Argument),
                                );
                            }
                        }
                    }
                }
            }
        }

        completions
    }

    fn generate_context_completions(
        &self,
        current_word: &str,
        context: &CliContext,
    ) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        if current_word.starts_with('$') {
            for (key, _) in std::env::vars() {
                if key.starts_with(&current_word[1..]) {
                    completions.push(
                        CompletionItem::new(format!("${}", key))
                            .with_description("Environment variable".to_string())
                            .with_type(CompletionType::Custom(vec![])),
                    );
                }
            }
        }

        if current_word.starts_with("--config") || current_word.starts_with("-c") {
            completions.extend(self.generate_config_completions());
        }

        if current_word.starts_with("./") || current_word.starts_with("../") {
            completions.extend(self.generator.generate_file_completions(current_word));
        }

        completions
    }

    fn generate_config_completions(&self) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        let config_locations = vec![
            "~/.jetcrab/config.toml",
            "./jetcrab.toml",
            "./.jetcrab/config.toml",
            "./config.toml",
        ];

        for location in config_locations {
            completions.push(
                CompletionItem::new(location.to_string())
                    .with_description("Configuration file".to_string())
                    .with_type(CompletionType::File),
            );
        }

        completions
    }

    fn apply_fuzzy_matching(
        &self,
        mut completions: Vec<CompletionItem>,
        query: &str,
    ) -> Vec<CompletionItem> {
        if query.is_empty() {
            return completions;
        }

        completions.retain(|item| {
            self.fuzzy_match(&item.value, query)
                || item
                    .description
                    .as_ref()
                    .map_or(false, |desc| self.fuzzy_match(desc, query))
        });

        completions.sort_by(|a, b| {
            let a_exact = a.value.starts_with(query);
            let b_exact = b.value.starts_with(query);

            match (a_exact, b_exact) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.value.len().cmp(&b.value.len()),
            }
        });

        completions
    }

    fn fuzzy_match(&self, text: &str, query: &str) -> bool {
        if self.case_sensitive {
            self.fuzzy_match_case_sensitive(text, query)
        } else {
            self.fuzzy_match_case_sensitive(&text.to_lowercase(), &query.to_lowercase())
        }
    }

    fn fuzzy_match_case_sensitive(&self, text: &str, query: &str) -> bool {
        let mut text_chars = text.chars();

        for query_char in query.chars() {
            if !text_chars.any(|c| c == query_char) {
                return false;
            }
        }

        true
    }

    pub fn generate_shell_completion_script(&self, shell: &str, app_name: &str) -> String {
        match shell {
            "bash" => self.generate_bash_completion_script(app_name),
            "zsh" => self.generate_zsh_completion_script(app_name),
            "fish" => self.generate_fish_completion_script(app_name),
            "powershell" => self.generate_powershell_completion_script(app_name),
            _ => format!("# Unsupported shell: {}", shell),
        }
    }

    fn generate_bash_completion_script(&self, app_name: &str) -> String {
        format!(
            r#"# Bash completion for {}
complete -F _{}_completion {}

_{}_completion() {{
    local cur prev opts
    COMPREPLY=()
    cur="${{COMP_WORDS[COMP_CWORD]}}"
    prev="${{COMP_WORDS[COMP_CWORD-1]}}"
    
    # Get completions from the application
    local completions
    completions=$({} --complete bash "$cur" 2>/dev/null)
    
    if [ $? -eq 0 ]; then
        COMPREPLY=( $(compgen -W "$completions" -- "$cur") )
        return 0
    fi
    
    # Fallback to basic completions
    case "$prev" in
        --config|-c)
            COMPREPLY=( $(compgen -f -- "$cur") )
            return 0
            ;;
        --help|-h|--version|-V)
            return 0
            ;;
    esac
    
    # Command completions
    local commands="run repl plugin"
    COMPREPLY=( $(compgen -W "$commands" -- "$cur") )
}}
"#,
            app_name, app_name, app_name, app_name, app_name
        )
    }

    fn generate_zsh_completion_script(&self, app_name: &str) -> String {
        format!(
            r#"# Zsh completion for {}
_{}() {{
    local context state line
    typeset -A opt_args
    
    _arguments -C \\
        '1: :->command' \\
        '*::arg:->args' \\
        '(-h --help)'{{-h,--help}}'[Show help information]' \\
        '(-V --version)'{{-V,--version}}'[Show version information]' \\
        '(-v --verbose)'{{-v,--verbose}}'[Enable verbose output]' \\
        '(--debug)'{{--debug}}'[Enable debug output]' \\
        '(-c --config)'{{-c,--config}}'[Configuration file]:config file:_files' \\
        && return 0
    
    case $state in
        command)
            local commands
            commands=(
                'run:Run a JavaScript file or load a Rust module'
                'repl:Start interactive REPL'
                'plugin:Manage CLI plugins'
            )
            _describe 'command' commands
            ;;
        args)
            case $line[1] in
                run)
                    _files
                    ;;
                plugin)
                    local plugin_commands
                    plugin_commands=(
                        'list:List all loaded plugins'
                        'load:Load a plugin from file or directory'
                        'unload:Unload a plugin by name'
                        'reload:Reload a plugin by name'
                        'enable:Enable a disabled plugin'
                        'disable:Disable a plugin'
                        'info:Show detailed information about a plugin'
                        'stats:Show plugin statistics'
                    )
                    _describe 'plugin command' plugin_commands
                    ;;
            esac
            ;;
    esac
}}

compdef _{} {}
"#,
            app_name, app_name, app_name, app_name
        )
    }

    fn generate_fish_completion_script(&self, app_name: &str) -> String {
        format!(
            r#"# Fish completion for {}
complete -c {} -n "__fish_use_subcommand" -a "run" -d "Run a JavaScript file or load a Rust module"
complete -c {} -n "__fish_use_subcommand" -a "repl" -d "Start interactive REPL"
complete -c {} -n "__fish_use_subcommand" -a "plugin" -d "Manage CLI plugins"

complete -c {} -s h -l help -d "Show help information"
complete -c {} -s V -l version -d "Show version information"
complete -c {} -s v -l verbose -d "Enable verbose output"
complete -c {} -l debug -d "Enable debug output"
complete -c {} -s c -l config -d "Configuration file" -r

# Plugin subcommands
complete -c {} -n "__fish_seen_subcommand_from plugin" -a "list" -d "List all loaded plugins"
complete -c {} -n "__fish_seen_subcommand_from plugin" -a "load" -d "Load a plugin from file or directory"
complete -c {} -n "__fish_seen_subcommand_from plugin" -a "unload" -d "Unload a plugin by name"
complete -c {} -n "__fish_seen_subcommand_from plugin" -a "reload" -d "Reload a plugin by name"
complete -c {} -n "__fish_seen_subcommand_from plugin" -a "enable" -d "Enable a disabled plugin"
complete -c {} -n "__fish_seen_subcommand_from plugin" -a "disable" -d "Disable a plugin"
complete -c {} -n "__fish_seen_subcommand_from plugin" -a "info" -d "Show detailed information about a plugin"
complete -c {} -n "__fish_seen_subcommand_from plugin" -a "stats" -d "Show plugin statistics"
"#,
            app_name,
            app_name,
            app_name,
            app_name,
            app_name,
            app_name,
            app_name,
            app_name,
            app_name,
            app_name,
            app_name,
            app_name,
            app_name,
            app_name,
            app_name,
            app_name,
            app_name
        )
    }

    fn generate_powershell_completion_script(&self, app_name: &str) -> String {
        format!(
            r#"# PowerShell completion for {}
Register-ArgumentCompleter -Native -CommandName {} -ScriptBlock {{
    param($commandName, $wordToComplete, $cursorPosition)
    
    $completions = @()
    
    # Get completions from the application
    try {{
        $result = & {} --complete powershell $wordToComplete 2>$null
        if ($LASTEXITCODE -eq 0) {{
            $completions = $result -split "`n" | Where-Object {{ $_ -ne "" }}
        }}
    }} catch {{
        # Fallback to basic completions
    }}
    
    # Basic command completions
    if ($completions.Count -eq 0) {{
        $commands = @("run", "repl", "plugin")
        $completions = $commands | Where-Object {{ $_ -like "$wordToComplete*" }}
    }}
    
    $completions | ForEach-Object {{
        [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }}
}}
"#,
            app_name, app_name, app_name
        )
    }

    pub fn install_completion(&self, shell: &str, app_name: &str) -> CliResult<String> {
        let script = self.generate_shell_completion_script(shell, app_name);

        let installation_instructions = match shell {
            "bash" => format!(
                r#"# Add to ~/.bashrc or ~/.bash_profile
source <({} --generate-completion bash)

# Or save to file and source it
{} --generate-completion bash > ~/.bash_completion.d/{}
echo "source ~/.bash_completion.d/{}" >> ~/.bashrc
"#,
                app_name, app_name, app_name, app_name
            ),
            "zsh" => format!(
                r#"# Add to ~/.zshrc
source <({} --generate-completion zsh)

# Or save to file and source it
{} --generate-completion zsh > ~/.zsh/completions/_{}
echo "fpath=(~/.zsh/completions $fpath)" >> ~/.zshrc
echo "autoload -U compinit && compinit" >> ~/.zshrc
"#,
                app_name, app_name, app_name
            ),
            "fish" => format!(
                r#"# Save to Fish completions directory
{} --generate-completion fish > ~/.config/fish/completions/{}.fish
"#,
                app_name, app_name
            ),
            "powershell" => format!(
                r#"# Add to PowerShell profile
{} --generate-completion powershell | Out-File -FilePath $PROFILE -Append

# Or run once to add to current session
{}
"#,
                app_name, script
            ),
            _ => {
                return Err(CliError::InvalidArgument {
                    argument: "shell".to_string(),
                    reason: format!("Unsupported shell: {}", shell),
                });
            }
        };

        Ok(format!(
            "{}\n\n# Completion script:\n{}",
            installation_instructions, script
        ))
    }
}

impl Default for AdvancedCompletionSystem {
    fn default() -> Self {
        Self::new()
    }
}
