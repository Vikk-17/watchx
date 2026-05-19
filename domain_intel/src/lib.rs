mod models;

use models::*;
use model::Domain;
// use std::net::*;
// use tokio::runtime::Runtime;
use hickory_resolver::{
    Resolver,
    net::runtime::TokioRuntimeProvider,
    config::*,
};

pub async fn lookup_domain(domain: String) -> Result<LookupIp, NetError> {
    let resolver = Resolver::builder_with_config(
        ResolverConfig::udp_and_tcp(&GOOGLE),
        TokioRuntimeProvider::default()
    ).build().unwrap();

    let lookup_future = resolver.lookup_ip(domain).await.unwrap();

    Ok(lookup_future)
}
