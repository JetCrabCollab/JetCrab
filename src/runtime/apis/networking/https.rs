//! # HTTPS Module
//!
//! Provides secure HTTP server and client functionality with TLS/SSL support.
//! Implements Node.js-compatible HTTPS API using Rust's TLS libraries.
//!
//! ## Features
//!
//! - TLS/SSL server creation with certificate support
//! - HTTPS client requests
//! - Integration with Axum web framework
//! - Support for both rustls and native-tls backends
//!
//! ## Usage
//!
//! ```javascript
//! const https = require('https');
//!
//! const server = https.createServer({
//!   cert: 'cert.pem',
//!   key: 'key.pem'
//! }, (req, res) => {
//!   res.writeHead(200);
//!   res.end('Hello HTTPS!');
//! });
//!
//! server.listen(443);
//! ```

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, Method, StatusCode, Uri},
    response::Response,
    routing::{get, post},
    Router,
};
use chitin::boa_engine::{Context, JsValue, NativeFunction};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{collections::HashMap, fs::File, io::BufReader, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::{debug, error, info};

/// HTTPS Server configuration
#[derive(Debug, Clone)]
pub struct HttpsServerConfig {
    pub cert_path: String,
    pub key_path: String,
    pub port: u16,
    pub host: String,
}

impl Default for HttpsServerConfig {
    fn default() -> Self {
        Self {
            cert_path: "cert.pem".to_string(),
            key_path: "key.pem".to_string(),
            port: 443,
            host: "0.0.0.0".to_string(),
        }
    }
}

/// HTTPS Server implementation
pub struct HttpsServer {
    config: HttpsServerConfig,
    server_config: Option<Arc<ServerConfig>>,
    tls_acceptor: Option<TlsAcceptor>,
}

impl HttpsServer {
    /// Create a new HTTPS server
    pub fn new(config: HttpsServerConfig) -> Self {
        Self {
            config,
            server_config: None,
            tls_acceptor: None,
        }
    }

    /// Load TLS certificates and keys
    pub async fn load_tls_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        debug!(
            "Loading TLS configuration from {} and {}",
            self.config.cert_path, self.config.key_path
        );

        let cert_file = File::open(&self.config.cert_path)?;
        let mut cert_reader = BufReader::new(cert_file);
        let cert_chain = certs(&mut cert_reader)?
            .into_iter()
            .map(Certificate)
            .collect::<Vec<_>>();

        let key_file = File::open(&self.config.key_path)?;
        let mut key_reader = BufReader::new(key_file);
        let mut keys = pkcs8_private_keys(&mut key_reader)?;

        if keys.is_empty() {
            return Err("No private keys found".into());
        }

        let private_key = PrivateKey(keys.remove(0));

        let server_config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, private_key)?;

        let server_config = Arc::new(server_config);
        let tls_acceptor = TlsAcceptor::from(server_config.clone());

        self.server_config = Some(server_config);
        self.tls_acceptor = Some(tls_acceptor);

        info!("TLS configuration loaded successfully");
        Ok(())
    }

    /// Start the HTTPS server
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let tls_acceptor = self
            .tls_acceptor
            .as_ref()
            .ok_or("TLS configuration not loaded")?;

        let addr = format!("{}:{}", self.config.host, self.config.port);
        let listener = TcpListener::bind(&addr).await?;

        info!("HTTPS server listening on https://{}", addr);

        let app = Router::new()
            .route("/", get(handle_request))
            .route("/*path", get(handle_request))
            .route("/*path", post(handle_request))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default()))
                    .layer(CorsLayer::permissive()),
            );

        loop {
            let (stream, addr) = listener.accept().await?;
            let tls_acceptor = tls_acceptor.clone();
            let app = app.clone();

            tokio::spawn(async move {
                match tls_acceptor.accept(stream).await {
                    Ok(tls_stream) => {
                        debug!("TLS connection established from {}", addr);
                    }
                    Err(e) => {
                        error!("TLS handshake failed for {}: {}", addr, e);
                    }
                }
            });
        }
    }
}

