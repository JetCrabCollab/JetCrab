use boa_engine::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::sleep;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerInfo {
    pub id: u32,
    pub thread_id: u32,
    pub status: WorkerStatus,
    pub start_time: u64,
    pub message_count: u64,
    pub last_activity: u64,
    pub script_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkerStatus {
    Starting,
    Running,
    Stopped,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerMessage {
    pub from: u32,
    pub to: u32,
    pub data: serde_json::Value,
    pub timestamp: u64,
    pub message_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Data,
    Error,
    Exit,
    Online,
    Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerThreadConfig {
    pub max_workers: u32,
    pub min_workers: u32,
    pub worker_timeout: Duration,
    pub message_timeout: Duration,
    pub enable_shared_array_buffer: bool,
    pub enable_transferable_objects: bool,
}

impl Default for WorkerThreadConfig {
    fn default() -> Self {
        Self {
            max_workers: num_cpus::get() as u32,
            min_workers: 1,
            worker_timeout: Duration::from_secs(30),
            message_timeout: Duration::from_secs(5),
            enable_shared_array_buffer: true,
            enable_transferable_objects: true,
        }
    }
}

pub struct WorkerThreadManager {
    config: WorkerThreadConfig,
    workers: Arc<RwLock<HashMap<u32, WorkerInfo>>>,
    message_tx: mpsc::UnboundedSender<WorkerMessage>,
    message_rx: Arc<Mutex<mpsc::UnboundedReceiver<WorkerMessage>>>,
    worker_counter: Arc<AtomicU32>,
    is_main_thread: bool,
}

impl WorkerThreadManager {
    pub fn new(config: WorkerThreadConfig) -> Self {
        let (message_tx, message_rx) = mpsc::unbounded_channel();

        Self {
            config,
            workers: Arc::new(RwLock::new(HashMap::new())),
            message_tx,
            message_rx: Arc::new(Mutex::new(message_rx)),
            worker_counter: Arc::new(AtomicU32::new(0)),
            is_main_thread: true,
        }
    }

    pub async fn create_worker(
        &self,
        script_path: &str,
    ) -> Result<u32, Box<dyn std::error::Error>> {
        let worker_id = self.worker_counter.fetch_add(1, Ordering::SeqCst);

        if worker_id >= self.config.max_workers {
            return Err("Maximum number of workers reached".into());
        }

        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let worker_info = WorkerInfo {
            id: worker_id,
            thread_id: 0,
            status: WorkerStatus::Starting,
            start_time,
            message_count: 0,
            last_activity: start_time,
            script_path: Some(script_path.to_string()),
        };

        {
            let mut workers = self.workers.write().await;
            workers.insert(worker_id, worker_info);
        }

        info!(
            "Created worker thread: ID={}, script={}",
            worker_id, script_path
        );

        Ok(worker_id)
    }

    pub async fn send_message(
        &self,
        worker_id: u32,
        message: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let workers = self.workers.read().await;
        if !workers.contains_key(&worker_id) {
            return Err(format!("Worker {} not found", worker_id).into());
        }

        let worker_message = WorkerMessage {
            from: 0,
            to: worker_id,
            data: message,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            message_type: MessageType::Message,
        };

        self.message_tx.send(worker_message)?;
        info!("Sent message to worker: ID={}", worker_id);

        Ok(())
    }

    pub async fn get_worker_info(&self, worker_id: u32) -> Option<WorkerInfo> {
        let workers = self.workers.read().await;
        workers.get(&worker_id).cloned()
    }

    pub async fn list_workers(&self) -> Vec<WorkerInfo> {
        let workers = self.workers.read().await;
        workers.values().cloned().collect()
    }

    pub async fn terminate_worker(&self, worker_id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut workers = self.workers.write().await;
        if let Some(worker) = workers.get_mut(&worker_id) {
            worker.status = WorkerStatus::Stopped;
            info!("Terminated worker: ID={}", worker_id);
        }

        Ok(())
    }

    pub async fn terminate_all_workers(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut workers = self.workers.write().await;
        for worker in workers.values_mut() {
            worker.status = WorkerStatus::Stopped;
        }
        info!("Terminated all workers");

        Ok(())
    }
}

pub struct WorkerThreadsAPI;

impl WorkerThreadsAPI {
    pub fn new() -> Self {
        Self
    }

    pub fn register(&self, context: &mut Context) -> Result<(), Box<dyn std::error::Error>> {
        info!("🧵 Registering Worker Threads API...");

        let worker_threads_code = r#"
        class Worker {
            constructor(filename, options = {}) {
                this.filename = filename;
                this.options = options;
                this.id = Math.floor(Math.random() * 1000000);
                this.online = false;
                this.terminated = false;
                
                console.log(`🧵 Worker created: ID=${this.id}, file=${filename}`);
                
                setTimeout(() => {
                    this.online = true;
                    if (this.ononline) {
                        this.ononline();
                    }
                }, 100);
            }
            
            postMessage(message, transferList) {
                if (this.terminated) {
                    throw new Error('Worker is terminated');
                }
                
                console.log(`🧵 Worker ${this.id} received message:`, message);
                
                setTimeout(() => {
                    if (this.onmessage) {
                        this.onmessage({
                            data: message,
                            type: 'message'
                        });
                    }
                }, 50);
            }
            
            terminate() {
                this.terminated = true;
                this.online = false;
                console.log(`🧵 Worker ${this.id} terminated`);
            }
            
            addEventListener(type, listener) {
                if (type === 'message') {
                    this.onmessage = listener;
                } else if (type === 'error') {
                    this.onerror = listener;
                } else if (type === 'online') {
                    this.ononline = listener;
                } else if (type === 'exit') {
                    this.onexit = listener;
                }
            }
            
            removeEventListener(type, listener) {
                if (type === 'message') {
                    this.onmessage = null;
                } else if (type === 'error') {
                    this.onerror = null;
                } else if (type === 'online') {
                    this.ononline = null;
                } else if (type === 'exit') {
                    this.onexit = null;
                }
            }
        }

        globalThis.worker_threads = {
            Worker: Worker,
            isMainThread: true,
            parentPort: null,
            threadId: 0,
            workerData: null,
            
            SHARE_ENV: 'SHARE_ENV',
            threadId: 0,
            
            getEnvironmentData: function(key) {
                return null;
            },
            
            setEnvironmentData: function(key, value) {
            },
            
            markAsUntransferable: function(object) {
                return object;
            },
            
            moveMessagePortToContext: function(port, context) {
                return port;
            },
            
            receiveMessageOnPort: function(port) {
                return null;
            },
            
            resourceLimits: {
                maxYoungGenerationSizeMb: 64,
                maxOldGenerationSizeMb: 1408,
                codeRangeSizeMb: 0,
                stackSizeMb: 4
            },
            
            SHARE_ENV: 'SHARE_ENV'
        };

        globalThis.Worker = Worker;
        "#;

        context.eval(boa_engine::Source::from_bytes(worker_threads_code))?;
        info!("✅ Worker Threads API registered successfully");
        Ok(())
    }
}
