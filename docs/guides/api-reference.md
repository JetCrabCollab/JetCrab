# JetCrab API Reference

This document provides a comprehensive reference for all JetCrab APIs.

## Overview

JetCrab provides a comprehensive set of APIs that are compatible with Node.js and modern JavaScript environments. These APIs are implemented in Rust for optimal performance and safety.

## Core APIs

### Console API

The Console API provides methods for logging and debugging.

#### `console.log(...args)`
Logs messages to the console.

```javascript
console.log('Hello, World!');
console.log('User:', user, 'Age:', age);
console.log('Object:', { name: 'John', age: 30 });
```

#### `console.error(...args)`
Logs error messages to the console.

```javascript
console.error('Error occurred:', error);
console.error('Failed to load:', filename);
```

#### `console.warn(...args)`
Logs warning messages to the console.

```javascript
console.warn('Deprecated API used');
console.warn('Memory usage high:', memoryUsage);
```

#### `console.info(...args)`
Logs informational messages to the console.

```javascript
console.info('Application started');
console.info('Configuration loaded');
```

#### `console.debug(...args)`
Logs debug messages to the console.

```javascript
console.debug('Debug info:', debugData);
console.debug('Variable value:', variable);
```

### Process API

The Process API provides information about the current process and environment.

#### `process.version`
Returns the Node.js version string.

```javascript
console.log('Node.js version:', process.version);
// Output: v18.0.0
```

#### `process.platform`
Returns the operating system platform.

```javascript
console.log('Platform:', process.platform);
// Output: linux, darwin, win32, etc.
```

#### `process.arch`
Returns the CPU architecture.

```javascript
console.log('Architecture:', process.arch);
// Output: x64, arm64, etc.
```

#### `process.argv`
Returns command line arguments.

```javascript
console.log('Arguments:', process.argv);
// Output: ['node', 'script.js', 'arg1', 'arg2']
```

#### `process.env`
Returns environment variables.

```javascript
console.log('NODE_ENV:', process.env.NODE_ENV);
console.log('PATH:', process.env.PATH);
```

#### `process.cwd()`
Returns the current working directory.

```javascript
console.log('Current directory:', process.cwd());
```

#### `process.exit([code])`
Exits the process with the specified code.

```javascript
process.exit(0);  // Success
process.exit(1);  // Error
```

#### `process.nextTick(callback)`
Schedules a callback to be called on the next tick of the event loop.

```javascript
process.nextTick(() => {
    console.log('This runs on the next tick');
});
```

## File System API

The File System API provides methods for working with files and directories.

### Synchronous Methods

#### `fs.readFileSync(path, [encoding])`
Reads a file synchronously.

```javascript
const fs = require('fs');

// Read as buffer
const data = fs.readFileSync('file.txt');

// Read as string
const text = fs.readFileSync('file.txt', 'utf8');
```

#### `fs.writeFileSync(path, data, [encoding])`
Writes data to a file synchronously.

```javascript
const fs = require('fs');

// Write string
fs.writeFileSync('output.txt', 'Hello, World!');

// Write buffer
fs.writeFileSync('data.bin', Buffer.from([1, 2, 3, 4]));
```

#### `fs.existsSync(path)`
Checks if a file or directory exists.

```javascript
const fs = require('fs');

if (fs.existsSync('file.txt')) {
    console.log('File exists');
}
```

#### `fs.statSync(path)`
Returns file or directory statistics.

```javascript
const fs = require('fs');

const stats = fs.statSync('file.txt');
console.log('Size:', stats.size);
console.log('Is file:', stats.isFile());
console.log('Is directory:', stats.isDirectory());
```

#### `fs.mkdirSync(path, [options])`
Creates a directory synchronously.

```javascript
const fs = require('fs');

// Create directory
fs.mkdirSync('new-directory');

// Create directory with permissions
fs.mkdirSync('new-directory', { mode: 0o755 });
```

#### `fs.rmdirSync(path)`
Removes a directory synchronously.

