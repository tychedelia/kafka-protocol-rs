use kafka_protocol::{
    messages::{ApiKey, ApiVersionsRequest, ApiVersionsResponse, RequestHeader},
    protocol::Builder,
};

mod common;
use common::*;

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
