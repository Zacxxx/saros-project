use bevy::prelude::*;
use std::collections::VecDeque;

/// Items held by an entity. Fixed 36-slot array (SoA: ids + quantities).
#[derive(Component)]
pub struct Inventory {
    pub item_ids:    [u32; 36],
    pub quantities:  [u32; 36],
    pub hotbar_slot: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            item_ids:   [0; 36],
            quantities: [0; 36],
            hotbar_slot: 0,
        }
    }
}

impl Inventory {
    /// Add `qty` of `item_id`. Returns leftover that didn't fit.
    pub fn add(&mut self, item_id: u32, qty: u32) -> u32 {
        let mut remaining = qty;
        for i in 0..36 {
            if self.item_ids[i] == item_id && self.quantities[i] < 64 {
                let space = 64 - self.quantities[i];
                let take = remaining.min(space);
                self.quantities[i] += take;
                remaining -= take;
                if remaining == 0 { return 0; }
            }
        }
        for i in 0..36 {
            if self.item_ids[i] == 0 {
                let take = remaining.min(64);
                self.item_ids[i] = item_id;
                self.quantities[i] = take;
                remaining -= take;
                if remaining == 0 { return 0; }
            }
        }
        remaining
    }

    pub fn hotbar_item(&self) -> (u32, u32) {
        (self.item_ids[self.hotbar_slot], self.quantities[self.hotbar_slot])
    }
}
