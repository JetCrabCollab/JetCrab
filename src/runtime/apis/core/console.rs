//! # Simple Console API
//!
//! A simplified console API implementation that works with Boa.

use boa_engine::{js_string, Context, JsResult};

/// Simple Console API implementation
pub struct SimpleConsoleAPI;

impl SimpleConsoleAPI {
    pub fn new() -> Self {
        Self
    }

    /// Register console methods in the global context
    pub fn register(&self, context: &mut Context) -> JsResult<()> {
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
}

impl Default for SimpleConsoleAPI {
    fn default() -> Self {
        Self::new()
    }
}
