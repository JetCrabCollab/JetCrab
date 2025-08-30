# JetCrab Server Deployment Architecture

## 🎯 **OVERVIEW**

Este documento descreve como fazer o deploy e delivery do JetCrab como um servidor JavaScript, inspirado na arquitetura do V8. O JetCrab será executado como um serviço standalone que pode executar código JavaScript em requisições HTTP, similar ao Node.js mas com nossa própria engine.

## 🏗️ **ARQUITETURA DE SERVIDOR**

### **1. Arquitetura Geral**

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   HTTP Client   │───▶│   JetCrab      │───▶│   JavaScript    │
│   (Browser/API) │    │   HTTP Server   │    │   Engine        │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌─────────────────┐
                       │   Request       │
                       │   Router        │
                       └─────────────────┘
                                │
                                ▼
                       ┌─────────────────┐
                       │   JavaScript    │
                       │   Context       │
                       │   Pool          │
                       └─────────────────┘
```

### **2. Componentes Principais**

#### **HTTP Server Layer**
- **Framework**: Actix-web ou Axum (Rust)
- **Protocols**: HTTP/1.1, HTTP/2, WebSocket
- **Ports**: Configurável (default: 8080)
- **SSL/TLS**: Suporte para HTTPS

#### **JavaScript Engine Layer**
- **JetCrab Engine**: Nossa engine JavaScript
- **Context Pool**: Múltiplos contextos de execução
- **Memory Management**: Garbage collection por contexto
- **Security**: Sandboxing e resource limits

#### **Request Processing Layer**
- **Routing**: URL-based routing para scripts
- **Middleware**: Authentication, logging, rate limiting
- **Script Loading**: File system ou database
- **Response Handling**: JSON, HTML, streaming

## 🚀 **IMPLEMENTAÇÃO TÉCNICA**

### **1. HTTP Server Implementation**

```rust
// Target: src/server/mod.rs
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use actix_web::middleware::{Logger, DefaultHeaders};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct JetCrabServer {
    engine_pool: Arc<RwLock<EnginePool>>,
    script_registry: Arc<RwLock<ScriptRegistry>>,
    config: ServerConfig,
}

impl JetCrabServer {
    pub async fn new(config: ServerConfig) -> Result<Self, ServerError> {
        let engine_pool = Arc::new(RwLock::new(EnginePool::new(config.engine_pool_size)?));
        let script_registry = Arc::new(RwLock::new(ScriptRegistry::new()?));
        
        Ok(Self {
            engine_pool,
            script_registry,
            config,
        })
    }
    
    pub async fn start(&self) -> Result<(), ServerError> {
        let engine_pool = Arc::clone(&self.engine_pool);
        let script_registry = Arc::clone(&self.script_registry);
        
        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .wrap(DefaultHeaders::new().add(("X-Powered-By", "JetCrab")))
                .app_data(web::Data::new(engine_pool.clone()))
                .app_data(web::Data::new(script_registry.clone()))
                .service(web::resource("/execute").to(execute_script))
                .service(web::resource("/scripts/{script_id}").to(get_script))
                .service(web::resource("/health").to(health_check))
                .service(web::resource("/metrics").to(get_metrics))
        })
        .bind(&self.config.bind_address)?
        .run()
        .await?;
        
        Ok(())
    }
}
```

### **2. Engine Pool Management**

```rust
// Target: src/server/engine_pool.rs
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::api::{Engine, Context, EngineConfig};

pub struct EngineInstance {
    pub engine: Engine,
    pub context: Context,
    pub last_used: std::time::Instant,
    pub memory_usage: usize,
    pub execution_count: u64,
}

pub struct EnginePool {
    engines: HashMap<u32, EngineInstance>,
    config: EnginePoolConfig,
    next_id: u32,
}

