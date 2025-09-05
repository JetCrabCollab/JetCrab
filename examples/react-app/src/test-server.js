console.log('🦀 Express Server Test');
console.log('📦 Creating server...');

const server = {
    port: 3000,
    routes: [],
    
    get: function(path) {
        this.routes.push('GET ' + path);
        console.log('📝 Route: GET ' + path);
        return this;
    },
    
    post: function(path) {
        this.routes.push('POST ' + path);
        console.log('📝 Route: POST ' + path);
        return this;
    },
    
    listen: function(port) {
        console.log('🚀 Server starting on port ' + port);
        console.log('📋 Routes: ' + this.routes.length);
        console.log('✅ Server running!');
        return this;
    }
};

server.get('/');
server.get('/api/users');
server.post('/api/users');
server.listen(3000);

console.log('🎉 Express server ready!');
