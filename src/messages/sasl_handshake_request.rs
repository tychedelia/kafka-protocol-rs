//! SaslHandshakeRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/SaslHandshakeRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-1
#[derive(Debug, Clone, PartialEq)]
pub struct SaslHandshakeRequest {
    /// The SASL mechanism chosen by the client.
    /// 
    /// Supported API versions: 0-1
    pub mechanism: StrBytes,

}

impl Encodable for SaslHandshakeRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, &self.mechanism)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.mechanism)?;

        Ok(total_size)
    }
}

impl Decodable for SaslHandshakeRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let mechanism = types::String.decode(buf)?;
        Ok(Self {
            mechanism,
        })
    }
}

impl Default for SaslHandshakeRequest {
    fn default() -> Self {
        Self {
            mechanism: Default::default(),
        }
    }
}

impl Message for SaslHandshakeRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for SaslHandshakeRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}

