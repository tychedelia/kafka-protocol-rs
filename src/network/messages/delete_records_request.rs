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


/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct DeleteRecordsPartition {
    /// The partition index.
    /// 
    /// Supported API versions: 0-1
    pub partition_index: i32,

    /// The deletion offset.
    /// 
    /// Supported API versions: 0-1
    pub offset: i64,

}

impl Encodable for DeleteRecordsPartition {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int64.encode(buf, &self.offset)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int64.compute_size(&self.offset)?;

        Ok(total_size)
    }
}

impl Decodable for DeleteRecordsPartition {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        let offset = types::Int64.decode(buf)?;
        Ok(Self {
            partition_index,
            offset,
        })
    }
}

impl Default for DeleteRecordsPartition {
    fn default() -> Self {
        Self {
            partition_index: 0,
            offset: 0,
        }
    }
}

impl Message for DeleteRecordsPartition {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct DeleteRecordsTopic {
    /// The topic name.
    /// 
    /// Supported API versions: 0-1
    pub name: super::TopicName,

    /// Each partition that we want to delete records from.
    /// 
    /// Supported API versions: 0-1
    pub partitions: Vec<DeleteRecordsPartition>,

}

impl Encodable for DeleteRecordsTopic {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
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

impl Decodable for DeleteRecordsTopic {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::String.decode(buf)?;
        let partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            name,
            partitions,
        })
    }
}

impl Default for DeleteRecordsTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl Message for DeleteRecordsTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct DeleteRecordsRequest {
    /// Each topic that we want to delete records from.
    /// 
    /// Supported API versions: 0-1
    pub topics: Vec<DeleteRecordsTopic>,

    /// How long to wait for the deletion to complete, in milliseconds.
    /// 
    /// Supported API versions: 0-1
    pub timeout_ms: i32,

}

impl Encodable for DeleteRecordsRequest {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;
        types::Int32.encode(buf, &self.timeout_ms)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
        total_size += types::Int32.compute_size(&self.timeout_ms)?;

        Ok(total_size)
    }
}

impl Decodable for DeleteRecordsRequest {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        let timeout_ms = types::Int32.decode(buf)?;
        Ok(Self {
            topics,
            timeout_ms,
        })
    }
}

impl Default for DeleteRecordsRequest {
    fn default() -> Self {
        Self {
            topics: Default::default(),
            timeout_ms: 0,
        }
    }
}

impl Message for DeleteRecordsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for DeleteRecordsRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}

