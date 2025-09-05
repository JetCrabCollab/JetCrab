// Simple Advanced APIs Demo for JetCrab

console.log("🚀 Starting JetCrab Simple Advanced APIs Demo...\n");

// --- Buffer API ---
console.log("--- Buffer API ---");
const buf1 = Buffer.alloc(10, 'a');
console.log("Buffer.alloc(10, 'a'):", buf1.toString());

const buf2 = Buffer.from('Hello JetCrab!', 'utf8');
console.log("Buffer.from('Hello JetCrab!'):", buf2.toString());

const buf3 = Buffer.from('48656c6c6f', 'hex');
console.log("Buffer.from('48656c6c6f', 'hex'):", buf3.toString());

const combined = Buffer.concat([buf1, buf2]);
console.log("Buffer.concat result:", combined.toString());

console.log("Buffer.isBuffer(buf1):", Buffer.isBuffer(buf1));
console.log("Buffer.byteLength('Hello'):", Buffer.byteLength('Hello'));

// --- Events API ---
console.log("\n--- Events API ---");
const emitter = new EventEmitter();

emitter.on('test', (data) => {
    console.log("Event received:", data);
});

emitter.on('error', (err) => {
    console.log("Error event:", err);
});

console.log("Emitting 'test' event...");
emitter.emit('test', 'Hello from EventEmitter!');

console.log("Listener count for 'test':", emitter.listenerCount('test'));
console.log("Event names:", emitter.eventNames());

// --- Stream API ---
console.log("\n--- Stream API ---");

// Create a readable stream
const readable = new stream.Readable();
readable.push('Hello ');
readable.push('Stream ');
readable.push('World!');
readable.push(null); // End of stream

readable.on('data', (chunk) => {
    console.log("Readable stream data:", chunk.toString());
});

readable.on('end', () => {
    console.log("Readable stream ended");
});

// Create a writable stream
const writable = new stream.Writable({
    write(chunk, encoding, callback) {
        console.log("Writable stream received:", chunk.toString());
        callback();
    }
});

writable.write('Hello Writable!');
writable.end();

// --- Util API ---
console.log("\n--- Util API ---");

const obj = {
    name: 'JetCrab',
    version: '0.4.0',
    features: ['Buffer', 'Events', 'Streams', 'Util'],
    nested: {
        engine: 'Boa',
        language: 'Rust'
    }
};

console.log("util.inspect(obj):");
console.log(util.inspect(obj, { depth: 2, colors: false }));

console.log("util.format('Hello %s, version %d', 'JetCrab', 1.0):");
console.log(util.format('Hello %s, version %d', 'JetCrab', 1.0));

console.log("util.isArray([1,2,3]):", util.isArray([1, 2, 3]));
console.log("util.isString('hello'):", util.isString('hello'));
console.log("util.isObject({}):", util.isObject({}));
console.log("util.isFunction(console.log):", util.isFunction(console.log));

// --- URL API ---
console.log("\n--- URL API ---");

const urlObj = new URL('https://example.com:8080/path?query=value#hash');
console.log("URL object:");
console.log("  href:", urlObj.href);
console.log("  protocol:", urlObj.protocol);
console.log("  hostname:", urlObj.hostname);
console.log("  port:", urlObj.port);
console.log("  pathname:", urlObj.pathname);
console.log("  search:", urlObj.search);
console.log("  hash:", urlObj.hash);

// URLSearchParams
const params = new URLSearchParams('name=JetCrab&version=0.4.0');
console.log("URLSearchParams:");
console.log("  get('name'):", params.get('name'));
console.log("  get('version'):", params.get('version'));
console.log("  toString():", params.toString());

// URL module functions
const parsed = url.parse('https://example.com/path?query=value');
console.log("url.parse result:");
console.log("  hostname:", parsed.hostname);
console.log("  pathname:", parsed.pathname);

const formatted = url.format({
    protocol: 'https:',
    hostname: 'example.com',
    pathname: '/test'
});
console.log("url.format result:", formatted);

const resolved = url.resolve('https://example.com/base/', 'path');
console.log("url.resolve result:", resolved);

console.log("\n✅ Simple Advanced APIs Demo completed! 🦀");


