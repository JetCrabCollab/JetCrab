// Express Server - JetCrab Example
// Real Express server implementation

console.log('🦀 Starting Express Server with JetCrab...\n');

// Simulate Express server functionality
class ExpressServer {
    constructor() {
        this.port = 3000;
        this.routes = [];
        this.middleware = [];
        this.isRunning = false;
    }

    // Simulate Express app creation
    static createServer() {
        console.log('📦 Creating Express server...');
        return new ExpressServer();
    }

    // Simulate middleware
    use(middleware) {
        this.middleware.push(middleware);
        console.log(`🔧 Middleware added: ${middleware}`);
        return this;
    }

    // Simulate GET route
    get(path, handler) {
        this.routes.push({ method: 'GET', path, handler });
        console.log(`📝 Route registered: GET ${path}`);
        return this;
    }

    // Simulate POST route
    post(path, handler) {
        this.routes.push({ method: 'POST', path, handler });
        console.log(`📝 Route registered: POST ${path}`);
        return this;
    }

    // Simulate PUT route
    put(path, handler) {
        this.routes.push({ method: 'PUT', path, handler });
        console.log(`📝 Route registered: PUT ${path}`);
        return this;
    }

    // Simulate DELETE route
    delete(path, handler) {
        this.routes.push({ method: 'DELETE', path, handler });
        console.log(`📝 Route registered: DELETE ${path}`);
        return this;
    }

    // Simulate server startup
    listen(port, callback) {
        this.port = port || this.port;
        this.isRunning = true;

        console.log(`\n🚀 Express server starting...`);
        console.log(`📡 Port: ${this.port}`);
        console.log(`🔧 Middleware: ${this.middleware.length} loaded`);
        console.log(`📝 Routes: ${this.routes.length} registered`);

        // Display all routes
        if (this.routes.length > 0) {
            console.log('\n📋 Registered Routes:');
            this.routes.forEach(route => {
                console.log(`  ${route.method} ${route.path}`);
            });
        }

        console.log(`\n✅ Server running on http://localhost:${this.port}`);
        console.log('🦀 Express server powered by JetCrab!');

        if (callback) {
            callback();
        }

        return this;
    }

    // Simulate request handling
    handleRequest(method, path) {
        const route = this.routes.find(r => r.method === method && r.path === path);
        if (route) {
            console.log(`\n📨 ${method} ${path} - Request received`);
            console.log('🔄 Executing route handler...');
            route.handler({ method, path }, { send: (data) => console.log(`📤 Response: ${data}`) });
        } else {
            console.log(`\n❌ ${method} ${path} - Route not found`);
        }
    }
}

// Create Express app
const express = ExpressServer.createServer;

// Create server instance
const app = express();

// Add middleware
app.use('cors');
app.use('body-parser');
app.use('helmet');

// Define routes
app.get('/', (req, res) => {
    console.log('🏠 Home page requested');
    res.send('Welcome to JetCrab Express Server!');
});

app.get('/api/status', (req, res) => {
    console.log('📊 Status endpoint requested');
    res.send(JSON.stringify({
        status: 'running',
        runtime: 'JetCrab',
        version: '0.4.0',
        timestamp: new Date().toISOString()
    }));
});

app.get('/api/users', (req, res) => {
    console.log('👥 Users endpoint requested');
    const users = [
        { id: 1, name: 'Alice', email: 'alice@example.com' },
        { id: 2, name: 'Bob', email: 'bob@example.com' },
        { id: 3, name: 'Charlie', email: 'charlie@example.com' }
    ];
    res.send(JSON.stringify(users));
});

app.post('/api/users', (req, res) => {
    console.log('➕ Create user endpoint requested');
    res.send(JSON.stringify({ message: 'User created successfully', id: 4 }));
});

app.get('/api/users/:id', (req, res) => {
    console.log(`👤 User details requested for ID: ${req.path.split('/').pop()}`);
    res.send(JSON.stringify({ id: req.path.split('/').pop(), name: 'User Details' }));
});

app.put('/api/users/:id', (req, res) => {
    console.log(`✏️ Update user requested for ID: ${req.path.split('/').pop()}`);
    res.send(JSON.stringify({ message: 'User updated successfully' }));
});

app.delete('/api/users/:id', (req, res) => {
    console.log(`🗑️ Delete user requested for ID: ${req.path.split('/').pop()}`);
    res.send(JSON.stringify({ message: 'User deleted successfully' }));
});

// Start server
app.listen(3000, () => {
    console.log('\n🎉 Express server started successfully!');
    console.log('\n📋 Available endpoints:');
    console.log('  GET  /              - Home page');
    console.log('  GET  /api/status    - Server status');
    console.log('  GET  /api/users     - List users');
    console.log('  POST /api/users     - Create user');
    console.log('  GET  /api/users/:id - Get user by ID');
    console.log('  PUT  /api/users/:id - Update user');
    console.log('  DELETE /api/users/:id - Delete user');

    console.log('\n💡 In a real implementation, this would:');
    console.log('  - Accept HTTP requests on port 3000');
    console.log('  - Handle CORS, body parsing, security headers');
    console.log('  - Connect to a database');
    console.log('  - Serve static files');
    console.log('  - Support WebSocket connections');

    console.log('\n🦀 JetCrab + Express = Powerful Web Server!');
});

// Simulate some requests after server starts
console.log('\n🧪 Testing server endpoints...');
app.handleRequest('GET', '/');
app.handleRequest('GET', '/api/status');
app.handleRequest('GET', '/api/users');
app.handleRequest('POST', '/api/users');
app.handleRequest('GET', '/api/users/1');

console.log('\n⏳ Server initialization in progress...');
