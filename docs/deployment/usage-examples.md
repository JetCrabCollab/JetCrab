# JetCrab Server Usage Examples

## 🚀 **QUICK START**

### **1. Starting the Server**

```bash
# Local development
cargo run --bin jetcrab-server

# Docker
docker run -p 8080:8080 jetcrab:latest

# With custom config
docker run -p 8080:8080 \
  -e JETCRAB_BIND_ADDRESS=0.0.0.0:8080 \
  -e JETCRAB_ENGINE_POOL_SIZE=20 \
  -e JETCRAB_MAX_MEMORY_MB=1024 \
  jetcrab:latest
```

### **2. Health Check**

```bash
curl http://localhost:8080/health
```

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:30:00Z",
  "engine_pool_size": 10,
  "active_engines": 8,
  "memory_usage_mb": 256,
  "uptime_seconds": 3600
}
```

## 📝 **EXECUTANDO JAVASCRIPT**

### **1. Basic Script Execution**

```bash
curl -X POST http://localhost:8080/execute \
  -H "Content-Type: application/json" \
  -d '{
    "script": "let x = 10; let y = 20; x + y;",
    "timeout_ms": 5000,
    "memory_limit_mb": 128
  }'
```

**Response:**
```json
{
  "result": 30,
  "execution_time_ms": 15,
  "memory_used_mb": 2,
  "errors": []
}
```

### **2. Complex JavaScript with Context**

```bash
curl -X POST http://localhost:8080/execute \
  -H "Content-Type: application/json" \
  -d '{
    "script": "
      function fibonacci(n) {
        if (n <= 1) return n;
        return fibonacci(n-1) + fibonacci(n-2);
      }
      
      let result = [];
      for (let i = 0; i < 10; i++) {
        result.push(fibonacci(i));
      }
      result;
    ",
    "context": {
      "max_iterations": 1000
    },
    "timeout_ms": 10000
  }'
```

**Response:**
```json
{
  "result": [0, 1, 1, 2, 3, 5, 8, 13, 21, 34],
  "execution_time_ms": 45,
  "memory_used_mb": 8,
  "errors": []
}
```

### **3. Error Handling**

```bash
curl -X POST http://localhost:8080/execute \
  -H "Content-Type: application/json" \
  -d '{
    "script": "undefined.method();",
    "timeout_ms": 5000
  }'
```

**Response:**
```json
{
  "result": null,
  "execution_time_ms": 12,
  "memory_used_mb": 1,
  "errors": [
    "TypeError: Cannot read property 'method' of undefined"
  ]
}
```

## 🔧 **ADVANCED FEATURES**

### **1. Script Registry - Store and Execute**

```bash
# Store a script
curl -X POST http://localhost:8080/scripts \
  -H "Content-Type: application/json" \
  -d '{
    "id": "math-utils",
    "script": "
      function add(a, b) { return a + b; }
      function multiply(a, b) { return a * b; }
      function divide(a, b) { 
        if (b === 0) throw new Error('Division by zero');
        return a / b; 
      }
      
      module.exports = { add, multiply, divide };
    ",
    "description": "Basic math utilities"
  }'

# Execute stored script
curl -X POST http://localhost:8080/scripts/math-utils/execute \
  -H "Content-Type: application/json" \
  -d '{
    "context": {
      "a": 15,
      "b": 3
    }
  }'
```

### **2. Batch Execution**

```bash
curl -X POST http://localhost:8080/execute/batch \
  -H "Content-Type: application/json" \
  -d '{
    "scripts": [
      {
        "id": "script1",
        "script": "2 + 2",
        "timeout_ms": 1000
      },
      {
        "id": "script2", 
        "script": "5 * 5",
        "timeout_ms": 1000
      },
      {
        "id": "script3",
        "script": "Math.sqrt(16)",
        "timeout_ms": 1000
      }
    ]
  }'
```

**Response:**
```json
{
  "results": [
    {
      "id": "script1",
      "result": 4,
      "execution_time_ms": 8,
      "memory_used_mb": 1,
      "errors": []
    },
    {
      "id": "script2",
      "result": 25,
      "execution_time_ms": 7,
      "memory_used_mb": 1,
      "errors": []
    },
    {
      "id": "script3",
      "result": 4,
      "execution_time_ms": 9,
      "memory_used_mb": 1,
      "errors": []
    }
  ],
  "total_execution_time_ms": 24,
  "total_memory_used_mb": 3
}
```

### **3. Streaming Execution**

```bash
curl -X POST http://localhost:8080/execute/stream \
  -H "Content-Type: application/json" \
  -d '{
    "script": "
      for (let i = 0; i < 5; i++) {
        yield { iteration: i, value: i * i };
        // Simulate some work
        await new Promise(resolve => setTimeout(resolve, 100));
      }
    ",
    "timeout_ms": 10000
  }'
