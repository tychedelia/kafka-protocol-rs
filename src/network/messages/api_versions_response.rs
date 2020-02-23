//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::{Buf, BufMut};
use log::error;

use franz_protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size,
};


/// Valid versions: 0-3
#[derive(Debug, Clone)]
pub struct ApiVersionsResponseKey {
    /// The minimum supported version, inclusive.
    /// 
    /// Supported API versions: 0-3
    pub min_version: i16,

    /// The maximum supported version, inclusive.
    /// 
    /// Supported API versions: 0-3
    pub max_version: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl MapEncodable for ApiVersionsResponseKey {
    type Key = i16;
    fn encode<B: BufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, key)?;
        types::Int16.encode(buf, &self.min_version)?;
        types::Int16.encode(buf, &self.max_version)?;
        if version == 3 {
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
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int16.compute_size(key)?;
        total_size += types::Int16.compute_size(&self.min_version)?;
        total_size += types::Int16.compute_size(&self.max_version)?;
        if version == 3 {
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

impl MapDecodable for ApiVersionsResponseKey {
    type Key = i16;
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::Int16.decode(buf)?;
        let min_version = types::Int16.decode(buf)?;
        let max_version = types::Int16.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 3 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok((key_field, Self {
            min_version,
            max_version,
            unknown_tagged_fields,
        }))
    }
}

impl Default for ApiVersionsResponseKey {
    fn default() -> Self {
        Self {
            min_version: 0,
            max_version: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ApiVersionsResponseKey {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[derive(Debug, Clone)]
pub struct ApiVersionsResponse {
    /// The top-level error code.
    /// 
    /// Supported API versions: 0-3
    pub error_code: i16,

    /// The APIs supported by the broker.
    /// 
    /// Supported API versions: 0-3
    pub api_keys: indexmap::IndexMap<i16, ApiVersionsResponseKey>,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 1-3
    pub throttle_time_ms: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for ApiVersionsResponse {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        if version == 3 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.api_keys)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.api_keys)?;
        }
        if version >= 1 {
            types::Int32.encode(buf, &self.throttle_time_ms)?;
        }
        if version == 3 {
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
        if version == 3 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.api_keys)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.api_keys)?;
        }
        if version >= 1 {
            total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        }
        if version == 3 {
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

impl Decodable for ApiVersionsResponse {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let api_keys = if version == 3 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let throttle_time_ms = if version >= 1 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 3 {
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
            api_keys,
            throttle_time_ms,
            unknown_tagged_fields,
        })
    }
}

impl Default for ApiVersionsResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            api_keys: Default::default(),
            throttle_time_ms: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ApiVersionsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

impl HeaderVersion for ApiVersionsResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}

