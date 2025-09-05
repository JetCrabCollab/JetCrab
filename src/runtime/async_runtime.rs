//! # Async Runtime
//!
//! Integration with Tokio for asynchronous operations in JetCrab.

use std::sync::Arc;
use tokio::runtime::Handle;
use tokio::sync::Mutex;
use tracing::debug;

/// Async runtime manager for JetCrab
pub struct AsyncRuntime {
    handle: Handle,
    task_counter: Arc<Mutex<u64>>,
}

impl AsyncRuntime {
    /// Create a new async runtime
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let handle = match Handle::try_current() {
            Ok(handle) => {
                debug!("Using existing Tokio runtime handle");
                handle
            }
            Err(_) => {
                debug!("No existing Tokio runtime, creating new one");
                let rt = tokio::runtime::Runtime::new()?;
                rt.handle().clone()
            }
        };

        debug!("Async runtime initialized successfully");

        Ok(Self {
            handle,
            task_counter: Arc::new(Mutex::new(0)),
        })
    }

    /// Spawn an async task and return a task ID
    pub fn spawn_task<F, R>(&self, future: F) -> u64
    where
        F: std::future::Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let counter = Arc::clone(&self.task_counter);

        self.handle.spawn(async move {
            let task_id = {
                let mut count = counter.lock().await;
                *count += 1;
                *count
            };

            debug!("Starting async task #{}", task_id);
            let result = future.await;
            debug!("Completed async task #{}", task_id);
            result
        });

        tokio::task::block_in_place(|| {
            self.handle.block_on(async {
                let mut count = self.task_counter.lock().await;
                *count += 1;
                *count
            })
        })
    }

    /// Block on a future (for synchronous APIs that need async operations)
    pub fn block_on<F, R>(&self, future: F) -> R
    where
        F: std::future::Future<Output = R>,
    {
        tokio::task::block_in_place(|| self.handle.block_on(future))
    }

    /// Get a handle to the runtime for spawning tasks
    pub fn handle(&self) -> Handle {
        self.handle.clone()
    }

    /// Shutdown the runtime gracefully
    pub fn shutdown(self) {
        debug!("Shutting down async runtime");
    }
}

impl Default for AsyncRuntime {
    fn default() -> Self {
        Self::new().expect("Failed to create async runtime")
    }
}

/// Async task manager for JavaScript promises
pub struct AsyncTaskManager {
    runtime: Arc<AsyncRuntime>,
    pending_tasks: Arc<Mutex<std::collections::HashMap<u64, tokio::task::JoinHandle<()>>>>,
}

impl AsyncTaskManager {
    pub fn new(runtime: Arc<AsyncRuntime>) -> Self {
        Self {
            runtime,
            pending_tasks: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    /// Create a JavaScript promise that resolves when the async task completes
    pub async fn create_promise<F, R>(&self, future: F) -> u64
    where
        F: std::future::Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let task_id = self.runtime.spawn_task(future);

        debug!("Created async task #{}", task_id);
        task_id
    }

    /// Check if a task is still running
    pub async fn is_task_running(&self, task_id: u64) -> bool {
        let tasks = self.pending_tasks.lock().await;
        tasks.contains_key(&task_id)
    }

    /// Cancel a running task
    pub async fn cancel_task(&self, task_id: u64) -> bool {
        let mut tasks = self.pending_tasks.lock().await;
        if let Some(handle) = tasks.remove(&task_id) {
            handle.abort();
            debug!("Cancelled task #{}", task_id);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_async_runtime_creation() {
        let runtime = AsyncRuntime::new();
        assert!(runtime.is_ok());
    }

    #[tokio::test]
    async fn test_block_on() {
        let runtime = AsyncRuntime::new().unwrap();
        let result = runtime.block_on(async {
            tokio::time::sleep(Duration::from_millis(10)).await;
            42
        });
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_spawn_task() {
        let runtime = AsyncRuntime::new().unwrap();
        let task_id = runtime.spawn_task(async {
            tokio::time::sleep(Duration::from_millis(10)).await;
            "completed"
        });
        assert!(task_id > 0);
    }
}
