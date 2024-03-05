//! Implementation of [the Kafka wire protocol](https://kafka.apache.org/protocol.html) in Rust.
//!
//! This library follows the approach of the Kafka project and generates the protocol through
//! [schema defined in JSON](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message).
//! This ensures not only compatibility, but easy synchronization when new features are added
//! upstream.
//!
//! The goal of this project is to provide 100% coverage of the Kafka API, as well as basic
//! utilities for working with the protocol, including compression and record serialization.
//!
//! # Messages
//! The [`messages`] module contains the generated request and response structs. These structs allow
//! easy serialization and deserialization via [`bytes::Bytes`].
//!
//! ```rust
//! use bytes::{Bytes, BytesMut};
//! use kafka_protocol::messages::RequestHeader;
//! use kafka_protocol::protocol::{StrBytes, Encodable, Decodable};
//!
//! let mut request_header = RequestHeader::default();
//! request_header.correlation_id = 1;
//! request_header.client_id = Some(StrBytes::from_static_str("test-client"));
//! let mut buf = BytesMut::new();
//! request_header.encode(&mut buf, 3);
//! assert_eq!(request_header, RequestHeader::decode(&mut buf, 3).unwrap());
//! ```
//! Note that every message implementation of [`Encodable::encode`](crate::protocol::Encodable::encode)
//! and [`Decodable::decode`](crate::protocol::Decodable::decode) requires a version to be provided
//! explicitly. This is because every  message contains *all* the fields that are valid for every
//! version. These fields are *not* marked [`Option`], which would represent nullability according
//! to a message's schema, but rather have a default value that represents a nil value. It is the
//! user's responsibility to ensure that only valid fields of the decoded message version are used.
//!
//! ## Sending a Request
//!
//! A request can be created by serializing a [`messages::RequestHeader`] and any given request
//! type to a [`bytes::Bytes`] buffer.
//!
//! ```rust
//! use kafka_protocol::protocol::{StrBytes, Encodable, HeaderVersion};
//! use bytes::{BytesMut, Bytes};
//! use kafka_protocol::messages::{RequestHeader, ApiKey, ApiVersionsRequest};
//! # use std::error::Error;
//!
//! let mut buf = BytesMut::new();
//! let mut req_header = RequestHeader::default();
//! req_header.request_api_version = 3;
//! req_header.request_api_key = ApiKey::ApiVersionsKey as i16;
//! req_header.client_id = Some(StrBytes::from_static_str("example"));
//! req_header.encode(&mut buf, ApiVersionsRequest::header_version(req_header.request_api_version)).unwrap();
//! let mut api_versions_req = ApiVersionsRequest::default();
//! api_versions_req.client_software_version = StrBytes::from_static_str("1.0");
//! api_versions_req.client_software_name = StrBytes::from_static_str("example-client");
//! api_versions_req.encode(&mut buf, req_header.request_api_version);
//!
//! # fn send_request(buf: &[u8]) -> Result<(), Box<dyn Error>> {
//! #     Ok(())
//! # }
//!
//! // implemented elsewhere
//! send_request(&buf[..]);
//! ```
//!
//! ## Deserializing an Unknown Request
//!
//! The [`messages`] module provides the enums [`messages::RequestKind`], [`messages::ResponseKind`],
//! and [`messages::ApiKey`] for matching on unknown requests and responses.
//!
//! A simple example for decoding an unknown message encoded in `buf`:
//! ```rust
//! use kafka_protocol::messages::{RequestHeader, ApiVersionsRequest, ApiKey, RequestKind};
//! use kafka_protocol::protocol::{Encodable, Decodable, StrBytes, HeaderVersion};
//! use bytes::{BytesMut, Buf};
//! use std::convert::TryFrom;
//! use kafka_protocol::protocol::buf::ByteBuf;
//! # let mut buf = BytesMut::new();
//! # let mut req_header = RequestHeader::default();
//! # req_header.request_api_version = 3;
//! # req_header.request_api_key = ApiKey::ApiVersionsKey as i16;
//! # req_header.client_id = Some(StrBytes::from_static_str("example"));
//! # req_header.encode(&mut buf, ApiVersionsRequest::header_version(req_header.request_api_version)).unwrap();
//! # let mut api_versions_req = ApiVersionsRequest::default();
//! # api_versions_req.client_software_version = StrBytes::from_static_str("1.0");
//! # api_versions_req.client_software_name = StrBytes::from_static_str("example-client");
//! # api_versions_req.encode(&mut buf, 3);
//!
//! let api_key = buf.peek_bytes(0..2).get_i16();
//! let api_version = buf.peek_bytes(2..4).get_i16();
//! let header_version = ApiKey::try_from(api_key).unwrap().request_header_version(api_version);
//!
//! let header = RequestHeader::decode(&mut buf, header_version).unwrap();
//! let api_key = ApiKey::try_from(header.request_api_version);
//! let req = match api_key {
//!     ApiVersionsKey => RequestKind::ApiVersionsRequest(ApiVersionsRequest::decode(&mut buf, header.request_api_version).unwrap()),
//! };
//!
//! // match on enum elsewhere and do work
//! match req {
//!     RequestKind::ApiVersionsRequest(req) => {
//!         assert_eq!(req.client_software_name.to_string(), "example-client".to_string());
//!     }
//!     _ => panic!()
//! }
//! ```
#![deny(missing_docs)]

pub mod compression;
pub mod error;
#[allow(clippy::all)]
pub mod messages;
pub mod protocol;
pub mod records;

pub use error::ResponseError;
pub use indexmap;
