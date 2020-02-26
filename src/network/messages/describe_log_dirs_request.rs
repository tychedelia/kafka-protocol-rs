//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;

use franz_protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct DescribableLogDirTopic {
    /// The topic name
    /// 
    /// Supported API versions: 0-1
    pub topic: super::TopicName,

    /// The partition indxes.
    /// 
    /// Supported API versions: 0-1
    pub partition_index: Vec<i32>,

}

impl Encodable for DescribableLogDirTopic {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, &self.topic)?;
        types::Array(types::Int32).encode(buf, &self.partition_index)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.topic)?;
        total_size += types::Array(types::Int32).compute_size(&self.partition_index)?;

        Ok(total_size)
    }
}

impl Decodable for DescribableLogDirTopic {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topic = types::String.decode(buf)?;
        let partition_index = types::Array(types::Int32).decode(buf)?;
        Ok(Self {
            topic,
            partition_index,
        })
    }
}

impl Default for DescribableLogDirTopic {
    fn default() -> Self {
        Self {
            topic: Default::default(),
            partition_index: Default::default(),
        }
    }
}

impl Message for DescribableLogDirTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct DescribeLogDirsRequest {
    /// Each topic that we want to describe log directories for, or null for all topics.
    /// 
    /// Supported API versions: 0-1
    pub topics: Option<Vec<DescribableLogDirTopic>>,

}

impl Encodable for DescribeLogDirsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for DescribeLogDirsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            topics,
        })
    }
}

impl Default for DescribeLogDirsRequest {
    fn default() -> Self {
        Self {
            topics: Some(Default::default()),
        }
    }
}

impl Message for DescribeLogDirsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for DescribeLogDirsRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}

