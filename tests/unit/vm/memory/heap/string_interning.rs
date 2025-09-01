//! String Interning Unit Tests
//! 
//! Tests for string interning system

use jetcrab::vm::memory::heap::string_interning::{StringInterner, InternedString};

#[test]
fn test_string_interner_creation() {
    let interner = StringInterner::new();
    
    assert!(interner.strings.is_empty());
    assert_eq!(interner.string_count, 0);
}

#[test]
fn test_string_interner_intern() {
    let mut interner = StringInterner::new();
    
    let string1 = "hello".to_string();
    let string2 = "world".to_string();
    
    let id1 = interner.intern(string1.clone());
    let id2 = interner.intern(string2.clone());
    
    assert_eq!(interner.string_count, 2);
    assert_ne!(id1, id2);
}

#[test]
fn test_string_interner_duplicate() {
    let mut interner = StringInterner::new();
    
    let string = "test".to_string();
    
    let id1 = interner.intern(string.clone());
    let id2 = interner.intern(string.clone());
    
    // Same string should get the same ID
    assert_eq!(id1, id2);
    assert_eq!(interner.string_count, 1);
}

#[test]
fn test_string_interner_get() {
    let mut interner = StringInterner::new();
    
    let original = "hello world".to_string();
    let id = interner.intern(original.clone());
    
    let retrieved = interner.get(id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), &original);
}

#[test]
fn test_string_interner_get_nonexistent() {
    let interner = StringInterner::new();
    
    let retrieved = interner.get(999);
    assert!(retrieved.is_none());
}

#[test]
fn test_string_interner_contains() {
    let mut interner = StringInterner::new();
    
    let string = "test string".to_string();
    let id = interner.intern(string.clone());
    
    assert!(interner.contains(&string));
    assert!(interner.contains_id(id));
}

#[test]
fn test_string_interner_clear() {
    let mut interner = StringInterner::new();
    
    interner.intern("string1".to_string());
    interner.intern("string2".to_string());
    
    assert_eq!(interner.string_count, 2);
    
    interner.clear();
    
    assert_eq!(interner.string_count, 0);
    assert!(interner.strings.is_empty());
}

#[test]
fn test_interned_string_creation() {
    let string = InternedString::new("test".to_string(), 42);
    
    assert_eq!(string.value, "test");
    assert_eq!(string.id, 42);
}

#[test]
fn test_interned_string_clone() {
    let original = InternedString::new("original".to_string(), 100);
    let cloned = original.clone();
    
    assert_eq!(original.value, cloned.value);
    assert_eq!(original.id, cloned.id);
}
