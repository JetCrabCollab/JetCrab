# JetCrab Examples

This document provides practical examples of using JetCrab for various tasks.

## Table of Contents

1. [Basic Examples](#basic-examples)
2. [Web Development](#web-development)
3. [File Processing](#file-processing)
4. [Async Operations](#async-operations)
5. [Rust Integration](#rust-integration)
6. [WebAssembly Examples](#webassembly-examples)
7. [Advanced Examples](#advanced-examples)

## Basic Examples

### Hello World
```javascript
// hello.js
console.log('Hello, JetCrab!');
console.log('Platform:', process.platform);
console.log('Node.js version:', process.version);
```

Run with:
```bash
jetcrab run hello.js
```

### Interactive REPL
```javascript
// Start REPL
jetcrab repl

// In REPL:
> console.log('Hello from REPL!');
> 2 + 2
4
> const name = 'JetCrab';
> `Hello, ${name}!`
'Hello, JetCrab!'
```

### Command Line Arguments
```javascript
// args.js
console.log('Arguments:', process.argv);
console.log('Script name:', process.argv[1]);
console.log('First argument:', process.argv[2]);

// Run with: jetcrab run args.js hello world
// Output:
// Arguments: ['node', 'args.js', 'hello', 'world']
// Script name: args.js
// First argument: hello
```

### Environment Variables
```javascript
// env.js
console.log('NODE_ENV:', process.env.NODE_ENV);
console.log('PATH:', process.env.PATH);
console.log('HOME:', process.env.HOME);

// Set environment variable and run:
// NODE_ENV=production jetcrab run env.js
```

## Web Development

### Simple HTTP Server
```javascript
// server.js
const http = require('http');

const server = http.createServer((req, res) => {
    res.writeHead(200, { 'Content-Type': 'text/html' });
    res.end(`
        <html>
            <head>
                <title>JetCrab Server</title>
            </head>
            <body>
                <h1>Welcome to JetCrab!</h1>
                <p>Request URL: ${req.url}</p>
                <p>Request Method: ${req.method}</p>
                <p>Platform: ${process.platform}</p>
            </body>
        </html>
    `);
});

const PORT = process.env.PORT || 3000;
server.listen(PORT, () => {
    console.log(`Server running on http://localhost:${PORT}`);
});
```

### REST API Server
```javascript
// api-server.js
const http = require('http');
const url = require('url');

const server = http.createServer((req, res) => {
    const parsedUrl = url.parse(req.url, true);
    const path = parsedUrl.pathname;
    const method = req.method;
    
    // Set CORS headers
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type');
    
    if (method === 'OPTIONS') {
        res.writeHead(200);
        res.end();
        return;
    }
    
    if (path === '/api/users' && method === 'GET') {
        const users = [
            { id: 1, name: 'John Doe', email: 'john@example.com' },
            { id: 2, name: 'Jane Smith', email: 'jane@example.com' }
        ];
        
        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify(users));
    } else if (path === '/api/users' && method === 'POST') {
        let body = '';
        req.on('data', chunk => {
            body += chunk.toString();
        });
        
        req.on('end', () => {
            try {
                const user = JSON.parse(body);
                user.id = Date.now(); // Simple ID generation
                
                res.writeHead(201, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify(user));
            } catch (error) {
                res.writeHead(400, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ error: 'Invalid JSON' }));
            }
        });
    } else {
        res.writeHead(404, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ error: 'Not Found' }));
    }
});

const PORT = process.env.PORT || 3000;
server.listen(PORT, () => {
    console.log(`API Server running on http://localhost:${PORT}`);
});
```

### Static File Server
```javascript
// static-server.js
const http = require('http');
const fs = require('fs');
const path = require('path');

