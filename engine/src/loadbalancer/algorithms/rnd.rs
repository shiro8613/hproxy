use std::sync::Arc;

use shared::types::Backend;

use crate::loadbalancer::algorithms::BalancingAlgorithm;

pub struct Random;

impl BalancingAlgorithm for Random {
    fn select<'a>(&self, backends: &'a [Arc<Backend>], _key: &str) -> Option<&'a Arc<Backend>> {
        let idx = fastrand::usize(0..backends.len().saturating_sub(1));
        Some(&backends[idx])
    }
}
