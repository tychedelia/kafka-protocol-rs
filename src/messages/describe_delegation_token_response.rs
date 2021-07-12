//! DescribeDelegationTokenResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/DescribeDelegationTokenResponse.json).
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


/// Valid versions: 0-2
#[derive(Debug, Clone, PartialEq)]
pub struct DescribedDelegationTokenRenewer {
    /// The renewer principal type
    /// 
    /// Supported API versions: 0-2
    pub principal_type: StrBytes,

    /// The renewer principal name
    /// 
    /// Supported API versions: 0-2
    pub principal_name: StrBytes,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for DescribedDelegationTokenRenewer {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
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
            total_size += types::CompactString.compute_size(&self.principal_type)?;
        } else {
            total_size += types::String.compute_size(&self.principal_type)?;
        }
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.principal_name)?;
        } else {
            total_size += types::String.compute_size(&self.principal_name)?;
        }
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

impl Decodable for DescribedDelegationTokenRenewer {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
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
            principal_type,
            principal_name,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribedDelegationTokenRenewer {
    fn default() -> Self {
        Self {
            principal_type: Default::default(),
            principal_name: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribedDelegationTokenRenewer {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

/// Valid versions: 0-2
#[derive(Debug, Clone, PartialEq)]
pub struct DescribedDelegationToken {
    /// The token principal type.
    /// 
    /// Supported API versions: 0-2
    pub principal_type: StrBytes,

    /// The token principal name.
    /// 
    /// Supported API versions: 0-2
    pub principal_name: StrBytes,

    /// The token issue timestamp in milliseconds.
    /// 
    /// Supported API versions: 0-2
    pub issue_timestamp: i64,

    /// The token expiry timestamp in milliseconds.
    /// 
    /// Supported API versions: 0-2
    pub expiry_timestamp: i64,

    /// The token maximum timestamp length in milliseconds.
    /// 
    /// Supported API versions: 0-2
    pub max_timestamp: i64,

    /// The token ID.
    /// 
    /// Supported API versions: 0-2
    pub token_id: StrBytes,

    /// The token HMAC.
    /// 
    /// Supported API versions: 0-2
    pub hmac: Bytes,

    /// Those who are able to renew this token before it expires.
    /// 
    /// Supported API versions: 0-2
    pub renewers: Vec<DescribedDelegationTokenRenewer>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for DescribedDelegationToken {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
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
        types::Int64.encode(buf, &self.issue_timestamp)?;
        types::Int64.encode(buf, &self.expiry_timestamp)?;
        types::Int64.encode(buf, &self.max_timestamp)?;
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
        if version >= 2 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.renewers)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.renewers)?;
        }
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
            total_size += types::CompactString.compute_size(&self.principal_type)?;
        } else {
            total_size += types::String.compute_size(&self.principal_type)?;
        }
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.principal_name)?;
        } else {
            total_size += types::String.compute_size(&self.principal_name)?;
        }
        total_size += types::Int64.compute_size(&self.issue_timestamp)?;
        total_size += types::Int64.compute_size(&self.expiry_timestamp)?;
        total_size += types::Int64.compute_size(&self.max_timestamp)?;
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
        if version >= 2 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.renewers)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.renewers)?;
        }
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

impl Decodable for DescribedDelegationToken {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
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
        let issue_timestamp = types::Int64.decode(buf)?;
        let expiry_timestamp = types::Int64.decode(buf)?;
        let max_timestamp = types::Int64.decode(buf)?;
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
        let renewers = if version >= 2 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
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
            principal_type,
            principal_name,
            issue_timestamp,
            expiry_timestamp,
            max_timestamp,
            token_id,
            hmac,
            renewers,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribedDelegationToken {
    fn default() -> Self {
        Self {
            principal_type: Default::default(),
            principal_name: Default::default(),
            issue_timestamp: 0,
            expiry_timestamp: 0,
            max_timestamp: 0,
            token_id: Default::default(),
            hmac: Default::default(),
            renewers: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribedDelegationToken {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

/// Valid versions: 0-2
#[derive(Debug, Clone, PartialEq)]
pub struct DescribeDelegationTokenResponse {
    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-2
    pub error_code: i16,

    /// The tokens.
    /// 
    /// Supported API versions: 0-2
    pub tokens: Vec<DescribedDelegationToken>,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 0-2
    pub throttle_time_ms: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for DescribeDelegationTokenResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        if version >= 2 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.tokens)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.tokens)?;
        }
        types::Int32.encode(buf, &self.throttle_time_ms)?;
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
        total_size += types::Int16.compute_size(&self.error_code)?;
        if version >= 2 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.tokens)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.tokens)?;
        }
        total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
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

impl Decodable for DescribeDelegationTokenResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let tokens = if version >= 2 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let throttle_time_ms = types::Int32.decode(buf)?;
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
            error_code,
            tokens,
            throttle_time_ms,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribeDelegationTokenResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            tokens: Default::default(),
            throttle_time_ms: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribeDelegationTokenResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

impl HeaderVersion for DescribeDelegationTokenResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 2 {
            1
        } else {
            0
        }
    }
}

