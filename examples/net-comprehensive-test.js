// Comprehensive Net Module Test
console.log('=== Net Module Comprehensive Test ===');

// Test IP validation functions
console.log('IP Validation Tests:');
console.log('net.isIP("127.0.0.1"):', net.isIP("127.0.0.1"));
console.log('net.isIP("::1"):', net.isIP("::1"));
console.log('net.isIP("invalid"):', net.isIP("invalid"));
console.log('net.isIPv4("192.168.1.1"):', net.isIPv4("192.168.1.1"));
console.log('net.isIPv6("2001:db8::1"):', net.isIPv6("2001:db8::1"));

// Test server creation
console.log('Server Creation Tests:');
const server = net.createServer();
console.log('Server created:', typeof server);
console.log('Server listening:', server.listening);
console.log('Server connections:', server.getConnections());

// Test server methods
server.listen(3000);
console.log('Server listening after listen():', server.listening);
server.close();
console.log('Server listening after close():', server.listening);

// Test client creation
console.log('Client Creation Tests:');
const client = net.createConnection({ port: 3000, host: 'localhost' });
console.log('Client created:', typeof client);
console.log('Client connecting:', client.connecting);
console.log('Client destroyed:', client.destroyed);
console.log('Client readable:', client.readable);
console.log('Client writable:', client.writable);

// Test client methods
client.connect(3000, 'localhost');
console.log('Client connecting after connect():', client.connecting);
const writeResult = client.write('test data');
console.log('Client write result:', writeResult);
client.end();
console.log('Client destroyed after end():', client.destroyed);

// Test connect alias
console.log('Connect Alias Test:');
const client2 = net.connect({ port: 3000 });
console.log('Client2 created via connect():', typeof client2);

console.log('=== Net Module Test Completed Successfully ===');


