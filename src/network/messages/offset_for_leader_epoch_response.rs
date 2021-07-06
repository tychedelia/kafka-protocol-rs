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


/// Valid versions: 0-3
#[derive(Debug, Clone)]
pub struct OffsetForLeaderPartitionResult {
    /// The error code 0, or if there was no error.
    /// 
    /// Supported API versions: 0-3
    pub error_code: i16,

    /// The partition index.
    /// 
    /// Supported API versions: 0-3
    pub partition_index: i32,

    /// The leader epoch of the partition.
    /// 
    /// Supported API versions: 1-3
    pub leader_epoch: i32,

    /// The end offset of the epoch.
    /// 
    /// Supported API versions: 0-3
    pub end_offset: i64,

}

impl Encodable for OffsetForLeaderPartitionResult {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        types::Int32.encode(buf, &self.partition_index)?;
        if version >= 1 {
            types::Int32.encode(buf, &self.leader_epoch)?;
        }
        types::Int64.encode(buf, &self.end_offset)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        if version >= 1 {
            total_size += types::Int32.compute_size(&self.leader_epoch)?;
        }
        total_size += types::Int64.compute_size(&self.end_offset)?;

        Ok(total_size)
    }
}

impl Decodable for OffsetForLeaderPartitionResult {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let partition_index = types::Int32.decode(buf)?;
        let leader_epoch = if version >= 1 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let end_offset = types::Int64.decode(buf)?;
        Ok(Self {
            error_code,
            partition_index,
            leader_epoch,
            end_offset,
        })
    }
}

impl Default for OffsetForLeaderPartitionResult {
    fn default() -> Self {
        Self {
            error_code: 0,
            partition_index: 0,
            leader_epoch: -1,
            end_offset: 0,
        }
    }
}

impl Message for OffsetForLeaderPartitionResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[derive(Debug, Clone)]
pub struct OffsetForLeaderTopicResult {
    /// The topic name.
    /// 
    /// Supported API versions: 0-3
    pub name: super::TopicName,

    /// Each partition in the topic we fetched offsets for.
    /// 
    /// Supported API versions: 0-3
    pub partitions: Vec<OffsetForLeaderPartitionResult>,

}

impl Encodable for OffsetForLeaderTopicResult {
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

impl Decodable for OffsetForLeaderTopicResult {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::String.decode(buf)?;
        let partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            name,
            partitions,
        })
    }
}

impl Default for OffsetForLeaderTopicResult {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl Message for OffsetForLeaderTopicResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[derive(Debug, Clone)]
pub struct OffsetForLeaderEpochResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 2-3
    pub throttle_time_ms: i32,

    /// Each topic we fetched offsets for.
    /// 
    /// Supported API versions: 0-3
    pub topics: Vec<OffsetForLeaderTopicResult>,

}

impl Encodable for OffsetForLeaderEpochResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 2 {
            types::Int32.encode(buf, &self.throttle_time_ms)?;
        }
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 2 {
            total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        }
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for OffsetForLeaderEpochResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = if version >= 2 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            throttle_time_ms,
            topics,
        })
    }
}

impl Default for OffsetForLeaderEpochResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            topics: Default::default(),
        }
    }
}

impl Message for OffsetForLeaderEpochResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

impl HeaderVersion for OffsetForLeaderEpochResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}

