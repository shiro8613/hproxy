use std::sync::Arc;

use shared::types::{Backend, BalancingAlgorithms};

use crate::loadbalancer::algorithms::{rnd::Random, rr::RoundRobin, wrr::WeightedRoundRobin};

pub mod rnd;
pub mod rr;
pub mod wrr;

pub trait BalancingAlgorithm: Send + Sync {
    fn build(&self, _backends: &[Arc<Backend>]) {}
    fn select<'a>(&self, backends: &'a [Arc<Backend>], ket: &str) -> Option<&'a Arc<Backend>>;
}

pub trait AlgorithmGetter {
    fn get(&self) -> Box<dyn BalancingAlgorithm>;
}

impl AlgorithmGetter for BalancingAlgorithms {
    fn get(&self) -> Box<dyn BalancingAlgorithm> {
        match self {
            BalancingAlgorithms::RoundRobin => Box::new(RoundRobin::default()),
            BalancingAlgorithms::Random => Box::new(Random),
            BalancingAlgorithms::WeightedRoundRobin => Box::new(WeightedRoundRobin::default()),
        }
    }
}
