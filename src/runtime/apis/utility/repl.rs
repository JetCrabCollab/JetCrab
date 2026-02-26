use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::CmdKind;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline::Result as RustylineResult;
use rustyline::{CompletionType, Config, Context, EditMode, Editor, Helper};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{Mutex, RwLock};
use tracing::{error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplSession {
    pub id: u32,
    pub start_time: u64,
    pub command_count: u64,
    pub last_command: String,
    pub is_active: bool,
    pub history: Vec<String>,
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplConfig {
    pub prompt: String,
    pub multiline_prompt: String,
    pub history_file: Option<String>,
    pub max_history_size: usize,
    pub auto_completion: bool,
    pub syntax_highlighting: bool,
    pub bracket_matching: bool,
    pub edit_mode: String,
    pub tab_completion: bool,
    pub case_sensitive: bool,
}

impl Default for ReplConfig {
    fn default() -> Self {
        Self {
            prompt: "jetcrab> ".to_string(),
            multiline_prompt: "  ... ".to_string(),
            history_file: Some(".jetcrab_history".to_string()),
            max_history_size: 1000,
            auto_completion: true,
            syntax_highlighting: true,
            bracket_matching: true,
            edit_mode: "emacs".to_string(),
            tab_completion: true,
            case_sensitive: false,
        }
    }
}

pub struct ReplHelper {
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: HistoryHinter,
}

impl Completer for ReplHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> RustylineResult<(usize, Vec<Pair>)> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for ReplHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for ReplHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> std::borrow::Cow<'b, str> {
        if default {
            std::borrow::Cow::Borrowed(prompt)
        } else {
            std::borrow::Cow::Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        std::borrow::Cow::Borrowed(hint)
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> std::borrow::Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize, forced: CmdKind) -> bool {
        self.highlighter.highlight_char(line, pos, forced)
    }
}

impl Validator for ReplHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> RustylineResult<validate::ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}

impl Helper for ReplHelper {}

pub struct ReplManager {
    config: ReplConfig,
    sessions: Arc<RwLock<HashMap<u32, ReplSession>>>,
    session_counter: Arc<Mutex<u32>>,
    editor: Arc<Mutex<Option<Editor<ReplHelper, rustyline::history::FileHistory>>>>,
}

impl ReplManager {
    pub fn new(config: ReplConfig) -> Self {
        Self {
            config,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            session_counter: Arc::new(Mutex::new(0)),
            editor: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn create_session(&self) -> Result<u32, Box<dyn std::error::Error>> {
        let session_id = {
            let mut counter = self.session_counter.lock().await;
            let id = *counter;
            *counter += 1;
            id
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let session = ReplSession {
            id: session_id,
            start_time: now,
            command_count: 0,
            last_command: String::new(),
            is_active: true,
            history: Vec::new(),
            variables: HashMap::new(),
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id, session);
        drop(sessions);

        info!("🔄 REPL Session {} created", session_id);
        Ok(session_id)
    }

    pub async fn start_repl(&self, session_id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.is_active = true;
            info!("🚀 REPL Session {} started", session_id);
        } else {
            return Err("Session not found".into());
        }
        drop(sessions);

        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Emacs)
            .build();

        let helper = ReplHelper {
            completer: FilenameCompleter::new(),
            highlighter: MatchingBracketHighlighter::new(),
            validator: MatchingBracketValidator::new(),
            hinter: HistoryHinter {},
        };

        let mut editor = Editor::with_config(config)?;
        editor.set_helper(Some(helper));

        if let Some(history_file) = &self.config.history_file {
            if let Err(e) = editor.load_history(history_file) {
                warn!("Failed to load history from {}: {}", history_file, e);
            }
        }

        let mut editor_guard = self.editor.lock().await;
        *editor_guard = Some(editor);
        drop(editor_guard);

        Ok(())
    }

    pub async fn stop_repl(&self, session_id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.is_active = false;
            info!("🛑 REPL Session {} stopped", session_id);
        } else {
            return Err("Session not found".into());
        }
        drop(sessions);

        if let Some(history_file) = &self.config.history_file {
            let mut editor_guard = self.editor.lock().await;
            if let Some(editor) = editor_guard.as_mut() {
                if let Err(e) = editor.save_history(history_file) {
                    warn!("Failed to save history to {}: {}", history_file, e);
                }
            }
        }

        Ok(())
    }

