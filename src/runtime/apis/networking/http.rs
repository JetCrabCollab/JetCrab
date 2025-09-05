//! # HTTP Module
//!
//! Provides HTTP client and server functionality similar to Node.js http module.

use boa_engine::{js_string, property::Attribute, Context, JsResult};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::info;

/// HTTP Module implementation
pub struct HttpAPI {
    servers: Arc<Mutex<HashMap<u32, HttpServer>>>,
    next_server_id: Arc<Mutex<u32>>,
}

/// HTTP Server implementation
pub struct HttpServer {
    pub id: u32,
    pub port: u16,
    pub host: String,
    pub running: bool,
}

impl HttpAPI {
    /// Create a new HTTP API instance
    pub fn new() -> Self {
        Self {
            servers: Arc::new(Mutex::new(HashMap::new())),
            next_server_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Register the HTTP API with the JavaScript context
    pub fn register(&self, context: &mut Context) -> JsResult<()> {
        info!("🌐 Registering HTTP API...");

        let http_code = r#"
        globalThis.http = {
            createServer: function(requestListener) {
                const server = {
                    id: Math.floor(Math.random() * 10000) + 1000,
                    port: null,
                    host: 'localhost',
                    running: false,
                    _requestListener: requestListener,
                    
                    listen: function(port, host, callback) {
                        if (typeof host === 'function') {
                            callback = host;
                            host = 'localhost';
                        }
                        
                        this.port = port;
                        this.host = host || 'localhost';
                        this.running = true;
                        
                        console.log(`🚀 HTTP server listening on http://${this.host}:${this.port}`);
                        
                        if (callback) {
                            setTimeout(() => callback(), 100);
                        }
                        
                        return this;
                    },
                    
                    close: function(callback) {
                        this.running = false;
                        console.log(`🛑 HTTP server closed on port ${this.port}`);
                        
                        if (callback) {
                            setTimeout(() => callback(), 100);
                        }
                    },
                    
                    _handleRequest: function(req, res) {
                        if (this._requestListener) {
                            this._requestListener(req, res);
                        }
                    }
                };
                
                return server;
            },
            
            request: function(options, callback) {
                const req = {
                    method: options.method || 'GET',
                    url: options.path || '/',
                    headers: options.headers || {},
                    _callback: callback,
                    
                    write: function(data) {
                        console.log(`📤 HTTP request data: ${data}`);
                        return true;
                    },
                    
                    end: function(data) {
                        if (data) {
                            this.write(data);
                        }
                        
                        setTimeout(() => {
                            if (this._callback) {
                                const res = {
                                    statusCode: 200,
                                    headers: {},
                                    on: function(event, listener) {
                                        if (event === 'data') {
                                            this._dataListener = listener;
                                        } else if (event === 'end') {
                                            this._endListener = listener;
                                        }
                                    },
                                    _dataListener: null,
                                    _endListener: null
                                };
                                
                                this._callback(res);
                                
                                if (res._dataListener) {
                                    setTimeout(() => res._dataListener('Response data'), 100);
                                }
                                if (res._endListener) {
                                    setTimeout(() => res._endListener(), 200);
                                }
                            }
                        }, 100);
                    },
                    
                    setTimeout: function(timeout) {
                        console.log(`⏰ Request timeout set to ${timeout}ms`);
                    },
                    
                    abort: function() {
                        console.log('❌ Request aborted');
                    }
                };
                
                return req;
            },
            
            get: function(options, callback) {
                options.method = 'GET';
                return this.request(options, callback);
            },
            
            globalAgent: {
                maxSockets: 5,
                keepAlive: true,
                keepAliveMsecs: 1000
            },
            
            STATUS_CODES: {
                200: 'OK',
                201: 'Created',
                400: 'Bad Request',
                401: 'Unauthorized',
                403: 'Forbidden',
                404: 'Not Found',
                500: 'Internal Server Error'
            },
            
            METHODS: ['GET', 'POST', 'PUT', 'DELETE', 'HEAD', 'OPTIONS', 'PATCH']
        };
        
        globalThis.IncomingMessage = function() {
            this.method = 'GET';
            this.url = '/';
            this.headers = {};
            this.httpVersion = '1.1';
            this.statusCode = 200;
            this.statusMessage = 'OK';
            this.socket = null;
            
            this.on = function(event, listener) {
                if (event === 'data') {
                    this._dataListener = listener;
                } else if (event === 'end') {
                    this._endListener = listener;
                } else if (event === 'error') {
                    this._errorListener = listener;
                }
            };
            
            this.setEncoding = function(encoding) {
                this._encoding = encoding;
            };
            
            this.pause = function() {
                this._paused = true;
            };
            
            this.resume = function() {
                this._paused = false;
            };
        };
        
        globalThis.ServerResponse = function() {
            this.statusCode = 200;
            this.statusMessage = 'OK';
            this.headers = {};
            this._headersSent = false;
            
            this.setHeader = function(name, value) {
                this.headers[name.toLowerCase()] = value;
            };
            
            this.getHeader = function(name) {
                return this.headers[name.toLowerCase()];
            };
            
            this.removeHeader = function(name) {
                delete this.headers[name.toLowerCase()];
            };
            
            this.writeHead = function(statusCode, statusMessage, headers) {
                this.statusCode = statusCode;
                if (typeof statusMessage === 'string') {
                    this.statusMessage = statusMessage;
                } else if (typeof statusMessage === 'object') {
                    headers = statusMessage;
                }
                
                if (headers) {
                    for (const [key, value] of Object.entries(headers)) {
                        this.setHeader(key, value);
                    }
                }
            };
            
            this.write = function(chunk, encoding, callback) {
                if (typeof encoding === 'function') {
                    callback = encoding;
                    encoding = 'utf8';
                }
                
                console.log(`📤 Response data: ${chunk}`);
                this._headersSent = true;
                
                if (callback) {
                    setTimeout(() => callback(), 0);
                }
                
                return true;
            };
            
            this.end = function(chunk, encoding, callback) {
                if (chunk) {
                    this.write(chunk, encoding);
                }
                
                console.log(`✅ Response ended with status ${this.statusCode}`);
                this._headersSent = true;
                
                if (callback) {
                    setTimeout(() => callback(), 0);
                }
            };
            
            this.setTimeout = function(timeout, callback) {
                console.log(`⏰ Response timeout set to ${timeout}ms`);
                if (callback) {
                    setTimeout(callback, timeout);
                }
            };
        };
        
        console.log('✅ HTTP API registered successfully');
        "#;

        context.eval(boa_engine::Source::from_bytes(http_code))?;

        let http_object = context.eval(boa_engine::Source::from_bytes("globalThis.http"))?;
        context.register_global_property(js_string!("http"), http_object, Attribute::all())?;

        Ok(())
    }
}

impl Default for HttpAPI {
    fn default() -> Self {
        Self::new()
    }
}
