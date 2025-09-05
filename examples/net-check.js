// Check if Net module is available
console.log('Checking Net module availability...');

if (typeof net !== 'undefined') {
    console.log('Net module is available');
    console.log('net.isIP function:', typeof net.isIP);
    console.log('net.createServer function:', typeof net.createServer);
    console.log('net.createConnection function:', typeof net.createConnection);
} else {
    console.log('Net module is NOT available');
}

console.log('Net check completed');


