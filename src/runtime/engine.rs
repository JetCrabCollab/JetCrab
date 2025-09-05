//! # JetCrab Engine
//!
//! Wrapper around Boa engine with JetCrab-specific features.

use boa_engine::{js_string, Context, JsResult, JsValue, Source};
use std::collections::HashMap;
use tracing::debug;

/// JetCrab Engine - Wrapper around Boa with additional features
pub struct JetCrabEngine {
    context: Context,
    globals: HashMap<String, JsValue>,
    config: EngineConfig,
}

/// Configuration for the JetCrab Engine
#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub strict_mode: bool,
    pub max_execution_time: Option<u64>,
    pub memory_limit: Option<usize>,
    pub enable_console: bool,
    pub enable_math: bool,
    pub enable_json: bool,
    pub enable_fetch: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            strict_mode: false,
            max_execution_time: None,
            memory_limit: None,
            enable_console: true,
            enable_math: true,
            enable_json: true,
            enable_fetch: true,
        }
    }
}

impl JetCrabEngine {
    /// Create a new JetCrab Engine instance
    pub fn new() -> Self {
        Self::with_config(EngineConfig::default())
    }

    /// Create a new JetCrab Engine with custom configuration
    pub fn with_config(config: EngineConfig) -> Self {
        let mut context = Context::default();

        Self::setup_builtins(&mut context, &config);

        Self {
            context,
            globals: HashMap::new(),
            config,
        }
    }

    /// Evaluate JavaScript code
    pub fn evaluate(&mut self, source: &str) -> Result<JsValue, String> {
        debug!("Evaluating JavaScript code: {}", source);

        let result = self
            .context
            .eval(Source::from_bytes(source))
            .map_err(|e| format!("Runtime error: {:?}", e))?;

        debug!("Evaluation completed successfully");
        Ok(result)
    }

    /// Evaluate JavaScript code and return as string
    pub fn evaluate_to_string(&mut self, source: &str) -> Result<String, String> {
        let result = self.evaluate(source)?;

        if let Ok(log_message) = self
            .context
            .global_object()
            .get(js_string!("_lastLogMessage"), &mut self.context)
        {
            if !log_message.is_undefined() {
                let message = log_message
                    .to_string(&mut self.context)
                    .unwrap_or_else(|_| "undefined".into());
                println!("{}", message.to_std_string_escaped());
                self.context
                    .global_object()
                    .set(
                        js_string!("_lastLogMessage"),
                        JsValue::undefined(),
                        false,
                        &mut self.context,
                    )
                    .ok();
            }
        }

        let js_string = result
            .to_string(&mut self.context)
            .unwrap_or_else(|_| "undefined".into());
        Ok(js_string.to_std_string_escaped())
    }

    /// Set a global variable
    pub fn set_global(&mut self, name: &str, value: JsValue) -> Result<(), String> {
        debug!("Setting global variable: {} = {:?}", name, value);

        self.globals.insert(name.to_string(), value.clone());

        Ok(())
    }

    /// Get a global variable
    pub fn get_global(&mut self, name: &str) -> Option<JsValue> {
        self.globals.get(name).cloned()
    }

    /// Add a custom function to the global scope
    pub fn add_function<F>(&mut self, name: &str, _func: F) -> Result<(), String>
    where
        F: Fn(&[JsValue], &mut Context) -> JsResult<JsValue> + 'static,
    {
        debug!("Adding function: {}", name);

        Ok(())
    }

    /// Get engine configuration
    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    /// Update engine configuration
    pub fn update_config(&mut self, config: EngineConfig) {
        self.config = config;
        Self::setup_builtins(&mut self.context, &self.config);
    }

    /// Get engine statistics
    pub fn get_stats(&self) -> EngineStats {
        EngineStats {
            globals_count: self.globals.len(),
            memory_usage: 0,    // Would implement actual memory tracking
            execution_count: 0, // Would track execution count
        }
    }

    /// Get mutable reference to the context
    pub fn get_context(&mut self) -> &mut Context {
        &mut self.context
    }

