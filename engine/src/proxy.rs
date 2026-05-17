use std::net::SocketAddr;

use async_trait::async_trait;
use dashmap::DashMap;
use pingora_core::{Result, upstreams::peer::HttpPeer};
use pingora_http::ResponseHeader;
use pingora_proxy::{ProxyHttp, Session};

use crate::loadbalancer::{LoadBalancer, WeakBackend};

pub struct HProxy {
    domains: DashMap<String, LoadBalancer>,
}

impl HProxy {
    pub fn new(domains: DashMap<String, LoadBalancer>) -> Self {
        Self { domains }
    }
}

#[async_trait]
impl ProxyHttp for HProxy {
    type CTX = Option<(WeakBackend, SocketAddr)>;

    fn new_ctx(&self) -> Self::CTX {
        None
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let peer = ctx.as_ref().unwrap();
        Ok(Box::new(HttpPeer::new(peer.1, false, String::new())))
    }

    async fn request_filter(&self, session: &mut Session, ctx: &mut Self::CTX) -> Result<bool> {
        let domain = session
            .req_header()
            .headers
            .get("Host")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.split(":").next().unwrap_or(s));
        if let Some(backend) = domain
            .and_then(|s| self.domains.get(s))
            .map(|b| b.get_next(""))
        {
            let is = if let Some(peer) = backend.get() {
                ctx.replace((backend, peer));
                false
            } else {
                true
            };

            Ok(is)
        } else {
            let mut error_response = ResponseHeader::build(404, None).unwrap();
            error_response
                .insert_header("Content-Type", "text/plain")
                .unwrap();

            session
                .write_response_header(Box::new(error_response), false)
                .await?;

            let msg = format!(
                "404: {} is not configured.",
                session.req_header().uri.host().unwrap_or("def")
            );
            session.write_response_body(Some(msg.into()), true).await?;
            Ok(true)
        }
    }
}
