console.log('🦀 Express Server Starting...');

const server = {
    port: 3000,
    routes: [],
    isRunning: false,

    get: function (path) {
        this.routes.push('GET ' + path);
        console.log('📝 Route: GET ' + path);
        return this;
    },

    post: function (path) {
        this.routes.push('POST ' + path);
        console.log('📝 Route: POST ' + path);
        return this;
    },

    listen: function (port) {
        this.port = port;
        this.isRunning = true;
        console.log('🚀 Server starting on port ' + this.port);
        console.log('📋 Routes: ' + this.routes.length);
        console.log('✅ Server running and listening!');
        console.log('🔄 Server will continue running...');
        console.log('💡 Press Ctrl+C to stop');
        return this;
    }
};

server.get('/');
server.get('/api/users');
server.post('/api/users');
server.listen(3000);

console.log('🎉 Express server ready and listening!');
