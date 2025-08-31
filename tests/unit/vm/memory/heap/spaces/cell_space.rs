use crate::vm::handle::HeapHandleId;
use crate::vm::memory::heap::spaces::cell_space::*;
use crate::vm::memory::heap::spaces::MemorySpace;
use crate::vm::types::MemorySize;

#[test]
fn test_cell_space_new() {
    let cell_space = CellSpace::new(100);
    assert_eq!(cell_space.total_size, 100 * 16);
    assert_eq!(cell_space.total_allocated().as_usize(), 0);
    assert_eq!(cell_space.total_free().as_usize(), 100 * 16);
    assert_eq!(cell_space.stats.object_count, 0);
}

#[test]
fn test_cell_space_allocate() {
    let mut cell_space = CellSpace::new(100);

    let handle = cell_space.allocate(MemorySize::new(8));
    assert!(handle.is_some());
    assert_eq!(cell_space.total_allocated().as_usize(), 16); // Cell size
    assert_eq!(cell_space.stats.object_count, 1);
    assert_eq!(cell_space.stats.allocation_count, 1);

    // Check object type tracking
    assert_eq!(
        cell_space.object_types.get(&handle.unwrap()),
        Some(&SmallObjectType::Number)
    );
}

#[test]
fn test_cell_space_deallocate() {
    let mut cell_space = CellSpace::new(100);

    let handle = cell_space.allocate(MemorySize::new(8)).unwrap();
    assert_eq!(cell_space.stats.object_count, 1);

    assert!(cell_space.deallocate(handle));
    assert_eq!(cell_space.stats.object_count, 0);
    assert_eq!(cell_space.stats.deallocation_count, 1);

    // Type tracking should be removed
    assert!(cell_space.object_types.is_empty());
}

#[test]
fn test_cell_space_efficiency() {
    let mut cell_space = CellSpace::new(100);

    // Allocate 50 cells
    for _ in 0..50 {
        cell_space.allocate(MemorySize::new(8));
    }

    let efficiency = cell_space.efficiency();
    assert_eq!(efficiency, 50.0); // 50/100 = 50%

    let density = cell_space.memory_density();
    assert_eq!(density, 50.0); // 50*16/100*16 = 50%
}

#[test]
fn test_cell_space_compact() {
    let mut cell_space = CellSpace::new(100);

    // Allocate some cells
    let handles: Vec<HeapHandleId> = (0..20)
        .map(|_| cell_space.allocate(MemorySize::new(8)).unwrap())
        .collect();

    // Deallocate some cells to create fragmentation
    cell_space.deallocate(handles[5]);
    cell_space.deallocate(handles[10]);
    cell_space.deallocate(handles[15]);

    let initial_fragmentation = cell_space.allocator.fragmentation();

    // Compact
    let stats = cell_space.compact();
    assert!(stats.fragmentation_after < stats.fragmentation_before);
    assert!(stats.objects_moved > 0);
}

#[test]
fn test_cell_space_health_score() {
    let mut cell_space = CellSpace::new(100);

    // Empty space should have reasonable health (no fragmentation, but low efficiency)
    let health = cell_space.health_score();
    assert!(health > 30.0);

    // Allocate some cells
    for _ in 0..50 {
        cell_space.allocate(MemorySize::new(8));
    }

    // Space with 50% usage should have reasonable health
    let health = cell_space.health_score();
    assert!(health > 40.0);
}
