use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use shared::types::Backend;

use crate::loadbalancer::algorithms::BalancingAlgorithm;

pub struct RoundRobin(AtomicUsize);

impl Default for RoundRobin {
    fn default() -> Self {
        Self(AtomicUsize::new(0))
    }
}

impl BalancingAlgorithm for RoundRobin {
    fn select<'a>(&self, backends: &'a [Arc<Backend>], _key: &str) -> Option<&'a Arc<Backend>> {
        let idx = self.0.fetch_add(1, Ordering::Relaxed);
        Some(&backends[idx % backends.len()])
    }
}
