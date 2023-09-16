//! BeginQuorumEpochResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/BeginQuorumEpochResponse.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}, Builder
};


/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct PartitionData {
    /// The partition index.
    /// 
    /// Supported API versions: 0
    pub partition_index: i32,

    /// 
    /// 
    /// Supported API versions: 0
    pub error_code: i16,

    /// The ID of the current leader or -1 if the leader is unknown.
    /// 
    /// Supported API versions: 0
    pub leader_id: super::BrokerId,

    /// The latest known leader epoch
    /// 
    /// Supported API versions: 0
    pub leader_epoch: i32,

}

impl Builder for PartitionData {
    type Builder = PartitionDataBuilder;

    fn builder() -> Self::Builder{
        PartitionDataBuilder::default()
    }
}

impl Encodable for PartitionData {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int16.encode(buf, &self.error_code)?;
        types::Int32.encode(buf, &self.leader_id)?;
        types::Int32.encode(buf, &self.leader_epoch)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::Int32.compute_size(&self.leader_id)?;
        total_size += types::Int32.compute_size(&self.leader_epoch)?;

        Ok(total_size)
    }
}

impl Decodable for PartitionData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        let leader_id = types::Int32.decode(buf)?;
        let leader_epoch = types::Int32.decode(buf)?;
        Ok(Self {
            partition_index,
            error_code,
            leader_id,
            leader_epoch,
        })
    }
}

impl Default for PartitionData {
    fn default() -> Self {
        Self {
            partition_index: 0,
            error_code: 0,
            leader_id: (0).into(),
            leader_epoch: 0,
        }
    }
}

impl Message for PartitionData {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct TopicData {
    /// The topic name.
    /// 
    /// Supported API versions: 0
    pub topic_name: super::TopicName,

    /// 
    /// 
    /// Supported API versions: 0
    pub partitions: Vec<PartitionData>,

}

impl Builder for TopicData {
    type Builder = TopicDataBuilder;

    fn builder() -> Self::Builder{
        TopicDataBuilder::default()
    }
}

impl Encodable for TopicData {
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

impl Decodable for TopicData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topic_name = types::String.decode(buf)?;
        let partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            topic_name,
            partitions,
        })
    }
}

impl Default for TopicData {
    fn default() -> Self {
        Self {
            topic_name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl Message for TopicData {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct BeginQuorumEpochResponse {
    /// The top level error code.
    /// 
    /// Supported API versions: 0
    pub error_code: i16,

    /// 
    /// 
    /// Supported API versions: 0
    pub topics: Vec<TopicData>,

}

impl Builder for BeginQuorumEpochResponse {
    type Builder = BeginQuorumEpochResponseBuilder;

    fn builder() -> Self::Builder{
        BeginQuorumEpochResponseBuilder::default()
    }
}

impl Encodable for BeginQuorumEpochResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for BeginQuorumEpochResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            error_code,
            topics,
        })
    }
}

impl Default for BeginQuorumEpochResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            topics: Default::default(),
        }
    }
}

impl Message for BeginQuorumEpochResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

impl HeaderVersion for BeginQuorumEpochResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}

