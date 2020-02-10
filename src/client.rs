use std::net::SocketAddr;

use crate::network::broker;
use crate::network::service_map::ServiceMap;
use crate::network::service::{ServiceExt, EagerService};
use crate::network::messages::metadata::Metadata0Request;

pub struct Client {
    pub inner: ServiceMap<SocketAddr, EagerService<broker::ClientState, broker::ClientError>>,
}

pub async fn connect(addr: SocketAddr) -> Result<Client, broker::ClientError> {
    let config = broker::ClientConfig::default();
    let mut inner = ServiceMap::new(move |addr| broker::connect(addr, config.clone()));

    let metadata = inner.call((addr, Metadata0Request {
        topics: vec![],
    })).await?;

    println!("Metadata:\n{:?}", metadata);

    Ok(Client {
        inner
    })
}
