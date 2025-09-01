use jetcrab::vm::memory::heap::gc::write_barrier::WriteBarrier;
use jetcrab::vm::memory::heap::gc::card_table::CardTable;
use jetcrab::vm::handle::HeapHandleId;

#[test]
fn test_write_barrier_creation() {
    let card_table = CardTable::new(1024 * 1024); // 1MB
    let write_barrier = WriteBarrier::new(card_table);
    
    assert!(write_barrier.is_enabled());
}

#[test]
fn test_write_barrier_enable_disable() {
    let card_table = CardTable::new(1024 * 1024);
    let mut write_barrier = WriteBarrier::new(card_table);
    
    assert!(write_barrier.is_enabled());
    
    write_barrier.disable();
    assert!(!write_barrier.is_enabled());
    
    write_barrier.enable();
    assert!(write_barrier.is_enabled());
}

#[test]
fn test_write_barrier_record_write() {
    let card_table = CardTable::new(1024 * 1024);
    let mut write_barrier = WriteBarrier::new(card_table);
    
    let old_value = HeapHandleId::new(100);
    let new_value = HeapHandleId::new(200);
    let address = 0x1000;
    
    // Record a write operation
    write_barrier.record_write(old_value, new_value, address);
    
    // The card table should be marked as dirty
    assert!(write_barrier.card_table().is_dirty(address));
}

#[test]
fn test_write_barrier_card_table_access() {
    let card_table = CardTable::new(1024 * 1024);
    let write_barrier = WriteBarrier::new(card_table);
    
    let card_table_ref = write_barrier.card_table();
    assert_eq!(card_table_ref.size(), 1024 * 1024);
}

#[test]
fn test_write_barrier_multiple_writes() {
    let card_table = CardTable::new(1024 * 1024);
    let mut write_barrier = WriteBarrier::new(card_table);
    
    // Record multiple writes
    write_barrier.record_write(HeapHandleId::new(1), HeapHandleId::new(2), 0x1000);
    write_barrier.record_write(HeapHandleId::new(3), HeapHandleId::new(4), 0x2000);
    write_barrier.record_write(HeapHandleId::new(5), HeapHandleId::new(6), 0x3000);
    
    // All addresses should be marked as dirty
    assert!(write_barrier.card_table().is_dirty(0x1000));
    assert!(write_barrier.card_table().is_dirty(0x2000));
    assert!(write_barrier.card_table().is_dirty(0x3000));
}

#[test]
fn test_write_barrier_no_change() {
    let card_table = CardTable::new(1024 * 1024);
    let mut write_barrier = WriteBarrier::new(card_table);
    
    let same_value = HeapHandleId::new(100);
    
    // Record a write with the same value (no actual change)
    write_barrier.record_write(same_value, same_value, 0x1000);
    
    // The card should still be marked as dirty since we recorded a write
    assert!(write_barrier.card_table().is_dirty(0x1000));
}
