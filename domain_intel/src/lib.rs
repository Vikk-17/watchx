pub mod model;
pub mod errors;

use hickory_resolver::lookup_ip::LookupIp;
use model::DomainInfo;
use errors::AppError;
// use std::net::*;
// use tokio::runtime::Runtime;
use std::net::IpAddr;
use hickory_resolver::{
    Resolver,
    net::runtime::TokioRuntimeProvider,
    config::*,
};

pub async fn lookup_domain(domain: String) -> Result<LookupIp, AppError> {
    let resolver = Resolver::builder_with_config(
        ResolverConfig::udp_and_tcp(&GOOGLE),
        TokioRuntimeProvider::default()
    ).build().unwrap();

    let lookup = resolver.lookup_ip(domain).await?;

    Ok(lookup)
}

pub fn map_lookup_to_domain_info(
    domain: String,
    lookup: LookupIp,
) -> DomainInfo {
    let mut ipv4 = Vec::new();
    let mut ipv6 = Vec::new();

    for ip in lookup.iter() {

        match ip {

            IpAddr::V4(addr) => {
                ipv4.push(addr.to_string());
            }

            IpAddr::V6(addr) => {
                ipv6.push(addr.to_string());
            }
        }
    }

    DomainInfo {
        domain,
        ipv4,
        ipv6,
        ttl: None,
        cname: vec![],
        mx: vec![],
        txt: vec![],
        ns: vec![],
    }
}