```

**Response (streaming):**
```json
{"iteration": 0, "value": 0}
{"iteration": 1, "value": 1}
{"iteration": 2, "value": 4}
{"iteration": 3, "value": 9}
{"iteration": 4, "value": 16}
```

## 🌐 **WEB INTEGRATION**

### **1. HTML Page with JetCrab**

```html
<!DOCTYPE html>
<html>
<head>
    <title>JetCrab JavaScript Server</title>
</head>
<body>
    <h1>JetCrab JavaScript Execution</h1>
    
    <div>
        <label for="code">JavaScript Code:</label>
        <textarea id="code" rows="10" cols="50">
function greet(name) {
    return `Hello, ${name}! Welcome to JetCrab!`;
}

let result = greet("Developer");
result;
        </textarea>
    </div>
    
    <button onclick="executeCode()">Execute</button>
    
    <div>
        <h3>Result:</h3>
        <pre id="result"></pre>
    </div>
    
    <script>
        async function executeCode() {
            const code = document.getElementById('code').value;
            const resultDiv = document.getElementById('result');
            
            try {
                const response = await fetch('http://localhost:8080/execute', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        script: code,
                        timeout_ms: 10000
                    })
                });
                
                const data = await response.json();
                
                if (data.errors && data.errors.length > 0) {
                    resultDiv.innerHTML = `Errors:\n${data.errors.join('\n')}`;
                } else {
                    resultDiv.innerHTML = `Result: ${JSON.stringify(data.result, null, 2)}\nExecution Time: ${data.execution_time_ms}ms\nMemory Used: ${data.memory_used_mb}MB`;
                }
            } catch (error) {
                resultDiv.innerHTML = `Error: ${error.message}`;
            }
        }
    </script>
</body>
</html>
```

### **2. Node.js Client**

```javascript
// jetcrab-client.js
const axios = require('axios');

class JetCrabClient {
    constructor(baseUrl = 'http://localhost:8080') {
        this.baseUrl = baseUrl;
    }
    
    async executeScript(script, options = {}) {
        try {
            const response = await axios.post(`${this.baseUrl}/execute`, {
                script,
                context: options.context,
                timeout_ms: options.timeout || 5000,
                memory_limit_mb: options.memoryLimit || 128
            });
            
            return response.data;
        } catch (error) {
            throw new Error(`JetCrab execution failed: ${error.message}`);
        }
    }
    
    async getHealth() {
        try {
            const response = await axios.get(`${this.baseUrl}/health`);
            return response.data;
        } catch (error) {
            throw new Error(`Health check failed: ${error.message}`);
        }
    }
    
    async getMetrics() {
        try {
            const response = await axios.get(`${this.baseUrl}/metrics`);
            return response.data;
        } catch (error) {
            throw new Error(`Metrics retrieval failed: ${error.message}`);
        }
    }
}

// Usage example
async function main() {
    const client = new JetCrabClient();
    
    try {
        // Check server health
        const health = await client.getHealth();
        console.log('Server Health:', health);
        
        // Execute JavaScript
        const result = await client.executeScript(`
            function calculateSum(numbers) {
                return numbers.reduce((sum, num) => sum + num, 0);
            }
            
            let numbers = [1, 2, 3, 4, 5];
            let sum = calculateSum(numbers);
            let average = sum / numbers.length;
            
            { sum, average, count: numbers.length };
        `);
        
        console.log('Execution Result:', result);
        
        // Get metrics
        const metrics = await client.getMetrics();
        console.log('Server Metrics:', metrics);
        
    } catch (error) {
        console.error('Error:', error.message);
    }
}

if (require.main === module) {
    main();
}

module.exports = JetCrabClient;
```

## 🔒 **SECURITY EXAMPLES**

### **1. Sandboxed Execution**

```bash
curl -X POST http://localhost:8080/execute/sandboxed \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-api-key" \
  -d '{
    "script": "console.log('Hello World');",
    "security_context": {
      "allowed_apis": ["console.log"],
      "max_execution_time_ms": 1000,
      "max_memory_mb": 64,
      "blocked_apis": ["process", "fs", "http"]
    }
  }'
