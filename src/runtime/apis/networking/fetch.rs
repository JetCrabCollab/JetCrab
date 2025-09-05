//! # Fetch API
//!
//! Implementation of the fetch() API for HTTP requests.

use boa_engine::{Context, JsResult};
use tracing::debug;

/// Fetch API implementation
pub struct FetchAPI;

impl FetchAPI {
    pub fn new() -> Self {
        Self
    }

    /// Register fetch function in the global context
    pub fn register(&self, context: &mut Context) -> JsResult<()> {
        let fetch_code = r#"
            function fetch(url, options = {}) {
                return Promise.resolve({
                    ok: true,
                    status: 200,
                    statusText: "OK",
                    url: url,
                    headers: {},
                    text: function() {
                        return Promise.resolve("Mock response for: " + url);
                    },
                    json: function() {
                        return Promise.resolve({ message: "Mock JSON response", url: url });
                    }
                });
            }
        "#;

        context.eval(boa_engine::Source::from_bytes(fetch_code))?;

        debug!("Fetch API registered successfully");
        Ok(())
    }
}

impl Default for FetchAPI {
    fn default() -> Self {
        Self::new()
    }
}
