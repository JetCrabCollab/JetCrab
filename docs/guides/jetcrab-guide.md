# JetCrab Guide

Welcome to the comprehensive guide for JetCrab, a modern JavaScript runtime built with Rust and powered by the Boa engine.

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Getting Started](#getting-started)
4. [Core Features](#core-features)
5. [API Reference](#api-reference)
6. [Advanced Usage](#advanced-usage)
7. [Development Tools](#development-tools)
8. [Troubleshooting](#troubleshooting)
9. [Examples](#examples)

## Introduction

JetCrab is a high-performance JavaScript runtime that combines the speed and safety of Rust with the flexibility of JavaScript. It provides:

- **Fast Execution**: Powered by the Boa JavaScript engine
- **Rust Integration**: Seamless interoperability between JavaScript and Rust
- **Node.js Compatibility**: Familiar APIs and module system
- **WebAssembly Support**: Run Rust modules as WebAssembly in JavaScript
- **Modern Tooling**: Built-in development tools and package management

## Installation

### From Source

```bash
git clone https://github.com/your-org/jetcrab.git
cd jetcrab
cargo build --release
```

### Using Package Managers

#### Windows (Chocolatey)
```powershell
choco install jetcrab
```

#### Linux (Snap)
```bash
sudo snap install jetcrab
```

#### macOS (Homebrew)
```bash
brew install jetcrab
```

## Getting Started

### Basic Usage

Run a JavaScript file:
```bash
jetcrab run app.js
```

Start an interactive REPL:
```bash
jetcrab repl
```

### Your First JetCrab Application

Create a simple JavaScript file (`hello.js`):

```javascript
console.log('Hello from JetCrab!');
console.log('Platform:', process.platform);
console.log('Node.js version:', process.version);
```

Run it:
```bash
jetcrab run hello.js
```

## Core Features

### JavaScript Runtime

JetCrab provides a complete JavaScript runtime with:

- **ES6+ Support**: Modern JavaScript features
- **Async/Await**: Full support for asynchronous programming
- **Modules**: ES6 modules and CommonJS compatibility
- **Promises**: Native Promise implementation

### Rust Integration

Load and use Rust modules in JavaScript:

```rust
// math.rs
#[export]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[export]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
```

```javascript
// app.js
const math = require('./math');
console.log(math.add(5, 3)); // 8
console.log(math.multiply(4, 7)); // 28
```

### WebAssembly Support

Compile Rust to WebAssembly and use it in JavaScript:

```rust
// wasm_example.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

```javascript
// app.js
const wasmModule = await import('./wasm_example.wasm');
console.log(wasmModule.fibonacci(10)); // 55
```

## API Reference

### Built-in APIs

JetCrab provides many built-in APIs similar to Node.js:

#### Console API
```javascript
console.log('Info message');
console.error('Error message');
console.warn('Warning message');
console.info('Information message');
```

#### Process API
```javascript
console.log('Platform:', process.platform);
console.log('Architecture:', process.arch);
console.log('Node.js version:', process.version);
console.log('Command line args:', process.argv);
console.log('Environment variables:', process.env);
```

#### File System API
```javascript
const fs = require('fs');

// Read file
const data = fs.readFileSync('file.txt', 'utf8');
console.log(data);

// Write file
fs.writeFileSync('output.txt', 'Hello JetCrab!');
```

#### HTTP API
```javascript
const http = require('http');

const server = http.createServer((req, res) => {
    res.writeHead(200, { 'Content-Type': 'text/plain' });
    res.end('Hello from JetCrab!');
});

server.listen(3000, () => {
    console.log('Server running on http://localhost:3000');
});
```

#### Timers API
```javascript
// setTimeout
setTimeout(() => {
    console.log('This runs after 1 second');
}, 1000);

// setInterval
const interval = setInterval(() => {
    console.log('This runs every 2 seconds');
}, 2000);

// Clear after 10 seconds
setTimeout(() => {
    clearInterval(interval);
}, 10000);
```

### Advanced APIs

#### Performance Hooks
```javascript
const perf_hooks = require('perf_hooks');

const start = perf_hooks.performance.now();
// Your code here
const end = perf_hooks.performance.now();
console.log(`Execution time: ${end - start} milliseconds`);
```

#### Worker Threads
```javascript
const { Worker, isMainThread, parentPort } = require('worker_threads');

if (isMainThread) {
    const worker = new Worker(__filename);
    worker.postMessage('Hello from main thread!');
    worker.on('message', (msg) => {
        console.log('Message from worker:', msg);
    });
} else {
    parentPort.on('message', (msg) => {
        console.log('Message from main thread:', msg);
        parentPort.postMessage('Hello from worker thread!');
    });
}
```

## Advanced Usage

### Configuration

Create a `jetcrab.toml` configuration file:

```toml
[api]
enabled_apis = ["console", "process", "fs", "http", "timers"]
disabled_apis = []
experimental_apis = ["worker_threads"]

[performance]
enable_lazy_loading = true
api_timeout_ms = 5000

[logging]
level = "info"
format = "json"

[development]
enable_source_maps = true
enable_hot_reload = true
```

### Package Management with Claw

JetCrab includes Claw, a package manager for both JavaScript and Rust packages:

#### Install JavaScript packages
```bash
claw install express lodash
```

#### Install Rust packages
```bash
claw install --rust serde tokio
```

#### Initialize a new project
```bash
claw init my-project
cd my-project
```

#### Build and run
```bash
claw build
claw run
```

### Development Tools

JetCrab includes comprehensive development tools:

#### File Watching
```bash
jetcrab dev --watch
```

#### Linting
```bash
jetcrab lint
```

#### Formatting
```bash
jetcrab format
```

#### Testing
```bash
jetcrab test
```

#### Debugging
```bash
jetcrab debug app.js
```

## Development Tools

### JetCrab CLI Commands

#### `jetcrab run <file>`
Run a JavaScript file or Rust module.

```bash
jetcrab run app.js
jetcrab run --rust src/lib.rs
```

#### `jetcrab repl`
Start an interactive REPL session.

```bash
jetcrab repl
```

#### `jetcrab dev`
Start development mode with file watching and hot reload.

```bash
jetcrab dev
```

#### `jetcrab build`
Build the project for production.

```bash
jetcrab build
```

### Claw Package Manager Commands

#### `claw init <name>`
Initialize a new JetCrab project.

```bash
claw init my-app
```

#### `claw install <packages>`
Install JavaScript or Rust packages.

```bash
claw install express
claw install --rust serde
```

#### `claw build`
Build the project.

```bash
claw build
```

#### `claw run`
Run the project.

```bash
claw run
```

## Troubleshooting

### Common Issues

#### Module Not Found
```
Error: Cannot find module 'express'
```

**Solution**: Install the module using Claw:
```bash
claw install express
```

#### Rust Compilation Error
```
Error: Failed to compile Rust module
```

**Solution**: Ensure you have Rust installed and the module is properly configured:
```bash
rustup update
cargo check
```

#### WebAssembly Loading Error
```
Error: Failed to load WebAssembly module
```

**Solution**: Ensure the WASM file is properly compiled and the path is correct:
```bash
wasm-pack build --target web
```

### Debug Mode

Enable debug mode for detailed logging:

```bash
RUST_LOG=debug jetcrab run app.js
```

### Performance Issues

If you experience performance issues:

1. Enable lazy loading in configuration
2. Use the performance hooks API to profile your code
3. Consider using WebAssembly for computationally intensive tasks

## Examples

### Web Server
```javascript
const http = require('http');

const server = http.createServer((req, res) => {
    if (req.url === '/') {
        res.writeHead(200, { 'Content-Type': 'text/html' });
        res.end(`
            <html>
                <body>
                    <h1>Welcome to JetCrab!</h1>
                    <p>Platform: ${process.platform}</p>
                    <p>Node.js version: ${process.version}</p>
                </body>
            </html>
        `);
    } else if (req.url === '/api/data') {
        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ message: 'Hello from JetCrab API!' }));
    } else {
        res.writeHead(404, { 'Content-Type': 'text/plain' });
        res.end('Not Found');
    }
});

const PORT = process.env.PORT || 3000;
server.listen(PORT, () => {
    console.log(`Server running on http://localhost:${PORT}`);
});
```

### File Processing
```javascript
const fs = require('fs');
const path = require('path');

function processDirectory(dir) {
    const files = fs.readdirSync(dir);
    
    files.forEach(file => {
        const filePath = path.join(dir, file);
        const stats = fs.statSync(filePath);
        
        if (stats.isDirectory()) {
            console.log(`Directory: ${file}`);
            processDirectory(filePath);
        } else {
            console.log(`File: ${file} (${stats.size} bytes)`);
        }
    });
}

processDirectory('./');
```

### Async Operations
```javascript
const fs = require('fs').promises;

async function readMultipleFiles() {
    try {
        const files = ['file1.txt', 'file2.txt', 'file3.txt'];
        const promises = files.map(file => fs.readFile(file, 'utf8'));
        
        const contents = await Promise.all(promises);
        
        contents.forEach((content, index) => {
            console.log(`File ${files[index]}: ${content.length} characters`);
        });
    } catch (error) {
        console.error('Error reading files:', error);
    }
}

readMultipleFiles();
```

### Rust Integration Example
```rust
// math_utils.rs
#[export]
pub fn calculate_fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    
    let mut a = 0;
    let mut b = 1;
    
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    
    b
}

#[export]
pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}
```

```javascript
// app.js
const mathUtils = require('./math_utils');

console.log('Fibonacci(10):', mathUtils.calculate_fibonacci(10));
console.log('Is 17 prime?', mathUtils.is_prime(17));
console.log('Is 15 prime?', mathUtils.is_prime(15));
```

## Contributing

We welcome contributions to JetCrab! Please see our [Contributing Guide](../CONTRIBUTING.md) for details.

## License

JetCrab is licensed under the MIT License. See [LICENSE](../../LICENSE) for details.

## Support

- **Documentation**: [docs.jetcrab.dev](https://docs.jetcrab.dev)
- **Issues**: [GitHub Issues](https://github.com/your-org/jetcrab/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-org/jetcrab/discussions)
- **Discord**: [JetCrab Community](https://discord.gg/jetcrab)

---

Happy coding with JetCrab! 🦀
