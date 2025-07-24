use bytes::Bytes;
use kafka_protocol::messages::{ApiVersionsRequest, ApiVersionsResponse};
use kafka_protocol::protocol::Decodable;
use kafka_protocol::ResponseError::UnsupportedVersion;

#[test]
fn api_versions() {
    let bytes: [u8; 25] = [
        0x12, 0x61, 0x70, 0x61, 0x63, 0x68, 0x65, 0x2d, 0x6b, 0x61, 0x66, 0x6b, 0x61, 0x2d, 0x6a,
        0x61, 0x76, 0x61, 0x06, 0x32, 0x2e, 0x38, 0x2e, 0x30, 0x00,
    ];

    let res = ApiVersionsRequest::decode(&mut Bytes::from(bytes.to_vec()), 3).unwrap();
    assert_eq!(res.client_software_name.to_string(), "apache-kafka-java");
    assert_eq!(res.client_software_version.to_string(), "2.8.0");
}

#[test]
fn api_versions_unsupported_version_response() {
    let bytes: [u8; 12] = [
        0x00, 0x23, 0x00, 0x00, 0x00, 0x01, 0x00, 0x12, 0x00, 0x00, 0x00, 0x03,
    ];

    let res = ApiVersionsResponse::decode(&mut Bytes::from(bytes.to_vec()), 4).unwrap();
    assert_eq!(res.api_keys.len(), 1);
    assert_eq!(res.error_code, UnsupportedVersion.code());
}