```javascript
const fs = require('fs');

fs.rmdirSync('empty-directory');
```

#### `fs.readdirSync(path)`
Reads directory contents synchronously.

```javascript
const fs = require('fs');

const files = fs.readdirSync('.');
console.log('Files:', files);
```

### Asynchronous Methods

#### `fs.readFile(path, [encoding], callback)`
Reads a file asynchronously.

```javascript
const fs = require('fs');

fs.readFile('file.txt', 'utf8', (err, data) => {
    if (err) {
        console.error('Error:', err);
        return;
    }
    console.log('File content:', data);
});
```

#### `fs.writeFile(path, data, [encoding], callback)`
Writes data to a file asynchronously.

```javascript
const fs = require('fs');

fs.writeFile('output.txt', 'Hello, World!', (err) => {
    if (err) {
        console.error('Error:', err);
        return;
    }
    console.log('File written successfully');
});
```

### Promise-based Methods

#### `fs.promises.readFile(path, [encoding])`
Reads a file using promises.

```javascript
const fs = require('fs');

async function readFile() {
    try {
        const data = await fs.promises.readFile('file.txt', 'utf8');
        console.log('File content:', data);
    } catch (err) {
        console.error('Error:', err);
    }
}
```

#### `fs.promises.writeFile(path, data, [encoding])`
Writes data to a file using promises.

```javascript
const fs = require('fs');

async function writeFile() {
    try {
        await fs.promises.writeFile('output.txt', 'Hello, World!');
        console.log('File written successfully');
    } catch (err) {
        console.error('Error:', err);
    }
}
```

## HTTP API

The HTTP API provides methods for creating HTTP servers and making HTTP requests.

### HTTP Server

#### `http.createServer([options], [requestListener])`
Creates an HTTP server.

```javascript
const http = require('http');

const server = http.createServer((req, res) => {
    res.writeHead(200, { 'Content-Type': 'text/plain' });
    res.end('Hello, World!');
});

server.listen(3000, () => {
    console.log('Server running on port 3000');
});
```

#### `server.listen(port, [host], [callback])`
Starts the server listening on the specified port.

```javascript
server.listen(3000, 'localhost', () => {
    console.log('Server started');
});
```

#### `server.close([callback])`
Stops the server from accepting new connections.

```javascript
server.close(() => {
    console.log('Server closed');
});
```

### HTTP Client

#### `http.get(url, [options], [callback])`
Makes an HTTP GET request.

```javascript
const http = require('http');

http.get('http://example.com', (res) => {
    let data = '';
    
    res.on('data', (chunk) => {
        data += chunk;
    });
    
    res.on('end', () => {
        console.log('Response:', data);
    });
});
```

#### `http.request(options, [callback])`
Makes an HTTP request.

```javascript
const http = require('http');

const options = {
    hostname: 'example.com',
    port: 80,
    path: '/api/data',
    method: 'POST',
    headers: {
        'Content-Type': 'application/json'
    }
};

const req = http.request(options, (res) => {
    console.log('Status:', res.statusCode);
    console.log('Headers:', res.headers);
});

req.write(JSON.stringify({ key: 'value' }));
req.end();
```

## Fetch API

The Fetch API provides a modern way to make HTTP requests.

#### `fetch(url, [options])`
Makes an HTTP request and returns a Promise.

```javascript
// Simple GET request
fetch('https://api.github.com/users/octocat')
    .then(response => response.json())
    .then(data => console.log(data))
    .catch(error => console.error('Error:', error));

// POST request with data
fetch('https://api.example.com/data', {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json'
    },
    body: JSON.stringify({ key: 'value' })
})
.then(response => response.json())
.then(data => console.log(data));
```

#### `Response` Object
The response object provides methods for handling the response.

```javascript
fetch('https://api.example.com/data')
    .then(response => {
        console.log('Status:', response.status);
        console.log('Headers:', response.headers);
        
        if (response.ok) {
            return response.json();
        } else {
            throw new Error('HTTP error: ' + response.status);
        }
    })
    .then(data => console.log(data))
    .catch(error => console.error('Error:', error));
```

