use std::{collections::BTreeSet, net::SocketAddr, ops::Deref};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Backend {
    pub endpoint: SocketAddr,
    pub weight: u32, //weight range 1..256
}

impl Deref for Backend {
    type Target = SocketAddr;

    fn deref(&self) -> &Self::Target {
        &self.endpoint
    }
}

impl PartialEq for Backend {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint == other.endpoint
    }
}

impl Eq for Backend {}

impl Ord for Backend {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.endpoint.cmp(&other.endpoint)
    }
}

impl PartialOrd for Backend {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Backends {
    pub algorithm: BalancingAlgorithms,
    pub backends: BTreeSet<Backend>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Domain {
    domain: String,
    path: String,
}

pub enum BalancingAlgorithms {
    RoundRobin,
    WeightedRoundRobin,
    Random,
}
