//! EnvelopeRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/EnvelopeRequest.json).
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

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct EnvelopeRequest {
    /// The embedded request header and data.
    ///
    /// Supported API versions: 0
    pub request_data: Bytes,

    /// Value of the initial client principal when the request is redirected by a broker.
    ///
    /// Supported API versions: 0
    pub request_principal: Option<Bytes>,

    /// The original client's address in bytes.
    ///
    /// Supported API versions: 0
    pub client_host_address: Bytes,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for EnvelopeRequest {
    type Builder = EnvelopeRequestBuilder;

    fn builder() -> Self::Builder {
        EnvelopeRequestBuilder::default()
    }
}

impl Encodable for EnvelopeRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::CompactBytes.encode(buf, &self.request_data)?;
        types::CompactBytes.encode(buf, &self.request_principal)?;
        types::CompactBytes.encode(buf, &self.client_host_address)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::CompactBytes.compute_size(&self.request_data)?;
        total_size += types::CompactBytes.compute_size(&self.request_principal)?;
        total_size += types::CompactBytes.compute_size(&self.client_host_address)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for EnvelopeRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let request_data = types::CompactBytes.decode(buf)?;
        let request_principal = types::CompactBytes.decode(buf)?;
        let client_host_address = types::CompactBytes.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            request_data,
            request_principal,
            client_host_address,
            unknown_tagged_fields,
        })
    }
}

impl Default for EnvelopeRequest {
    fn default() -> Self {
        Self {
            request_data: Default::default(),
            request_principal: Some(Default::default()),
            client_host_address: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for EnvelopeRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for EnvelopeRequest {
    fn header_version(version: i16) -> i16 {
        2
    }
}
