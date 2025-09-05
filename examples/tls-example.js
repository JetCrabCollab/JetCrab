// TLS Example - Secure Connections
// This example demonstrates how to use the TLS module for secure communication

console.log('🔒 Starting TLS Example...');

// Test TLS Server
console.log('🖥️ Creating TLS Server...');
const tlsServer = tls.createServer({
    key: 'server-key.pem',
    cert: 'server-cert.pem',
    ca: 'ca-cert.pem',
    requestCert: true,
    rejectUnauthorized: true
}, (socket) => {
    console.log('🔗 New TLS connection established');

    socket.on('data', (data) => {
        console.log('📥 Server received data:', data.toString());

        // Echo back the data
        socket.write('Echo: ' + data.toString());
    });

    socket.on('end', () => {
        console.log('🔌 TLS connection ended');
    });

    socket.on('error', (error) => {
        console.error('❌ TLS Server error:', error);
    });

    // Send welcome message
    socket.write('Welcome to TLS Server!\n');
});

// Start the server
tlsServer.listen(8443, '127.0.0.1', () => {
    console.log('🚀 TLS Server listening on 127.0.0.1:8443');
});

// Test TLS Client
console.log('👤 Creating TLS Client...');
const tlsClient = tls.connect({
    port: 8443,
    host: '127.0.0.1',
    key: 'client-key.pem',
    cert: 'client-cert.pem',
    ca: 'ca-cert.pem',
    rejectUnauthorized: true
}, () => {
    console.log('🔗 TLS Client connected');

    // Send some data
    tlsClient.write('Hello from TLS Client!');
});

tlsClient.on('data', (data) => {
    console.log('📥 Client received data:', data.toString());
});

tlsClient.on('end', () => {
    console.log('🔌 TLS Client connection ended');
});

tlsClient.on('error', (error) => {
    console.error('❌ TLS Client error:', error);
});

// Test TLS Secure Context
console.log('🔐 Creating TLS Secure Context...');
const secureContext = tls.createSecureContext({
    key: 'server-key.pem',
    cert: 'server-cert.pem',
    ca: 'ca-cert.pem',
    ciphers: 'TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256',
    honorCipherOrder: true,
    minVersion: 'TLSv1.2',
    maxVersion: 'TLSv1.3'
});

console.log('🔒 Secure Context created');
console.log('🔐 Cipher:', secureContext.getCipher());
console.log('📋 Protocol:', secureContext.getProtocol());

// Test TLS utilities
console.log('🛠️ Testing TLS utilities...');

// Get available ciphers
const ciphers = tls.getCiphers();
console.log('🔐 Available ciphers:', ciphers.slice(0, 5), '...');

// Get available curves
const curves = tls.getCurves();
console.log('📈 Available curves:', curves);

// Get available hashes
const hashes = tls.getHashes();
console.log('🔢 Available hashes:', hashes);

// Get available protocols
const protocols = tls.getProtocols();
console.log('📋 Available protocols:', protocols);

// Test certificate parsing
const certString = `-----BEGIN CERTIFICATE-----
MIICljCCAX4CCQDQ5Y7Z8Z8Z8TANBgkqhkiG9w0BAQsFADBAMQswCQYDVQQGEwJV
UzELMAkGA1UECAwCQ0ExFjAUBgNVBAcMDVNhbiBGcmFuY2lzY28xCzAJBgNVBAoM
AkNBMB4XDTIzMDEwMTAwMDAwMFoXDTI0MDEwMTAwMDAwMFowQDELMAkGA1UECAwC
Q0ExFjAUBgNVBAcMDVNhbiBGcmFuY2lzY28xCzAJBgNVBAoMAkNBMA0GCSqGSIb3
DQEBCwUAA0EAQK8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8
Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8
Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8Z8
-----END CERTIFICATE-----`;

const parsedCert = tls.parseCertString(certString);
console.log('📜 Parsed certificate:', parsedCert);

// Test server identity checking
const serverIdentity = tls.checkServerIdentity('example.com', parsedCert);
console.log('🔍 Server identity check:', serverIdentity);

// Test ALPN protocol conversion
const alpnProtocols = ['http/1.1', 'h2'];
const convertedProtocols = tls.convertALPNProtocols(alpnProtocols);
console.log('🔄 Converted ALPN protocols:', convertedProtocols);

// Test TLS constants
console.log('📊 TLS Constants:');
console.log('🔐 DEFAULT_ECDH_CURVE:', tls.DEFAULT_ECDH_CURVE);
console.log('📋 DEFAULT_MAX_VERSION:', tls.DEFAULT_MAX_VERSION);
console.log('📋 DEFAULT_MIN_VERSION:', tls.DEFAULT_MIN_VERSION);
console.log('📦 SLAB_BUFFER_SIZE:', tls.SLAB_BUFFER_SIZE);

// Test SSL options
console.log('⚙️ SSL Options:');
console.log('🔒 SSL_OP_ALL:', tls.SSL_OP_ALL);
console.log('🔒 SSL_OP_NO_SSLv2:', tls.SSL_OP_NO_SSLv2);
console.log('🔒 SSL_OP_NO_SSLv3:', tls.SSL_OP_NO_SSLv3);
console.log('🔒 SSL_OP_NO_TLSv1:', tls.SSL_OP_NO_TLSv1);
console.log('🔒 SSL_OP_NO_TLSv1_1:', tls.SSL_OP_NO_TLSv1_1);
console.log('🔒 SSL_OP_NO_TLSv1_2:', tls.SSL_OP_NO_TLSv1_2);
console.log('🔒 SSL_OP_NO_TLSv1_3:', tls.SSL_OP_NO_TLSv1_3);

// Simulate some work
setTimeout(() => {
    console.log('⏰ TLS example work completed');

    // Close connections
    tlsClient.end();
    tlsServer.close();

    console.log('✅ TLS example completed');
}, 3000);

console.log('✅ TLS example setup completed');



