//! String Interning - String deduplication and optimization system
//!
//! This module implements string interning that:
//! - Deduplicates identical strings
//! - Provides fast string comparison
//! - Reduces memory usage
//! - Enables string pooling

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;

/// Unique identifier for interned strings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StringId(u64);

impl Default for StringId {
    fn default() -> Self {
        Self::new()
    }
}

impl StringId {
    /// Generate a new unique string ID
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        Self(id)
    }

    /// Get the raw ID value
    pub fn value(&self) -> u64 {
        self.0
    }
}

/// Interned string entry
#[derive(Debug, Clone)]
pub struct InternedString {
    /// Unique string identifier
    pub id: StringId,
    /// The actual string value
    pub value: String,
    /// Reference count
    pub ref_count: u32,
    /// String length
    pub length: usize,
    /// Hash value for fast comparison
    pub hash: u64,
    /// Creation timestamp
    pub created_at: std::time::Instant,
}

impl InternedString {
    /// Create a new interned string
    pub fn new(value: String) -> Self {
        let hash = Self::calculate_hash(&value);
        Self {
            id: StringId::new(),
            value: value.clone(),
            ref_count: 1,
            length: value.len(),
            hash,
            created_at: std::time::Instant::now(),
        }
    }

    /// Calculate hash for the string
    fn calculate_hash(value: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    /// Increment reference count
    pub fn increment_ref(&mut self) {
        self.ref_count += 1;
    }

    /// Decrement reference count
    pub fn decrement_ref(&mut self) -> bool {
        if self.ref_count > 0 {
            self.ref_count -= 1;
        }
        self.ref_count == 0
    }

    /// Get reference count
    pub fn ref_count(&self) -> u32 {
        self.ref_count
    }

    /// Get string value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get string length
    pub fn length(&self) -> usize {
        self.length
    }

    /// Get string hash
    pub fn hash(&self) -> u64 {
        self.hash
    }

    /// Check if string is empty
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Get age of the string
    pub fn age(&self) -> std::time::Duration {
        self.created_at.elapsed()
    }
}

impl PartialEq for InternedString {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for InternedString {}

impl Hash for InternedString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// String interning table
#[derive(Debug)]
pub struct StringInternTable {
    /// String ID to interned string mapping
    strings: HashMap<StringId, InternedString>,
    /// String value to string ID mapping
    value_to_id: HashMap<String, StringId>,
    /// Hash to string ID mapping for collision resolution
    hash_to_id: HashMap<u64, Vec<StringId>>,
    /// Statistics
    stats: StringInternStats,
}

impl StringInternTable {
    /// Create a new string interning table
    pub fn new() -> Self {
        Self {
            strings: HashMap::new(),
            value_to_id: HashMap::new(),
            hash_to_id: HashMap::new(),
            stats: StringInternStats::default(),
        }
    }

    /// Intern a string (get existing or create new)
    pub fn intern(&mut self, value: &str) -> StringId {
        // Check if string already exists
        if let Some(&id) = self.value_to_id.get(value) {
            // Increment reference count
            if let Some(string) = self.strings.get_mut(&id) {
                string.increment_ref();
            }
            self.stats.hits += 1;
            return id;
        }

        // Create new interned string
        let interned = InternedString::new(value.to_string());
        let id = interned.id;
        let hash = interned.hash;

        // Store the string
        self.strings.insert(id, interned);
        self.value_to_id.insert(value.to_string(), id);

        // Update hash mapping
        self.hash_to_id.entry(hash).or_default().push(id);

        // Update statistics
        self.stats.total_strings += 1;
        self.stats.total_memory += value.len();
        self.stats.misses += 1;

        id
    }

    /// Get interned string by ID
    pub fn get(&self, id: StringId) -> Option<&InternedString> {
        self.strings.get(&id)
    }

    /// Get interned string by value
    pub fn get_by_value(&self, value: &str) -> Option<&InternedString> {
        self.value_to_id
            .get(value)
            .and_then(|&id| self.strings.get(&id))
    }

    /// Check if string is interned
    pub fn is_interned(&self, value: &str) -> bool {
        self.value_to_id.contains_key(value)
    }