const server = http.createServer((req, res) => {
    let filePath = '.' + req.url;
    if (filePath === './') {
        filePath = './index.html';
    }
    
    const extname = path.extname(filePath).toLowerCase();
    const mimeTypes = {
        '.html': 'text/html',
        '.js': 'text/javascript',
        '.css': 'text/css',
        '.json': 'application/json',
        '.png': 'image/png',
        '.jpg': 'image/jpg',
        '.gif': 'image/gif',
        '.svg': 'image/svg+xml',
        '.wav': 'audio/wav',
        '.mp4': 'video/mp4',
        '.woff': 'application/font-woff',
        '.ttf': 'application/font-ttf',
        '.eot': 'application/vnd.ms-fontobject',
        '.otf': 'application/font-otf',
        '.wasm': 'application/wasm'
    };
    
    const contentType = mimeTypes[extname] || 'application/octet-stream';
    
    fs.readFile(filePath, (error, content) => {
        if (error) {
            if (error.code === 'ENOENT') {
                res.writeHead(404, { 'Content-Type': 'text/html' });
                res.end(`
                    <html>
                        <body>
                            <h1>404 - File Not Found</h1>
                            <p>The requested file ${req.url} was not found.</p>
                        </body>
                    </html>
                `);
            } else {
                res.writeHead(500);
                res.end('Server Error: ' + error.code);
            }
        } else {
            res.writeHead(200, { 'Content-Type': contentType });
            res.end(content, 'utf-8');
        }
    });
});

const PORT = process.env.PORT || 3000;
server.listen(PORT, () => {
    console.log(`Static file server running on http://localhost:${PORT}`);
});
```

## File Processing

### File Reader
```javascript
// file-reader.js
const fs = require('fs');
const path = require('path');

function readFile(filePath) {
    try {
        const content = fs.readFileSync(filePath, 'utf8');
        console.log(`File: ${filePath}`);
        console.log(`Size: ${content.length} characters`);
        console.log(`Content:\n${content}`);
    } catch (error) {
        console.error(`Error reading file ${filePath}:`, error.message);
    }
}

// Read a file
readFile('package.json');
```

### Directory Scanner
```javascript
// dir-scanner.js
const fs = require('fs');
const path = require('path');

function scanDirectory(dirPath, depth = 0) {
    const indent = '  '.repeat(depth);
    
    try {
        const items = fs.readdirSync(dirPath);
        
        items.forEach(item => {
            const itemPath = path.join(dirPath, item);
            const stats = fs.statSync(itemPath);
            
            if (stats.isDirectory()) {
                console.log(`${indent}📁 ${item}/`);
                scanDirectory(itemPath, depth + 1);
            } else {
                console.log(`${indent}📄 ${item} (${stats.size} bytes)`);
            }
        });
    } catch (error) {
        console.error(`Error scanning directory ${dirPath}:`, error.message);
    }
}

// Scan current directory
scanDirectory('.');
```

### File Watcher
```javascript
// file-watcher.js
const fs = require('fs');
const path = require('path');

function watchFile(filePath) {
    if (!fs.existsSync(filePath)) {
        console.error(`File ${filePath} does not exist`);
        return;
    }
    
    console.log(`Watching file: ${filePath}`);
    
    fs.watchFile(filePath, (curr, prev) => {
        console.log(`File changed: ${filePath}`);
        console.log(`Previous size: ${prev.size} bytes`);
        console.log(`Current size: ${curr.size} bytes`);
        console.log(`Modified: ${curr.mtime}`);
        console.log('---');
    });
}

// Watch a file
watchFile('package.json');
```

### CSV Parser
```javascript
// csv-parser.js
const fs = require('fs');

function parseCSV(filePath) {
    try {
        const content = fs.readFileSync(filePath, 'utf8');
        const lines = content.split('\n');
        const headers = lines[0].split(',');
        const data = [];
        
        for (let i = 1; i < lines.length; i++) {
            if (lines[i].trim()) {
                const values = lines[i].split(',');
                const row = {};
                
                headers.forEach((header, index) => {
                    row[header.trim()] = values[index] ? values[index].trim() : '';
                });
                
                data.push(row);
            }
        }
        
        return data;
    } catch (error) {
        console.error('Error parsing CSV:', error.message);
        return [];
    }
}

// Parse CSV file
const data = parseCSV('data.csv');
console.log('Parsed data:', JSON.stringify(data, null, 2));
```

## Async Operations

### Promise-based File Operations
```javascript
// async-files.js
const fs = require('fs').promises;
const path = require('path');

