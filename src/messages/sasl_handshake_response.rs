//! SaslHandshakeResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/SaslHandshakeResponse.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use anyhow::{bail, Result};
use bytes::Bytes;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Builder, Decodable,
    Decoder, Encodable, Encoder, HeaderVersion, MapDecodable, MapEncodable, Message, StrBytes,
    VersionRange,
};

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct SaslHandshakeResponse {
    /// The error code, or 0 if there was no error.
    ///
    /// Supported API versions: 0-1
    pub error_code: i16,

    /// The mechanisms enabled in the server.
    ///
    /// Supported API versions: 0-1
    pub mechanisms: Vec<StrBytes>,
}

impl Builder for SaslHandshakeResponse {
    type Builder = SaslHandshakeResponseBuilder;

    fn builder() -> Self::Builder {
        SaslHandshakeResponseBuilder::default()
    }
}

impl Encodable for SaslHandshakeResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::Int16.encode(buf, &self.error_code)?;
        types::Array(types::String).encode(buf, &self.mechanisms)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::Array(types::String).compute_size(&self.mechanisms)?;

        Ok(total_size)
    }
}

impl Decodable for SaslHandshakeResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let error_code = types::Int16.decode(buf)?;
        let mechanisms = types::Array(types::String).decode(buf)?;
        Ok(Self {
            error_code,
            mechanisms,
        })
    }
}

impl Default for SaslHandshakeResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            mechanisms: Default::default(),
        }
    }
}

impl Message for SaslHandshakeResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for SaslHandshakeResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}
