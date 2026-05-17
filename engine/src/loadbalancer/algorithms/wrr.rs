use std::sync::Arc;

use crossbeam_skiplist::SkipSet;
use shared::types::Backend;

use crate::loadbalancer::algorithms::BalancingAlgorithm;

#[derive(Debug)]
struct WeightedIndex {
    next_position: u64,
    idx: usize,
    weight: u32,
}

pub struct WeightedRoundRobin(SkipSet<WeightedIndex>);

impl Ord for WeightedIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.next_position.cmp(&other.next_position) {
            std::cmp::Ordering::Equal => match other.weight.cmp(&self.weight) {
                std::cmp::Ordering::Equal => self.idx.cmp(&other.idx),
                weight_ordering => weight_ordering,
            },
            position_ordering => position_ordering,
        }
    }
}

impl PartialEq for WeightedIndex {
    fn eq(&self, other: &Self) -> bool {
        self.next_position == other.next_position
            && self.weight == other.weight
            && self.idx == other.idx
    }
}
impl Eq for WeightedIndex {}

impl PartialOrd for WeightedIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for WeightedRoundRobin {
    fn default() -> Self {
        Self(SkipSet::new())
    }
}

impl BalancingAlgorithm for WeightedRoundRobin {
    fn build(&self, _backends: &[Arc<Backend>]) {
        self.0.clear();
        for (idx, backend) in _backends.iter().enumerate() {
            self.0.insert(WeightedIndex {
                idx,
                weight: backend.weight,
                next_position: 0,
            });
        }
    }

    fn select<'a>(&self, backends: &'a [Arc<Backend>], _key: &str) -> Option<&'a Arc<Backend>> {
        if let Some(selected) = self.0.pop_front() {
            let idx = selected.idx;
            let penalty = (1_000_000 / selected.weight) as u64;
            self.0.insert(WeightedIndex {
                idx,
                weight: selected.weight,
                next_position: selected.next_position + penalty,
            });

            Some(&backends[idx])
        } else {
            None
        }
    }
}
