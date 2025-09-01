use super::{MemorySpace, SpaceStats, SpaceType};
use crate::vm::handle::HeapHandleId;
use crate::vm::types::MemorySize;
use std::collections::HashMap;

pub struct PromotionPolicies {
    pub new_space_threshold: f64,
    pub old_space_threshold: f64,
    pub large_object_threshold: usize,
    pub tenuring_threshold: usize,
}

impl Default for PromotionPolicies {
    fn default() -> Self {
        Self {
            new_space_threshold: 0.75,
            old_space_threshold: 0.85,
            large_object_threshold: 1024 * 1024,
            tenuring_threshold: 3,
        }
    }
}

pub struct AllocationStrategies {
    pub small_object_threshold: usize,
    pub medium_object_threshold: usize,
    pub large_object_threshold: usize,
}

impl Default for AllocationStrategies {
    fn default() -> Self {
        Self {
            small_object_threshold: 64,
            medium_object_threshold: 1024,
            large_object_threshold: 1024 * 1024,
        }
    }
}

pub struct SpaceCoordinator {
    spaces: HashMap<SpaceType, Box<dyn MemorySpace>>,
    promotion_policies: PromotionPolicies,
    allocation_strategies: AllocationStrategies,
    object_generations: HashMap<HeapHandleId, usize>,
    object_tenure: HashMap<HeapHandleId, usize>,
}

impl SpaceCoordinator {
    pub fn new() -> Self {
        Self {
            spaces: HashMap::new(),
            promotion_policies: PromotionPolicies::default(),
            allocation_strategies: AllocationStrategies::default(),
            object_generations: HashMap::new(),
            object_tenure: HashMap::new(),
        }
    }

    pub fn register_space(&mut self, space_type: SpaceType, space: Box<dyn MemorySpace>) {
        self.spaces.insert(space_type, space);
    }

    pub fn allocate(
        &mut self,
        size: MemorySize,
        object_type: super::ObjectType,
    ) -> Option<HeapHandleId> {
        let space_type = self.select_space_for_allocation(size, object_type);

        if let Some(space) = self.spaces.get_mut(&space_type) {
            if let Some(handle) = space.allocate(size) {
                self.object_generations.insert(handle, 0);
                self.object_tenure.insert(handle, 0);
                return Some(handle);
            }
        }

        None
    }

    pub fn deallocate(&mut self, handle: HeapHandleId) -> bool {
        for space in self.spaces.values_mut() {
            if space.deallocate(handle) {
                self.object_generations.remove(&handle);
                self.object_tenure.remove(&handle);
                return true;
            }
        }
        false
    }

    pub fn promote_object(&mut self, handle: HeapHandleId) -> bool {
        let current_generation = self.object_generations.get(&handle).copied().unwrap_or(0);
        let new_generation = current_generation + 1;

        // Get space types
        let old_space_type = match current_generation {
            0 => SpaceType::NewSpace,
            1 => SpaceType::OldSpace,
            _ => SpaceType::OldSpace,
        };

        let new_space_type = match new_generation {
            0 => SpaceType::NewSpace,
            1 => SpaceType::OldSpace,
            _ => SpaceType::OldSpace,
        };

        // Extract object from old space
        if let Some(old_space) = self.spaces.get_mut(&old_space_type) {
            if let Some(object_data) = old_space.extract_object(handle) {
                // Allocate in new space
                if let Some(new_space) = self.spaces.get_mut(&new_space_type) {
                    if let Some(new_handle) = new_space.allocate_object(object_data) {
                        // Deallocate from old space
                        if let Some(old_space) = self.spaces.get_mut(&old_space_type) {
                            old_space.deallocate(handle);
                        }
                        self.object_generations.insert(new_handle, new_generation);
                        self.object_tenure.insert(new_handle, 0);
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn should_promote(&self, handle: HeapHandleId) -> bool {
        let tenure = self.object_tenure.get(&handle).copied().unwrap_or(0);
        tenure >= self.promotion_policies.tenuring_threshold
    }

    pub fn increment_tenure(&mut self, handle: HeapHandleId) {
        let current_tenure = self.object_tenure.get(&handle).copied().unwrap_or(0);
        self.object_tenure.insert(handle, current_tenure + 1);
    }

    pub fn get_space_stats(&self) -> HashMap<SpaceType, SpaceStats> {
        let mut stats = HashMap::new();
        for (space_type, space) in &self.spaces {
            stats.insert(space_type.clone(), space.stats());
        }
        stats
    }

    pub fn get_spaces(&self) -> Vec<SpaceType> {
        self.spaces.keys().cloned().collect()
    }

    pub fn get_total_memory_usage(&self) -> MemorySize {
        let mut total = MemorySize::new(0);
        for space in self.spaces.values() {
            total = total + space.total_allocated();
        }
        total
    }

    pub fn get_total_free_memory(&self) -> MemorySize {
        let mut total = MemorySize::new(0);
        for space in self.spaces.values() {
            total = total + space.total_free();
        }
        total
    }

    fn select_space_for_allocation(
        &self,
        size: MemorySize,
        object_type: super::ObjectType,
    ) -> SpaceType {
        match object_type {
            super::ObjectType::String | super::ObjectType::Number | super::ObjectType::Boolean => {
                if size.bytes() <= self.allocation_strategies.small_object_threshold {
                    SpaceType::CellSpace
                } else {
                    SpaceType::NewSpace
                }
            }
            super::ObjectType::Array => {
                if size.bytes() <= self.allocation_strategies.medium_object_threshold {
                    SpaceType::NewSpace
                } else {
                    SpaceType::OldSpace
                }
            }
            super::ObjectType::Object | super::ObjectType::Function => {
                if size.bytes() <= self.allocation_strategies.medium_object_threshold {
                    SpaceType::NewSpace
                } else if size.bytes() <= self.allocation_strategies.large_object_threshold {
                    SpaceType::OldSpace
                } else {
                    SpaceType::LargeObjectSpace
                }
            }
            super::ObjectType::Code | super::ObjectType::Large => SpaceType::CodeSpace,
        }
    }

    fn get_space_for_generation(&mut self, generation: usize) -> Option<&mut Box<dyn MemorySpace>> {
        let space_type = match generation {
            0 => SpaceType::NewSpace,
            1 => SpaceType::OldSpace,
            _ => SpaceType::OldSpace,
        };

        // Clone the key to avoid borrow checker issues
        let key = space_type.clone();
        self.spaces.get_mut(&key)
    }
}

impl Default for SpaceCoordinator {
    fn default() -> Self {
        Self::new()
    }
}
