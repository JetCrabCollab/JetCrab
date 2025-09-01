use jetcrab::vm::memory::heap::generational::{GenerationalGc, Generation, GenerationConfig};

#[test]
fn test_generational_gc_creation() {
    let config = GenerationConfig::default();
    let gc = GenerationalGc::new(config);
    
    assert!(gc.generations.is_empty());
    assert_eq!(gc.generation_count, 0);
}

#[test]
fn test_generational_gc_add_generation() {
    let mut gc = GenerationalGc::new(GenerationConfig::default());
    
    let young_gen = Generation::new("young".to_string(), 64 * 1024 * 1024); // 64MB
    let old_gen = Generation::new("old".to_string(), 256 * 1024 * 1024); // 256MB
    
    gc.add_generation(young_gen);
    gc.add_generation(old_gen);
    
    assert_eq!(gc.generation_count, 2);
    assert_eq!(gc.generations.len(), 2);
}

#[test]
fn test_generational_gc_get_generation() {
    let mut gc = GenerationalGc::new(GenerationConfig::default());
    
    let gen = Generation::new("test".to_string(), 1024 * 1024);
    gc.add_generation(gen);
    
    let retrieved = gc.get_generation("test");
    assert!(retrieved.is_some());
    
    let non_existent = gc.get_generation("nonexistent");
    assert!(non_existent.is_none());
}

#[test]
fn test_generation_creation() {
    let gen = Generation::new("test_gen".to_string(), 1024 * 1024);
    
    assert_eq!(gen.name, "test_gen");
    assert_eq!(gen.size, 1024 * 1024);
    assert_eq!(gen.allocated, 0);
    assert_eq!(gen.object_count, 0);
}

#[test]
fn test_generation_allocation() {
    let mut gen = Generation::new("test".to_string(), 1024 * 1024);
    
    let allocated = gen.allocate(1024);
    assert!(allocated);
    
    assert_eq!(gen.allocated, 1024);
    assert_eq!(gen.object_count, 1);
}

#[test]
fn test_generation_is_full() {
    let mut gen = Generation::new("test".to_string(), 1024);
    
    assert!(!gen.is_full());
    
    gen.allocate(1024);
    assert!(gen.is_full());
}

#[test]
fn test_generation_usage_percentage() {
    let mut gen = Generation::new("test".to_string(), 1000);
    
    assert_eq!(gen.usage_percentage(), 0.0);
    
    gen.allocate(500);
    assert_eq!(gen.usage_percentage(), 50.0);
    
    gen.allocate(500);
    assert_eq!(gen.usage_percentage(), 100.0);
}

#[test]
fn test_generation_config_default() {
    let config = GenerationConfig::default();
    
    assert_eq!(config.young_generation_size, 64 * 1024 * 1024); // 64MB
    assert_eq!(config.old_generation_size, 256 * 1024 * 1024); // 256MB
    assert_eq!(config.promotion_threshold, 10);
    assert!(config.enable_compaction);
}

#[test]
fn test_generation_config_custom() {
    let config = GenerationConfig::new()
        .with_young_generation_size(32 * 1024 * 1024) // 32MB
        .with_old_generation_size(128 * 1024 * 1024) // 128MB
        .with_promotion_threshold(5)
        .with_compaction(false);
    
    assert_eq!(config.young_generation_size, 32 * 1024 * 1024);
    assert_eq!(config.old_generation_size, 128 * 1024 * 1024);
    assert_eq!(config.promotion_threshold, 5);
    assert!(!config.enable_compaction);
}
