use std::{
    collections::{BTreeMap, BTreeSet},
    time::Instant,
};

use crate::keyboard::{KeyCode, KeyboardLayout};

pub trait LayoutHint {
    /// Updates the internal state of the layout hint with the given key press.
    fn receive_key_press(&mut self, key_code: KeyCode, time: Instant);

    /// For a specific position, gives the rank of each key.
    ///
    /// The rank specifies how suitable a key is for a given position. The higher the rank, the more suitable the key.
    fn rank_keys_for_position(&self, position: (usize, usize)) -> BTreeMap<KeyCode, f64>;
}

pub struct LayoutCreator {
    layout_hints: Vec<Box<dyn LayoutHint>>,
}

impl LayoutCreator {
    pub fn new(layout_hints: Vec<Box<dyn LayoutHint>>) -> Self {
        Self { layout_hints }
    }

    pub fn create_layout(&self) -> KeyboardLayout {
        // Step 1: find the rankings for every position.
        let mut rankings = vec::new();
        for row in 0..3 {
            let mut row_rankings = vec::new();
            for column in 0..10 {
                let rankings = self.rank_keys_for_position((row, column));
                row_rankings.push(rankings);
            }
            rankings.push(row_rankings);
        }

        // Step 2: find the highest overall rank and lock it in. Repeat this until all positions are allocated.
        let mut layout = KeyboardLayout::default();
        let mut used_positions = BTreeSet::new();
        let mut used_keys = BTreeSet::new();
        for _ in 0..30 {
            let mut highest_rank = 0.0;
            let mut best_position = None;
            let mut best_key = None;
            for row in 0..3 {
                for column in 0..10 {
                    if used_positions.contains(&(row, column)) {
                        continue;
                    }
                    for (key, rank) in &rankings[row][column] {
                        if used_keys.contains(key) {
                            continue;
                        }
                        if *rank > highest_rank {
                            highest_rank = *rank;
                            best_position = Some((row, column));
                            best_key = Some(*key);
                        }
                    }
                }
            }
            let (row, column) = best_position.unwrap();
            let key = best_key.unwrap();
            layout.set_key_at(row, column, key);
            used_positions.insert(position);
            used_keys.insert(key);
        }
        layout
    }
}

impl LayoutHint for LayoutCreator {
    fn rank_keys_for_position(&self, position: (usize, usize)) -> BTreeMap<KeyCode, f64> {
        let mut key_rankings = BTreeMap::new();
        for hint in &self.layout_hints {
            for (key, rank) in hint.rank_keys_for_position(position) {
                *key_rankings.entry(key).or_insert(0.0) += rank;
            }
        }
        key_rankings
    }

    fn receive_key_press(&mut self, key_code: KeyCode, time: Instant) {
        for hint in &mut self.layout_hints {
            hint.receive_key_press(key_code, time);
        }
    }
}
