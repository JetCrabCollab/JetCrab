//! # Direct APIs
//!
//! Direct injection of APIs into the Boa context without using NativeFunction.

use boa_engine::{js_string, Context, JsObject, JsResult, JsValue};
use std::env;

/// Direct APIs implementation
pub struct DirectAPIs;

impl DirectAPIs {
    pub fn new() -> Self {
        Self
    }

    /// Register all APIs directly in the global context
    pub fn register(&self, context: &mut Context) -> JsResult<()> {
        self.register_console(context)?;

        self.register_process(context)?;

        self.register_http(context)?;

        self.register_fs(context)?;

        self.register_path(context)?;

        self.register_os(context)?;

        self.register_require(context)?;

        self.register_timers(context)?;

        Ok(())
    }

    /// Register console object
    fn register_console(&self, context: &mut Context) -> JsResult<()> {
        let console_code = r#"
            ({
                log: function(...args) {
                    let message = args.map(arg => String(arg)).join(' ');
                    globalThis._lastLogMessage = message;
                    return message;
                },
                error: function(...args) {
                    let message = args.map(arg => String(arg)).join(' ');
                    globalThis._lastLogMessage = "ERROR: " + message;
                    return "ERROR: " + message;
                },
                warn: function(...args) {
                    let message = args.map(arg => String(arg)).join(' ');
                    globalThis._lastLogMessage = "WARN: " + message;
                    return "WARN: " + message;
                },
                info: function(...args) {
                    let message = args.map(arg => String(arg)).join(' ');
                    globalThis._lastLogMessage = "INFO: " + message;
                    return "INFO: " + message;
                }
            })
        "#;

        let console = context.eval(boa_engine::Source::from_bytes(console_code))?;

        context
            .global_object()
            .set(js_string!("console"), console, false, context)?;
        Ok(())
    }

    /// Register process object
    fn register_process(&self, context: &mut Context) -> JsResult<()> {
        let process = JsObject::default();

        let argv = self.get_argv(context)?;
        process.set(js_string!("argv"), argv, false, context)?;

        let env = self.get_env(context)?;
        process.set(js_string!("env"), env, false, context)?;

        process.set(js_string!("version"), js_string!("v18.0.0"), false, context)?;

        let cwd_value: JsValue = match env::current_dir() {
            Ok(path) => js_string!(path.to_string_lossy().to_string()).into(),
            Err(_) => js_string!(".").into(),
        };
        process.set(js_string!("cwd"), cwd_value, false, context)?;

        context
            .global_object()
            .set(js_string!("process"), process, false, context)?;

        Ok(())
    }

    /// Get command line arguments
    fn get_argv(&self, context: &mut Context) -> JsResult<JsValue> {
        let args: Vec<String> = env::args().collect();
        let mut js_args = Vec::new();

        for arg in args {
            js_args.push(JsValue::from(js_string!(arg)));
        }

        Ok(JsValue::from(
            boa_engine::object::builtins::JsArray::from_iter(js_args, context),
        ))
    }

    /// Get environment variables
    fn get_env(&self, context: &mut Context) -> JsResult<JsValue> {
        let env_obj = JsObject::default();

        for (key, value) in env::vars() {
            env_obj.set(js_string!(key), js_string!(value), false, context)?;
        }

        Ok(env_obj.into())
    }

    /// Register HTTP module
    fn register_http(&self, context: &mut Context) -> JsResult<()> {
        let http_api = crate::runtime::apis::networking::HttpAPI::new();
        http_api.register(context).map_err(|e| {
            boa_engine::JsError::from(
                boa_engine::JsNativeError::typ()
                    .with_message(format!("Failed to register HTTP API: {}", e)),
            )
        })?;
        Ok(())
    }

    /// Register FS module
    fn register_fs(&self, context: &mut Context) -> JsResult<()> {
        let fs_api = crate::runtime::apis::filesystem::NativeFsModule::new();
        fs_api.register(context).map_err(|e| {
            boa_engine::JsError::from(
                boa_engine::JsNativeError::typ()
                    .with_message(format!("Failed to register FS API: {}", e)),
            )
        })?;
        Ok(())
    }

    /// Register Path module
    fn register_path(&self, context: &mut Context) -> JsResult<()> {
        let path_api = crate::runtime::apis::filesystem::PathAPI::new();
        path_api.register(context).map_err(|e| {
            boa_engine::JsError::from(
                boa_engine::JsNativeError::typ()
                    .with_message(format!("Failed to register path API: {}", e)),
            )
        })?;
        Ok(())
    }

    /// Register OS module
    fn register_os(&self, context: &mut Context) -> JsResult<()> {
        let os_api = crate::runtime::apis::system::OsAPI::new();
        os_api.register(context).map_err(|e| {
            boa_engine::JsError::from(
                boa_engine::JsNativeError::typ()
                    .with_message(format!("Failed to register OS API: {}", e)),
            )
        })?;
        Ok(())
    }

    /// Register require system
    fn register_require(&self, context: &mut Context) -> JsResult<()> {
        let require_api = crate::runtime::apis::core::RequireAPI::new();
        require_api.register(context).map_err(|e| {
            boa_engine::JsError::from(
                boa_engine::JsNativeError::typ()
                    .with_message(format!("Failed to register require API: {}", e)),
            )
        })?;
        Ok(())
    }

    /// Register timers module
    fn register_timers(&self, context: &mut Context) -> JsResult<()> {
        let timers_api = crate::runtime::apis::utility::TimersAPI::new();
        timers_api.register(context).map_err(|e| {
            boa_engine::JsError::from(
                boa_engine::JsNativeError::typ()
                    .with_message(format!("Failed to register timers API: {}", e)),
            )
        })?;
        Ok(())
    }
}

impl Default for DirectAPIs {
    fn default() -> Self {
        Self::new()
    }
}
