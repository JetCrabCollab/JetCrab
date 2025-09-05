// JetCrab API Demo - Comprehensive example

console.log("🦀 JetCrab API Demo\n");

// File System Demo
console.log("📁 File System Operations:");
console.log("Reading file:", fs.readFileSync('package.json'));
console.log("File exists:", fs.existsSync('README.md'));
console.log("Directory listing:", fs.readdirSync('.'));

// Path Operations Demo
console.log("\n📁 Path Operations:");
console.log("Join paths:", path.join('src', 'components', 'Button.js'));
console.log("Resolve path:", path.resolve('..', 'parent-dir'));
console.log("Directory name:", path.dirname('/home/user/documents/file.txt'));
console.log("Base name:", path.basename('/home/user/documents/file.txt'));
console.log("Extension:", path.extname('document.pdf'));
console.log("Is absolute:", path.isAbsolute('/home/user'));

// OS Information Demo
console.log("\n💻 System Information:");
console.log("Platform:", os.platform());
console.log("Architecture:", os.arch());
console.log("OS Type:", os.type());
console.log("Release:", os.release());
console.log("Uptime:", os.uptime(), "seconds");
console.log("Total Memory:", Math.round(os.totalmem() / 1024 / 1024 / 1024), "GB");
console.log("Free Memory:", Math.round(os.freemem() / 1024 / 1024 / 1024), "GB");
console.log("Home Directory:", os.homedir());
console.log("Temp Directory:", os.tmpdir());
console.log("Hostname:", os.hostname());

// CPU Information
var cpus = os.cpus();
console.log("CPU Count:", cpus.length);
if (cpus.length > 0) {
    console.log("CPU Model:", cpus[0].model);
    console.log("CPU Speed:", cpus[0].speed, "MHz");
}

// User Information
var userInfo = os.userInfo();
console.log("Username:", userInfo.username);
console.log("User ID:", userInfo.uid);
console.log("Group ID:", userInfo.gid);
console.log("Shell:", userInfo.shell);

// Crypto Demo
console.log("\n🔐 Cryptographic Operations:");
var hash = crypto.createHash('sha256');
hash.update('Hello, JetCrab!');
console.log("SHA256 hash:", hash.digest('hex'));

var randomBytes = crypto.randomBytes(8);
console.log("Random bytes:", randomBytes);

var hmac = crypto.createHmac('sha256', 'secret-key');
hmac.update('Hello, World!');
console.log("HMAC:", hmac.digest('hex'));

// Timers Demo
console.log("\n⏰ Timer Operations:");
var timerId = setTimeout(function () {
    console.log("Timer executed after 100ms");
}, 100);

var intervalId = setInterval(function () {
    console.log("Interval tick");
}, 200);

// Clear timers after 500ms
setTimeout(function () {
    clearTimeout(timerId);
    clearInterval(intervalId);
    console.log("Timers cleared");
}, 500);

console.log("\n✅ JetCrab API Demo completed!");
console.log("All Node.js-like APIs are working in JetCrab! 🦀");
