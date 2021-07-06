//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use protocol_base::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-1
#[derive(Debug, Clone, PartialEq)]
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

impl Encodable for SaslHandshakeResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        types::Array(types::String).encode(buf, &self.mechanisms)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::Array(types::String).compute_size(&self.mechanisms)?;

        Ok(total_size)
    }
}

impl Decodable for SaslHandshakeResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
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
}

impl HeaderVersion for SaslHandshakeResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}

