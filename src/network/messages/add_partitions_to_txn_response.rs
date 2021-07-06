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


/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct AddPartitionsToTxnPartitionResult {
    /// The response error code.
    /// 
    /// Supported API versions: 0-1
    pub error_code: i16,

}

impl MapEncodable for AddPartitionsToTxnPartitionResult {
    type Key = i32;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, key)?;
        types::Int16.encode(buf, &self.error_code)?;

        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(key)?;
        total_size += types::Int16.compute_size(&self.error_code)?;

        Ok(total_size)
    }
}

impl MapDecodable for AddPartitionsToTxnPartitionResult {
    type Key = i32;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::Int32.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        Ok((key_field, Self {
            error_code,
        }))
    }
}

impl Default for AddPartitionsToTxnPartitionResult {
    fn default() -> Self {
        Self {
            error_code: 0,
        }
    }
}

impl Message for AddPartitionsToTxnPartitionResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct AddPartitionsToTxnTopicResult {
    /// The results for each partition
    /// 
    /// Supported API versions: 0-1
    pub results: indexmap::IndexMap<i32, AddPartitionsToTxnPartitionResult>,

}

impl MapEncodable for AddPartitionsToTxnTopicResult {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, key)?;
        types::Array(types::Struct { version }).encode(buf, &self.results)?;

        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(key)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.results)?;

        Ok(total_size)
    }
}

impl MapDecodable for AddPartitionsToTxnTopicResult {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::String.decode(buf)?;
        let results = types::Array(types::Struct { version }).decode(buf)?;
        Ok((key_field, Self {
            results,
        }))
    }
}

impl Default for AddPartitionsToTxnTopicResult {
    fn default() -> Self {
        Self {
            results: Default::default(),
        }
    }
}

impl Message for AddPartitionsToTxnTopicResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct AddPartitionsToTxnResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 0-1
    pub throttle_time_ms: i32,

    /// The results for each topic.
    /// 
    /// Supported API versions: 0-1
    pub results: indexmap::IndexMap<super::TopicName, AddPartitionsToTxnTopicResult>,

}

impl Encodable for AddPartitionsToTxnResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.throttle_time_ms)?;
        types::Array(types::Struct { version }).encode(buf, &self.results)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.results)?;

        Ok(total_size)
    }
}

impl Decodable for AddPartitionsToTxnResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = types::Int32.decode(buf)?;
        let results = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            throttle_time_ms,
            results,
        })
    }
}

impl Default for AddPartitionsToTxnResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            results: Default::default(),
        }
    }
}

impl Message for AddPartitionsToTxnResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for AddPartitionsToTxnResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}

