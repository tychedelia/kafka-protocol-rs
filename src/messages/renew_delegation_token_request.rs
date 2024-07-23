//! RenewDelegationTokenRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/RenewDelegationTokenRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use anyhow::bail;
use bytes::Bytes;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Builder, Decodable,
    DecodeError, Decoder, Encodable, EncodeError, Encoder, HeaderVersion, MapDecodable,
    MapEncodable, Message, StrBytes, VersionRange,
};

/// Valid versions: 0-2
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct RenewDelegationTokenRequest {
    /// The HMAC of the delegation token to be renewed.
    ///
    /// Supported API versions: 0-2
    pub hmac: Bytes,

    /// The renewal time period in milliseconds.
    ///
    /// Supported API versions: 0-2
    pub renew_period_ms: i64,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for RenewDelegationTokenRequest {
    type Builder = RenewDelegationTokenRequestBuilder;

    fn builder() -> Self::Builder {
        RenewDelegationTokenRequestBuilder::default()
    }
}

impl Encodable for RenewDelegationTokenRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 2 {
            types::CompactBytes.encode(buf, &self.hmac)?;
        } else {
            types::Bytes.encode(buf, &self.hmac)?;
        }
        types::Int64.encode(buf, &self.renew_period_ms)?;
        if version >= 2 {
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
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 2 {
            total_size += types::CompactBytes.compute_size(&self.hmac)?;
        } else {
            total_size += types::Bytes.compute_size(&self.hmac)?;
        }
        total_size += types::Int64.compute_size(&self.renew_period_ms)?;
        if version >= 2 {
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

impl Decodable for RenewDelegationTokenRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let hmac = if version >= 2 {
            types::CompactBytes.decode(buf)?
        } else {
            types::Bytes.decode(buf)?
        };
        let renew_period_ms = types::Int64.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            hmac,
            renew_period_ms,
            unknown_tagged_fields,
        })
    }
}

impl Default for RenewDelegationTokenRequest {
    fn default() -> Self {
        Self {
            hmac: Default::default(),
            renew_period_ms: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for RenewDelegationTokenRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = Some(VersionRange { min: 0, max: 0 });
}

impl HeaderVersion for RenewDelegationTokenRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 2 {
            2
        } else {
            1
        }
    }
}
