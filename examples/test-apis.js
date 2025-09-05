// Test all the new APIs implemented in JetCrab

console.log("🦀 Testing JetCrab APIs...\n");

// Test File System API
console.log("📁 Testing File System API:");
console.log("fs.readFileSync('test.txt'):", fs.readFileSync('test.txt'));
console.log("fs.existsSync('package.json'):", fs.existsSync('package.json'));
console.log("fs constants:", fs.F_OK, fs.R_OK, fs.W_OK);

// Test Path API
console.log("\n📁 Testing Path API:");
console.log("path.join('dir1', 'dir2', 'file.txt'):", path.join('dir1', 'dir2', 'file.txt'));
console.log("path.resolve('..', 'parent'):", path.resolve('..', 'parent'));
console.log("path.dirname('/home/user/file.txt'):", path.dirname('/home/user/file.txt'));
console.log("path.basename('/home/user/file.txt'):", path.basename('/home/user/file.txt'));
console.log("path.extname('file.txt'):", path.extname('file.txt'));
console.log("path.isAbsolute('/home/user'):", path.isAbsolute('/home/user'));
console.log("path.sep:", path.sep);

// Test Timers API
console.log("\n⏰ Testing Timers API:");
var timerId = setTimeout(function () {
    console.log("Timer executed!");
}, 100);
console.log("setTimeout returned ID:", timerId);

var intervalId = setInterval(function () {
    console.log("Interval executed!");
}, 200);
console.log("setInterval returned ID:", intervalId);

// Clear timers after a short delay
setTimeout(function () {
    clearTimeout(timerId);
    clearInterval(intervalId);
    console.log("Timers cleared");
}, 300);

// Test Crypto API
console.log("\n🔐 Testing Crypto API:");
var hash = crypto.createHash('sha256');
hash.update('hello world');
console.log("SHA256 hash:", hash.digest('hex'));

var randomBytes = crypto.randomBytes(16);
console.log("Random bytes (hex):", randomBytes);

var hmac = crypto.createHmac('sha256', 'secret');
hmac.update('hello world');
console.log("HMAC:", hmac.digest('hex'));

// Test OS API
console.log("\n💻 Testing OS API:");
console.log("os.platform():", os.platform());
console.log("os.arch():", os.arch());
console.log("os.type():", os.type());
console.log("os.release():", os.release());
console.log("os.uptime():", os.uptime());
console.log("os.totalmem():", os.totalmem());
console.log("os.freemem():", os.freemem());
console.log("os.homedir():", os.homedir());
console.log("os.tmpdir():", os.tmpdir());
console.log("os.hostname():", os.hostname());
console.log("os.EOL:", JSON.stringify(os.EOL));

var cpus = os.cpus();
console.log("os.cpus().length:", cpus.length);
if (cpus.length > 0) {
    console.log("First CPU model:", cpus[0].model);
    console.log("First CPU speed:", cpus[0].speed);
}

var userInfo = os.userInfo();
console.log("os.userInfo():", userInfo);

console.log("\n✅ All API tests completed!");
