//! # Require System
//!
//! Implements Node.js-style require() function for loading native modules.

use boa_engine::{js_string, property::Attribute, Context, JsResult};
use std::collections::HashMap;
use tracing::info;

/// Require system implementation
pub struct RequireAPI {
    native_modules: HashMap<String, String>,
}

impl RequireAPI {
    /// Create a new Require API instance
    pub fn new() -> Self {
        let mut native_modules = HashMap::new();

        native_modules.insert("http".to_string(), "globalThis.http".to_string());
        native_modules.insert("fs".to_string(), "globalThis.fs".to_string());
        native_modules.insert("path".to_string(), "globalThis.path".to_string());
        native_modules.insert("os".to_string(), "globalThis.os".to_string());
        native_modules.insert("process".to_string(), "globalThis.process".to_string());
        native_modules.insert("console".to_string(), "globalThis.console".to_string());
        native_modules.insert(
            "child_process".to_string(),
            "globalThis.child_process".to_string(),
        );
        native_modules.insert("crypto".to_string(), "globalThis.crypto".to_string());
        native_modules.insert("url".to_string(), "globalThis.url".to_string());
        native_modules.insert("util".to_string(), "globalThis.util".to_string());
        native_modules.insert("events".to_string(), "globalThis.events".to_string());
        native_modules.insert("stream".to_string(), "globalThis.stream".to_string());
        native_modules.insert("buffer".to_string(), "globalThis.Buffer".to_string());
        native_modules.insert(
            "querystring".to_string(),
            "globalThis.querystring".to_string(),
        );
        native_modules.insert("timers".to_string(), "globalThis.timers".to_string());

        Self { native_modules }
    }

    /// Register the require system with the JavaScript context
    pub fn register(&self, context: &mut Context) -> JsResult<()> {
        info!("📦 Registering Require System...");

        let require_code = r#"
        const moduleCache = new Map();
        
        function require(moduleName) {
            if (typeof moduleName !== 'string') {
                throw new TypeError('Module name must be a string');
            }
            
            if (moduleCache.has(moduleName)) {
                return moduleCache.get(moduleName).exports;
            }
            
            const nativeModules = {
                'http': globalThis.http,
                'fs': globalThis.fs,
                'path': globalThis.path,
                'os': globalThis.os,
                'process': globalThis.process,
                'console': globalThis.console,
                'child_process': globalThis.child_process,
                'crypto': globalThis.crypto,
                'url': globalThis.url,
                'util': globalThis.util,
                'events': globalThis.events,
                'stream': globalThis.stream,
                'buffer': globalThis.Buffer,
                'querystring': globalThis.querystring,
                'timers': globalThis.timers
            };
            
            if (nativeModules[moduleName]) {
                const module = {
                    exports: nativeModules[moduleName],
                    id: moduleName,
                    filename: moduleName,
                    loaded: true,
                    parent: null,
                    children: [],
                    paths: []
                };
                
                moduleCache.set(moduleName, module);
                return module.exports;
            }
            
            if (moduleName.startsWith('./') || moduleName.startsWith('../') || moduleName.startsWith('/')) {
                throw new Error(`Cannot find module '${moduleName}'. File system modules not yet implemented.`);
            }
            
            throw new Error(`Cannot find module '${moduleName}'. NPM packages not yet implemented.`);
        }
        
        const Module = {
            _cache: moduleCache,
            _extensions: {
                '.js': function(module, filename) {
                    throw new Error('File system modules not yet implemented');
                },
                '.json': function(module, filename) {
                    throw new Error('JSON modules not yet implemented');
                },
                '.node': function(module, filename) {
                    throw new Error('Native modules not yet implemented');
                }
            },
            _resolveFilename: function(request, parent) {
                if (typeof request !== 'string') {
                    throw new TypeError('Module name must be a string');
                }
                
                const nativeModules = [
                    'http', 'fs', 'path', 'os', 'process', 'console',
                    'child_process', 'crypto', 'url', 'util', 'events',
                    'stream', 'buffer', 'querystring', 'timers'
                ];
                
                if (nativeModules.includes(request)) {
                    return request;
                }
                
                return request;
            },
            _load: function(request, parent) {
                return require(request);
            }
        };
        
        const exports = {};
        
        const module = {
            exports: exports,
            id: '<repl>',
            filename: '<repl>',
            loaded: false,
            parent: null,
            children: [],
            paths: []
        };
        
        globalThis.require = require;
        globalThis.module = module;
        globalThis.exports = exports;
        globalThis.Module = Module;
        
        console.log('✅ Require System registered successfully');
        "#;

        context.eval(boa_engine::Source::from_bytes(require_code))?;

        Ok(())
    }
}

impl Default for RequireAPI {
    fn default() -> Self {
        Self::new()
    }
}