async function processFiles() {
    try {
        // Read multiple files concurrently
        const files = ['file1.txt', 'file2.txt', 'file3.txt'];
        const promises = files.map(file => fs.readFile(file, 'utf8'));
        
        const contents = await Promise.all(promises);
        
        contents.forEach((content, index) => {
            console.log(`File ${files[index]}: ${content.length} characters`);
        });
        
        // Write processed data
        const output = contents.join('\n---\n');
        await fs.writeFile('combined.txt', output);
        console.log('Combined file written successfully');
        
    } catch (error) {
        console.error('Error processing files:', error.message);
    }
}

processFiles();
```

### HTTP Client with Retry
```javascript
// http-client.js
const http = require('http');

function makeRequest(url, retries = 3) {
    return new Promise((resolve, reject) => {
        const req = http.get(url, (res) => {
            let data = '';
            
            res.on('data', chunk => {
                data += chunk;
            });
            
            res.on('end', () => {
                resolve({ status: res.statusCode, data });
            });
        });
        
        req.on('error', (error) => {
            if (retries > 0) {
                console.log(`Request failed, retrying... (${retries} retries left)`);
                setTimeout(() => {
                    makeRequest(url, retries - 1)
                        .then(resolve)
                        .catch(reject);
                }, 1000);
            } else {
                reject(error);
            }
        });
        
        req.setTimeout(5000, () => {
            req.destroy();
            reject(new Error('Request timeout'));
        });
    });
}

async function fetchData() {
    try {
        const response = await makeRequest('http://httpbin.org/json');
        console.log('Response status:', response.status);
        console.log('Response data:', response.data);
    } catch (error) {
        console.error('Failed to fetch data:', error.message);
    }
}

fetchData();
```

### Batch Processing
```javascript
// batch-processor.js
const fs = require('fs').promises;

async function processBatch(items, batchSize = 5) {
    const results = [];
    
    for (let i = 0; i < items.length; i += batchSize) {
        const batch = items.slice(i, i + batchSize);
        console.log(`Processing batch ${Math.floor(i / batchSize) + 1}/${Math.ceil(items.length / batchSize)}`);
        
        const batchPromises = batch.map(async (item) => {
            // Simulate processing
            await new Promise(resolve => setTimeout(resolve, 100));
            return `Processed: ${item}`;
        });
        
        const batchResults = await Promise.all(batchPromises);
        results.push(...batchResults);
        
        // Small delay between batches
        await new Promise(resolve => setTimeout(resolve, 50));
    }
    
    return results;
}

async function main() {
    const items = Array.from({ length: 20 }, (_, i) => `Item ${i + 1}`);
    const results = await processBatch(items, 5);
    
    console.log('All items processed:');
    results.forEach(result => console.log(result));
}

main();
```

## Rust Integration

### Basic Rust Module
```rust
// src/lib.rs
use std::collections::HashMap;

#[export]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[export]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[export]
pub fn fibonacci(n: u32) -> u64 {
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

#[export]
pub fn count_words(text: &str) -> HashMap<String, u32> {
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *word_count.entry(word).or_insert(0) += 1;
    }
    
    word_count
}
```

```javascript
// use-rust.js
const mathUtils = require('./src/lib.rs');

console.log('2 + 3 =', mathUtils.add(2, 3));
console.log('4 * 5 =', mathUtils.multiply(4, 5));
console.log('Fibonacci(10) =', mathUtils.fibonacci(10));
console.log('Is 17 prime?', mathUtils.is_prime(17));
console.log('Is 15 prime?', mathUtils.is_prime(15));

const text = "Hello world hello rust world";
const wordCount = mathUtils.count_words(text);
console.log('Word count:', wordCount);
```

### Advanced Rust Module
```rust
// src/advanced.rs
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Cache {
    data: Mutex<HashMap<String, (String, u64)>>,
    ttl: u64,
}

impl Cache {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
            ttl: ttl_seconds,
        }
    }
    
    pub fn set(&self, key: String, value: String) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut data = self.data.lock().unwrap();
        data.insert(key, (value, now));
    }
    
    pub fn get(&self, key: &str) -> Option<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut data = self.data.lock().unwrap();
        
        if let Some((value, timestamp)) = data.get(key) {
            if now - timestamp < self.ttl {
                return Some(value.clone());
            } else {
                data.remove(key);
            }
        }
        
        None
    }
    
    pub fn clear(&self) {
        let mut data = self.data.lock().unwrap();
        data.clear();
    }
    
    pub fn size(&self) -> usize {
        let data = self.data.lock().unwrap();
        data.len()
    }
}

