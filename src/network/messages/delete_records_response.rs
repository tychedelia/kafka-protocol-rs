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
pub struct DeleteRecordsPartitionResult {
    /// The partition index.
    /// 
    /// Supported API versions: 0-1
    pub partition_index: i32,

    /// The partition low water mark.
    /// 
    /// Supported API versions: 0-1
    pub low_watermark: i64,

    /// The deletion error code, or 0 if the deletion succeeded.
    /// 
    /// Supported API versions: 0-1
    pub error_code: i16,

}

impl Encodable for DeleteRecordsPartitionResult {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int64.encode(buf, &self.low_watermark)?;
        types::Int16.encode(buf, &self.error_code)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int64.compute_size(&self.low_watermark)?;
        total_size += types::Int16.compute_size(&self.error_code)?;

        Ok(total_size)
    }
}

impl Decodable for DeleteRecordsPartitionResult {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        let low_watermark = types::Int64.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        Ok(Self {
            partition_index,
            low_watermark,
            error_code,
        })
    }
}

impl Default for DeleteRecordsPartitionResult {
    fn default() -> Self {
        Self {
            partition_index: 0,
            low_watermark: 0,
            error_code: 0,
        }
    }
}

impl Message for DeleteRecordsPartitionResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct DeleteRecordsTopicResult {
    /// The topic name.
    /// 
    /// Supported API versions: 0-1
    pub name: super::TopicName,

    /// Each partition that we wanted to delete records from.
    /// 
    /// Supported API versions: 0-1
    pub partitions: Vec<DeleteRecordsPartitionResult>,

}

impl Encodable for DeleteRecordsTopicResult {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, &self.name)?;
        types::Array(types::Struct { version }).encode(buf, &self.partitions)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.name)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.partitions)?;

        Ok(total_size)
    }
}

impl Decodable for DeleteRecordsTopicResult {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::String.decode(buf)?;
        let partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            name,
            partitions,
        })
    }
}

impl Default for DeleteRecordsTopicResult {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl Message for DeleteRecordsTopicResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct DeleteRecordsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 0-1
    pub throttle_time_ms: i32,

    /// Each topic that we wanted to delete records from.
    /// 
    /// Supported API versions: 0-1
    pub topics: Vec<DeleteRecordsTopicResult>,

}

impl Encodable for DeleteRecordsResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.throttle_time_ms)?;
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for DeleteRecordsResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = types::Int32.decode(buf)?;
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            throttle_time_ms,
            topics,
        })
    }
}

impl Default for DeleteRecordsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            topics: Default::default(),
        }
    }
}

impl Message for DeleteRecordsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for DeleteRecordsResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}

