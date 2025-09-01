use jetcrab::vm::memory::{Memory, MemoryConfig, MemoryStats};

#[test]
fn test_memory_creation() {
    let config = MemoryConfig::default();
    let memory = Memory::new(config);
    
    assert!(memory.is_initialized());
    assert_eq!(memory.total_size, config.initial_size);
}

#[test]
fn test_memory_with_custom_config() {
    let config = MemoryConfig::new()
        .with_initial_size(128 * 1024 * 1024) // 128MB
        .with_max_size(512 * 1024 * 1024); // 512MB
    
    let memory = Memory::new(config);
    
    assert_eq!(memory.total_size, 128 * 1024 * 1024);
    assert_eq!(memory.max_size, 512 * 1024 * 1024);
}

#[test]
fn test_memory_allocation() {
    let mut memory = Memory::new(MemoryConfig::default());
    
    let addr = memory.allocate(1024);
    assert!(addr.is_some());
    
    let stats = memory.stats();
    assert_eq!(stats.total_allocated, 1024);
    assert_eq!(stats.allocation_count, 1);
}

#[test]
fn test_memory_deallocation() {
    let mut memory = Memory::new(MemoryConfig::default());
    
    let addr = memory.allocate(1024).unwrap();
    
    let result = memory.deallocate(addr);
    assert!(result);
    
    let stats = memory.stats();
    assert_eq!(stats.total_allocated, 0);
    assert_eq!(stats.deallocation_count, 1);
}

#[test]
fn test_memory_stats() {
    let mut memory = Memory::new(MemoryConfig::default());
    
    memory.allocate(1024);
    memory.allocate(2048);
    
    let stats = memory.stats();
    
    assert_eq!(stats.total_allocated, 3072);
    assert_eq!(stats.allocation_count, 2);
    assert_eq!(stats.deallocation_count, 0);
    assert!(stats.fragmentation >= 0.0);
}

#[test]
fn test_memory_config_default() {
    let config = MemoryConfig::default();
    
    assert_eq!(config.initial_size, 64 * 1024 * 1024); // 64MB
    assert_eq!(config.max_size, 1024 * 1024 * 1024); // 1GB
    assert_eq!(config.page_size, 4096);
    assert!(config.enable_compaction);
}

#[test]
fn test_memory_config_custom() {
    let config = MemoryConfig::new()
        .with_initial_size(32 * 1024 * 1024) // 32MB
        .with_max_size(256 * 1024 * 1024) // 256MB
        .with_page_size(8192)
        .with_compaction(false);
    
    assert_eq!(config.initial_size, 32 * 1024 * 1024);
    assert_eq!(config.max_size, 256 * 1024 * 1024);
    assert_eq!(config.page_size, 8192);
    assert!(!config.enable_compaction);
}

#[test]
fn test_memory_page_management() {
    let mut memory = Memory::new(MemoryConfig::default());
    
    // Allocate memory that spans multiple pages
    let large_addr = memory.allocate(8192); // 2 pages
    assert!(large_addr.is_some());
    
    let stats = memory.stats();
    assert!(stats.page_count >= 2);
}

#[test]
fn test_memory_fragmentation() {
    let mut memory = Memory::new(MemoryConfig::default());
    
    // Allocate and deallocate to create fragmentation
    let handles: Vec<_> = (0..10)
        .map(|_| memory.allocate(1024).unwrap())
        .collect();
    
    // Deallocate some to create holes
    memory.deallocate(handles[2]);
    memory.deallocate(handles[5]);
    memory.deallocate(handles[8]);
    
    let before_fragmentation = memory.stats().fragmentation;
    
    // Compact the memory
    let compaction_stats = memory.compact();
    
    let after_fragmentation = memory.stats().fragmentation;
    assert!(after_fragmentation < before_fragmentation);
    assert!(compaction_stats.pages_merged > 0);
}

#[test]
fn test_memory_limits() {
    let config = MemoryConfig::new()
        .with_max_size(1024 * 1024); // 1MB
    
    let mut memory = Memory::new(config);
    
    // Try to allocate more than the limit
    let large_allocation = memory.allocate(2 * 1024 * 1024); // 2MB
    assert!(large_allocation.is_none());
    
    // Small allocation should work
    let small_allocation = memory.allocate(512);
    assert!(small_allocation.is_some());
}