## Timer APIs

The Timer APIs provide methods for scheduling code execution.

#### `setTimeout(callback, delay, [...args])`
Schedules a callback to be called after a delay.

```javascript
setTimeout(() => {
    console.log('This runs after 1 second');
}, 1000);

// With arguments
setTimeout((message) => {
    console.log(message);
}, 1000, 'Hello, World!');
```

#### `setInterval(callback, delay, [...args])`
Schedules a callback to be called repeatedly.

```javascript
const interval = setInterval(() => {
    console.log('This runs every 2 seconds');
}, 2000);

// Clear after 10 seconds
setTimeout(() => {
    clearInterval(interval);
}, 10000);
```

#### `clearTimeout(timeoutId)`
Cancels a timeout.

```javascript
const timeoutId = setTimeout(() => {
    console.log('This will not run');
}, 1000);

clearTimeout(timeoutId);
```

#### `clearInterval(intervalId)`
Cancels an interval.

```javascript
const intervalId = setInterval(() => {
    console.log('This will not run');
}, 1000);

clearInterval(intervalId);
```

#### `setImmediate(callback, [...args])`
Schedules a callback to be called on the next tick.

```javascript
setImmediate(() => {
    console.log('This runs on the next tick');
});
```

## Event API

The Event API provides methods for handling events.

#### `EventEmitter`
Base class for objects that emit events.

```javascript
const { EventEmitter } = require('events');

class MyEmitter extends EventEmitter {}

const myEmitter = new MyEmitter();

// Listen for events
myEmitter.on('event', (data) => {
    console.log('Event received:', data);
});

// Emit events
myEmitter.emit('event', { message: 'Hello, World!' });
```

#### `emitter.on(eventName, listener)`
Adds a listener for the specified event.

```javascript
emitter.on('data', (chunk) => {
    console.log('Data received:', chunk);
});
```

#### `emitter.emit(eventName, [...args])`
Emits an event with the specified arguments.

```javascript
emitter.emit('data', 'Hello, World!');
```

#### `emitter.removeListener(eventName, listener)`
Removes a specific listener.

```javascript
emitter.removeListener('data', dataHandler);
```

#### `emitter.removeAllListeners([eventName])`
Removes all listeners for an event or all events.

```javascript
emitter.removeAllListeners('data');
emitter.removeAllListeners();
```

## Buffer API

The Buffer API provides methods for working with binary data.

#### `Buffer.from(data, [encoding])`
Creates a new Buffer from data.

```javascript
const buf1 = Buffer.from('Hello, World!', 'utf8');
const buf2 = Buffer.from([1, 2, 3, 4]);
const buf3 = Buffer.from('Hello', 'base64');
```

#### `Buffer.alloc(size, [fill], [encoding])`
Allocates a new Buffer of the specified size.

```javascript
const buf = Buffer.alloc(10);
const buf2 = Buffer.alloc(10, 'a');
```

#### `Buffer.concat(list, [totalLength])`
Concatenates an array of Buffer instances.

```javascript
const buf1 = Buffer.from('Hello');
const buf2 = Buffer.from('World');
const buf3 = Buffer.concat([buf1, buf2]);
```

#### `buffer.toString([encoding], [start], [end])`
Converts Buffer to string.

```javascript
const buf = Buffer.from('Hello, World!');
console.log(buf.toString('utf8'));
console.log(buf.toString('base64'));
```

#### `buffer.length`
Returns the length of the Buffer.

```javascript
const buf = Buffer.from('Hello, World!');
console.log('Length:', buf.length);
```

## Path API

The Path API provides methods for working with file paths.

#### `path.join(...paths)`
Joins path segments using the platform-specific separator.

```javascript
const path = require('path');

const fullPath = path.join('/home', 'user', 'documents', 'file.txt');
console.log(fullPath); // /home/user/documents/file.txt
```

