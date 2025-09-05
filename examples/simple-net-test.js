// Simple Net Module Test
console.log('Testing Net Module...');

// Test basic net functions
console.log('net.isIP("127.0.0.1"):', net.isIP("127.0.0.1"));
console.log('net.isIPv4("127.0.0.1"):', net.isIPv4("127.0.0.1"));
console.log('net.isIPv6("::1"):', net.isIPv6("::1"));

// Test creating a server
console.log('Creating TCP server...');
const server = net.createServer();
console.log('Server created successfully');

// Test server properties
console.log('Server listening:', server.listening);
console.log('Server connections:', server.getConnections());

// Test creating a client
console.log('Creating TCP client...');
const client = net.createConnection({ port: 3000 });
console.log('Client created successfully');

// Test client properties
console.log('Client connecting:', client.connecting);
console.log('Client destroyed:', client.destroyed);

console.log('Net module test completed!');


