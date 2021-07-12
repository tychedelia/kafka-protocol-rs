//! ExpireDelegationTokenResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ExpireDelegationTokenResponse.json).
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
pub struct ExpireDelegationTokenResponse {
    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-2
    pub error_code: i16,

    /// The timestamp in milliseconds at which this token expires.
    /// 
    /// Supported API versions: 0-2
    pub expiry_timestamp_ms: i64,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 0-2
    pub throttle_time_ms: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for ExpireDelegationTokenResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        types::Int64.encode(buf, &self.expiry_timestamp_ms)?;
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
        total_size += types::Int64.compute_size(&self.expiry_timestamp_ms)?;
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

impl Decodable for ExpireDelegationTokenResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let expiry_timestamp_ms = types::Int64.decode(buf)?;
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
            expiry_timestamp_ms,
            throttle_time_ms,
            unknown_tagged_fields,
        })
    }
}

impl Default for ExpireDelegationTokenResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            expiry_timestamp_ms: 0,
            throttle_time_ms: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ExpireDelegationTokenResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

impl HeaderVersion for ExpireDelegationTokenResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 2 {
            1
        } else {
            0
        }
    }
}

