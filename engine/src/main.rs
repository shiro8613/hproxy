use std::collections::BTreeSet;

use dashmap::DashMap;
use pingora_core::server::Server;
use pingora_proxy::http_proxy_service;
use shared::types::{Backend, Backends, BalancingAlgorithms};

use crate::{loadbalancer::LoadBalancer, proxy::HProxy};

pub mod loadbalancer;
pub mod proxy;

fn main() {
    let mut server = Server::new(None).expect("error");
    server.bootstrap();

    let mut bb = BTreeSet::new();

    bb.insert(Backend {
        endpoint: "127.0.0.1:8080".parse().unwrap(),
        weight: 10,
    });
    // bb.insert(Backend {
    //     endpoint: "1.1.1.2:8080".parse().unwrap(),
    //     weight: 10,
    // });
    // bb.insert(Backend {
    //     endpoint: "1.1.1.3:8080".parse().unwrap(),
    //     weight: 10,
    // });
    // bb.insert(Backend {
    //     endpoint: "1.1.1.4:8080".parse().unwrap(),
    //     weight: 20,
    // });

    let a = Backends {
        algorithm: BalancingAlgorithms::WeightedRoundRobin,
        backends: bb,
    };

    let backends = DashMap::new();
    let load = LoadBalancer::new(a);
    backends.insert("localhost".to_string(), load);

    let mut proxy = http_proxy_service(&server.configuration, HProxy::new(backends));
    proxy.add_tcp("0.0.0.0:8081");

    server.add_service(proxy);
    server.run_forever();
}