#[export]
pub fn create_cache(ttl_seconds: u64) -> Cache {
    Cache::new(ttl_seconds)
}

#[export]
pub fn cache_set(cache: &mut Cache, key: String, value: String) {
    cache.set(key, value);
}

#[export]
pub fn cache_get(cache: &Cache, key: &str) -> Option<String> {
    cache.get(key)
}

#[export]
pub fn cache_clear(cache: &mut Cache) {
    cache.clear();
}

#[export]
pub fn cache_size(cache: &Cache) -> usize {
    cache.size()
}
```

```javascript
// use-advanced-rust.js
const { create_cache, cache_set, cache_get, cache_clear, cache_size } = require('./src/advanced.rs');

// Create cache with 60 second TTL
const cache = create_cache(60);

// Set some values
cache_set(cache, 'user:1', 'John Doe');
cache_set(cache, 'user:2', 'Jane Smith');
cache_set(cache, 'config:theme', 'dark');

// Get values
console.log('User 1:', cache_get(cache, 'user:1'));
console.log('User 2:', cache_get(cache, 'user:2'));
console.log('Theme:', cache_get(cache, 'config:theme'));
console.log('Cache size:', cache_size(cache));

// Clear cache
cache_clear(cache);
console.log('Cache size after clear:', cache_size(cache));
```

## WebAssembly Examples

### Basic WASM Module
```rust
// wasm-example/src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
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

#[wasm_bindgen]
pub struct Calculator {
    value: f64,
}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculator {
        Calculator { value: 0.0 }
    }
    
    #[wasm_bindgen]
    pub fn add(&mut self, n: f64) {
        self.value += n;
    }
    
    #[wasm_bindgen]
    pub fn multiply(&mut self, n: f64) {
        self.value *= n;
    }
    
    #[wasm_bindgen]
    pub fn get_value(&self) -> f64 {
        self.value
    }
    
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.value = 0.0;
    }
}
```

```javascript
// use-wasm.js
import { add, multiply, fibonacci, Calculator } from './wasm-example/pkg/wasm_example.js';

console.log('2 + 3 =', add(2, 3));
console.log('4 * 5 =', multiply(4, 5));
console.log('Fibonacci(10) =', fibonacci(10));

// Use the Calculator class
const calc = new Calculator();
calc.add(10);
calc.multiply(2);
calc.add(5);
console.log('Calculator value:', calc.get_value()); // 25

calc.reset();
console.log('After reset:', calc.get_value()); // 0
```

### Image Processing WASM
```rust
// image-processor/src/lib.rs
use wasm_bindgen::prelude::*;
use image::{ImageBuffer, RgbImage, Rgb};

#[wasm_bindgen]
pub fn process_image(data: &[u8]) -> Vec<u8> {
    // Load image from bytes
    let img = image::load_from_memory(data).unwrap();
    let rgb_img = img.to_rgb8();
    
    // Apply grayscale filter
    let mut processed = RgbImage::new(rgb_img.width(), rgb_img.height());
    
    for (x, y, pixel) in rgb_img.enumerate_pixels() {
        let gray = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3;
        let gray_pixel = Rgb([gray as u8, gray as u8, gray as u8]);
        processed.put_pixel(x, y, gray_pixel);
    }
    
    // Convert back to bytes
    let mut result = Vec::new();
    processed.write_to(&mut std::io::Cursor::new(&mut result), image::ImageFormat::Png).unwrap();
    result
}

#[wasm_bindgen]
pub fn resize_image(data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let img = image::load_from_memory(data).unwrap();
    let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);
    
    let mut result = Vec::new();
    resized.write_to(&mut std::io::Cursor::new(&mut result), image::ImageFormat::Png).unwrap();
    result
}
```

```javascript
// image-processor.js
import { process_image, resize_image } from './image-processor/pkg/image_processor.js';
import fs from 'fs';

// Read image file
const imageData = fs.readFileSync('input.jpg');

// Process image (convert to grayscale)
const processedData = process_image(imageData);
fs.writeFileSync('output_grayscale.png', processedData);

