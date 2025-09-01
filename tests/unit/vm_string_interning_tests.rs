//! Unit tests for VM String Interning

use jetcrab::vm::memory::heap::string_interning::{
    InternedString, StringId, StringInternStats, StringInternTable, StringInterningConfig,
    StringInterningManager, StringMemoryStats, ThreadSafeStringInternTable,
};

#[test]
fn test_string_id_creation() {
    let string_id = StringId::new();
    assert!(string_id.value() > 0);
}

#[test]
fn test_string_id_default() {
    let string_id = StringId::default();
    assert!(string_id.value() > 0);
}

#[test]
fn test_string_id_value() {
    let string_id = StringId::new();
    let value = string_id.value();
    assert!(value > 0);
}

#[test]
fn test_string_id_equality() {
    let string_id1 = StringId::new();
    let string_id2 = StringId::new();
    assert_ne!(string_id1, string_id2);
    assert_eq!(string_id1, string_id1);
}

#[test]
fn test_interned_string_new() {
    let interned = InternedString::new("test".to_string());
    assert_eq!(interned.value(), "test");
    assert_eq!(interned.length(), 4);
    assert_eq!(interned.ref_count(), 1);
    assert!(!interned.is_empty());
    assert!(interned.id.value() > 0);
}

#[test]
fn test_interned_string_empty() {
    let interned = InternedString::new("".to_string());
    assert_eq!(interned.value(), "");
    assert_eq!(interned.length(), 0);
    assert!(interned.is_empty());
}

#[test]
fn test_interned_string_increment_ref() {
    let mut interned = InternedString::new("test".to_string());
    assert_eq!(interned.ref_count(), 1);

    interned.increment_ref();
    assert_eq!(interned.ref_count(), 2);

    interned.increment_ref();
    assert_eq!(interned.ref_count(), 3);
}

#[test]
fn test_interned_string_decrement_ref() {
    let mut interned = InternedString::new("test".to_string());
    interned.increment_ref();
    interned.increment_ref();
    assert_eq!(interned.ref_count(), 3);

    let should_remove = interned.decrement_ref();
    assert!(!should_remove);
    assert_eq!(interned.ref_count(), 2);

    let should_remove = interned.decrement_ref();
    assert!(!should_remove);
    assert_eq!(interned.ref_count(), 1);

    let should_remove = interned.decrement_ref();
    assert!(should_remove);
    assert_eq!(interned.ref_count(), 0);
}

#[test]
fn test_interned_string_decrement_ref_below_zero() {
    let mut interned = InternedString::new("test".to_string());
    assert_eq!(interned.ref_count(), 1);

    let should_remove = interned.decrement_ref();
    assert!(should_remove);
    assert_eq!(interned.ref_count(), 0);

    let should_remove = interned.decrement_ref();
    assert!(should_remove);
    assert_eq!(interned.ref_count(), 0);
}

#[test]
fn test_interned_string_hash() {
    let interned1 = InternedString::new("test".to_string());
    let interned2 = InternedString::new("test".to_string());

    assert_eq!(interned1.hash(), interned2.hash());

    let interned3 = InternedString::new("different".to_string());
    assert_ne!(interned1.hash(), interned3.hash());
}

#[test]
fn test_interned_string_age() {
    let interned = InternedString::new("test".to_string());
    let age = interned.age();
    assert!(age.as_nanos() >= 0);
}

#[test]
fn test_interned_string_equality() {
    let interned1 = InternedString::new("test".to_string());
    let interned2 = InternedString::new("test".to_string());

    assert_ne!(interned1, interned2);
    assert_eq!(interned1, interned1);
}

#[test]
fn test_string_intern_table_new() {
    let table = StringInternTable::new();
    let stats = table.stats();
    assert_eq!(stats.total_strings, 0);
    assert_eq!(stats.total_memory, 0);
}

#[test]
fn test_string_intern_table_default() {
    let table = StringInternTable::default();
    let stats = table.stats();
    assert_eq!(stats.total_strings, 0);
}

#[test]
fn test_string_intern_table_intern() {
    let mut table = StringInternTable::new();
    let string_id = table.intern("test");

    assert!(string_id.value() > 0);
    let stats = table.stats();
    assert_eq!(stats.total_strings, 1);
}

#[test]
fn test_string_intern_table_intern_duplicate() {
    let mut table = StringInternTable::new();
    let string_id1 = table.intern("test");
    let string_id2 = table.intern("test");

    assert_eq!(string_id1, string_id2);
    let stats = table.stats();
    assert_eq!(stats.total_strings, 1);
}

#[test]
fn test_string_intern_table_intern_different() {
    let mut table = StringInternTable::new();
    let string_id1 = table.intern("test1");
    let string_id2 = table.intern("test2");

    assert_ne!(string_id1, string_id2);
    let stats = table.stats();
    assert_eq!(stats.total_strings, 2);
}

#[test]
fn test_string_intern_table_get() {
    let mut table = StringInternTable::new();
    let string_id = table.intern("test");

    let interned = table.get(string_id);
    assert!(interned.is_some());
    assert_eq!(interned.unwrap().value(), "test");

    let not_found = table.get(StringId::new());
    assert!(not_found.is_none());
}

#[test]
fn test_string_intern_table_get_by_value() {
    let mut table = StringInternTable::new();
    let _string_id = table.intern("test");

    let interned = table.get_by_value("test");
    assert!(interned.is_some());
    assert_eq!(interned.unwrap().value(), "test");

    let not_found = table.get_by_value("nonexistent");
    assert!(not_found.is_none());
}