    pub async fn execute_command(
        &self,
        session_id: u32,
        command: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.command_count += 1;
            session.last_command = command.to_string();
            session.history.push(command.to_string());

            if session.history.len() > self.config.max_history_size {
                session.history.remove(0);
            }

            info!(
                "⚡ REPL Session {} executing command: {}",
                session_id, command
            );

            let result = format!("Executed: {}", command);
            Ok(result)
        } else {
            Err("Session not found".into())
        }
    }

    pub async fn read_line(
        &self,
        _session_id: u32,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let mut editor_guard = self.editor.lock().await;
        if let Some(editor) = editor_guard.as_mut() {
            let readline = editor.readline(&self.config.prompt);
            match readline {
                Ok(line) => {
                    editor.add_history_entry(line.as_str())?;
                    Ok(Some(line))
                }
                Err(ReadlineError::Interrupted) => {
                    info!("REPL interrupted by user");
                    Ok(None)
                }
                Err(ReadlineError::Eof) => {
                    info!("REPL reached end of file");
                    Ok(None)
                }
                Err(err) => {
                    error!("REPL readline error: {}", err);
                    Err(err.into())
                }
            }
        } else {
            Err("Editor not initialized".into())
        }
    }

    pub async fn get_session_count(&self) -> u32 {
        let sessions = self.sessions.read().await;
        sessions.len() as u32
    }

    pub async fn get_session_info(&self, session_id: u32) -> Option<ReplSession> {
        let sessions = self.sessions.read().await;
        sessions.get(&session_id).cloned()
    }

    pub async fn get_all_sessions(&self) -> Vec<ReplSession> {
        let sessions = self.sessions.read().await;
        sessions.values().cloned().collect()
    }

    pub async fn clear_history(&self, session_id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.history.clear();
            info!("🗑️ REPL Session {} history cleared", session_id);
        } else {
            return Err("Session not found".into());
        }
        drop(sessions);

        let mut editor_guard = self.editor.lock().await;
        if let Some(editor) = editor_guard.as_mut() {
            editor.clear_history();
        }

        Ok(())
    }

    pub async fn set_variable(
        &self,
        session_id: u32,
        name: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session
                .variables
                .insert(name.to_string(), value.to_string());
            info!(
                "📝 REPL Session {} variable set: {} = {}",
                session_id, name, value
            );
        } else {
            return Err("Session not found".into());
        }
        drop(sessions);

        Ok(())
    }

    pub async fn get_variable(&self, session_id: u32, name: &str) -> Option<String> {
        let sessions = self.sessions.read().await;
        if let Some(session) = sessions.get(&session_id) {
            session.variables.get(name).cloned()
        } else {
            None
        }
    }

    pub async fn list_variables(&self, session_id: u32) -> HashMap<String, String> {
        let sessions = self.sessions.read().await;
        if let Some(session) = sessions.get(&session_id) {
            session.variables.clone()
        } else {
            HashMap::new()
        }
    }
}

pub struct ReplAPI {
    repl_manager: Arc<Mutex<ReplManager>>,
}

impl ReplAPI {
    pub fn new() -> Self {
        Self {
            repl_manager: Arc::new(Mutex::new(ReplManager::new(ReplConfig::default()))),
        }
    }

