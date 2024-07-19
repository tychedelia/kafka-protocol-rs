use bytes::{Buf, BufMut, Bytes, BytesMut};
use kafka_protocol::{
    messages::{
        create_topics_request::CreatableTopic,
        fetch_request::{FetchPartition, FetchTopic},
        produce_request::{PartitionProduceData, TopicProduceData},
        ApiKey, ApiVersionsRequest, ApiVersionsResponse, CreateTopicsRequest, CreateTopicsResponse,
        FetchRequest, FetchResponse, ProduceRequest, ProduceResponse, RequestHeader,
        ResponseHeader, TopicName,
    },
    protocol::{Builder, Decodable, Encodable, HeaderVersion, StrBytes},
    records::{
        Compression, Record, RecordBatchDecoder, RecordBatchEncoder, RecordEncodeOptions,
        TimestampType,
    },
};
use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};
use testcontainers::{
    core::{wait::LogWaitStrategy, ContainerPort, WaitFor},
    runners::SyncRunner,
    Container, GenericImage, ImageExt,
};

#[test]
fn get_api_versions() {
    let container = start_kafka();
    let mut socket = connect_to_kafka(&container);

    let version = 2;
    let header = RequestHeader::builder()
        .request_api_key(ApiKey::ApiVersionsKey as i16)
        .request_api_version(version)
        .build()
        .unwrap();
    let request = ApiVersionsRequest::builder().build().unwrap();
    send_request(&mut socket, header, request);

    let result: ApiVersionsResponse = receive_response(&mut socket, version).1;

    assert_eq!(result.error_code, 0);
    // On this specific version of kafka
    assert_eq!(result.api_keys.len(), 55);
}

#[test]
fn record_batch_produce_fetch() {
    let topic_name = TopicName(StrBytes::from_static_str("record_batch_produce_fetch"));

    let container = start_kafka();
    let mut socket = connect_to_kafka(&container);

    let records = vec![
        new_record(0, true),
        new_record(1, true),
        new_record(2, true),
    ];

    let mut encoded = BytesMut::new();
    RecordBatchEncoder::encode(
        &mut encoded,
        records.iter(),
        &RecordEncodeOptions {
            version: 2,
            compression: Compression::None,
        },
    )
    .unwrap();

    create_topic(topic_name.clone(), &mut socket);

    // Sometimes (rarely) the produce request will fail with error code 6 (NOT_LEADER_OR_FOLLOWER)
    // Hopefully a 1s sleep will prevent that
    std::thread::sleep(Duration::from_secs(1));

    produce_records(topic_name.clone(), 9, encoded.freeze(), &mut socket);
    fetch_records(topic_name.clone(), 12, records, &mut socket);
}

#[test]
fn message_set_v1_produce_fetch() {
    let topic_name = TopicName(StrBytes::from_static_str("message_set_v1_produce_fetch"));

    let container = start_kafka();
    let mut socket = connect_to_kafka(&container);

    let records = vec![
        new_record(0, false),
        new_record(1, false),
        new_record(2, false),
    ];

    let mut encoded = BytesMut::new();
    RecordBatchEncoder::encode(
        &mut encoded,
        records.iter(),
        &RecordEncodeOptions {
            version: 1,
            compression: Compression::None,
        },
    )
    .unwrap();

    create_topic(topic_name.clone(), &mut socket);

    // Sometimes (rarely) the produce request will fail with error code 6 (NOT_LEADER_OR_FOLLOWER)
    // Hopefully a 1s sleep will prevent that
    std::thread::sleep(Duration::from_secs(1));

    produce_records(topic_name.clone(), 2, encoded.freeze(), &mut socket);
    fetch_records(topic_name.clone(), 3, records, &mut socket);
}

fn send_request<T: Encodable + HeaderVersion>(
    socket: &mut TcpStream,
    header: RequestHeader,
    body: T,
) {
    let mut bytes = BytesMut::new();

    header
        .encode(&mut bytes, T::header_version(header.request_api_version))
        .unwrap();
    body.encode(&mut bytes, header.request_api_version).unwrap();

    let size = bytes.len() as i32;
    socket.write_all(&size.to_be_bytes()).unwrap();
    socket.write_all(&bytes).unwrap();
}

fn receive_response<T: Decodable + HeaderVersion>(
    socket: &mut TcpStream,
    version: i16,
) -> (ResponseHeader, T) {
    let mut buffer = BytesMut::new();

    let message_size = loop {
        read(socket, &mut buffer);
        if buffer.len() >= 4 {
            break buffer.get_u32();
        }
    };

    loop {
        if buffer.len() == message_size as usize {
            return (
                ResponseHeader::decode(&mut buffer, T::header_version(version)).unwrap(),
                T::decode(&mut buffer, version).unwrap(),
            );
        }
        read(socket, &mut buffer);
    }
}

fn read(socket: &mut TcpStream, dest: &mut BytesMut) {
    let mut tmp = [0; 1000];
    let read = socket.read(&mut tmp).unwrap();
    dest.put_slice(&tmp[..read]);
}

