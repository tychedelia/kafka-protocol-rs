use std::net::SocketAddr;

use crate::network::broker;
use crate::network::service_map::ServiceMap;
use crate::network::service::{ServiceExt, EagerService};
use crate::network::messages::metadata_request::{MetadataRequest};

pub struct Client {
    pub inner: ServiceMap<SocketAddr, EagerService<broker::ClientState, broker::ClientError>>,
}

pub async fn connect(addr: SocketAddr) -> Result<Client, broker::ClientError> {
    let config = broker::ClientConfig::default();
    let mut inner = ServiceMap::new(move |addr| broker::connect(addr, config.clone()));

    let req = MetadataRequest::default();

    let metadata = inner.call((addr, req)).await?;

    println!("Metadata:\n{:?}", metadata);

    Ok(Client {
        inner
    })
}
