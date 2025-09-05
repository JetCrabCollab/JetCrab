// Net Module Test
console.log('Testing Net Module...');

// Test net.isIP functions
console.log('Testing IP validation:');
console.log('net.isIP("127.0.0.1"):', net.isIP("127.0.0.1"));
console.log('net.isIPv4("127.0.0.1"):', net.isIPv4("127.0.0.1"));
console.log('net.isIPv6("::1"):', net.isIPv6("::1"));
console.log('net.isIP("invalid"):', net.isIP("invalid"));

// Test creating a TCP server
console.log('\nTesting TCP Server:');
const server = net.createServer((socket) => {
    console.log('Client connected to server');

    socket.on('data', (data) => {
        console.log('Server received:', data);
        socket.write('Echo: ' + data);
    });

    socket.on('end', () => {
        console.log('Client disconnected from server');
    });
});

server.on('error', (err) => {
    console.error('Server error:', err);
});

// Test server methods
console.log('Server listening:', server.listening);
console.log('Server connections:', server.getConnections());

// Test creating a TCP client
console.log('\nTesting TCP Client:');
const client = net.createConnection({ port: 3000, host: 'localhost' }, () => {
    console.log('Client connected to server');
    client.write('Hello from client!');
});

client.on('data', (data) => {
    console.log('Client received:', data);
    client.end();
});

client.on('end', () => {
    console.log('Client disconnected');
});

client.on('error', (err) => {
    console.error('Client error:', err);
});

// Test socket methods
console.log('Client connecting:', client.connecting);
console.log('Client destroyed:', client.destroyed);
console.log('Client readable:', client.readable);
console.log('Client writable:', client.writable);

// Test socket address methods
console.log('Client local address:', client.localAddress());
console.log('Client local port:', client.localPort());
console.log('Client remote address:', client.remoteAddress());
console.log('Client remote port:', client.remotePort());
console.log('Client remote family:', client.remoteFamily());

// Test socket configuration
client.setKeepAlive(true, 1000);
client.setNoDelay(true);
client.setTimeout(5000);

console.log('\nNet module test completed successfully!');


