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
pub struct DescribeLogDirsPartition {
    /// The partition index.
    /// 
    /// Supported API versions: 0-1
    pub partition_index: i32,

    /// The size of the log segments in this partition in bytes.
    /// 
    /// Supported API versions: 0-1
    pub partition_size: i64,

    /// The lag of the log's LEO w.r.t. partition's HW (if it is the current log for the partition) or current replica's LEO (if it is the future log for the partition)
    /// 
    /// Supported API versions: 0-1
    pub offset_lag: i64,

    /// True if this log is created by AlterReplicaLogDirsRequest and will replace the current log of the replica in the future.
    /// 
    /// Supported API versions: 0-1
    pub is_future_key: bool,

}

impl Encodable for DescribeLogDirsPartition {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int64.encode(buf, &self.partition_size)?;
        types::Int64.encode(buf, &self.offset_lag)?;
        types::Boolean.encode(buf, &self.is_future_key)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int64.compute_size(&self.partition_size)?;
        total_size += types::Int64.compute_size(&self.offset_lag)?;
        total_size += types::Boolean.compute_size(&self.is_future_key)?;

        Ok(total_size)
    }
}

impl Decodable for DescribeLogDirsPartition {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        let partition_size = types::Int64.decode(buf)?;
        let offset_lag = types::Int64.decode(buf)?;
        let is_future_key = types::Boolean.decode(buf)?;
        Ok(Self {
            partition_index,
            partition_size,
            offset_lag,
            is_future_key,
        })
    }
}

impl Default for DescribeLogDirsPartition {
    fn default() -> Self {
        Self {
            partition_index: 0,
            partition_size: 0,
            offset_lag: 0,
            is_future_key: false,
        }
    }
}

impl Message for DescribeLogDirsPartition {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct DescribeLogDirsTopic {
    /// The topic name.
    /// 
    /// Supported API versions: 0-1
    pub name: super::TopicName,

    /// 
    /// 
    /// Supported API versions: 0-1
    pub partitions: Vec<DescribeLogDirsPartition>,

}

impl Encodable for DescribeLogDirsTopic {
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

impl Decodable for DescribeLogDirsTopic {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::String.decode(buf)?;
        let partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            name,
            partitions,
        })
    }
}

impl Default for DescribeLogDirsTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl Message for DescribeLogDirsTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct DescribeLogDirsResult {
    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-1
    pub error_code: i16,

    /// The absolute log directory path.
    /// 
    /// Supported API versions: 0-1
    pub log_dir: StrBytes,

    /// Each topic.
    /// 
    /// Supported API versions: 0-1
    pub topics: Vec<DescribeLogDirsTopic>,

}

impl Encodable for DescribeLogDirsResult {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        types::String.encode(buf, &self.log_dir)?;
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::String.compute_size(&self.log_dir)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for DescribeLogDirsResult {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let log_dir = types::String.decode(buf)?;
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            error_code,
            log_dir,
            topics,
        })
    }
}

impl Default for DescribeLogDirsResult {
    fn default() -> Self {
        Self {
            error_code: 0,
            log_dir: Default::default(),
            topics: Default::default(),
        }
    }
}

impl Message for DescribeLogDirsResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct DescribeLogDirsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 0-1
    pub throttle_time_ms: i32,

    /// The log directories.
    /// 
    /// Supported API versions: 0-1
    pub results: Vec<DescribeLogDirsResult>,

}

impl Encodable for DescribeLogDirsResponse {
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

impl Decodable for DescribeLogDirsResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = types::Int32.decode(buf)?;
        let results = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            throttle_time_ms,
            results,
        })
    }
}

impl Default for DescribeLogDirsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            results: Default::default(),
        }
    }
}

impl Message for DescribeLogDirsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for DescribeLogDirsResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}

