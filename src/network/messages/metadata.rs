use franz_derive::{Encodable, Decodable};
use franz_protocol::{Request, NoDowngrade, NoUpgrade, RequestType};

#[derive(Debug)]
pub struct Metadata;

impl RequestType for Metadata {
    const KEY: i16 = 3;
    const MAX_VERSION: i16 = 0;
}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct Metadata0Request {
    #[franz(Array(Struct))]
    pub topics: Vec<Topic0Arg>,
}

impl Request for Metadata0Request {
    type Type = Metadata;
    const VERSION: i16 = 0;
    type Response = Metadata0Response;
    type Downgrade = NoDowngrade;
    type Upgrade = NoUpgrade;
}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct Topic0Arg {
    #[franz(String)]
    pub name: String,
}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct Metadata0Response {
    #[franz(Array(Struct))]
    pub brokers: Option<Vec<Broker0>>,
    #[franz(Array(Struct))]
    pub topics: Option<Vec<Topic0>>,
}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct Broker0 {
    #[franz(Int32)]
    pub node_id: i32,
    #[franz(String)]
    pub host: String,
    #[franz(Int32)]
    pub port: i32,
}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct Topic0 {
    #[franz(Int16)]
    pub error_code: i16,
    #[franz(String)]
    pub name: String,
    #[franz(Array(Struct))]
    pub partitions: Option<Vec<Partition0>>,
}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct Partition0 {
    #[franz(Int16)]
    pub error_code: i16,
    #[franz(Int32)]
    pub partition_index: i32,
    #[franz(Int32)]
    pub leader_id: i32,
    #[franz(Array(Int32))]
    pub replica_nodes: Option<Vec<i32>>,
    #[franz(Array(Int32))]
    pub isr_nodes: Option<Vec<i32>>,
}