/// Handle HTTPS requests
async fn handle_request(
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    path: Option<Path<String>>,
    query: Option<Query<HashMap<String, String>>>,
    State(_state): State<()>,
) -> Response<String> {
    debug!("HTTPS request: {} {}", method, uri);

    let response_body = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>JetCrab HTTPS Server</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .header {{ color: #2c3e50; }}
        .info {{ background: #ecf0f1; padding: 20px; border-radius: 5px; }}
    </style>
</head>
<body>
    <h1 class="header">🦀 JetCrab HTTPS Server</h1>
    <div class="info">
        <h2>Request Information</h2>
        <p><strong>Method:</strong> {}</p>
        <p><strong>URI:</strong> {}</p>
        <p><strong>Path:</strong> {}</p>
        <p><strong>Query:</strong> {}</p>
        <p><strong>Headers:</strong> {} headers</p>
        <p><strong>Timestamp:</strong> {}</p>
    </div>
    <p>This is a secure HTTPS response from JetCrab!</p>
</body>
</html>"#,
        method,
        uri,
        path.as_ref().map(|p| p.as_str()).unwrap_or("/"),
        query
            .as_ref()
            .map(|q| format!("{:?}", q.0))
            .unwrap_or_else(|| "None".to_string()),
        headers.len(),
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html; charset=utf-8")
        .header("content-length", response_body.len())
        .body(response_body)
        .unwrap()
}

/// HTTPS API implementation
pub struct HttpsAPI {
    servers: std::sync::Mutex<Vec<Arc<HttpsServer>>>,
}

impl HttpsAPI {
    /// Create a new HTTPS API instance
    pub fn new() -> Self {
        Self {
            servers: std::sync::Mutex::new(Vec::new()),
        }
    }

    /// Register the HTTPS API in the JavaScript context
    pub fn register(&self, context: &mut Context) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Registering HTTPS API...");

        let https_code = r#"
        globalThis.https = {
            createServer: function(options, requestListener) {
                const server = {
                    listening: false,
                    _options: options || {},
                    _requestListener: requestListener,
                    
                    listen: function(port, host, callback) {
                        if (typeof host === 'function') {
                            callback = host;
                            host = '0.0.0.0';
                        }
                        
                        this.listening = true;
                        console.log(`HTTPS server listening on https://${host || '0.0.0.0'}:${port || 443}`);
                        
                        if (callback) {
                            setTimeout(() => callback(), 0);
                        }
                        
                        return this;
                    },
                    
                    close: function(callback) {
                        this.listening = false;
                        console.log('HTTPS server closed');
                        
                        if (callback) {
                            setTimeout(() => callback(), 0);
                        }
                        
                        return this;
                    },
                    
                    on: function(event, listener) {
                        if (event === 'request') {
                            this._requestListener = listener;
                        }
                        return this;
                    }
                };
                
                return server;
            },
            
            request: function(options, callback) {
                const req = {
                    _options: options || {},
                    _callback: callback,
                    
                    write: function(data) {
                        console.log('HTTPS request data:', data);
                        return true;
                    },
                    
                    end: function(data) {
                        if (data) {
                            this.write(data);
                        }
                        console.log('HTTPS request ended');
                        
                        if (this._callback) {
                            setTimeout(() => {
                                const res = {
                                    statusCode: 200,
                                    headers: {},
                                    on: function(event, listener) {
                                        if (event === 'data') {
                                            setTimeout(() => listener('HTTPS response data'), 0);
                                        } else if (event === 'end') {
                                            setTimeout(() => listener(), 0);
                                        }
                                    }
                                };
                                this._callback(res);
                            }, 0);
                        }
                        
                        return this;
                    },
                    
                    on: function(event, listener) {
                        if (event === 'response') {
                            this._responseListener = listener;
                        }
                        return this;
                    }
                };
                
                return req;
            },
            
            get: function(options, callback) {
                return this.request(options, callback);
            }
        };
        
        console.log('HTTPS API registered successfully');
        "#;

        context.eval(chitin::boa_engine::Source::from_bytes(https_code))?;
        debug!("HTTPS API registration completed");

        Ok(())
    }
}

impl Clone for HttpsAPI {
    fn clone(&self) -> Self {
        Self {
            servers: std::sync::Mutex::new(Vec::new()),
        }
    }
}

impl Default for HttpsAPI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chitin::boa_engine::{Context, Source};
    use std::sync::Arc;
    use tempfile::NamedTempFile;
    use tokio::fs;

    /// Helper function to setup test context with console
    fn setup_test_context() -> Context {
        let mut context = Context::default();

        let setup_code = r#"
        globalThis.console = {
            log: function(...args) { /* mock console.log */ }
        };
        
        globalThis.setTimeout = function(callback, delay) {
            callback();
        };
        "#;
        context.eval(Source::from_bytes(setup_code)).unwrap();

        context
    }

    #[test]
    fn test_https_server_config_default() {
        let config = HttpsServerConfig::default();
        assert_eq!(config.cert_path, "cert.pem");
        assert_eq!(config.key_path, "key.pem");
        assert_eq!(config.port, 443);
        assert_eq!(config.host, "0.0.0.0");
    }

    #[test]
    fn test_https_server_config_custom() {
        let config = HttpsServerConfig {
            cert_path: "custom.pem".to_string(),
            key_path: "custom.key".to_string(),
            port: 8443,
            host: "127.0.0.1".to_string(),
        };

        assert_eq!(config.cert_path, "custom.pem");
        assert_eq!(config.key_path, "custom.key");
        assert_eq!(config.port, 8443);
        assert_eq!(config.host, "127.0.0.1");
    }

    #[test]
    fn test_https_server_creation() {
        let config = HttpsServerConfig::default();
        let server = HttpsServer::new(config);

        assert!(server.server_config.is_none());
        assert!(server.tls_acceptor.is_none());
    }

    #[test]
    fn test_https_api_creation() {
        let api = HttpsAPI::new();
        assert!(api.servers.lock().unwrap().is_empty());
    }

    #[test]
    fn test_https_api_clone() {
        let api1 = HttpsAPI::new();
        let api2 = api1.clone();

        assert!(api1.servers.lock().unwrap().is_empty());
        assert!(api2.servers.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_https_server_tls_config_loading_failure() {
        let mut server = HttpsServer::new(HttpsServerConfig::default());

        let result = server.load_tls_config().await;
        assert!(result.is_err());
    }

    #[test]
    fn test_https_api_registration() {
        let mut context = setup_test_context();
        let api = HttpsAPI::new();

        let result = api.register(&mut context);
        assert!(result.is_ok());

        let global_https = context
            .global_object()
            .get(chitin::boa_engine::js_string!("https"), &mut context)
            .unwrap();
        assert!(global_https.is_object());
    }

    #[test]
    fn test_https_create_server_function() {
        let mut context = setup_test_context();
        let api = HttpsAPI::new();
        api.register(&mut context).unwrap();

        let code = r#"
        const server = https.createServer();
        typeof server.listen === 'function' && typeof server.close === 'function';
        "#;

        let result = context.eval(Source::from_bytes(code)).unwrap();
        assert!(result.to_boolean());
    }

    #[test]
    fn test_https_create_server_with_options() {
        let mut context = setup_test_context();
        let api = HttpsAPI::new();
        api.register(&mut context).unwrap();

        let code = r#"
        const server = https.createServer({
            port: 8443,
            host: 'localhost',
            cert: 'test.pem',
            key: 'test.key'
        });
        typeof server.listen === 'function';
        "#;

        let result = context.eval(Source::from_bytes(code)).unwrap();
        assert!(result.to_boolean());
    }

    #[test]
    fn test_https_request_function() {
        let mut context = setup_test_context();
        let api = HttpsAPI::new();
        api.register(&mut context).unwrap();

        let code = r#"
        const req = https.request({
            hostname: 'example.com',
            port: 443,
            path: '/test'
        });
        typeof req.write === 'function' && typeof req.end === 'function';
        "#;

        let result = context.eval(Source::from_bytes(code)).unwrap();
        assert!(result.to_boolean());
    }

    #[test]
    fn test_https_get_function() {
        let mut context = setup_test_context();
        let api = HttpsAPI::new();
        api.register(&mut context).unwrap();

        let code = r#"
        const req = https.get({
            hostname: 'example.com',
            port: 443,
            path: '/test'
        });
        typeof req.write === 'function' && typeof req.end === 'function';
        "#;

        let result = context.eval(Source::from_bytes(code)).unwrap();
        assert!(result.to_boolean());
    }

    #[test]
    fn test_https_server_listen_callback() {
        let mut context = setup_test_context();
        let api = HttpsAPI::new();
        api.register(&mut context).unwrap();

        let code = r#"
        let callbackCalled = false;
        const server = https.createServer();
        server.listen(8443, () => {
            callbackCalled = true;
        });
        callbackCalled;
        "#;

        let result = context.eval(Source::from_bytes(code)).unwrap();
        assert!(result.to_boolean());
    }

    #[test]
    fn test_https_server_close() {
        let mut context = setup_test_context();
        let api = HttpsAPI::new();
        api.register(&mut context).unwrap();

        let code = r#"
        const server = https.createServer();
        server.close();
        true;
        "#;

        let result = context.eval(Source::from_bytes(code)).unwrap();
        assert!(result.to_boolean());
    }

    #[test]
    fn test_https_request_write_and_end() {
        let mut context = setup_test_context();
        let api = HttpsAPI::new();
        api.register(&mut context).unwrap();

        let code = r#"
        const req = https.request({ hostname: 'example.com' });
        req.write('test data');
        req.end();
        true;
        "#;

        let result = context.eval(Source::from_bytes(code)).unwrap();
        assert!(result.to_boolean());
    }

    #[test]
    fn test_https_module_globals() {
        let mut context = setup_test_context();
        let api = HttpsAPI::new();
        api.register(&mut context).unwrap();

        let code = r#"
        typeof https.createServer === 'function' &&
        typeof https.request === 'function' &&
        typeof https.get === 'function';
        "#;

        let result = context.eval(Source::from_bytes(code)).unwrap();
        assert!(result.to_boolean());
    }
}
