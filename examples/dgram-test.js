// DGRAM Module Test
console.log('=== DGRAM Module Test ===');

// Test creating UDP socket
console.log('Creating UDP socket...');
const socket = dgram.createSocket('udp4');
console.log('Socket created:', typeof socket);
console.log('Socket type:', socket.type);
console.log('Socket bound:', socket.bound);
console.log('Socket closed:', socket.closed);

// Test socket methods
console.log('Testing socket methods...');
socket.bind(8080, 'localhost');
console.log('Socket bound after bind():', socket.bound);
console.log('Socket address:', socket.address());

// Test send method
const sendResult = socket.send('Hello UDP!', 8081, 'localhost');
console.log('Send result:', sendResult);

// Test socket configuration
socket.setBroadcast(true);
socket.setMulticastTTL(128);
socket.setMulticastLoopback(false);
socket.setRecvBufferSize(16384);
socket.setSendBufferSize(16384);

console.log('Receive buffer size:', socket.getRecvBufferSize());
console.log('Send buffer size:', socket.getSendBufferSize());

// Test multicast operations
socket.addMembership('224.0.0.1');
socket.dropMembership('224.0.0.1');

// Test socket4 and socket6 creation
console.log('Testing socket4 creation...');
const socket4 = dgram.createSocket4();
console.log('Socket4 type:', socket4.type);

console.log('Testing socket6 creation...');
const socket6 = dgram.createSocket6();
console.log('Socket6 type:', socket6.type);

// Test close
socket.close();
console.log('Socket closed after close():', socket.closed);

console.log('=== DGRAM Module Test Completed Successfully ===');


