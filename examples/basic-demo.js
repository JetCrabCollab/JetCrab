// JetCrab Basic API Demo

console.log("🦀 JetCrab Basic API Demo\n");

// File System Demo
console.log("📁 File System:");
console.log("Reading file:", fs.readFileSync('package.json'));

// Path Demo
console.log("\n📁 Path Operations:");
console.log("Join:", path.join('src', 'components', 'Button.js'));
console.log("Dirname:", path.dirname('/home/user/file.txt'));

// OS Demo
console.log("\n💻 System Info:");
console.log("Platform:", os.platform());
console.log("Arch:", os.arch());
console.log("Memory:", Math.round(os.totalmem() / 1024 / 1024 / 1024), "GB");

// Crypto Demo
console.log("\n🔐 Crypto:");
var hash = crypto.createHash('sha256');
hash.update('Hello JetCrab!');
console.log("Hash:", hash.digest('hex'));

console.log("\n✅ Demo completed! 🦀");