    pub fn register(
        &self,
        context: &mut chitin::boa_engine::Context,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔄 Registering REPL API...");

        let repl_code = r#"
        globalThis.repl = {
            start: function(options) {
                const sessionId = Math.floor(Math.random() * 1000);
                console.log('Starting REPL session:', sessionId, 'with options:', options);
                
                return {
                    id: sessionId,
                    options: options || {},
                    isActive: true,
                    
                    readLine: function(prompt) {
                        console.log('Reading line with prompt:', prompt || 'jetcrab> ');
                        return Promise.resolve('console.log("Hello from REPL!");');
                    },
                    
                    execute: function(command) {
                        console.log('Executing command:', command);
                        return Promise.resolve('Command executed successfully');
                    },
                    
                    stop: function() {
                        console.log('Stopping REPL session');
                        this.isActive = false;
                        return Promise.resolve();
                    },
                    
                    clearHistory: function() {
                        console.log('Clearing REPL history');
                        return Promise.resolve();
                    },
                    
                    setVariable: function(name, value) {
                        console.log('Setting variable:', name, '=', value);
                        return Promise.resolve();
                    },
                    
                    getVariable: function(name) {
                        console.log('Getting variable:', name);
                        return Promise.resolve('undefined');
                    },
                    
                    listVariables: function() {
                        console.log('Listing all variables');
                        return Promise.resolve({});
                    }
                };
            },
            
            createSession: function(config) {
                const sessionId = Math.floor(Math.random() * 1000);
                console.log('Creating REPL session:', sessionId, 'with config:', config);
                
                return {
                    id: sessionId,
                    config: config || {},
                    startTime: Date.now(),
                    commandCount: 0,
                    isActive: false,
                    
                    start: function() {
                        console.log('Starting session:', this.id);
                        this.isActive = true;
                        return Promise.resolve();
                    },
                    
                    stop: function() {
                        console.log('Stopping session:', this.id);
                        this.isActive = false;
                        return Promise.resolve();
                    },
                    
                    execute: function(command) {
                        console.log('Session', this.id, 'executing:', command);
                        this.commandCount++;
                        return Promise.resolve('Command executed');
                    }
                };
            },
            
            getSessions: function() {
                console.log('Getting all REPL sessions');
                return Promise.resolve([]);
            },
            
            getSession: function(id) {
                console.log('Getting REPL session:', id);
                return Promise.resolve(null);
            },
            
            clearAllHistory: function() {
                console.log('Clearing all REPL history');
                return Promise.resolve();
            },
            
            setGlobalVariable: function(name, value) {
                console.log('Setting global variable:', name, '=', value);
                return Promise.resolve();
            },
            
            getGlobalVariable: function(name) {
                console.log('Getting global variable:', name);
                return Promise.resolve('undefined');
            },
            
            listGlobalVariables: function() {
                console.log('Listing all global variables');
                return Promise.resolve({});
            },
            
            config: {
                prompt: 'jetcrab> ',
                multilinePrompt: '  ... ',
                historyFile: '.jetcrab_history',
                maxHistorySize: 1000,
                autoCompletion: true,
                syntaxHighlighting: true,
                bracketMatching: true,
                editMode: 'emacs',
                tabCompletion: true,
                caseSensitive: false
            },
            
            utils: {
                formatResult: function(result) {
                    if (typeof result === 'undefined') {
                        return 'undefined';
                    } else if (result === null) {
                        return 'null';
                    } else if (typeof result === 'string') {
                        return `"${result}"`;
                    } else if (typeof result === 'object') {
                        return JSON.stringify(result, null, 2);
                    } else {
                        return String(result);
                    }
                },
                
                isCompleteExpression: function(code) {
                    const openBraces = (code.match(/\{/g) || []).length;
                    const closeBraces = (code.match(/\}/g) || []).length;
                    const openParens = (code.match(/\(/g) || []).length;
                    const closeParens = (code.match(/\)/g) || []).length;
                    const openBrackets = (code.match(/\[/g) || []).length;
                    const closeBrackets = (code.match(/\]/g) || []).length;
                    
                    return openBraces === closeBraces && 
                           openParens === closeParens && 
                           openBrackets === closeBrackets;
                },
                
                getCompletion: function(line, pos) {
                    console.log('Getting completion for:', line, 'at position:', pos);
                    return Promise.resolve([]);
                },
                
                getHistory: function() {
                    console.log('Getting REPL history');
                    return Promise.resolve([]);
                },
                
                searchHistory: function(query) {
                    console.log('Searching history for:', query);
                    return Promise.resolve([]);
                }
            }
        };
        "#;

        context.eval(chitin::boa_engine::Source::from_bytes(repl_code))?;
        info!("✅ REPL API registered successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_repl_config_default() {
        let config = ReplConfig::default();
        assert_eq!(config.prompt, "jetcrab> ");
        assert_eq!(config.multiline_prompt, "  ... ");
        assert_eq!(config.history_file, Some(".jetcrab_history".to_string()));
        assert_eq!(config.max_history_size, 1000);
        assert!(config.auto_completion);
        assert!(config.syntax_highlighting);
        assert!(config.bracket_matching);
        assert_eq!(config.edit_mode, "emacs");
        assert!(config.tab_completion);
        assert!(!config.case_sensitive);
    }

    #[test]
    async fn test_repl_manager_creation() {
        let config = ReplConfig::default();
        let manager = ReplManager::new(config);
        assert_eq!(manager.get_session_count().await, 0);
    }

    #[test]
    async fn test_repl_manager_create_session() {
        let config = ReplConfig::default();
        let manager = ReplManager::new(config);

        let result = manager.create_session().await;
        assert!(result.is_ok());
        let session_id = result.unwrap();
        assert_eq!(session_id, 0);
    }

    #[test]
    async fn test_repl_manager_start_stop_session() {
        let config = ReplConfig::default();
        let manager = ReplManager::new(config);

        let session_id = manager.create_session().await.unwrap();

        let result = manager.start_repl(session_id).await;
        assert!(result.is_ok());

        let session_info = manager.get_session_info(session_id).await;
        assert!(session_info.is_some());
        assert!(session_info.unwrap().is_active);

        let result = manager.stop_repl(session_id).await;
        assert!(result.is_ok());
    }

    #[test]
    async fn test_repl_manager_execute_command() {
        let config = ReplConfig::default();
        let manager = ReplManager::new(config);

        let session_id = manager.create_session().await.unwrap();
        let command = "console.log('Hello, World!');";

        let result = manager.execute_command(session_id, command).await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Executed:"));

        let session_info = manager.get_session_info(session_id).await;
        assert!(session_info.is_some());
        assert_eq!(session_info.unwrap().command_count, 1);
    }

    #[test]
    async fn test_repl_manager_clear_history() {
        let config = ReplConfig::default();
        let manager = ReplManager::new(config);

        let session_id = manager.create_session().await.unwrap();
        manager
            .execute_command(session_id, "test command")
            .await
            .unwrap();

        let result = manager.clear_history(session_id).await;
        assert!(result.is_ok());

        let session_info = manager.get_session_info(session_id).await;
        assert!(session_info.is_some());
        assert!(session_info.unwrap().history.is_empty());
    }

    #[test]
    async fn test_repl_manager_variables() {
        let config = ReplConfig::default();
        let manager = ReplManager::new(config);

        let session_id = manager.create_session().await.unwrap();

        let result = manager
            .set_variable(session_id, "testVar", "testValue")
            .await;
        assert!(result.is_ok());

        let value = manager.get_variable(session_id, "testVar").await;
        assert_eq!(value, Some("testValue".to_string()));

        let variables = manager.list_variables(session_id).await;
        assert_eq!(variables.len(), 1);
        assert_eq!(variables.get("testVar"), Some(&"testValue".to_string()));
    }

    #[test]
    async fn test_repl_manager_get_counts() {
        let config = ReplConfig::default();
        let manager = ReplManager::new(config);

        manager.create_session().await.unwrap();
        manager.create_session().await.unwrap();

        assert_eq!(manager.get_session_count().await, 2);
    }

    #[test]
    async fn test_repl_manager_get_all_sessions() {
        let config = ReplConfig::default();
        let manager = ReplManager::new(config);

        manager.create_session().await.unwrap();
        manager.create_session().await.unwrap();

        let sessions = manager.get_all_sessions().await;
        assert_eq!(sessions.len(), 2);
    }

    #[test]
    async fn test_repl_api_creation() {
        let api = ReplAPI::new();
        assert!(api.repl_manager.lock().await.get_session_count().await == 0);
    }

    #[test]
    async fn test_repl_api_register() {
        let api = ReplAPI::new();
        let mut context = chitin::boa_engine::Context::default();
        let result = api.register(&mut context);
        assert!(result.is_ok());
    }

    #[test]
    async fn test_repl_session_creation() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let session = ReplSession {
            id: 1,
            start_time: now,
            command_count: 0,
            last_command: String::new(),
            is_active: true,
            history: Vec::new(),
            variables: HashMap::new(),
        };

        assert_eq!(session.id, 1);
        assert_eq!(session.command_count, 0);
        assert!(session.is_active);
        assert!(session.history.is_empty());
        assert!(session.variables.is_empty());
    }

    #[test]
    async fn test_repl_config_custom() {
        let config = ReplConfig {
            prompt: "custom> ".to_string(),
            multiline_prompt: "    ".to_string(),
            history_file: Some("custom_history".to_string()),
            max_history_size: 500,
            auto_completion: false,
            syntax_highlighting: false,
            bracket_matching: false,
            edit_mode: "vi".to_string(),
            tab_completion: false,
            case_sensitive: true,
        };

        assert_eq!(config.prompt, "custom> ");
        assert_eq!(config.multiline_prompt, "    ");
        assert_eq!(config.history_file, Some("custom_history".to_string()));
        assert_eq!(config.max_history_size, 500);
        assert!(!config.auto_completion);
        assert!(!config.syntax_highlighting);
        assert!(!config.bracket_matching);
        assert_eq!(config.edit_mode, "vi");
        assert!(!config.tab_completion);
        assert!(config.case_sensitive);
    }
}