#### `path.resolve(...paths)`
Resolves an absolute path.

```javascript
const path = require('path');

const absolutePath = path.resolve('file.txt');
console.log(absolutePath); // /current/working/directory/file.txt
```

#### `path.dirname(path)`
Returns the directory name of a path.

```javascript
const path = require('path');

const dir = path.dirname('/home/user/file.txt');
console.log(dir); // /home/user
```

#### `path.basename(path, [ext])`
Returns the last portion of a path.

```javascript
const path = require('path');

const filename = path.basename('/home/user/file.txt');
console.log(filename); // file.txt

const nameWithoutExt = path.basename('/home/user/file.txt', '.txt');
console.log(nameWithoutExt); // file
```

#### `path.extname(path)`
Returns the extension of a path.

```javascript
const path = require('path');

const ext = path.extname('/home/user/file.txt');
console.log(ext); // .txt
```

## URL API

The URL API provides methods for working with URLs.

#### `new URL(input, [base])`
Creates a new URL object.

```javascript
const url = new URL('https://example.com/path?query=value');
console.log(url.href); // https://example.com/path?query=value
console.log(url.protocol); // https:
console.log(url.hostname); // example.com
console.log(url.pathname); // /path
console.log(url.search); // ?query=value
```

#### `url.toString()`
Returns the string representation of the URL.

```javascript
const url = new URL('https://example.com/path');
console.log(url.toString()); // https://example.com/path
```

## Crypto API

The Crypto API provides cryptographic functionality.

#### `crypto.createHash(algorithm)`
Creates a hash object.

```javascript
const crypto = require('crypto');

const hash = crypto.createHash('sha256');
hash.update('Hello, World!');
const digest = hash.digest('hex');
console.log(digest);
```

#### `crypto.createHmac(algorithm, key)`
Creates an HMAC object.

```javascript
const crypto = require('crypto');

const hmac = crypto.createHmac('sha256', 'secret-key');
hmac.update('Hello, World!');
const digest = hmac.digest('hex');
console.log(digest);
```

#### `crypto.randomBytes(size)`
Generates cryptographically strong random bytes.

```javascript
const crypto = require('crypto');

const randomBytes = crypto.randomBytes(16);
console.log(randomBytes.toString('hex'));
```

## Performance Hooks API

The Performance Hooks API provides methods for measuring performance.

#### `performance.now()`
Returns the current timestamp in milliseconds.

```javascript
const { performance } = require('perf_hooks');

const start = performance.now();
// ... some operation ...
const end = performance.now();
console.log(`Operation took ${end - start} milliseconds`);
```

#### `performance.mark(name)`
Creates a performance mark.

```javascript
const { performance } = require('perf_hooks');

performance.mark('start');
// ... some operation ...
performance.mark('end');
```

#### `performance.measure(name, startMark, endMark)`
Measures the time between two marks.

```javascript
const { performance } = require('perf_hooks');

performance.mark('start');
// ... some operation ...
performance.mark('end');
performance.measure('operation', 'start', 'end');
```

## Worker Threads API

The Worker Threads API provides methods for creating and managing worker threads.

#### `new Worker(filename, [options])`
Creates a new worker thread.

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

#### `worker.postMessage(value)`
Sends a message to the worker thread.

```javascript
worker.postMessage({ command: 'start', data: 'Hello' });
```

#### `worker.on('message', callback)`
Listens for messages from the worker thread.

```javascript
worker.on('message', (data) => {
    console.log('Received:', data);
});
```

## Examples

### Complete HTTP Server
```javascript
const http = require('http');
const fs = require('fs');
const path = require('path');

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

## Resources

- **[JetCrab Guide](jetcrab-guide.md)** - Comprehensive usage guide
- **[CLI Reference](cli-reference.md)** - Command-line interface documentation
- **[Examples](examples.md)** - Code examples and tutorials
- **[Contributing Guide](../contributing.md)** - How to contribute

---

**JetCrab API Reference** - Complete API documentation 🦀