// Resize image
const resizedData = resize_image(imageData, 800, 600);
fs.writeFileSync('output_resized.png', resizedData);

console.log('Image processing complete!');
```

## Advanced Examples

### Real-time Chat Server
```javascript
// chat-server.js
const http = require('http');
const { EventEmitter } = require('events');

class ChatServer extends EventEmitter {
    constructor() {
        super();
        this.clients = new Map();
        this.messageHistory = [];
    }
    
    handleRequest(req, res) {
        if (req.method === 'GET' && req.url === '/') {
            this.serveHTML(res);
        } else if (req.method === 'POST' && req.url === '/send') {
            this.handleMessage(req, res);
        } else if (req.method === 'GET' && req.url === '/messages') {
            this.serveMessages(res);
        } else {
            res.writeHead(404);
            res.end('Not Found');
        }
    }
    
    serveHTML(res) {
        const html = `
            <!DOCTYPE html>
            <html>
                <head>
                    <title>JetCrab Chat</title>
                    <style>
                        body { font-family: Arial, sans-serif; margin: 20px; }
                        #messages { border: 1px solid #ccc; height: 300px; overflow-y: scroll; padding: 10px; }
                        #input { width: 70%; padding: 5px; }
                        #send { padding: 5px 10px; }
                    </style>
                </head>
                <body>
                    <h1>JetCrab Chat</h1>
                    <div id="messages"></div>
                    <input type="text" id="input" placeholder="Enter message...">
                    <button id="send">Send</button>
                    
                    <script>
                        const messagesDiv = document.getElementById('messages');
                        const input = document.getElementById('input');
                        const sendBtn = document.getElementById('send');
                        
                        function addMessage(message) {
                            const div = document.createElement('div');
                            div.textContent = message;
                            messagesDiv.appendChild(div);
                            messagesDiv.scrollTop = messagesDiv.scrollHeight;
                        }
                        
                        function sendMessage() {
                            const message = input.value.trim();
                            if (message) {
                                fetch('/send', {
                                    method: 'POST',
                                    headers: { 'Content-Type': 'application/json' },
                                    body: JSON.stringify({ message })
                                });
                                input.value = '';
                            }
                        }
                        
                        sendBtn.addEventListener('click', sendMessage);
                        input.addEventListener('keypress', (e) => {
                            if (e.key === 'Enter') sendMessage();
                        });
                        
                        // Poll for new messages
                        setInterval(() => {
                            fetch('/messages')
                                .then(res => res.json())
                                .then(data => {
                                    messagesDiv.innerHTML = '';
                                    data.forEach(msg => addMessage(msg));
                                });
                        }, 1000);
                    </script>
                </body>
            </html>
        `;
        
        res.writeHead(200, { 'Content-Type': 'text/html' });
        res.end(html);
    }
    
    handleMessage(req, res) {
        let body = '';
        req.on('data', chunk => {
            body += chunk.toString();
        });
        
        req.on('end', () => {
            try {
                const { message } = JSON.parse(body);
                const timestamp = new Date().toISOString();
                const fullMessage = `[${timestamp}] ${message}`;
                
                this.messageHistory.push(fullMessage);
                if (this.messageHistory.length > 100) {
                    this.messageHistory.shift();
                }
                
                res.writeHead(200, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ success: true }));
            } catch (error) {
                res.writeHead(400, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ error: 'Invalid JSON' }));
            }
        });
    }
    
    serveMessages(res) {
        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify(this.messageHistory));
    }
}

const server = http.createServer((req, res) => {
    const chatServer = new ChatServer();
    chatServer.handleRequest(req, res);
});

const PORT = process.env.PORT || 3000;
server.listen(PORT, () => {
    console.log(`Chat server running on http://localhost:${PORT}`);
});
```

### Database-like Storage
```javascript
// simple-db.js
const fs = require('fs');
const path = require('path');

class SimpleDB {
    constructor(dbPath) {
        this.dbPath = dbPath;
        this.data = new Map();
        this.load();
    }
    
    load() {
        try {
            if (fs.existsSync(this.dbPath)) {
                const content = fs.readFileSync(this.dbPath, 'utf8');
                const entries = JSON.parse(content);
                this.data = new Map(entries);
            }
        } catch (error) {
            console.error('Error loading database:', error.message);
        }
    }
    