```

### **2. Rate Limited Execution**

```bash
# First request (success)
curl -X POST http://localhost:8080/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: user-key-1" \
  -d '{"script": "1 + 1"}'

# Multiple rapid requests (rate limited)
for i in {1..10}; do
  curl -X POST http://localhost:8080/execute \
    -H "Content-Type: application/json" \
    -H "X-API-Key: user-key-1" \
    -d '{"script": "1 + 1"}'
done
```

## 📊 **MONITORING AND DEBUGGING**

### **1. Real-time Metrics**

```bash
# Get current metrics
curl http://localhost:8080/metrics

# Watch metrics in real-time
watch -n 1 'curl -s http://localhost:8080/metrics | grep jetcrab_engine_pool_size'
```

### **2. Debug Mode Execution**

```bash
curl -X POST http://localhost:8080/execute/debug \
  -H "Content-Type: application/json" \
  -d '{
    "script": "
      let x = 10;
      debugger;
      let y = x * 2;
      y;
    ",
    "debug_options": {
      "enable_breakpoints": true,
      "step_through": true,
      "show_variables": true
    }
  }'
```

## 🚀 **PRODUCTION DEPLOYMENT**

### **1. Load Balancer Configuration (Nginx)**

```nginx
# /etc/nginx/sites-available/jetcrab
upstream jetcrab_backend {
    server 127.0.0.1:8080;
    server 127.0.0.1:8081;
    server 127.0.0.1:8082;
}

server {
    listen 80;
    server_name jetcrab.example.com;
    
    location / {
        proxy_pass http://jetcrab_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # Timeout settings
        proxy_connect_timeout 30s;
        proxy_send_timeout 30s;
        proxy_read_timeout 30s;
    }
    
    location /health {
        proxy_pass http://jetcrab_backend;
        access_log off;
    }
    
    location /metrics {
        proxy_pass http://jetcrab_backend;
        access_log off;
    }
}
```

### **2. Systemd Service with Auto-restart**

```ini
# /etc/systemd/system/jetcrab.service
[Unit]
Description=JetCrab JavaScript Server
After=network.target
StartLimitIntervalSec=0

[Service]
Type=simple
User=jetcrab
Group=jetcrab
WorkingDirectory=/opt/jetcrab
ExecStart=/opt/jetcrab/jetcrab
Restart=always
RestartSec=1
StartLimitBurst=5

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096

# Environment
Environment=JETCRAB_BIND_ADDRESS=0.0.0.0:8080
Environment=JETCRAB_ENGINE_POOL_SIZE=20
Environment=JETCRAB_MAX_MEMORY_MB=1024
Environment=JETCRAB_LOG_LEVEL=info

[Install]
WantedBy=multi-user.target
```

## 🎯 **PERFORMANCE TESTING**

### **1. Load Testing with Apache Bench**

```bash
# Test basic execution
ab -n 1000 -c 10 -p test-script.json \
   -T application/json \
   http://localhost:8080/execute

# Test concurrent users
ab -n 10000 -c 100 -p test-script.json \
   -T application/json \
   http://localhost:8080/execute
```

**test-script.json:**
```json
{
  "script": "let sum = 0; for(let i = 0; i < 1000; i++) { sum += i; } sum;",
  "timeout_ms": 5000
}
```

### **2. Memory Usage Monitoring**

```bash
# Monitor memory usage
watch -n 1 'ps aux | grep jetcrab | grep -v grep'

# Monitor with htop
htop -p $(pgrep jetcrab)
```

## 🏆 **CONCLUSÃO**

O JetCrab Server oferece uma solução completa para execução de JavaScript no servidor com:

✅ **Fácil de usar**: API REST simples e intuitiva
✅ **Seguro**: Sandboxing e rate limiting
✅ **Escalável**: Pool de engines e load balancing
✅ **Monitorável**: Health checks e metrics
✅ **Production ready**: Docker, Kubernetes, systemd
✅ **Flexível**: Suporte a context, timeouts, memory limits

**Com estes exemplos, você pode começar a usar o JetCrab Server imediatamente para executar JavaScript no servidor de forma segura e eficiente!** 🚀
