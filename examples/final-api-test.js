// Final API Test for JetCrab

console.log("🚀 Final API Test for JetCrab...\n");

// Test Buffer API
console.log("--- Buffer API ---");
try {
    const buf = Buffer.alloc(5, 'a');
    console.log("Buffer.alloc(5, 'a'):", buf.toString());
    console.log("Buffer.isBuffer(buf):", Buffer.isBuffer(buf));
} catch (e) {
    console.log("Buffer API error:", e.message);
}

// Test EventEmitter
console.log("\n--- EventEmitter API ---");
try {
    const emitter = new EventEmitter();
    emitter.on('test', (data) => {
        console.log("Event received:", data);
    });
    emitter.emit('test', 'Hello EventEmitter!');
    console.log("EventEmitter working!");
} catch (e) {
    console.log("EventEmitter API error:", e.message);
}

// Test Stream API
console.log("\n--- Stream API ---");
try {
    const readable = new stream.Readable();
    readable.push('Hello Stream!');
    readable.push(null);
    console.log("Stream.Readable created!");
} catch (e) {
    console.log("Stream API error:", e.message);
}

// Test Util API
console.log("\n--- Util API ---");
try {
    const obj = { name: 'JetCrab', version: '0.4.0' };
    console.log("util.inspect(obj):", util.inspect(obj));
    console.log("util.isObject(obj):", util.isObject(obj));
} catch (e) {
    console.log("Util API error:", e.message);
}

// Test URL API
console.log("\n--- URL API ---");
try {
    const urlObj = new URL('https://example.com/path?query=value');
    console.log("URL.hostname:", urlObj.hostname);
    console.log("URL.pathname:", urlObj.pathname);
} catch (e) {
    console.log("URL API error:", e.message);
}

// Test QueryString API
console.log("\n--- QueryString API ---");
try {
    const parsed = querystring.parse('name=JetCrab&version=0.4.0');
    console.log("querystring.parse result:", parsed);
    const stringified = querystring.stringify({ name: 'JetCrab', version: '0.4.0' });
    console.log("querystring.stringify result:", stringified);
} catch (e) {
    console.log("QueryString API error:", e.message);
}

// Test Assert API
console.log("\n--- Assert API ---");
try {
    assert.ok(true, 'This should pass');
    assert.equal(1, 1, 'These should be equal');
    console.log("Assert API working!");
} catch (e) {
    console.log("Assert API error:", e.message);
}

console.log("\n✅ Final API test completed! 🦀");


