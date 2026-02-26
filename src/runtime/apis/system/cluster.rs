use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::process::Command;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::sleep;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerInfo {
    pub id: u32,
    pub pid: u32,
    pub status: WorkerStatus,
    pub start_time: u64,
    pub message_count: u64,
    pub last_activity: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkerStatus {
    Starting,
    Online,
    Disconnected,
    Dead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterMessage {
    pub from: u32,
    pub to: u32,
    pub data: serde_json::Value,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    pub max_workers: u32,
    pub min_workers: u32,
    pub restart_delay: Duration,
    pub heartbeat_interval: Duration,
    pub load_balance_strategy: LoadBalanceStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalanceStrategy {
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    Random,
}

impl Default for ClusterConfig {
    fn default() -> Self {
        Self {
            max_workers: num_cpus::get() as u32,
            min_workers: 1,
            restart_delay: Duration::from_secs(1),
            heartbeat_interval: Duration::from_secs(5),
            load_balance_strategy: LoadBalanceStrategy::RoundRobin,
        }
    }
}

pub struct ClusterManager {
    config: ClusterConfig,
    workers: Arc<RwLock<HashMap<u32, WorkerInfo>>>,
    message_tx: mpsc::UnboundedSender<ClusterMessage>,
    message_rx: Arc<Mutex<mpsc::UnboundedReceiver<ClusterMessage>>>,
    worker_counter: Arc<Mutex<u32>>,
    is_master: bool,
    is_worker: bool,
    worker_id: Option<u32>,
}

impl ClusterManager {
    pub fn new(config: ClusterConfig) -> Self {
        let (message_tx, message_rx) = mpsc::unbounded_channel();

        Self {
            config,
            workers: Arc::new(RwLock::new(HashMap::new())),
            message_tx,
            message_rx: Arc::new(Mutex::new(message_rx)),
            worker_counter: Arc::new(Mutex::new(0)),
            is_master: false,
            is_worker: false,
            worker_id: None,
        }
    }

    pub async fn setup_master(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚀 Setting up Cluster Master...");
        self.is_master = true;

        let workers = self.workers.clone();
        let config = self.config.clone();
        let message_tx = self.message_tx.clone();
        let worker_counter = self.worker_counter.clone();

        tokio::spawn(async move {
            Self::master_worker_manager(workers, config, message_tx, worker_counter).await;
        });

        let workers = self.workers.clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            Self::heartbeat_monitor(workers, config).await;
        });

        info!("✅ Cluster Master setup complete");
        Ok(())
    }

    pub async fn setup_worker(&mut self, worker_id: u32) -> Result<(), Box<dyn std::error::Error>> {
        info!("👷 Setting up Cluster Worker {}...", worker_id);
        self.is_worker = true;
        self.worker_id = Some(worker_id);

        let message_rx = self.message_rx.clone();
        let worker_id = worker_id;

        tokio::spawn(async move {
            Self::worker_message_handler(message_rx, worker_id).await;
        });

        info!("✅ Cluster Worker {} setup complete", worker_id);
        Ok(())
    }

    pub async fn fork(&self, script_path: &str) -> Result<u32, Box<dyn std::error::Error>> {
        if !self.is_master {
            return Err("Only master process can fork workers".into());
        }

        let mut counter = self.worker_counter.lock().await;
        let worker_id = *counter;
        *counter += 1;
        drop(counter);

        info!(
            "🔄 Forking worker {} with script: {}",
            worker_id, script_path
        );

        let mut cmd = Command::new("jetcrab");
        cmd.arg("run");
        cmd.arg(script_path);
        cmd.env("CLUSTER_WORKER_ID", worker_id.to_string());
        cmd.env("CLUSTER_IS_WORKER", "true");

        let child = cmd.spawn()?;
        let pid = child.id().unwrap_or(0) as u32;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let worker_info = WorkerInfo {
            id: worker_id,
            pid,
            status: WorkerStatus::Starting,
            start_time: now,
            message_count: 0,
            last_activity: now,
        };

        let mut workers = self.workers.write().await;
        workers.insert(worker_id, worker_info);
        drop(workers);

        info!("✅ Worker {} forked with PID: {}", worker_id, pid);
        Ok(worker_id)
    }

    pub async fn send_message(
        &self,
        to_worker: u32,
        data: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let from_worker = self.worker_id.unwrap_or(0);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let message = ClusterMessage {
            from: from_worker,
            to: to_worker,
            data,
            timestamp: now,
        };

        self.message_tx.send(message)?;
        Ok(())
    }

    pub async fn broadcast_message(
        &self,
        data: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let workers = self.workers.read().await;
        let worker_ids: Vec<u32> = workers.keys().cloned().collect();
        drop(workers);

        for worker_id in worker_ids {
            self.send_message(worker_id, data.clone()).await?;
        }

        Ok(())
    }

    pub async fn get_worker_count(&self) -> u32 {
        let workers = self.workers.read().await;
        workers.len() as u32
    }

    pub async fn get_worker_info(&self, worker_id: u32) -> Option<WorkerInfo> {
        let workers = self.workers.read().await;
        workers.get(&worker_id).cloned()
    }

    pub async fn get_all_workers(&self) -> Vec<WorkerInfo> {
        let workers = self.workers.read().await;
        workers.values().cloned().collect()
    }

    pub async fn kill_worker(&self, worker_id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut workers = self.workers.write().await;
        if let Some(worker) = workers.get_mut(&worker_id) {
            worker.status = WorkerStatus::Dead;
            info!("💀 Worker {} marked as dead", worker_id);
        }
        drop(workers);

        Ok(())
    }

    pub async fn restart_worker(
        &self,
        worker_id: u32,
        script_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.kill_worker(worker_id).await?;
        sleep(self.config.restart_delay).await;
        self.fork(script_path).await?;
        Ok(())
    }

    pub async fn get_load_balance_target(&self) -> Option<u32> {
        let workers = self.workers.read().await;
        let online_workers: Vec<&WorkerInfo> = workers
            .values()
            .filter(|w| matches!(w.status, WorkerStatus::Online))
            .collect();

        if online_workers.is_empty() {
            return None;
        }

        match self.config.load_balance_strategy {
            LoadBalanceStrategy::RoundRobin => {
                let target = online_workers
                    .iter()
                    .min_by_key(|w| w.message_count)
                    .map(|w| w.id);
                target
            }
            LoadBalanceStrategy::LeastConnections => {
                let target = online_workers
                    .iter()
                    .min_by_key(|w| w.last_activity)
                    .map(|w| w.id);
                target
            }
            LoadBalanceStrategy::WeightedRoundRobin => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let target = online_workers
                    .iter()
                    .min_by_key(|w| w.message_count + ((now - w.start_time) / 60))
                    .map(|w| w.id);
                target
            }
            LoadBalanceStrategy::Random => {
                let index = fastrand::usize(..online_workers.len());
                Some(online_workers[index].id)
            }
        }
    }

    async fn master_worker_manager(
        workers: Arc<RwLock<HashMap<u32, WorkerInfo>>>,
        config: ClusterConfig,
        _message_tx: mpsc::UnboundedSender<ClusterMessage>,
        worker_counter: Arc<Mutex<u32>>,
    ) {
        info!("🎯 Master worker manager started");

        loop {
            sleep(Duration::from_secs(1)).await;

            let mut workers_guard = workers.write().await;
            let mut to_remove = Vec::new();

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            for (worker_id, worker) in workers_guard.iter_mut() {
                if now - worker.start_time > 30 {
                    worker.status = WorkerStatus::Online;
                }

                if matches!(worker.status, WorkerStatus::Dead) {
                    to_remove.push(*worker_id);
                }
            }

            for worker_id in to_remove {
                workers_guard.remove(&worker_id);
                info!("🗑️ Removed dead worker {}", worker_id);
            }

            if workers_guard.len() < config.min_workers as usize {
                let needed = config.min_workers as usize - workers_guard.len();
                for _ in 0..needed {
                    let mut counter = worker_counter.lock().await;
                    let worker_id = *counter;
                    *counter += 1;
                    drop(counter);

                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    let worker_info = WorkerInfo {
                        id: worker_id,
                        pid: 0,
                        status: WorkerStatus::Starting,
                        start_time: now,
                        message_count: 0,
                        last_activity: now,
                    };

                    workers_guard.insert(worker_id, worker_info);
                    info!("🔄 Auto-created worker {}", worker_id);
                }
            }

            drop(workers_guard);
        }
    }

    async fn heartbeat_monitor(
        workers: Arc<RwLock<HashMap<u32, WorkerInfo>>>,
        config: ClusterConfig,
    ) {
        info!("💓 Heartbeat monitor started");

        loop {
            sleep(config.heartbeat_interval).await;

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let mut workers_guard = workers.write().await;
            for (worker_id, worker) in workers_guard.iter_mut() {
                if now - worker.last_activity > config.heartbeat_interval.as_secs() * 2 {
                    warn!("⚠️ Worker {} appears unresponsive", worker_id);
                    worker.status = WorkerStatus::Disconnected;
                }
            }
            drop(workers_guard);
        }
    }

    async fn worker_message_handler(
        message_rx: Arc<Mutex<mpsc::UnboundedReceiver<ClusterMessage>>>,
        worker_id: u32,
    ) {
        info!("📨 Worker {} message handler started", worker_id);

        loop {
            let mut rx = message_rx.lock().await;
            if let Some(message) = rx.recv().await {
                if message.to == worker_id || message.to == 0 {
                    info!(
                        "📬 Worker {} received message from worker {}: {:?}",
                        worker_id, message.from, message.data
                    );

                }
            } else {
                break;
            }
        }
    }
}

