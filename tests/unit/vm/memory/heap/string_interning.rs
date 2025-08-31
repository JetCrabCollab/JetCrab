//! String Interning Unit Tests
//! 
//! Tests for string interning system

use jetcrab::vm::memory::heap::string_interning::{
    StringId, InternedString, StringInternTable, StringInterningManager,
    StringInterningConfig
};

#[test]
fn test_string_id_generation() {
    let id1 = StringId::new();
    let id2 = StringId::new();
    assert_ne!(id1, id2);
    assert!(id1.value() > 0);
    assert!(id2.value() > id1.value());
}

#[test]
fn test_interned_string_creation() {
    let interned = InternedString::new("test".to_string());
    assert_eq!(interned.value(), "test");
    assert_eq!(interned.length(), 4);
    assert_eq!(interned.ref_count(), 1);
    assert!(!interned.is_empty());
}

#[test]
fn test_interned_string_reference_counting() {
    let mut interned = InternedString::new("test".to_string());
    assert_eq!(interned.ref_count(), 1);
    
    interned.increment_ref();
    assert_eq!(interned.ref_count(), 2);
    
    assert!(!interned.decrement_ref());
    assert_eq!(interned.ref_count(), 1);
    
    assert!(interned.decrement_ref());
    assert_eq!(interned.ref_count(), 0);
}

#[test]
fn test_interned_string_empty() {
    let interned = InternedString::new("".to_string());
    assert!(interned.is_empty());
    assert_eq!(interned.length(), 0);
}

#[test]
fn test_string_intern_table_creation() {
    let table = StringInternTable::new();
    assert_eq!(table.stats().total_strings, 0);
    assert_eq!(table.stats().hits, 0);
    assert_eq!(table.stats().misses, 0);
}

#[test]
fn test_string_intern_table_intern_new_string() {
    let mut table = StringInternTable::new();
    let id = table.intern("test");
    
    assert!(id.value() > 0);
    assert_eq!(table.stats().total_strings, 1);
    assert_eq!(table.stats().misses, 1);
    assert_eq!(table.stats().hits, 0);
}

#[test]
fn test_string_intern_table_intern_existing_string() {
    let mut table = StringInternTable::new();
    let id1 = table.intern("test");
    let id2 = table.intern("test");
    
    assert_eq!(id1, id2);
    assert_eq!(table.stats().total_strings, 1);
    assert_eq!(table.stats().misses, 1);
    assert_eq!(table.stats().hits, 1);
}

#[test]
fn test_string_intern_table_get_by_id() {
    let mut table = StringInternTable::new();
    let id = table.intern("test");
    
    let interned = table.get(id);
    assert!(interned.is_some());
    let interned = interned.unwrap();
    assert_eq!(interned.value(), "test");
}

#[test]
fn test_string_intern_table_get_by_value() {
    let mut table = StringInternTable::new();
    table.intern("test");
    
    let interned = table.get_by_value("test");
    assert!(interned.is_some());
    let interned = interned.unwrap();
    assert_eq!(interned.value(), "test");
}

#[test]
fn test_string_intern_table_is_interned() {
    let mut table = StringInternTable::new();
    table.intern("test");
    
    assert!(table.is_interned("test"));
    assert!(!table.is_interned("other"));
}

#[test]
fn test_string_intern_table_release() {
    let mut table = StringInternTable::new();
    let id = table.intern("test");
    
    assert_eq!(table.stats().total_strings, 1);
    assert!(table.release(id));
    assert_eq!(table.stats().total_strings, 0);
    assert_eq!(table.stats().releases, 1);
}

#[test]
fn test_string_intern_table_memory_stats() {
    let mut table = StringInternTable::new();
    table.intern("test");
    table.intern("another");
    
    let stats = table.memory_stats();
    assert_eq!(stats.unique_strings, 2);
    assert!(stats.total_memory > 0);
    assert!(stats.avg_length > 0);
}

#[test]
fn test_string_intern_table_compact() {
    let mut table = StringInternTable::new();
    let id = table.intern("test");
    table.release(id);
    
    let removed = table.compact();
    assert_eq!(removed, 1);
    assert_eq!(table.stats().total_strings, 0);
}

#[test]
fn test_string_interning_manager_creation() {
    let config = StringInterningConfig::default();
    let manager = StringInterningManager::new(config);
    assert_eq!(manager.config().min_length, 4);
    assert_eq!(manager.config().max_length, 1024);
}

#[test]
fn test_string_interning_manager_short_string() {
    let config = StringInterningConfig::default();
    let manager = StringInterningManager::new(config);
    
    let id = manager.intern("abc");
    assert!(id.value() > 0);
    assert_eq!(manager.stats().total_strings, 0); // Short strings not interned
}

#[test]
fn test_string_interning_manager_long_string() {
    let config = StringInterningConfig::default();
    let manager = StringInterningManager::new(config);
    
    let long_string = "a".repeat(2048);
    let id = manager.intern(&long_string);
    assert!(id.value() > 0);
    assert_eq!(manager.stats().total_strings, 0); // Long strings not interned
}

#[test]
fn test_string_interning_manager_valid_string() {
    let config = StringInterningConfig::default();
    let manager = StringInterningManager::new(config);
    
    let id = manager.intern("valid_string");
    assert!(id.value() > 0);
    assert_eq!(manager.stats().total_strings, 1);
}

#[test]
fn test_string_interning_manager_duplicate_string() {
    let config = StringInterningConfig::default();
    let manager = StringInterningManager::new(config);
    
    let id1 = manager.intern("valid_string");
    let id2 = manager.intern("valid_string");
    
    assert_eq!(id1, id2);
    assert_eq!(manager.stats().total_strings, 1);
    assert_eq!(manager.stats().hits, 1);
}

#[test]
fn test_string_interning_config_default() {
    let config = StringInterningConfig::default();
    assert_eq!(config.min_length, 4);
    assert_eq!(config.max_length, 1024);
    assert!(config.auto_compact);
    assert_eq!(config.compaction_threshold, 0.1);
}
