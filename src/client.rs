use std::net::SocketAddr;

use protocol_base::StrBytes;

use crate::network::broker;
use crate::network::service_map::ServiceMap;
use crate::network::service::{ServiceExt, EagerService};
use crate::network::messages::metadata_request::{MetadataRequest, MetadataRequestTopic};
use crate::network::messages::fetch_request::{FetchRequest, FetchableTopic, FetchPartition};
use crate::network::records::RecordBatchDecoder;

pub struct Client {
    pub inner: ServiceMap<SocketAddr, EagerService<broker::ClientState, broker::ClientError>>,
}

pub async fn connect(addr: SocketAddr) -> Result<Client, broker::ClientError> {
    let config = broker::ClientConfig::default();
    let mut inner = ServiceMap::new(move |addr| broker::connect(addr, config.clone()));

    let mut req = MetadataRequest::default();
    req.topics = Some(vec![
        MetadataRequestTopic {
            name: StrBytes::from_str("foo").into(),
            ..Default::default()
        }
    ]);

    let metadata = inner.call((addr, req)).await?;

    println!("Metadata:\n{:?}", metadata);

    fn fetch_partition(index: i32) -> FetchPartition {
        let mut res = FetchPartition::default();
        res.max_bytes = 1000000;
        res.partition_index = index;
        res.fetch_offset = 3;
        res
    }

    let mut req = FetchRequest::default();
    req.replica_id = -1;
    req.max_wait = 5000;
    req.min_bytes = 100;
    req.topics.push(FetchableTopic {
        name: StrBytes::from_str("foo").into(),
        fetch_partitions: (0..1).map(fetch_partition).collect(),
    });
    let result = inner.call((addr, req)).await?;
    println!("Records:\n{:?}", result);

    for topic in result.topics {
        for partition in topic.partitions {
            let mut records = partition.records.unwrap();

            println!("{:?}", records);

            let batch = RecordBatchDecoder::decode(&mut records)?;
            println!("Batch (topic = {:?}, partition = {}):\n{:?}", topic.name, partition.partition_index, batch);
        }
    }

    Ok(Client {
        inner
    })
}
