use std::{
    collections::BTreeSet,
    net::SocketAddr,
    sync::{Arc, Weak},
};

use arc_swap::ArcSwap;
use shared::types::{Backend, Backends, BalancingAlgorithms};

use crate::loadbalancer::algorithms::{AlgorithmGetter, BalancingAlgorithm};

pub mod algorithms;

pub struct LoadBalancer {
    algorithm: ArcSwap<Box<dyn BalancingAlgorithm>>,
    backends: ArcSwap<Box<[Arc<Backend>]>>,
}

pub struct WeakBackend(Option<Weak<Backend>>);

impl LoadBalancer {
    pub fn new(_backends: Backends) -> Self {
        let backends = Self::to_arc_boxed(_backends.backends);
        let algorithm = Self::update_and_create(_backends.algorithm, &backends);
        Self {
            algorithm: ArcSwap::from_pointee(algorithm),
            backends: ArcSwap::from_pointee(backends),
        }
    }

    pub fn set_algorithm(&self, algorithm: BalancingAlgorithms) {
        self.algorithm.store(Arc::new(Self::update_and_create(
            algorithm,
            &self.backends.load(),
        )));
    }

    pub fn set_backends(&self, backends: BTreeSet<Backend>) {
        let backends = Self::update(&**self.algorithm.load().as_ref(), backends);
        self.backends.store(Arc::new(backends));
    }

    pub fn get_next(&self, key: &str) -> WeakBackend {
        let algo = self.algorithm.load();
        let backedns = self.backends.load();
        let backend = algo.select(&backedns, key);
        WeakBackend::new(backend)
    }

    #[inline]
    fn to_arc_boxed(backends: BTreeSet<Backend>) -> Box<[Arc<Backend>]> {
        backends.into_iter().map(Arc::new).collect()
    }

    #[inline]
    fn update_and_create(
        algorithm: BalancingAlgorithms,
        backends: &[Arc<Backend>],
    ) -> Box<dyn BalancingAlgorithm> {
        let algo = algorithm.get();
        algo.build(backends);
        algo
    }

    #[inline]
    fn update(
        algorithm: &dyn BalancingAlgorithm,
        backends: BTreeSet<Backend>,
    ) -> Box<[Arc<Backend>]> {
        let backends = Self::to_arc_boxed(backends);
        algorithm.build(&backends);
        backends
    }
}

impl WeakBackend {
    pub fn new(backend: Option<&Arc<Backend>>) -> Self {
        Self(backend.map(Arc::downgrade))
    }

    pub fn get(&self) -> Option<SocketAddr> {
        if let Some(backend) = &self.0 {
            backend.upgrade().map(|b| b.endpoint)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn balance_test() {
        let mut bb = BTreeSet::new();

        bb.insert(Backend {
            endpoint: "1.1.1.1:8080".parse().unwrap(),
            weight: 10,
        });
        bb.insert(Backend {
            endpoint: "1.1.1.2:8080".parse().unwrap(),
            weight: 10,
        });
        bb.insert(Backend {
            endpoint: "1.1.1.3:8080".parse().unwrap(),
            weight: 10,
        });
        bb.insert(Backend {
            endpoint: "1.1.1.4:8080".parse().unwrap(),
            weight: 20,
        });

        let a = Backends {
            algorithm: BalancingAlgorithms::WeightedRoundRobin,
            backends: bb,
        };

        let balancer = LoadBalancer::new(a);
        for _i in 0..20 {
            if let Some(b) = balancer.get_next("").get() {
                println!("{b}");
            }
        }
    }
}