    /// Release a string reference
    pub fn release(&mut self, id: StringId) -> bool {
        if let Some(string) = self.strings.get_mut(&id) {
            if string.decrement_ref() {
                // String has no more references, remove it
                let value = string.value.clone();
                let hash = string.hash;

                // Remove from all mappings
                self.strings.remove(&id);
                self.value_to_id.remove(&value);

                // Remove from hash mapping
                if let Some(ids) = self.hash_to_id.get_mut(&hash) {
                    ids.retain(|&x| x != id);
                    if ids.is_empty() {
                        self.hash_to_id.remove(&hash);
                    }
                }

                // Update statistics
                self.stats.total_strings -= 1;
                self.stats.total_memory = self.stats.total_memory.saturating_sub(value.len());
                self.stats.releases += 1;

                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Get all interned strings
    pub fn all_strings(&self) -> &HashMap<StringId, InternedString> {
        &self.strings
    }

    /// Get statistics
    pub fn stats(&self) -> &StringInternStats {
        &self.stats
    }

    /// Get memory usage statistics
    pub fn memory_stats(&self) -> StringMemoryStats {
        let unique_strings = self.strings.len();
        let total_memory = self.stats.total_memory;
        let avg_length = if unique_strings > 0 {
            total_memory / unique_strings
        } else {
            0
        };

        StringMemoryStats {
            unique_strings,
            total_memory,
            avg_length,
            memory_saved: self.calculate_memory_saved(),
        }
    }

    /// Calculate memory saved through interning
    fn calculate_memory_saved(&self) -> usize {
        let mut memory_saved = 0;
        for string in self.strings.values() {
            if string.ref_count > 1 {
                memory_saved += string.length * (string.ref_count as usize - 1);
            }
        }
        memory_saved
    }

    /// Clear all strings (for testing/debugging)
    pub fn clear(&mut self) {
        self.strings.clear();
        self.value_to_id.clear();
        self.hash_to_id.clear();
        self.stats = StringInternStats::default();
    }

    /// Compact the table by removing unreferenced strings
    pub fn compact(&mut self) -> usize {
        let mut removed = 0;
        let ids_to_remove: Vec<StringId> = self
            .strings
            .iter()
            .filter(|(_, string)| string.ref_count == 0)
            .map(|(&id, _)| id)
            .collect();

        for id in ids_to_remove {
            if self.release(id) {
                removed += 1;
            }
        }

        removed
    }
}

impl Default for StringInternTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe string interning table
#[derive(Debug)]
pub struct ThreadSafeStringInternTable {
    inner: RwLock<StringInternTable>,
}

impl ThreadSafeStringInternTable {
    /// Create a new thread-safe string interning table
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(StringInternTable::new()),
        }
    }

    /// Intern a string
    pub fn intern(&self, value: &str) -> StringId {
        self.inner.write().unwrap().intern(value)
    }

    /// Get interned string by ID
    pub fn get(&self, id: StringId) -> Option<InternedString> {
        self.inner.read().unwrap().get(id).cloned()
    }

    /// Get interned string by value
    pub fn get_by_value(&self, value: &str) -> Option<InternedString> {
        self.inner.read().unwrap().get_by_value(value).cloned()
    }

    /// Check if string is interned
    pub fn is_interned(&self, value: &str) -> bool {
        self.inner.read().unwrap().is_interned(value)
    }

    /// Release a string reference
    pub fn release(&self, id: StringId) -> bool {
        self.inner.write().unwrap().release(id)
    }

    /// Get statistics
    pub fn stats(&self) -> StringInternStats {
        self.inner.read().unwrap().stats().clone()
    }

    /// Get memory statistics
    pub fn memory_stats(&self) -> StringMemoryStats {
        self.inner.read().unwrap().memory_stats()
    }

    /// Compact the table
    pub fn compact(&self) -> usize {
        self.inner.write().unwrap().compact()
    }
}

impl Default for ThreadSafeStringInternTable {
    fn default() -> Self {
        Self::new()
    }
}

/// String interning statistics
#[derive(Debug, Clone, Default)]
pub struct StringInternStats {
    /// Total number of interned strings
    pub total_strings: usize,
    /// Total memory used by unique strings
    pub total_memory: usize,
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Number of string releases
    pub releases: u64,
}

/// String memory statistics
#[derive(Debug, Clone)]
pub struct StringMemoryStats {
    /// Number of unique strings
    pub unique_strings: usize,
    /// Total memory used
    pub total_memory: usize,
    /// Average string length
    pub avg_length: usize,
    /// Memory saved through interning
    pub memory_saved: usize,
}

/// String interning manager
#[derive(Debug)]
pub struct StringInterningManager {
    /// Main interning table
    table: ThreadSafeStringInternTable,
    /// Configuration
    config: StringInterningConfig,
}

impl StringInterningManager {
    /// Create a new string interning manager
    pub fn new(config: StringInterningConfig) -> Self {
        Self {
            table: ThreadSafeStringInternTable::new(),
            config,
        }
    }

    /// Intern a string
    pub fn intern(&self, value: &str) -> StringId {
        // Check if string meets minimum length requirement
        if value.len() < self.config.min_length {
            return StringId::new(); // Return new ID for short strings
        }

        // Check if string meets maximum length requirement
        if value.len() > self.config.max_length {
            return StringId::new(); // Return new ID for very long strings
        }

        self.table.intern(value)
    }

    /// Get interned string by ID
    pub fn get(&self, id: StringId) -> Option<InternedString> {
        self.table.get(id)
    }

    /// Get interned string by value
    pub fn get_by_value(&self, value: &str) -> Option<InternedString> {
        self.table.get_by_value(value)
    }

    /// Check if string is interned
    pub fn is_interned(&self, value: &str) -> bool {
        self.table.is_interned(value)
    }

    /// Release a string reference
    pub fn release(&self, id: StringId) -> bool {
        self.table.release(id)
    }

    /// Get statistics
    pub fn stats(&self) -> StringInternStats {
        self.table.stats()
    }

    /// Get memory statistics
    pub fn memory_stats(&self) -> StringMemoryStats {
        self.table.memory_stats()
    }

    /// Compact the table
    pub fn compact(&self) -> usize {
        self.table.compact()
    }

    /// Get configuration
    pub fn config(&self) -> &StringInterningConfig {
        &self.config
    }
}

impl Default for StringInterningManager {
    fn default() -> Self {
        Self::new(StringInterningConfig::default())
    }
}

/// String interning configuration
#[derive(Debug, Clone)]
pub struct StringInterningConfig {
    /// Minimum string length to intern
    pub min_length: usize,
    /// Maximum string length to intern
    pub max_length: usize,
    /// Enable automatic compaction
    pub auto_compact: bool,
    /// Compaction threshold (percentage of unreferenced strings)
    pub compaction_threshold: f64,
}

impl Default for StringInterningConfig {
    fn default() -> Self {
        Self {
            min_length: 4,    // Only intern strings >= 4 characters
            max_length: 1024, // Only intern strings <= 1KB
            auto_compact: true,
            compaction_threshold: 0.1, // Compact when 10% are unreferenced
        }
    }
}
