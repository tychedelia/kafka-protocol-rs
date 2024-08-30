use bytes::{Buf, Bytes};
use kafka_protocol::messages::{ApiKey, RequestHeader};
use kafka_protocol::protocol::buf::ByteBuf;
use kafka_protocol::protocol::Decodable;
use std::convert::TryFrom;

#[test]
fn request_header() {
    let mut bytes = Bytes::from(vec![
        0x00, 0x12, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x61, 0x64, 0x6d, 0x69, 0x6e,
        0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2d, 0x31, 0x00,
    ]);

    let api_key = bytes.peek_bytes(0..2).get_i16();
    let api_version = bytes.peek_bytes(2..4).get_i16();
    let header_version = ApiKey::try_from(api_key)
        .unwrap()
        .request_header_version(api_version);
    let res = RequestHeader::decode(&mut bytes, header_version).unwrap();

    assert_eq!(res.request_api_key, 18);
    assert_eq!(res.request_api_version, 3);
    assert_eq!(res.client_id.unwrap().to_string(), "adminclient-1");
}