fn start_kafka() -> Container<GenericImage> {
    GenericImage::new("bitnami/kafka", "3.6.1-debian-11-r24")
        .with_exposed_port(ContainerPort::Tcp(9092))
        .with_wait_for(WaitFor::Log(LogWaitStrategy::stdout(
            b"Kafka Server started",
        )))
        .with_env_var("KAFKA_CFG_LISTENERS", "BROKER://:9092,CONTROLLER://:9093")
        .with_env_var("KAFKA_CFG_ADVERTISED_LISTENERS", "BROKER://127.0.0.1:9092")
        .with_env_var(
            "KAFKA_CFG_LISTENER_SECURITY_PROTOCOL_MAP",
            "CONTROLLER:PLAINTEXT,BROKER:PLAINTEXT",
        )
        .with_env_var("KAFKA_CFG_INTER_BROKER_LISTENER_NAME", "BROKER")
        .with_env_var("KAFKA_CFG_CONTROLLER_LISTENER_NAMES", "CONTROLLER")
        .with_env_var("KAFKA_CFG_PROCESS_ROLES", "controller,broker")
        .with_env_var("KAFKA_CFG_CONTROLLER_QUORUM_VOTERS", "0@localhost:9093")
        .with_env_var("KAFKA_CFG_NODE_ID", "0")
        .with_env_var("ALLOW_PLAINTEXT_LISTENER", "yes")
        .start()
        .unwrap()
}

fn connect_to_kafka(container: &Container<GenericImage>) -> TcpStream {
    let host = container.get_host().unwrap();
    let port = container.get_host_port_ipv4(9092).unwrap();
    TcpStream::connect((host.to_string(), port)).unwrap()
}

fn create_topic(topic_name: TopicName, socket: &mut TcpStream) {
    let version = 7;
    let header = RequestHeader::builder()
        .request_api_key(ApiKey::CreateTopicsKey as i16)
        .request_api_version(version)
        .build()
        .unwrap();

    let request = CreateTopicsRequest::builder()
        .timeout_ms(5000)
        .topics(
            [(
                topic_name.clone(),
                CreatableTopic::builder()
                    .num_partitions(1)
                    .replication_factor(1)
                    .build()
                    .unwrap(),
            )]
            .into(),
        )
        .build()
        .unwrap();

    send_request(socket, header, request);
    let result: CreateTopicsResponse = receive_response(socket, version).1;

    assert_eq!(result.throttle_time_ms, 0, "response throttle time");

    let topic = result.topics.get(&topic_name).unwrap();

    assert_eq!(topic.error_code, 0, "topic error code");
    assert_eq!(topic.error_message, None, "topic error message");
}

fn produce_records(topic_name: TopicName, version: i16, records: Bytes, socket: &mut TcpStream) {
    let header = RequestHeader::builder()
        .request_api_key(ApiKey::ProduceKey as i16)
        .request_api_version(version)
        .build()
        .unwrap();

    let request = ProduceRequest::builder()
        .acks(1)
        .timeout_ms(5000)
        .topic_data(
            [(
                topic_name.clone(),
                TopicProduceData::builder()
                    .partition_data(vec![PartitionProduceData::builder()
                        .index(0)
                        .records(Some(records))
                        .build()
                        .unwrap()])
                    .build()
                    .unwrap(),
            )]
            .into(),
        )
        .build()
        .unwrap();

    send_request(socket, header, request);

    let result: ProduceResponse = receive_response(socket, version).1;

    assert_eq!(result.throttle_time_ms, 0, "produce response throttle time");

    let topic_response = result.responses.get(&topic_name).unwrap();
    let partition_response = topic_response.partition_responses.first().unwrap();

    assert_eq!(
        partition_response.index, 0,
        "produce response partition index"
    );
    assert_eq!(
        partition_response.error_message, None,
        "produce recponse partition error message"
    );
    assert_eq!(
        partition_response.error_code, 0,
        "produce response partition error code"
    );
}

fn fetch_records(
    topic_name: TopicName,
    version: i16,
    expected: Vec<Record>,
    socket: &mut TcpStream,
) {
    let header = RequestHeader::builder()
        .request_api_key(ApiKey::FetchKey as i16)
        .request_api_version(version)
        .build()
        .unwrap();

    let request = FetchRequest::builder()
        .topics(vec![FetchTopic::builder()
            .topic(topic_name.clone())
            .partitions(vec![FetchPartition::builder()
                .partition(0)
                .fetch_offset(0)
                .build()
                .unwrap()])
            .build()
            .unwrap()])
        .build()
        .unwrap();

    send_request(socket, header, request);

    let result: FetchResponse = receive_response(socket, version).1;

    assert_eq!(result.throttle_time_ms, 0, "fetch response throttle time");
    assert_eq!(result.error_code, 0, "fetch response error code");

    let topic_response = result.responses.first().unwrap();

    assert_eq!(
        topic_response.topic, topic_name,
        "fetch response topic name"
    );

    let partition_response = topic_response.partitions.first().unwrap();

    assert_eq!(
        partition_response.partition_index, 0,
        "fetch response partition index"
    );
    assert_eq!(
        partition_response.error_code, 0,
        "fetch resopnse partition error code"
    );

    let mut fetched_records = partition_response.records.clone().unwrap();
    let fetched_records = RecordBatchDecoder::decode(&mut fetched_records).unwrap();

    eprintln!("{expected:#?}");
    eprintln!("{fetched_records:#?}");

    assert_eq!(expected, fetched_records);
}

fn new_record(offset: i64, v2: bool) -> Record {
    Record {
        transactional: false,
        control: false,
        partition_leader_epoch: if v2 { 0 } else { -1 },
        producer_id: -1,
        producer_epoch: -1,
        timestamp_type: TimestampType::Creation,
        timestamp: offset,
        sequence: if v2 { offset as _ } else { -1 },
        offset,
        key: Some(format!("key{offset}").into()),
        value: Some(format!("value{offset}").into()),
        headers: Default::default(),
    }
}