pub struct ClusterAPI {
    cluster_manager: Arc<Mutex<ClusterManager>>,
}

impl ClusterAPI {
    pub fn new() -> Self {
        Self {
            cluster_manager: Arc::new(Mutex::new(ClusterManager::new(ClusterConfig::default()))),
        }
    }

    pub fn register(
        &self,
        context: &mut chitin::boa_engine::Context,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔄 Registering Cluster API...");

        let cluster_code = r#"
        globalThis.cluster = {
            isMaster: false,
            isWorker: false,
            worker: null,
            workers: {},
            settings: {},
            
            setupMaster: function(config) {
                return new Promise((resolve, reject) => {
                    console.log('Setting up cluster master with config:', config);
                    globalThis.cluster.isMaster = true;
                    resolve();
                });
            },
            
            setupWorker: function(workerId) {
                return new Promise((resolve, reject) => {
                    console.log('Setting up cluster worker:', workerId);
                    globalThis.cluster.isWorker = true;
                    globalThis.cluster.worker = { id: workerId };
                    resolve();
                });
            },
            
            fork: function(script) {
                return new Promise((resolve, reject) => {
                    console.log('Forking worker with script:', script);
                    const workerId = Math.floor(Math.random() * 1000);
                    globalThis.cluster.workers[workerId] = {
                        id: workerId,
                        pid: workerId + 1000,
                        status: 'online',
                        startTime: Date.now()
                    };
                    resolve(workerId);
                });
            },
            
            send: function(workerId, data) {
                return new Promise((resolve, reject) => {
                    console.log('Sending message to worker', workerId, ':', data);
                    resolve();
                });
            },
            
            broadcast: function(data) {
                return new Promise((resolve, reject) => {
                    console.log('Broadcasting message to all workers:', data);
                    resolve();
                });
            },
            
            getWorkerCount: function() {
                return Object.keys(globalThis.cluster.workers).length;
            },
            
            getWorkerInfo: function(workerId) {
                return globalThis.cluster.workers[workerId] || null;
            },
            
            getAllWorkers: function() {
                return Object.values(globalThis.cluster.workers);
            },
            
            killWorker: function(workerId) {
                return new Promise((resolve, reject) => {
                    console.log('Killing worker:', workerId);
                    if (globalThis.cluster.workers[workerId]) {
                        globalThis.cluster.workers[workerId].status = 'dead';
                    }
                    resolve();
                });
            },
            
            restartWorker: function(workerId, script) {
                return new Promise((resolve, reject) => {
                    console.log('Restarting worker', workerId, 'with script:', script);
                    globalThis.cluster.killWorker(workerId).then(() => {
                        return globalThis.cluster.fork(script);
                    }).then(() => {
                        resolve();
                    });
                });
            },
            
            getLoadBalanceTarget: function() {
                const workers = Object.values(globalThis.cluster.workers);
                if (workers.length === 0) return null;
                
                const target = workers.find(w => w.status === 'online');
                return target ? target.id : null;
            }
        };
        "#;

        context.eval(chitin::boa_engine::Source::from_bytes(cluster_code))?;
        info!("✅ Cluster API registered successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_cluster_config_default() {
        let config = ClusterConfig::default();
        assert_eq!(config.max_workers, num_cpus::get() as u32);
        assert_eq!(config.min_workers, 1);
        assert_eq!(config.restart_delay, Duration::from_secs(1));
        assert_eq!(config.heartbeat_interval, Duration::from_secs(5));
    }

    #[test]
    async fn test_cluster_manager_creation() {
        let config = ClusterConfig::default();
        let manager = ClusterManager::new(config);
        assert!(!manager.is_master);
        assert!(!manager.is_worker);
        assert_eq!(manager.worker_id, None);
    }

    #[test]
    async fn test_cluster_manager_setup_master() {
        let config = ClusterConfig::default();
        let mut manager = ClusterManager::new(config);
        let result = manager.setup_master().await;
        assert!(result.is_ok());
        assert!(manager.is_master);
    }

    #[test]
    async fn test_cluster_manager_setup_worker() {
        let config = ClusterConfig::default();
        let mut manager = ClusterManager::new(config);
        let result = manager.setup_worker(1).await;
        assert!(result.is_ok());
        assert!(manager.is_worker);
        assert_eq!(manager.worker_id, Some(1));
    }

    #[test]
    async fn test_cluster_manager_fork() {
        let config = ClusterConfig::default();
        let mut manager = ClusterManager::new(config);
        manager.setup_master().await.unwrap();

        let result = manager.fork("test.js").await;
        assert!(result.is_ok());
        let worker_id = result.unwrap();
        assert_eq!(worker_id, 0);
    }

    #[test]
    async fn test_cluster_manager_send_message() {
        let config = ClusterConfig::default();
        let mut manager = ClusterManager::new(config);
        manager.setup_worker(1).await.unwrap();

        let data = serde_json::json!({"test": "message"});
        let result = manager.send_message(2, data).await;
        assert!(result.is_ok());
    }

    #[test]
    async fn test_cluster_manager_broadcast_message() {
        let config = ClusterConfig::default();
        let mut manager = ClusterManager::new(config);
        manager.setup_master().await.unwrap();

        let data = serde_json::json!({"broadcast": "message"});
        let result = manager.broadcast_message(data).await;
        assert!(result.is_ok());
    }

    #[test]
    async fn test_cluster_manager_get_worker_count() {
        let config = ClusterConfig::default();
        let mut manager = ClusterManager::new(config);
        manager.setup_master().await.unwrap();

        let count = manager.get_worker_count().await;
        assert_eq!(count, 0);

        manager.fork("test.js").await.unwrap();
        let count = manager.get_worker_count().await;
        assert_eq!(count, 1);
    }

    #[test]
    async fn test_cluster_manager_get_worker_info() {
        let config = ClusterConfig::default();
        let mut manager = ClusterManager::new(config);
        manager.setup_master().await.unwrap();

        let worker_id = manager.fork("test.js").await.unwrap();
        let info = manager.get_worker_info(worker_id).await;
        assert!(info.is_some());

        let info = info.unwrap();
        assert_eq!(info.id, worker_id);
        assert!(matches!(info.status, WorkerStatus::Starting));
    }

    #[test]
    async fn test_cluster_manager_get_all_workers() {
        let config = ClusterConfig::default();
        let mut manager = ClusterManager::new(config);
        manager.setup_master().await.unwrap();

        manager.fork("test1.js").await.unwrap();
        manager.fork("test2.js").await.unwrap();

        let workers = manager.get_all_workers().await;
        assert_eq!(workers.len(), 2);
    }

    #[test]
    async fn test_cluster_manager_kill_worker() {
        let config = ClusterConfig::default();
        let mut manager = ClusterManager::new(config);
        manager.setup_master().await.unwrap();

        let worker_id = manager.fork("test.js").await.unwrap();
        let result = manager.kill_worker(worker_id).await;
        assert!(result.is_ok());

        let info = manager.get_worker_info(worker_id).await;
        assert!(info.is_some());
        assert!(matches!(info.unwrap().status, WorkerStatus::Dead));
    }

    #[test]
    async fn test_cluster_manager_restart_worker() {
        let config = ClusterConfig::default();
        let mut manager = ClusterManager::new(config);
        manager.setup_master().await.unwrap();

        let worker_id = manager.fork("test.js").await.unwrap();
        let result = manager.restart_worker(worker_id, "test.js").await;
        assert!(result.is_ok());
    }

    #[test]
    async fn test_cluster_manager_get_load_balance_target() {
        let config = ClusterConfig::default();
        let mut manager = ClusterManager::new(config);
        manager.setup_master().await.unwrap();

        let target = manager.get_load_balance_target().await;
        assert!(target.is_none());

        let worker_id = manager.fork("test.js").await.unwrap();

        let mut workers = manager.workers.write().await;
        if let Some(worker) = workers.get_mut(&worker_id) {
            worker.status = WorkerStatus::Online;
        }
        drop(workers);

        let target = manager.get_load_balance_target().await;
        assert!(target.is_some());
    }

    #[test]
    async fn test_cluster_api_creation() {
        let api = ClusterAPI::new();
        assert!(api.cluster_manager.lock().await.is_master == false);
    }

    #[test]
    async fn test_cluster_api_register() {
        let api = ClusterAPI::new();
        let mut context = chitin::boa_engine::Context::default();
        let result = api.register(&mut context);
        assert!(result.is_ok());
    }

    #[test]
    async fn test_worker_info_creation() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let info = WorkerInfo {
            id: 1,
            pid: 1001,
            status: WorkerStatus::Online,
            start_time: now,
            message_count: 5,
            last_activity: now,
        };

        assert_eq!(info.id, 1);
        assert_eq!(info.pid, 1001);
        assert!(matches!(info.status, WorkerStatus::Online));
        assert_eq!(info.message_count, 5);
    }

    #[test]
    async fn test_cluster_message_creation() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let message = ClusterMessage {
            from: 1,
            to: 2,
            data: serde_json::json!({"test": "data"}),
            timestamp: now,
        };

        assert_eq!(message.from, 1);
        assert_eq!(message.to, 2);
        assert_eq!(message.data["test"], "data");
    }

    #[test]
    async fn test_load_balance_strategies() {
        let config = ClusterConfig {
            max_workers: 4,
            min_workers: 2,
            restart_delay: Duration::from_secs(2),
            heartbeat_interval: Duration::from_secs(10),
            load_balance_strategy: LoadBalanceStrategy::LeastConnections,
        };

        assert_eq!(config.max_workers, 4);
        assert_eq!(config.min_workers, 2);
        assert_eq!(config.restart_delay, Duration::from_secs(2));
        assert_eq!(config.heartbeat_interval, Duration::from_secs(10));
        assert!(matches!(
            config.load_balance_strategy,
            LoadBalanceStrategy::LeastConnections
        ));
    }
}
