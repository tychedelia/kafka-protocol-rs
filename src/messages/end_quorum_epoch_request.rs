//! EndQuorumEpochRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/EndQuorumEpochRequest.json).
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

    /// The current leader ID that is resigning
    /// 
    /// Supported API versions: 0
    pub leader_id: super::BrokerId,

    /// The current epoch
    /// 
    /// Supported API versions: 0
    pub leader_epoch: i32,

    /// A sorted list of preferred successors to start the election
    /// 
    /// Supported API versions: 0
    pub preferred_successors: Vec<i32>,

}

impl Builder for PartitionData {
    type Builder = PartitionDataBuilder;

    fn builder() -> Self::Builder{
        PartitionDataBuilder::default()
    }
}

impl Encodable for PartitionData {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        buf.put_i32(self.partition_index);
        types::Int32.encode(buf, &self.leader_id)?;
        buf.put_i32(self.leader_epoch);
        types::Array(types::Int32).encode(buf, &self.preferred_successors)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += 4;
        total_size += types::Int32.compute_size(&self.leader_id)?;
        total_size += 4;
        total_size += types::Array(types::Int32).compute_size(&self.preferred_successors)?;

        Ok(total_size)
    }
}

impl Decodable for PartitionData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = buf.try_get_i32()?;
        let leader_id = types::Int32.decode(buf)?;
        let leader_epoch = buf.try_get_i32()?;
        let preferred_successors = types::Array(types::Int32).decode(buf)?;
        Ok(Self {
            partition_index,
            leader_id,
            leader_epoch,
            preferred_successors,
        })
    }
}

impl Default for PartitionData {
    fn default() -> Self {
        Self {
            partition_index: 0,
            leader_id: (0).into(),
            leader_epoch: 0,
            preferred_successors: Default::default(),
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
pub struct EndQuorumEpochRequest {
    /// 
    /// 
    /// Supported API versions: 0
    pub cluster_id: Option<StrBytes>,

    /// 
    /// 
    /// Supported API versions: 0
    pub topics: Vec<TopicData>,

}

impl Builder for EndQuorumEpochRequest {
    type Builder = EndQuorumEpochRequestBuilder;

    fn builder() -> Self::Builder{
        EndQuorumEpochRequestBuilder::default()
    }
}

impl Encodable for EndQuorumEpochRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, &self.cluster_id)?;
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.cluster_id)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for EndQuorumEpochRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let cluster_id = types::String.decode(buf)?;
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            cluster_id,
            topics,
        })
    }
}

impl Default for EndQuorumEpochRequest {
    fn default() -> Self {
        Self {
            cluster_id: None,
            topics: Default::default(),
        }
    }
}

impl Message for EndQuorumEpochRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

impl HeaderVersion for EndQuorumEpochRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}

