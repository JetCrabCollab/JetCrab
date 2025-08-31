use crate::vm::handle::HeapHandleId;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum BarrierType {
    None,
    CardMarking,
    ObjectMarking,
    Hybrid,
}

pub struct CardTable {
    card_size: usize,
    cards: Vec<bool>,
    total_cards: usize,
}

impl CardTable {
    pub fn new(heap_size: usize, card_size: usize) -> Self {
        let total_cards = (heap_size + card_size - 1) / card_size;
        Self {
            card_size,
            cards: vec![false; total_cards],
            total_cards,
        }
    }

    pub fn mark_card(&mut self, address: usize) {
        let card_index = address / self.card_size;
        if card_index < self.total_cards {
            self.cards[card_index] = true;
        }
    }

    pub fn is_card_dirty(&self, card_index: usize) -> bool {
        if card_index < self.total_cards {
            self.cards[card_index]
        } else {
            false
        }
    }

    pub fn clear_card(&mut self, card_index: usize) {
        if card_index < self.total_cards {
            self.cards[card_index] = false;
        }
    }

    pub fn clear_all_cards(&mut self) {
        for card in &mut self.cards {
            *card = false;
        }
    }

    pub fn get_dirty_cards(&self) -> Vec<usize> {
        self.cards
            .iter()
            .enumerate()
            .filter_map(|(index, &is_dirty)| {
                if is_dirty {
                    Some(index)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_card_address_range(&self, card_index: usize) -> (usize, usize) {
        let start = card_index * self.card_size;
        let end = start + self.card_size;
        (start, end)
    }
}

pub struct WriteBarrier {
    card_table: CardTable,
    dirty_objects: HashSet<HeapHandleId>,
    barrier_type: BarrierType,
    enabled: bool,
}

impl WriteBarrier {
    pub fn new(heap_size: usize, barrier_type: BarrierType) -> Self {
        let card_size = match barrier_type {
            BarrierType::CardMarking | BarrierType::Hybrid => 512,
            _ => 0,
        };

        Self {
            card_table: CardTable::new(heap_size, card_size),
            dirty_objects: HashSet::new(),
            barrier_type,
            enabled: true,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn record_write(&mut self, object_id: HeapHandleId, field_address: usize) {
        if !self.enabled {
            return;
        }

        match self.barrier_type {
            BarrierType::None => {}
            BarrierType::CardMarking => {
                self.card_table.mark_card(field_address);
            }
            BarrierType::ObjectMarking => {
                self.dirty_objects.insert(object_id);
            }
            BarrierType::Hybrid => {
                self.card_table.mark_card(field_address);
                self.dirty_objects.insert(object_id);
            }
        }
    }

    pub fn record_reference_write(&mut self, source_object: HeapHandleId, target_object: HeapHandleId) {
        if !self.enabled {
            return;
        }

        match self.barrier_type {
            BarrierType::None => {}
            BarrierType::CardMarking => {
                self.card_table.mark_card(source_object.as_usize());
            }
            BarrierType::ObjectMarking => {
                self.dirty_objects.insert(source_object);
                self.dirty_objects.insert(target_object);
            }
            BarrierType::Hybrid => {
                self.card_table.mark_card(source_object.as_usize());
                self.dirty_objects.insert(source_object);
                self.dirty_objects.insert(target_object);
            }
        }
    }

    pub fn get_dirty_objects(&self) -> &HashSet<HeapHandleId> {
        &self.dirty_objects
    }

    pub fn clear_dirty_objects(&mut self) {
        self.dirty_objects.clear();
    }

    pub fn get_dirty_cards(&self) -> Vec<usize> {
        self.card_table.get_dirty_cards()
    }

    pub fn clear_dirty_cards(&mut self) {
        self.card_table.clear_all_cards();
    }

    pub fn get_card_table(&self) -> &CardTable {
        &self.card_table
    }

    pub fn get_card_table_mut(&mut self) -> &mut CardTable {
        &mut self.card_table
    }

    pub fn set_barrier_type(&mut self, barrier_type: BarrierType) {
        self.barrier_type = barrier_type.clone();
        match barrier_type {
            BarrierType::CardMarking | BarrierType::Hybrid => {
                let heap_size = self.card_table.total_cards * self.card_table.card_size;
                self.card_table = CardTable::new(heap_size, 512);
            }
            _ => {}
        }
    }

    pub fn get_barrier_type(&self) -> &BarrierType {
        &self.barrier_type
    }

    pub fn get_stats(&self) -> WriteBarrierStats {
        WriteBarrierStats {
            total_dirty_objects: self.dirty_objects.len(),
            total_dirty_cards: self.card_table.get_dirty_cards().len(),
            barrier_type: self.barrier_type.clone(),
            enabled: self.enabled,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WriteBarrierStats {
    pub total_dirty_objects: usize,
    pub total_dirty_cards: usize,
    pub barrier_type: BarrierType,
    pub enabled: bool,
}

impl Default for WriteBarrier {
    fn default() -> Self {
        Self::new(1024 * 1024, BarrierType::Hybrid)
    }
}