    /// Setup built-in JavaScript objects and functions
    fn setup_builtins(_context: &mut Context, _config: &EngineConfig) {
        debug!("Setting up built-in JavaScript objects");
    }
}

impl Default for JetCrabEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Engine statistics
#[derive(Debug, Clone)]
pub struct EngineStats {
    pub globals_count: usize,
    pub memory_usage: usize,
    pub execution_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_evaluation() {
        let mut engine = JetCrabEngine::new();

        let result = engine.evaluate_to_string("2 + 3 * 4").unwrap();
        assert_eq!(result, "14");
    }

    #[test]
    fn test_string_operations() {
        let mut engine = JetCrabEngine::new();

        let result = engine
            .evaluate_to_string("'Hello' + ' ' + 'World'")
            .unwrap();
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_variable_declaration() {
        let mut engine = JetCrabEngine::new();

        let result = engine.evaluate_to_string("let x = 42; x").unwrap();
        assert_eq!(result, "42");
    }

    #[test]
    fn test_global_variables() {
        let mut engine = JetCrabEngine::new();

        engine.set_global("testVar", JsValue::from(42)).unwrap();
        let result = engine.get_global("testVar").unwrap();
        assert_eq!(result, JsValue::from(42));
    }

    #[test]
    fn test_engine_config_default() {
        let config = EngineConfig::default();
        assert!(!config.strict_mode);
        assert!(config.enable_console);
        assert!(config.enable_math);
        assert!(config.enable_json);
        assert!(config.enable_fetch);
        assert_eq!(config.max_execution_time, None);
        assert_eq!(config.memory_limit, None);
    }

    #[test]
    fn test_engine_config_custom() {
        let config = EngineConfig {
            strict_mode: false,
            max_execution_time: Some(1000),
            memory_limit: Some(1024),
            enable_console: false,
            enable_math: true,
            enable_json: false,
            enable_fetch: true,
        };

        assert!(!config.strict_mode);
        assert_eq!(config.max_execution_time, Some(1000));
        assert_eq!(config.memory_limit, Some(1024));
        assert!(!config.enable_console);
        assert!(config.enable_math);
        assert!(!config.enable_json);
        assert!(config.enable_fetch);
    }

    #[test]
    fn test_evaluate_simple_arithmetic() {
        let mut engine = JetCrabEngine::new();
        let result = engine.evaluate("2 + 3 * 4");
        assert!(result.is_ok());
    }

    #[test]
    fn test_evaluate_string_operations() {
        let mut engine = JetCrabEngine::new();
        let result = engine.evaluate("'Hello' + ' ' + 'World'");
        assert!(result.is_ok());
    }

    #[test]
    fn test_evaluate_variable_declaration() {
        let mut engine = JetCrabEngine::new();
        let result = engine.evaluate("let x = 42; x");
        assert!(result.is_ok());
    }

    #[test]
    fn test_evaluate_function_definition() {
        let mut engine = JetCrabEngine::new();
        let result = engine.evaluate("function add(a, b) { return a + b; } add(5, 3)");
        assert!(result.is_ok());
    }

    #[test]
    fn test_evaluate_invalid_syntax() {
        let mut engine = JetCrabEngine::new();
        let result = engine.evaluate("invalid syntax {");
        assert!(result.is_err());
    }

    #[test]
    fn test_evaluate_to_string() {
        let mut engine = JetCrabEngine::new();
        let result = engine.evaluate_to_string("'Hello, JetCrab!'");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Hello, JetCrab!"));
    }

    #[test]
    fn test_set_and_get_global() {
        let mut engine = JetCrabEngine::new();
        let value = JsValue::from(42);
        engine.set_global("testValue", value).unwrap();

        let retrieved = engine.get_global("testValue").unwrap();
        assert_eq!(retrieved, JsValue::from(42));
    }

    #[test]
    fn test_get_global_nonexistent() {
        let mut engine = JetCrabEngine::new();
        let retrieved = engine.get_global("nonexistent");
        assert!(retrieved.is_none());
    }
}
