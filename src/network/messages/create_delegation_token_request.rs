//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;

use protocol_base::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-2
#[derive(Debug, Clone)]
pub struct CreatableRenewers {
    /// The type of the Kafka principal.
    /// 
    /// Supported API versions: 0-2
    pub principal_type: StrBytes,

    /// The name of the Kafka principal.
    /// 
    /// Supported API versions: 0-2
    pub principal_name: StrBytes,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for CreatableRenewers {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 2 {
            types::CompactString.encode(buf, &self.principal_type)?;
        } else {
            types::String.encode(buf, &self.principal_type)?;
        }
        if version == 2 {
            types::CompactString.encode(buf, &self.principal_name)?;
        } else {
            types::String.encode(buf, &self.principal_name)?;
        }
        if version == 2 {
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
        if version == 2 {
            total_size += types::CompactString.compute_size(&self.principal_type)?;
        } else {
            total_size += types::String.compute_size(&self.principal_type)?;
        }
        if version == 2 {
            total_size += types::CompactString.compute_size(&self.principal_name)?;
        } else {
            total_size += types::String.compute_size(&self.principal_name)?;
        }
        if version == 2 {
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

impl Decodable for CreatableRenewers {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let principal_type = if version == 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let principal_name = if version == 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 2 {
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

impl Default for CreatableRenewers {
    fn default() -> Self {
        Self {
            principal_type: Default::default(),
            principal_name: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreatableRenewers {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

/// Valid versions: 0-2
#[derive(Debug, Clone)]
pub struct CreateDelegationTokenRequest {
    /// A list of those who are allowed to renew this token before it expires.
    /// 
    /// Supported API versions: 0-2
    pub renewers: Vec<CreatableRenewers>,

    /// The maximum lifetime of the token in milliseconds, or -1 to use the server side default.
    /// 
    /// Supported API versions: 0-2
    pub max_lifetime_ms: i64,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for CreateDelegationTokenRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 2 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.renewers)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.renewers)?;
        }
        types::Int64.encode(buf, &self.max_lifetime_ms)?;
        if version == 2 {
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
        if version == 2 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.renewers)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.renewers)?;
        }
        total_size += types::Int64.compute_size(&self.max_lifetime_ms)?;
        if version == 2 {
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

impl Decodable for CreateDelegationTokenRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let renewers = if version == 2 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let max_lifetime_ms = types::Int64.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 2 {
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
            renewers,
            max_lifetime_ms,
            unknown_tagged_fields,
        })
    }
}

impl Default for CreateDelegationTokenRequest {
    fn default() -> Self {
        Self {
            renewers: Default::default(),
            max_lifetime_ms: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreateDelegationTokenRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

impl HeaderVersion for CreateDelegationTokenRequest {
    fn header_version(version: i16) -> i16 {
        if version == 2 {
            2
        } else {
            1
        }
    }
}

