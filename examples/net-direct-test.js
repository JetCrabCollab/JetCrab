// Test Net module directly
if (typeof net !== 'undefined') {
    console.log('Net module is available');
    console.log('Testing net.isIP("127.0.0.1"):', net.isIP("127.0.0.1"));
} else {
    console.log('Net module is NOT available');
}


