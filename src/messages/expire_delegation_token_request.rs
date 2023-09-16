//! ExpireDelegationTokenRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ExpireDelegationTokenRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}, Builder
};


/// Valid versions: 0-2
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct ExpireDelegationTokenRequest {
    /// The HMAC of the delegation token to be expired.
    /// 
    /// Supported API versions: 0-2
    pub hmac: Bytes,

    /// The expiry time period in milliseconds.
    /// 
    /// Supported API versions: 0-2
    pub expiry_time_period_ms: i64,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for ExpireDelegationTokenRequest {
    type Builder = ExpireDelegationTokenRequestBuilder;

    fn builder() -> Self::Builder{
        ExpireDelegationTokenRequestBuilder::default()
    }
}

impl Encodable for ExpireDelegationTokenRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 2 {
            types::CompactBytes.encode(buf, &self.hmac)?;
        } else {
            types::Bytes.encode(buf, &self.hmac)?;
        }
        types::Int64.encode(buf, &self.expiry_time_period_ms)?;
        if version >= 2 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 2 {
            total_size += types::CompactBytes.compute_size(&self.hmac)?;
        } else {
            total_size += types::Bytes.compute_size(&self.hmac)?;
        }
        total_size += types::Int64.compute_size(&self.expiry_time_period_ms)?;
        if version >= 2 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for ExpireDelegationTokenRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let hmac = if version >= 2 {
            types::CompactBytes.decode(buf)?
        } else {
            types::Bytes.decode(buf)?
        };
        let expiry_time_period_ms = types::Int64.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            hmac,
            expiry_time_period_ms,
            unknown_tagged_fields,
        })
    }
}

impl Default for ExpireDelegationTokenRequest {
    fn default() -> Self {
        Self {
            hmac: Default::default(),
            expiry_time_period_ms: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ExpireDelegationTokenRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

impl HeaderVersion for ExpireDelegationTokenRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 2 {
            2
        } else {
            1
        }
    }
}

