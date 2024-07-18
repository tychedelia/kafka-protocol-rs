use bytes::{Buf, BufMut, BytesMut};
use docker_compose_runner::DockerCompose;
use docker_compose_runner::*;
use kafka_protocol::{
    messages::{ApiKey, ApiVersionsRequest, ApiVersionsResponse, RequestHeader, ResponseHeader},
    protocol::{Builder, Decodable, Encodable, HeaderVersion},
};
use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

#[test]
fn integration_test() {
    let _docker_compose = docker_compose("tests/docker-compose.yaml");

    let mut socket = TcpStream::connect("localhost:9092").unwrap();

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

fn docker_compose(file_path: &str) -> DockerCompose {
    pub static IMAGE_WAITERS: [Image; 1] = [Image {
        name: "bitnami/kafka:3.6.1-debian-11-r24",
        log_regex_to_wait_for: r"Kafka Server started",
        timeout: Duration::from_secs(120),
    }];
    DockerCompose::new(&IMAGE_WAITERS, |_| {}, file_path)
}
