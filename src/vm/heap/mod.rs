pub mod allocation;
pub mod entries;
pub mod garbage_collection;
pub mod management;
pub mod types;

pub use allocation::HeapAllocatorImpl;
pub use entries::HeapEntry;
pub use garbage_collection::GarbageCollectorImpl;
pub use management::{Heap, HeapAllocator, GarbageCollector, HeapManager, HeapOperations};
pub use types::{HeapStats, HeapMetrics};
