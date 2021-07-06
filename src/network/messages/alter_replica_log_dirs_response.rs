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
pub struct AlterReplicaLogDirPartitionResult {
    /// The partition index.
    /// 
    /// Supported API versions: 0-1
    pub partition_index: i32,

    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-1
    pub error_code: i16,

}

impl Encodable for AlterReplicaLogDirPartitionResult {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int16.encode(buf, &self.error_code)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int16.compute_size(&self.error_code)?;

        Ok(total_size)
    }
}

impl Decodable for AlterReplicaLogDirPartitionResult {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        Ok(Self {
            partition_index,
            error_code,
        })
    }
}

impl Default for AlterReplicaLogDirPartitionResult {
    fn default() -> Self {
        Self {
            partition_index: 0,
            error_code: 0,
        }
    }
}

impl Message for AlterReplicaLogDirPartitionResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct AlterReplicaLogDirTopicResult {
    /// The name of the topic.
    /// 
    /// Supported API versions: 0-1
    pub topic_name: super::TopicName,

    /// The results for each partition.
    /// 
    /// Supported API versions: 0-1
    pub partitions: Vec<AlterReplicaLogDirPartitionResult>,

}

impl Encodable for AlterReplicaLogDirTopicResult {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, &self.topic_name)?;
        types::Array(types::Struct { version }).encode(buf, &self.partitions)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.topic_name)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.partitions)?;

        Ok(total_size)
    }
}

impl Decodable for AlterReplicaLogDirTopicResult {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topic_name = types::String.decode(buf)?;
        let partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            topic_name,
            partitions,
        })
    }
}

impl Default for AlterReplicaLogDirTopicResult {
    fn default() -> Self {
        Self {
            topic_name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl Message for AlterReplicaLogDirTopicResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct AlterReplicaLogDirsResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 0-1
    pub throttle_time_ms: i32,

    /// The results for each topic.
    /// 
    /// Supported API versions: 0-1
    pub results: Vec<AlterReplicaLogDirTopicResult>,

}

impl Encodable for AlterReplicaLogDirsResponse {
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

impl Decodable for AlterReplicaLogDirsResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = types::Int32.decode(buf)?;
        let results = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            throttle_time_ms,
            results,
        })
    }
}

impl Default for AlterReplicaLogDirsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            results: Default::default(),
        }
    }
}

impl Message for AlterReplicaLogDirsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for AlterReplicaLogDirsResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}

