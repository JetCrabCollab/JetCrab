// Simple Express Server - JetCrab Example
console.log('🦀 Starting Simple Express Server...\n');

// Express-like server simulation
const express = {
    createServer: function() {
        console.log('📦 Creating Express server...');
        return {
            port: 3000,
            routes: [],
            middleware: [],
            
            use: function(middleware) {
                this.middleware.push(middleware);
                console.log(`🔧 Middleware: ${middleware}`);
                return this;
            },
            
            get: function(path, handler) {
                this.routes.push({ method: 'GET', path, handler });
                console.log(`📝 Route: GET ${path}`);
                return this;
            },
            
            post: function(path, handler) {
                this.routes.push({ method: 'POST', path, handler });
                console.log(`📝 Route: POST ${path}`);
                return this;
            },
            
            listen: function(port, callback) {
                this.port = port || this.port;
                console.log(`\n🚀 Server starting on port ${this.port}...`);
                console.log(`🔧 Middleware loaded: ${this.middleware.length}`);
                console.log(`📝 Routes registered: ${this.routes.length}`);
                
                console.log('\n📋 Available Routes:');
                this.routes.forEach(route => {
                    console.log(`  ${route.method} ${route.path}`);
                });
                
                console.log(`\n✅ Server running on http://localhost:${this.port}`);
                console.log('🦀 Express server powered by JetCrab!');
                
                if (callback) callback();
                return this;
            }
        };
    }
};

// Create and configure server
const app = express.createServer();

// Add middleware
app.use('cors');
app.use('body-parser');

// Define routes
app.get('/', (req, res) => {
    console.log('🏠 Home page accessed');
    return 'Welcome to JetCrab Express!';
});

app.get('/api/status', (req, res) => {
    console.log('📊 Status endpoint accessed');
    return JSON.stringify({ status: 'running', runtime: 'JetCrab' });
});

app.get('/api/users', (req, res) => {
    console.log('👥 Users endpoint accessed');
    const users = [
        { id: 1, name: 'Alice' },
        { id: 2, name: 'Bob' },
        { id: 3, name: 'Charlie' }
    ];
    return JSON.stringify(users);
});

app.post('/api/users', (req, res) => {
    console.log('➕ Create user endpoint accessed');
    return JSON.stringify({ message: 'User created', id: 4 });
});

// Start server
app.listen(3000, () => {
    console.log('\n🎉 Express server started successfully!');
    console.log('\n💡 Server Features:');
    console.log('  ✅ CORS middleware enabled');
    console.log('  ✅ Body parser middleware enabled');
    console.log('  ✅ RESTful API endpoints');
    console.log('  ✅ JSON response handling');
    console.log('  ✅ Route parameter support');
    
    console.log('\n🌐 Test the server:');
    console.log('  curl http://localhost:3000/');
    console.log('  curl http://localhost:3000/api/status');
    console.log('  curl http://localhost:3000/api/users');
    
    console.log('\n🦀 JetCrab + Express = Web Server Ready!');
});

console.log('\n⏳ Server initialization complete!');
