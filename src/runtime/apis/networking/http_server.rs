
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, warn};

/// HTTP Server implementation for JetCrab
pub struct HttpServer {
    port: u16,
    routes: Vec<Route>,
    middleware: Vec<String>,
    is_running: bool,
    request_count: u64,
}

/// Route definition
#[derive(Debug, Clone)]
pub struct Route {
    pub method: String,
    pub path: String,
    pub handler: String,
}

impl HttpServer {
    /// Create a new HTTP server
    pub fn new() -> Self {
        Self {
            port: 3000,
            routes: Vec::new(),
            middleware: Vec::new(),
            is_running: false,
            request_count: 0,
        }
    }

    /// Set the port for the server
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Add middleware
    pub fn use_middleware(&mut self, middleware: &str) -> &mut Self {
        self.middleware.push(middleware.to_string());
        info!("🔧 Middleware: {}", middleware);
        self
    }

    /// Add a GET route
    pub fn get(&mut self, path: &str, handler: &str) -> &mut Self {
        self.routes.push(Route {
            method: "GET".to_string(),
            path: path.to_string(),
            handler: handler.to_string(),
        });
        info!("📝 Route: GET {}", path);
        self
    }

    /// Add a POST route
    pub fn post(&mut self, path: &str, handler: &str) -> &mut Self {
        self.routes.push(Route {
            method: "POST".to_string(),
            path: path.to_string(),
            handler: handler.to_string(),
        });
        info!("📝 Route: POST {}", path);
        self
    }

    /// Add a PUT route
    pub fn put(&mut self, path: &str, handler: &str) -> &mut Self {
        self.routes.push(Route {
            method: "PUT".to_string(),
            path: path.to_string(),
            handler: handler.to_string(),
        });
        info!("📝 Route: PUT {}", path);
        self
    }

    /// Add a DELETE route
    pub fn delete(&mut self, path: &str, handler: &str) -> &mut Self {
        self.routes.push(Route {
            method: "DELETE".to_string(),
            path: path.to_string(),
            handler: handler.to_string(),
        });
        info!("📝 Route: DELETE {}", path);
        self
    }

    /// Start the HTTP server
    pub async fn listen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_running = true;

        info!("🚀 HTTP Server starting on port {}...", self.port);
        info!("🔧 Middleware loaded: {}", self.middleware.len());
        info!("📝 Routes registered: {}", self.routes.len());

        info!("📋 Available Routes:");
        for route in &self.routes {
            info!("  {} {}", route.method, route.path);
        }

        info!("✅ Server running on http://localhost:{}", self.port);
        info!("🦀 HTTP Server powered by JetCrab + Axum + Tokio!");

        let app = self.create_router();

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = TcpListener::bind(addr).await?;

        info!("🎯 Server Status: LISTENING");
        info!("📡 Ready to accept HTTP connections");
        info!("🔄 Server will continue running until interrupted");
        info!("💡 Press Ctrl+C to stop the server");

        axum::serve(listener, app).await?;

        Ok(())
    }

    /// Create the Axum router
    fn create_router(&self) -> Router {
        let mut router = Router::new();

        router = router.layer(CorsLayer::permissive());

        router = router.layer(TraceLayer::new_for_http());

        for route in &self.routes {
            match route.method.as_str() {
                "GET" => {
                    router = router.route(&route.path, get(handle_get_request));
                }
                "POST" => {
                    router = router.route(&route.path, post(handle_post_request));
                }
                "PUT" => {
                    router = router.route(&route.path, put(handle_put_request));
                }
                "DELETE" => {
                    router = router.route(&route.path, delete(handle_delete_request));
                }
                _ => {
                    warn!("❌ Unsupported HTTP method: {}", route.method);
                }
            }
        }

        router = router
            .route("/", get(handle_root))
            .route("/api/status", get(handle_status))
            .route("/api/users", get(handle_get_users))
            .route("/api/users", post(handle_create_user))
            .route("/api/users/:id", get(handle_get_user))
            .route("/api/users/:id", put(handle_update_user))
            .route("/api/users/:id", delete(handle_delete_user));

        router
    }
}

async fn handle_root() -> Json<Value> {
    Json(json!({
        "message": "Welcome to JetCrab HTTP Server!",
        "runtime": "JetCrab",
        "framework": "Axum + Tokio"
    }))
}

async fn handle_status() -> Json<Value> {
    Json(json!({
        "status": "running",
        "runtime": "JetCrab",
        "framework": "Axum + Tokio",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn handle_get_users() -> Json<Value> {
    let users = json!([
        {"id": 1, "name": "Alice", "email": "alice@example.com"},
        {"id": 2, "name": "Bob", "email": "bob@example.com"},
        {"id": 3, "name": "Charlie", "email": "charlie@example.com"}
    ]);
    Json(users)
}

async fn handle_create_user() -> Json<Value> {
    Json(json!({
        "message": "User created successfully",
        "id": 4,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn handle_get_user(Path(id): Path<u32>) -> Json<Value> {
    Json(json!({
        "id": id,
        "name": "Alice",
        "email": "alice@example.com",
        "created_at": "2024-01-01T00:00:00Z"
    }))
}

async fn handle_update_user(Path(id): Path<u32>) -> Json<Value> {
    Json(json!({
        "message": "User updated successfully",
        "id": id,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn handle_delete_user(Path(id): Path<u32>) -> Json<Value> {
    Json(json!({
        "message": "User deleted successfully",
        "id": id,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn handle_get_request() -> Json<Value> {
    Json(json!({
        "method": "GET",
        "message": "GET request handled by JetCrab",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn handle_post_request() -> Json<Value> {
    Json(json!({
        "method": "POST",
        "message": "POST request handled by JetCrab",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn handle_put_request() -> Json<Value> {
    Json(json!({
        "method": "PUT",
        "message": "PUT request handled by JetCrab",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn handle_delete_request() -> Json<Value> {
    Json(json!({
        "method": "DELETE",
        "message": "DELETE request handled by JetCrab",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_http_server_creation() {
        let server = HttpServer::new();
        assert_eq!(server.port, 3000);
        assert_eq!(server.routes.len(), 0);
        assert_eq!(server.middleware.len(), 0);
        assert!(!server.is_running);
    }

    #[tokio::test]
    async fn test_http_server_routes() {
        let mut server = HttpServer::new();
        server.get("/", "root_handler");
        server.post("/api/users", "create_user");

        assert_eq!(server.routes.len(), 2);
        assert_eq!(server.routes[0].method, "GET");
        assert_eq!(server.routes[0].path, "/");
        assert_eq!(server.routes[1].method, "POST");
        assert_eq!(server.routes[1].path, "/api/users");
    }

    #[tokio::test]
    async fn test_http_server_middleware() {
        let mut server = HttpServer::new();
        server.use_middleware("cors");
        server.use_middleware("helmet");

        assert_eq!(server.middleware.len(), 2);
        assert_eq!(server.middleware[0], "cors");
        assert_eq!(server.middleware[1], "helmet");
    }
}
