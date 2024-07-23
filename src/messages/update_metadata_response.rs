//! UpdateMetadataResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/UpdateMetadataResponse.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use anyhow::{bail, Result};
use bytes::Bytes;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Decodable, Decoder,
    Encodable, Encoder, HeaderVersion, MapDecodable, MapEncodable, Message, StrBytes, VersionRange,
};

/// Valid versions: 0-8
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateMetadataResponse {
    /// The error code, or 0 if there was no error.
    ///
    /// Supported API versions: 0-8
    pub error_code: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl UpdateMetadataResponse {
    /// Sets `error_code` to the passed value.
    ///
    /// The error code, or 0 if there was no error.
    ///
    /// Supported API versions: 0-8
    pub fn with_error_code(mut self, value: i16) -> Self {
        self.error_code = value;
        self
    }
    /// Sets unknown_tagged_fields to the passed value.
    pub fn with_unknown_tagged_fields(mut self, value: BTreeMap<i32, Bytes>) -> Self {
        self.unknown_tagged_fields = value;
        self
    }
    /// Inserts an entry into unknown_tagged_fields.
    pub fn with_unknown_tagged_field(mut self, key: i32, value: Bytes) -> Self {
        self.unknown_tagged_fields.insert(key, value);
        self
    }
}

impl Encodable for UpdateMetadataResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::Int16.encode(buf, &self.error_code)?;
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::Int16.compute_size(&self.error_code)?;
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for UpdateMetadataResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let error_code = types::Int16.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            error_code,
            unknown_tagged_fields,
        })
    }
}

impl Default for UpdateMetadataResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for UpdateMetadataResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for UpdateMetadataResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 6 {
            1
        } else {
            0
        }
    }
}
