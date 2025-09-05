// Buffer API Test for JetCrab

console.log("📦 Testing Buffer API...\n");

// Test Buffer.alloc
const buf1 = Buffer.alloc(10, 'a');
console.log("Buffer.alloc(10, 'a'):", buf1.toString());

// Test Buffer.from with string
const buf2 = Buffer.from('Hello JetCrab!', 'utf8');
console.log("Buffer.from('Hello JetCrab!'):", buf2.toString());

// Test Buffer.from with hex
const buf3 = Buffer.from('48656c6c6f', 'hex');
console.log("Buffer.from('48656c6c6f', 'hex'):", buf3.toString());

// Test Buffer.concat
const combined = Buffer.concat([buf1, buf2]);
console.log("Buffer.concat result:", combined.toString());

// Test Buffer.isBuffer
console.log("Buffer.isBuffer(buf1):", Buffer.isBuffer(buf1));

// Test Buffer.byteLength
console.log("Buffer.byteLength('Hello'):", Buffer.byteLength('Hello'));

console.log("\n✅ Buffer API test completed!");