    save() {
        try {
            const entries = Array.from(this.data.entries());
            fs.writeFileSync(this.dbPath, JSON.stringify(entries, null, 2));
        } catch (error) {
            console.error('Error saving database:', error.message);
        }
    }
    
    set(key, value) {
        this.data.set(key, value);
        this.save();
    }
    
    get(key) {
        return this.data.get(key);
    }
    
    delete(key) {
        const result = this.data.delete(key);
        this.save();
        return result;
    }
    
    has(key) {
        return this.data.has(key);
    }
    
    keys() {
        return Array.from(this.data.keys());
    }
    
    values() {
        return Array.from(this.data.values());
    }
    
    entries() {
        return Array.from(this.data.entries());
    }
    
    clear() {
        this.data.clear();
        this.save();
    }
    
    size() {
        return this.data.size;
    }
}

// Usage example
const db = new SimpleDB('./data.json');

// Set some data
db.set('user:1', { name: 'John Doe', email: 'john@example.com' });
db.set('user:2', { name: 'Jane Smith', email: 'jane@example.com' });
db.set('config:theme', 'dark');
db.set('config:language', 'en');

// Get data
console.log('User 1:', db.get('user:1'));
console.log('Theme:', db.get('config:theme'));

// List all keys
console.log('All keys:', db.keys());

// Check if key exists
console.log('Has user:1?', db.has('user:1'));
console.log('Has user:3?', db.has('user:3'));

// Delete a key
db.delete('config:language');
console.log('Keys after deletion:', db.keys());

console.log('Database size:', db.size());
```

### Performance Monitor
```javascript
// performance-monitor.js
const { performance } = require('perf_hooks');
const fs = require('fs');

class PerformanceMonitor {
    constructor() {
        this.metrics = [];
        this.startTime = performance.now();
    }
    
    mark(name) {
        const timestamp = performance.now();
        this.metrics.push({
            name,
            timestamp,
            time: timestamp - this.startTime
        });
    }
    
    measure(name, startMark, endMark) {
        const start = this.metrics.find(m => m.name === startMark);
        const end = this.metrics.find(m => m.name === endMark);
        
        if (start && end) {
            const duration = end.timestamp - start.timestamp;
            this.metrics.push({
                name: `measure:${name}`,
                timestamp: end.timestamp,
                time: end.time,
                duration
            });
        }
    }
    
    getReport() {
        const totalTime = performance.now() - this.startTime;
        const measures = this.metrics.filter(m => m.name.startsWith('measure:'));
        
        return {
            totalTime,
            measures: measures.map(m => ({
                name: m.name.replace('measure:', ''),
                duration: m.duration
            })),
            allMetrics: this.metrics
        };
    }
    
    saveReport(filename) {
        const report = this.getReport();
        fs.writeFileSync(filename, JSON.stringify(report, null, 2));
    }
}

// Usage example
const monitor = new PerformanceMonitor();

// Mark start of operation
monitor.mark('start');

// Simulate some work
setTimeout(() => {
    monitor.mark('work1-start');
    
    // Simulate CPU-intensive work
    let sum = 0;
    for (let i = 0; i < 1000000; i++) {
        sum += i;
    }
    
    monitor.mark('work1-end');
    monitor.measure('cpu-work', 'work1-start', 'work1-end');
    
    // Simulate I/O work
    monitor.mark('io-start');
    setTimeout(() => {
        monitor.mark('io-end');
        monitor.measure('io-work', 'io-start', 'io-end');
        
        // Mark end and generate report
        monitor.mark('end');
        monitor.measure('total', 'start', 'end');
        
        console.log('Performance Report:');
        console.log(JSON.stringify(monitor.getReport(), null, 2));
        
        // Save report to file
        monitor.saveReport('performance-report.json');
    }, 100);
}, 50);
```

## Resources

- **[JetCrab Guide](jetcrab-guide.md)** - Comprehensive usage guide
- **[CLI Reference](cli-reference.md)** - Command-line interface documentation
- **[API Reference](api-reference.md)** - Complete API documentation
- **[Contributing Guide](../contributing.md)** - How to contribute

---

**JetCrab Examples** - Practical examples and tutorials 🦀
