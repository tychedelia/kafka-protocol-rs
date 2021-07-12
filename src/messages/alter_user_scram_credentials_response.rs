//! AlterUserScramCredentialsResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/AlterUserScramCredentialsResponse.json).
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


/// Valid versions: 0
#[derive(Debug, Clone, PartialEq)]
pub struct AlterUserScramCredentialsResult {
    /// The user name.
    /// 
    /// Supported API versions: 0
    pub user: StrBytes,

    /// The error code.
    /// 
    /// Supported API versions: 0
    pub error_code: i16,

    /// The error message, if any.
    /// 
    /// Supported API versions: 0
    pub error_message: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for AlterUserScramCredentialsResult {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::CompactString.encode(buf, &self.user)?;
        types::Int16.encode(buf, &self.error_code)?;
        types::CompactString.encode(buf, &self.error_message)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::CompactString.compute_size(&self.user)?;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::CompactString.compute_size(&self.error_message)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for AlterUserScramCredentialsResult {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let user = types::CompactString.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        let error_message = types::CompactString.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let mut unknown_value = vec![0; size as usize];
            buf.try_copy_to_slice(&mut unknown_value)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            user,
            error_code,
            error_message,
            unknown_tagged_fields,
        })
    }
}

impl Default for AlterUserScramCredentialsResult {
    fn default() -> Self {
        Self {
            user: Default::default(),
            error_code: 0,
            error_message: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AlterUserScramCredentialsResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[derive(Debug, Clone, PartialEq)]
pub struct AlterUserScramCredentialsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 0
    pub throttle_time_ms: i32,

    /// The results for deletions and alterations, one per affected user.
    /// 
    /// Supported API versions: 0
    pub results: Vec<AlterUserScramCredentialsResult>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for AlterUserScramCredentialsResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.throttle_time_ms)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.results)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        total_size += types::CompactArray(types::Struct { version }).compute_size(&self.results)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for AlterUserScramCredentialsResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = types::Int32.decode(buf)?;
        let results = types::CompactArray(types::Struct { version }).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let mut unknown_value = vec![0; size as usize];
            buf.try_copy_to_slice(&mut unknown_value)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            throttle_time_ms,
            results,
            unknown_tagged_fields,
        })
    }
}

impl Default for AlterUserScramCredentialsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            results: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AlterUserScramCredentialsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

impl HeaderVersion for AlterUserScramCredentialsResponse {
    fn header_version(version: i16) -> i16 {
        1
    }
}

