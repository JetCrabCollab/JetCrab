// Simple API test for JetCrab

console.log("🦀 Testing JetCrab APIs...\n");

// Test File System API
console.log("📁 File System API:");
console.log("fs.readFileSync result:", fs.readFileSync('test.txt'));

// Test Path API
console.log("\n📁 Path API:");
console.log("path.join result:", path.join('dir1', 'dir2', 'file.txt'));

// Test OS API
console.log("\n💻 OS API:");
console.log("os.platform():", os.platform());
console.log("os.arch():", os.arch());

console.log("\n✅ Basic API tests completed!");
