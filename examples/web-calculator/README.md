# Web Calculator - JetCrab Example

A modern web calculator built with JetCrab runtime, demonstrating advanced features like HTTP server, WebSocket communication, and Rust/JavaScript integration.

## Features Demonstrated

- HTTP server with routing
- WebSocket real-time communication
- Rust/JavaScript integration via WebAssembly
- File system operations
- JSON API endpoints
- Real-time calculation history
- Modern web interface

## Getting Started

### 1. Initialize the Project

```bash
# Navigate to the example directory
cd examples/web-calculator

# Initialize a new JetCrab project
cpm init

# Install dependencies
cpm install
```

### 2. Build the WebAssembly Module

```bash
# Build the Rust library to WebAssembly
cpm build
```

### 3. Start the Server

```bash
# Start the HTTP server
jetcrab run server.js

# Or use development mode with hot reload
cpm dev
```

### 4. Access the Application

Open your browser and navigate to:
- **Main Application**: http://localhost:3000
- **API Documentation**: http://localhost:3000/api/docs
- **WebSocket Test**: http://localhost:3000/ws-test

## Project Structure

```
web-calculator/
├── README.md           # This file
├── package.json        # Project configuration
├── Cargo.toml         # Rust dependencies
├── server.js          # HTTP server and main application
├── public/            # Static web assets
│   ├── index.html     # Main web interface
│   ├── style.css      # Styling
│   └── app.js         # Client-side JavaScript
├── src/               # Rust source code
│   └── lib.rs         # Calculator engine
├── routes/            # API route handlers
│   ├── calculator.js  # Calculator API
│   └── websocket.js   # WebSocket handlers
└── tests/             # Test files
    ├── api.test.js    # API tests
    └── calculator.test.js
```

## API Endpoints

### Calculator API

- `POST /api/calculate` - Perform calculations
- `GET /api/history` - Get calculation history
- `DELETE /api/history` - Clear history

### WebSocket Events

- `calculation` - Send calculation request
- `result` - Receive calculation result
- `history` - Receive history updates

## Example Usage

### JavaScript API

```javascript
// Perform a calculation
const response = await fetch('/api/calculate', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ expression: '2 + 3 * 4' })
});
const result = await response.json();
console.log(result); // { result: 14, expression: '2 + 3 * 4' }
```

### WebSocket API

```javascript
const ws = new WebSocket('ws://localhost:3000/ws');
ws.onopen = () => {
    ws.send(JSON.stringify({
        type: 'calculation',
        expression: '10 / 2'
    }));
};
ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log('Result:', data.result);
};
```

## Advanced Features

1. **Real-time Updates**: WebSocket communication for live calculation updates
2. **History Management**: Persistent calculation history with JSON storage
3. **Error Handling**: Comprehensive error handling and validation
4. **Performance**: Rust-powered calculation engine for complex operations
5. **Modern UI**: Responsive web interface with real-time feedback

## Development

### Running Tests

```bash
# Run all tests
cpm test

# Run specific test suite
jetcrab run tests/api.test.js
```

### Building for Production

```bash
# Build optimized version
cpm build --release

# Start production server
NODE_ENV=production jetcrab run server.js
```

## Next Steps

- Add user authentication
- Implement calculation sharing
- Add more mathematical functions
- Create mobile app version
- Add calculation visualization

