//! # OS API
//!
//! Provides operating system-related utilities and information.

use chitin::boa_engine::{js_string, property::Attribute, Context, JsResult};
use sysinfo::System;

/// OS API implementation
pub struct OsAPI {
    system: System,
}

impl OsAPI {
    /// Create a new OS API instance
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }

    /// Register the OS API with the JavaScript context
    pub fn register(&self, context: &mut Context) -> JsResult<()> {
        let os_code = format!(
            r#"
            ({{
                platform: "{}",
                arch: "{}",
                totalmem: {},
                freemem: {},
                cpus: {},
                EOL: "{}",
                endianness: "LE",
                constants: {{
                    F_OK: 0,
                    R_OK: 4,
                    W_OK: 2,
                    X_OK: 1
                }}
            }})
            "#,
            std::env::consts::OS,
            std::env::consts::ARCH,
            self.system.total_memory(),
            self.system.free_memory(),
            self.system.cpus().len(),
            if cfg!(windows) { "\\r\\n" } else { "\\n" }
        );

        let os_object = context.eval(chitin::boa_engine::Source::from_bytes(&os_code))?;

        context.register_global_property(js_string!("os"), os_object, Attribute::all())?;

        Ok(())
    }
}

impl Default for OsAPI {
    fn default() -> Self {
        Self::new()
    }
}
