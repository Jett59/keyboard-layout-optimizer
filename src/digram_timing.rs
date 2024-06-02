//! Provides a `LayoutHint` implementation that uses digram timing data to rank keys.
//!
//! Essentially, this looks at how quickly the user can type specific digrams (pairs of characters) and uses that to rank keys.
//! In theory it should assign the most common digrams to the fastest key pairs.
//!
//! In practice, the way we do this is to measure how fast it is (on average) to switch to and from each key position.
//! We also record how frequently each key is used.

use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

use crate::{
    keyboard::{KeyCode, KeyboardLayout},
    layout_creator::LayoutHint,
};

/// Helper function to allow for adjusting the averages.
fn adjust_average_duration(
    previous_average: Duration,
    new_duration: Duration,
    new_count: usize,
) -> Duration {
    if new_duration > previous_average {
        previous_average + (new_duration - previous_average) / new_count as u32
    } else {
        previous_average - (previous_average - new_duration) / new_count as u32
    }
}

#[derive(Debug, Clone, Default)]
pub struct DigramTimingHint {
    last_key: Option<KeyCode>,
    last_time: Option<Instant>,

    frequencies: BTreeMap<KeyCode, usize>,
    average_time_to: BTreeMap<(usize, usize), Duration>,
    average_time_from: BTreeMap<(usize, usize), Duration>,
}

impl LayoutHint for DigramTimingHint {
    fn receive_key_press(&mut self, key_code: KeyCode, time: Instant) {
        if let Some(last_key) = self.last_key {
            let last_time = self.last_time.unwrap();
            let time_between_keys = time.duration_since(last_time);
            // If there was more than a second between keys, we  assume something else happened. We shouldn't count this in our stats.
            if time_between_keys < Duration::from_secs(1) {
                let count = *self
                    .frequencies
                    .entry(key_code)
                    .and_modify(|frequency| *frequency += 1)
                    .or_insert(1);
                let last_count = *self.frequencies.get(&last_key).unwrap_or(&0);
                let this_position = KeyboardLayout::QWERTY.position_of(key_code).unwrap();
                let last_position = KeyboardLayout::QWERTY.position_of(last_key).unwrap();
                self.average_time_to
                    .entry(this_position)
                    .and_modify(|average| {
                        *average = adjust_average_duration(*average, time_between_keys, count)
                    })
                    .or_insert(time_between_keys);

                self.average_time_from
                    .entry(last_position)
                    .and_modify(|average| {
                        *average = adjust_average_duration(*average, time_between_keys, last_count)
                    })
                    .or_insert(time_between_keys);
            }
        }
        self.last_key = Some(key_code);
        self.last_time = Some(time);
    }

    fn rank_keys_for_position(&self, position: (usize, usize)) -> BTreeMap<KeyCode, f64> {
        self.frequencies
            .iter()
            .filter_map(|(&key, &frequency)| {
                let mut rank = 0.0;
                if let Some(time_to) = self.average_time_to.get(&(position.0, position.1)) {
                    rank += frequency as f64 / time_to.as_secs_f64();
                }
                if let Some(time_from) = self.average_time_from.get(&(position.0, position.1)) {
                    rank += frequency as f64 / time_from.as_secs_f64();
                }
                if rank > 0.0 {
                    Some((key, rank))
                } else {
                    None
                }
            })
            .collect()
    }
}