impl EnginePool {
    pub fn new(config: EnginePoolConfig) -> Result<Self, EnginePoolError> {
        let mut pool = Self {
            engines: HashMap::new(),
            config,
            next_id: 0,
        };
        
        // Pre-warm engine pool
        for _ in 0..config.initial_pool_size {
            pool.create_engine()?;
        }
        
        Ok(pool)
    }
    
    pub async fn get_engine(&mut self) -> Result<u32, EnginePoolError> {
        // Find available engine or create new one
        for (id, instance) in &mut self.engines {
            if instance.execution_count < self.config.max_executions_per_engine {
                instance.last_used = std::time::Instant::now();
                instance.execution_count += 1;
                return Ok(*id);
            }
        }
        
        // Create new engine if pool not full
        if self.engines.len() < self.config.max_pool_size {
            self.create_engine()
        } else {
            Err(EnginePoolError::PoolExhausted)
        }
    }
    
    fn create_engine(&mut self) -> Result<u32, EnginePoolError> {
        let engine_config = EngineConfig::default()
            .with_memory_config(self.config.memory_config.clone())
            .with_optimization_level(self.config.optimization_level.clone())
            .with_security_level(self.config.security_level.clone());
        
        let engine = Engine::new(engine_config)?;
        let context = Context::new();
        
        let id = self.next_id;
        self.next_id += 1;
        
        self.engines.insert(id, EngineInstance {
            engine,
            context,
            last_used: std::time::Instant::now(),
            memory_usage: 0,
            execution_count: 0,
        });
        
        Ok(id)
    }
}
```

### **3. Script Execution Handler**

```rust
// Target: src/server/script_handler.rs
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Deserialize)]
pub struct ExecuteRequest {
    pub script: String,
    pub context: Option<serde_json::Value>,
    pub timeout_ms: Option<u64>,
    pub memory_limit_mb: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct ExecuteResponse {
    pub result: serde_json::Value,
    pub execution_time_ms: u64,
    pub memory_used_mb: usize,
    pub errors: Vec<String>,
}

pub async fn execute_script(
    req: web::Json<ExecuteRequest>,
    engine_pool: web::Data<Arc<RwLock<EnginePool>>>,
    script_registry: web::Data<Arc<RwLock<ScriptRegistry>>>,
) -> Result<HttpResponse, actix_web::Error> {
    let start_time = std::time::Instant::now();
    
    // Get engine from pool
    let mut pool = engine_pool.write().await;
    let engine_id = pool.get_engine().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Engine pool error: {}", e))
    })?;
    
    let engine_instance = pool.engines.get_mut(&engine_id)
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Engine not found"))?;
    
    // Execute JavaScript code
    let result = tokio::time::timeout(
        std::time::Duration::from_millis(req.timeout_ms.unwrap_or(5000)),
        execute_in_engine(&mut engine_instance.engine, &mut engine_instance.context, &req.script)
    ).await.map_err(|_| {
        actix_web::error::ErrorRequestTimeout("Script execution timeout")
    })??;
    
    let execution_time = start_time.elapsed().as_millis() as u64;
    let memory_used = engine_instance.memory_usage;
    
    let response = ExecuteResponse {
        result,
        execution_time_ms: execution_time,
        memory_used_mb: memory_used / 1024 / 1024,
        errors: Vec::new(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

async fn execute_in_engine(
    engine: &mut Engine,
    context: &mut Context,
    script: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Compile and execute JavaScript
    let compiled = engine.compile(script)?;
    let result = engine.execute(compiled, context)?;
    
    // Convert result to JSON
    Ok(serde_json::to_value(result)?)
}
```

## 📦 **STRATEGIES DE DEPLOY**

### **1. Docker Container**

```dockerfile
# Dockerfile
FROM rust:1.70-slim as builder

WORKDIR /usr/src/jetcrab
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/jetcrab/target/release/jetcrab /usr/local/bin/jetcrab

EXPOSE 8080
CMD ["jetcrab"]
```

```yaml
# docker-compose.yml
version: '3.8'
services:
  jetcrab:
    build: .
    ports:
      - "8080:8080"
    environment:
      - JETCRAB_BIND_ADDRESS=0.0.0.0:8080
      - JETCRAB_ENGINE_POOL_SIZE=10
      - JETCRAB_MAX_MEMORY_MB=512
    volumes:
      - ./scripts:/app/scripts
      - ./config:/app/config
    restart: unless-stopped
```

### **2. Kubernetes Deployment**

```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: jetcrab-server
  labels:
    app: jetcrab
spec:
  replicas: 3
  selector:
    matchLabels:
      app: jetcrab
  template:
    metadata:
      labels:
        app: jetcrab
    spec:
      containers:
      - name: jetcrab
        image: jetcrab:latest
        ports:
        - containerPort: 8080
        env:
        - name: JETCRAB_BIND_ADDRESS
          value: "0.0.0.0:8080"
        - name: JETCRAB_ENGINE_POOL_SIZE
          value: "10"
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
```

### **3. Systemd Service (Linux)**

```ini
# /etc/systemd/system/jetcrab.service
[Unit]
Description=JetCrab JavaScript Server
After=network.target

[Service]
Type=simple
User=jetcrab
Group=jetcrab
WorkingDirectory=/opt/jetcrab
ExecStart=/opt/jetcrab/jetcrab
Restart=always
RestartSec=5
Environment=JETCRAB_BIND_ADDRESS=0.0.0.0:8080
Environment=JETCRAB_ENGINE_POOL_SIZE=10
Environment=JETCRAB_MAX_MEMORY_MB=512

[Install]
WantedBy=multi-user.target
```

## 🔧 **CONFIGURAÇÃO DO SERVIDOR**

### **1. Configuration File**

```toml
# config/server.toml
[server]
bind_address = "0.0.0.0:8080"
max_connections = 1000
request_timeout_ms = 30000
enable_compression = true
enable_cors = true

[engine_pool]
initial_pool_size = 5
max_pool_size = 20
max_executions_per_engine = 1000
engine_cleanup_interval_ms = 300000

[security]
enable_sandboxing = true
max_memory_mb = 512
max_execution_time_ms = 10000
allowed_file_paths = ["/app/scripts"]
blocked_apis = ["fs.writeFile", "process.exit"]

[logging]
level = "info"
format = "json"
output = "stdout"
enable_request_logging = true

[monitoring]
enable_metrics = true
metrics_port = 9090
enable_health_checks = true
health_check_interval_ms = 30000
```

### **2. Environment Variables**

```bash
# .env
JETCRAB_BIND_ADDRESS=0.0.0.0:8080
JETCRAB_ENGINE_POOL_SIZE=10
JETCRAB_MAX_MEMORY_MB=512
JETCRAB_LOG_LEVEL=info
JETCRAB_ENABLE_METRICS=true
JETCRAB_METRICS_PORT=9090
JETCRAB_SCRIPTS_PATH=/app/scripts
JETCRAB_TEMP_PATH=/tmp/jetcrab
```

## 📊 **MONITORING E METRICS**

### **1. Health Check Endpoint**

```rust
// Health check implementation
pub async fn health_check(
    engine_pool: web::Data<Arc<RwLock<EnginePool>>>,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = engine_pool.read().await;
    
    let health_status = HealthStatus {
        status: "healthy",
        timestamp: chrono::Utc::now(),
        engine_pool_size: pool.engines.len(),
        active_engines: pool.engines.values()
            .filter(|e| e.last_used.elapsed().as_secs() < 300)
            .count(),
        memory_usage_mb: pool.engines.values()
            .map(|e| e.memory_usage)
            .sum::<usize>() / 1024 / 1024,
        uptime_seconds: std::time::Instant::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .as_secs(),
    };
    
    Ok(HttpResponse::Ok().json(health_status))
}
```

### **2. Metrics Endpoint (Prometheus)**

```rust
// Metrics implementation
pub async fn get_metrics(
    engine_pool: web::Data<Arc<RwLock<EnginePool>>>,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = engine_pool.read().await;
    
    let metrics = format!(
        "# HELP jetcrab_engine_pool_size Current engine pool size\n\
         # TYPE jetcrab_engine_pool_size gauge\n\
         jetcrab_engine_pool_size {}\n\
         \n\
         # HELP jetcrab_active_engines Number of active engines\n\
         # TYPE jetcrab_active_engines gauge\n\
         jetcrab_active_engines {}\n\
         \n\
         # HELP jetcrab_total_memory_bytes Total memory usage\n\
         # TYPE jetcrab_total_memory_bytes gauge\n\
         jetcrab_total_memory_bytes {}\n",
        pool.engines.len(),
        pool.engines.values()
            .filter(|e| e.last_used.elapsed().as_secs() < 300)
            .count(),
        pool.engines.values()
            .map(|e| e.memory_usage)
            .sum::<usize>()
    );
    
    Ok(HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4; charset=utf-8")
        .body(metrics))
}
```

## 🚀 **DEPLOYMENT WORKFLOW**

### **1. CI/CD Pipeline**

```yaml
# .github/workflows/deploy.yml
name: Deploy JetCrab Server

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - run: cargo test
    - run: cargo build --release

  build-docker:
    needs: test
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Build Docker image
      run: docker build -t jetcrab:latest .
    - name: Push to registry
      run: |
        echo ${{ secrets.DOCKER_PASSWORD }} | docker login -u ${{ secrets.DOCKER_USERNAME }} --password-stdin
        docker tag jetcrab:latest ${{ secrets.DOCKER_REGISTRY }}/jetcrab:latest
        docker push ${{ secrets.DOCKER_REGISTRY }}/jetcrab:latest

  deploy:
    needs: build-docker
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
    - name: Deploy to production
      run: |
        # Deploy to Kubernetes or other platform
        kubectl set image deployment/jetcrab-server jetcrab=${{ secrets.DOCKER_REGISTRY }}/jetcrab:latest
```

### **2. Deployment Commands**

```bash
# Local development
cargo run --bin jetcrab-server

# Docker
docker run -p 8080:8080 jetcrab:latest

# Kubernetes
kubectl apply -f k8s/

# Systemd
sudo systemctl enable jetcrab
sudo systemctl start jetcrab
sudo systemctl status jetcrab
```

## 🔒 **SECURITY CONSIDERATIONS**

### **1. Sandboxing**
- **Process Isolation**: Cada engine roda em contexto isolado
- **Resource Limits**: Memory, CPU, execution time limits
- **API Restrictions**: Bloqueio de APIs sensíveis (fs, process)
- **Network Isolation**: Controle de acesso à rede

### **2. Authentication & Authorization**
- **API Keys**: Para acesso aos endpoints
- **Rate Limiting**: Prevenção de abuse
- **Request Validation**: Sanitização de inputs
- **Audit Logging**: Log de todas as execuções

## 📈 **SCALING STRATEGIES**

### **1. Horizontal Scaling**
- **Load Balancer**: Distribuição de carga entre instâncias
- **Auto-scaling**: Baseado em CPU/memory usage
- **Session Affinity**: Para scripts com estado

### **2. Vertical Scaling**
- **Engine Pool Size**: Ajuste dinâmico do pool
- **Memory Allocation**: Otimização por workload
- **CPU Affinity**: Bind de engines a cores específicos

## 🏆 **CONCLUSÃO**

O JetCrab como servidor JavaScript oferece:

✅ **Performance**: Engine Rust nativo, rápido e eficiente
✅ **Scalability**: Pool de engines, load balancing
✅ **Security**: Sandboxing, resource limits, API restrictions
✅ **Monitoring**: Health checks, metrics, logging
✅ **Deployment**: Docker, Kubernetes, systemd
✅ **Production Ready**: CI/CD, monitoring, security

**Esta arquitetura permite que o JetCrab seja usado em produção como um servidor JavaScript robusto e escalável, similar ao V8 mas com nossa própria engine!** 🚀
