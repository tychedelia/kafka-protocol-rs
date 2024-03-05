//! CreateDelegationTokenResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/CreateDelegationTokenResponse.json).
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

/// Valid versions: 0-3
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct CreateDelegationTokenResponse {
    /// The top-level error, or zero if there was no error.
    ///
    /// Supported API versions: 0-3
    pub error_code: i16,

    /// The principal type of the token owner.
    ///
    /// Supported API versions: 0-3
    pub principal_type: StrBytes,

    /// The name of the token owner.
    ///
    /// Supported API versions: 0-3
    pub principal_name: StrBytes,

    /// The principal type of the requester of the token.
    ///
    /// Supported API versions: 3
    pub token_requester_principal_type: StrBytes,

    /// The principal type of the requester of the token.
    ///
    /// Supported API versions: 3
    pub token_requester_principal_name: StrBytes,

    /// When this token was generated.
    ///
    /// Supported API versions: 0-3
    pub issue_timestamp_ms: i64,

    /// When this token expires.
    ///
    /// Supported API versions: 0-3
    pub expiry_timestamp_ms: i64,

    /// The maximum lifetime of this token.
    ///
    /// Supported API versions: 0-3
    pub max_timestamp_ms: i64,

    /// The token UUID.
    ///
    /// Supported API versions: 0-3
    pub token_id: StrBytes,

    /// HMAC of the delegation token.
    ///
    /// Supported API versions: 0-3
    pub hmac: Bytes,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    ///
    /// Supported API versions: 0-3
    pub throttle_time_ms: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for CreateDelegationTokenResponse {
    type Builder = CreateDelegationTokenResponseBuilder;

    fn builder() -> Self::Builder {
        CreateDelegationTokenResponseBuilder::default()
    }
}

impl Encodable for CreateDelegationTokenResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        if version >= 2 {
            types::CompactString.encode(buf, &self.principal_type)?;
        } else {
            types::String.encode(buf, &self.principal_type)?;
        }
        if version >= 2 {
            types::CompactString.encode(buf, &self.principal_name)?;
        } else {
            types::String.encode(buf, &self.principal_name)?;
        }
        if version >= 3 {
            types::CompactString.encode(buf, &self.token_requester_principal_type)?;
        } else {
            if !self.token_requester_principal_type.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 3 {
            types::CompactString.encode(buf, &self.token_requester_principal_name)?;
        } else {
            if !self.token_requester_principal_name.is_empty() {
                bail!("failed to encode");
            }
        }
        types::Int64.encode(buf, &self.issue_timestamp_ms)?;
        types::Int64.encode(buf, &self.expiry_timestamp_ms)?;
        types::Int64.encode(buf, &self.max_timestamp_ms)?;
        if version >= 2 {
            types::CompactString.encode(buf, &self.token_id)?;
        } else {
            types::String.encode(buf, &self.token_id)?;
        }
        if version >= 2 {
            types::CompactBytes.encode(buf, &self.hmac)?;
        } else {
            types::Bytes.encode(buf, &self.hmac)?;
        }
        types::Int32.encode(buf, &self.throttle_time_ms)?;
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
        total_size += types::Int16.compute_size(&self.error_code)?;
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.principal_type)?;
        } else {
            total_size += types::String.compute_size(&self.principal_type)?;
        }
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.principal_name)?;
        } else {
            total_size += types::String.compute_size(&self.principal_name)?;
        }
        if version >= 3 {
            total_size +=
                types::CompactString.compute_size(&self.token_requester_principal_type)?;
        } else {
            if !self.token_requester_principal_type.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 3 {
            total_size +=
                types::CompactString.compute_size(&self.token_requester_principal_name)?;
        } else {
            if !self.token_requester_principal_name.is_empty() {
                bail!("failed to encode");
            }
        }
        total_size += types::Int64.compute_size(&self.issue_timestamp_ms)?;
        total_size += types::Int64.compute_size(&self.expiry_timestamp_ms)?;
        total_size += types::Int64.compute_size(&self.max_timestamp_ms)?;
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.token_id)?;
        } else {
            total_size += types::String.compute_size(&self.token_id)?;
        }
        if version >= 2 {
            total_size += types::CompactBytes.compute_size(&self.hmac)?;
        } else {
            total_size += types::Bytes.compute_size(&self.hmac)?;
        }
        total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
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

impl Decodable for CreateDelegationTokenResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let principal_type = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let principal_name = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let token_requester_principal_type = if version >= 3 {
            types::CompactString.decode(buf)?
        } else {
            Default::default()
        };
        let token_requester_principal_name = if version >= 3 {
            types::CompactString.decode(buf)?
        } else {
            Default::default()
        };
        let issue_timestamp_ms = types::Int64.decode(buf)?;
        let expiry_timestamp_ms = types::Int64.decode(buf)?;
        let max_timestamp_ms = types::Int64.decode(buf)?;
        let token_id = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let hmac = if version >= 2 {
            types::CompactBytes.decode(buf)?
        } else {
            types::Bytes.decode(buf)?
        };
        let throttle_time_ms = types::Int32.decode(buf)?;
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
            error_code,
            principal_type,
            principal_name,
            token_requester_principal_type,
            token_requester_principal_name,
            issue_timestamp_ms,
            expiry_timestamp_ms,
            max_timestamp_ms,
            token_id,
            hmac,
            throttle_time_ms,
            unknown_tagged_fields,
        })
    }
}

impl Default for CreateDelegationTokenResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            principal_type: Default::default(),
            principal_name: Default::default(),
            token_requester_principal_type: Default::default(),
            token_requester_principal_name: Default::default(),
            issue_timestamp_ms: 0,
            expiry_timestamp_ms: 0,
            max_timestamp_ms: 0,
            token_id: Default::default(),
            hmac: Default::default(),
            throttle_time_ms: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreateDelegationTokenResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for CreateDelegationTokenResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 2 {
            1
        } else {
            0
        }
    }
}
