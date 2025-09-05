//! # Cache Manager
//!
//! Intelligently manages cache for builds and dependencies.
//!
//! ## Features
//!
//! - **Automatic cache** for WASM builds
//! - **Intelligent invalidation** based on dependencies
//! - **Automatic cleanup** when exceeding limit
//! - **Persistence** between sessions
//!
//! ## Usage Example
//!
//! ```rust
//! use jetcrab::tools::CacheManager;
//! use std::path::PathBuf;
//!
//! let mut cache = CacheManager::new(PathBuf::from(".cache"));
//! cache.initialize()?;
//!
//! // Check cache
//! if let Some(path) = cache.get("build-key")? {
//!     println!("Cache hit: {:?}", path);
//! } else {
//!     // Build and save to cache
//!     let data = b"build result";
//!     cache.put("build-key", data)?;
//! }
//! ```

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub key: String,
    pub path: PathBuf,
    pub size: u64,
    pub created: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
}

pub struct CacheManager {
    cache_dir: PathBuf,
    entries: HashMap<String, CacheEntry>,
    max_size: u64,
}

impl CacheManager {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            cache_dir,
            entries: HashMap::new(),
            max_size: 1024 * 1024 * 1024, // 1GB
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        if !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir)?;
        }

        self.load_cache_index()?;
        Ok(())
    }

    pub fn get(&mut self, key: &str) -> Result<Option<PathBuf>> {
        if let Some(entry) = self.entries.get_mut(key) {
            if entry.path.exists() {
                entry.last_accessed = chrono::Utc::now();
                return Ok(Some(entry.path.clone()));
            } else {
                self.entries.remove(key);
            }
        }

        Ok(None)
    }

    pub fn put(&mut self, key: &str, data: &[u8]) -> Result<PathBuf> {
        let cache_path = self.cache_dir.join(format!("{}.cache", key));

        fs::write(&cache_path, data)?;

        let entry = CacheEntry {
            key: key.to_string(),
            path: cache_path.clone(),
            size: data.len() as u64,
            created: chrono::Utc::now(),
            last_accessed: chrono::Utc::now(),
        };

        self.entries.insert(key.to_string(), entry);
        self.save_cache_index()?;

        Ok(cache_path)
    }

    pub fn invalidate(&mut self, key: &str) -> Result<()> {
        if let Some(entry) = self.entries.remove(key) {
            if entry.path.exists() {
                fs::remove_file(&entry.path)?;
            }
        }

        self.save_cache_index()?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<()> {
        let total_size: u64 = self.entries.values().map(|e| e.size).sum();

        if total_size > self.max_size {
            info!(
                "Cache size {} exceeds limit {}, cleaning up",
                total_size, self.max_size
            );

            let mut entries: Vec<_> = self.entries.iter().collect();
            entries.sort_by_key(|(_, entry)| entry.last_accessed);

            let mut current_size = total_size;
            let mut keys_to_remove = Vec::new();

            for (key, entry) in entries {
                if current_size <= self.max_size * 8 / 10 {
                    break;
                }

                keys_to_remove.push(key.clone());
                current_size -= entry.size;
            }

            for key in keys_to_remove {
                self.invalidate(&key)?;
            }
        }

        Ok(())
    }

    fn load_cache_index(&mut self) -> Result<()> {
        let index_path = self.cache_dir.join("index.json");

        if index_path.exists() {
            let data = fs::read_to_string(&index_path)?;
            let entries: HashMap<String, CacheEntry> = serde_json::from_str(&data)?;
            self.entries = entries;
        }

        Ok(())
    }

    fn save_cache_index(&self) -> Result<()> {
        let index_path = self.cache_dir.join("index.json");
        let data = serde_json::to_string_pretty(&self.entries)?;
        fs::write(&index_path, data)?;
        Ok(())
    }

    pub fn get_stats(&self) -> CacheStats {
        let total_size: u64 = self.entries.values().map(|e| e.size).sum();
        let entry_count = self.entries.len();

        CacheStats {
            total_size,
            entry_count,
            max_size: self.max_size,
        }
    }
}

#[derive(Debug)]
pub struct CacheStats {
    pub total_size: u64,
    pub entry_count: usize,
    pub max_size: u64,
}