#[test]
fn test_string_intern_table_contains() {
    let mut table = StringInternTable::new();
    let string_id = table.intern("test");

    assert!(table.get(string_id).is_some());
    assert!(table.is_interned("test"));
    assert!(!table.is_interned("nonexistent"));
}

#[test]
fn test_string_intern_table_release() {
    let mut table = StringInternTable::new();
    let string_id = table.intern("test");

    let released = table.release(string_id);
    assert!(released);

    let not_found = table.get(string_id);
    assert!(not_found.is_none());
}

#[test]
fn test_string_intern_table_release_nonexistent() {
    let mut table = StringInternTable::new();
    let string_id = StringId::new();

    let released = table.release(string_id);
    assert!(!released);
}

#[test]
fn test_string_intern_table_release_with_multiple_refs() {
    let mut table = StringInternTable::new();
    let string_id1 = table.intern("test");
    let string_id2 = table.intern("test");

    assert_eq!(string_id1, string_id2);

    let released = table.release(string_id1);
    assert!(!released);

    let interned = table.get(string_id1);
    assert!(interned.is_some());
    assert_eq!(interned.unwrap().ref_count(), 1);
}

#[test]
fn test_string_intern_table_clear() {
    let mut table = StringInternTable::new();
    let _string_id1 = table.intern("test1");
    let _string_id2 = table.intern("test2");

    table.clear();
    let stats = table.stats();
    assert_eq!(stats.total_strings, 0);
}

#[test]
fn test_string_intern_table_stats() {
    let mut table = StringInternTable::new();
    let _string_id1 = table.intern("test1");
    let _string_id2 = table.intern("test2");
    let _string_id3 = table.intern("test1");

    let stats = table.stats();
    assert_eq!(stats.total_strings, 2);
    assert!(stats.total_memory > 0);
}

#[test]
fn test_string_intern_table_compact() {
    let mut table = StringInternTable::new();
    let string_id1 = table.intern("test1");
    let string_id2 = table.intern("test2");

    table.release(string_id1);
    table.release(string_id2);

    let removed_count = table.compact();
    assert_eq!(removed_count, 0);

    let stats = table.stats();
    assert_eq!(stats.total_strings, 0);
}

#[test]
fn test_string_intern_table_memory_stats() {
    let mut table = StringInternTable::new();
    let _string_id1 = table.intern("test1");
    let _string_id2 = table.intern("test2");

    let memory_stats = table.memory_stats();
    assert!(memory_stats.unique_strings >= 0);
    assert!(memory_stats.total_memory > 0);
}

#[test]
fn test_string_intern_table_all_strings() {
    let mut table = StringInternTable::new();
    let _string_id1 = table.intern("test1");
    let _string_id2 = table.intern("test2");

    let all_strings = table.all_strings();
    assert_eq!(all_strings.len(), 2);
}

#[test]
fn test_string_intern_stats() {
    let stats = StringInternStats {
        total_strings: 10,
        total_memory: 1024,
        hits: 5,
        misses: 3,
        releases: 2,
    };

    assert_eq!(stats.total_strings, 10);
    assert_eq!(stats.total_memory, 1024);
    assert_eq!(stats.hits, 5);
    assert_eq!(stats.misses, 3);
    assert_eq!(stats.releases, 2);
}

#[test]
fn test_string_intern_table_memory_usage() {
    let mut table = StringInternTable::new();
    let _string_id = table.intern("test");

    let memory_stats = table.memory_stats();
    assert!(memory_stats.total_memory > 0);
}

#[test]
fn test_string_memory_stats() {
    let memory_stats = StringMemoryStats {
        unique_strings: 5,
        total_memory: 1024,
        avg_length: 20,
        memory_saved: 100,
    };

    assert_eq!(memory_stats.unique_strings, 5);
    assert_eq!(memory_stats.total_memory, 1024);
    assert_eq!(memory_stats.avg_length, 20);
    assert_eq!(memory_stats.memory_saved, 100);
}

#[test]
fn test_string_interning_config() {
    let config = StringInterningConfig {
        min_length: 4,
        max_length: 1024,
        auto_compact: true,
        compaction_threshold: 0.1,
    };

    assert_eq!(config.min_length, 4);
    assert_eq!(config.max_length, 1024);
    assert!(config.auto_compact);
    assert_eq!(config.compaction_threshold, 0.1);
}

#[test]
fn test_string_interning_config_default() {
    let config = StringInterningConfig::default();
    assert_eq!(config.min_length, 4);
    assert_eq!(config.max_length, 1024);
    assert!(config.auto_compact);
    assert_eq!(config.compaction_threshold, 0.1);
}

#[test]
fn test_string_interning_manager() {
    let config = StringInterningConfig::default();
    let manager = StringInterningManager::new(config);

    let string_id = manager.intern("test");
    assert!(string_id.value() > 0);

    let interned = manager.get(string_id);
    assert!(interned.is_some());
    assert_eq!(interned.unwrap().value(), "test");
}

#[test]
fn test_string_interning_manager_default() {
    let manager = StringInterningManager::default();
    let string_id = manager.intern("test");
    assert!(string_id.value() > 0);
}

#[test]
fn test_thread_safe_string_intern_table() {
    let table = ThreadSafeStringInternTable::new();
    let string_id = table.intern("test");

    assert!(string_id.value() > 0);

    let interned = table.get(string_id);
    assert!(interned.is_some());
    assert_eq!(interned.unwrap().value(), "test");
}
