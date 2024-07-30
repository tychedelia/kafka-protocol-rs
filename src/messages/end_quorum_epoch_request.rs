//! EndQuorumEpochRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/EndQuorumEpochRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use anyhow::{bail, Result};
use bytes::Bytes;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Decodable, Decoder,
    Encodable, Encoder, HeaderVersion, MapDecodable, MapEncodable, Message, StrBytes, VersionRange,
};

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
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

impl EndQuorumEpochRequest {
    /// Sets `cluster_id` to the passed value.
    ///
    ///
    ///
    /// Supported API versions: 0
    pub fn with_cluster_id(mut self, value: Option<StrBytes>) -> Self {
        self.cluster_id = value;
        self
    }
    /// Sets `topics` to the passed value.
    ///
    ///
    ///
    /// Supported API versions: 0
    pub fn with_topics(mut self, value: Vec<TopicData>) -> Self {
        self.topics = value;
        self
    }
}

impl Encodable for EndQuorumEpochRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::String.encode(buf, &self.cluster_id)?;
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.cluster_id)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for EndQuorumEpochRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let cluster_id = types::String.decode(buf)?;
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self { cluster_id, topics })
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
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
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

impl PartitionData {
    /// Sets `partition_index` to the passed value.
    ///
    /// The partition index.
    ///
    /// Supported API versions: 0
    pub fn with_partition_index(mut self, value: i32) -> Self {
        self.partition_index = value;
        self
    }
    /// Sets `leader_id` to the passed value.
    ///
    /// The current leader ID that is resigning
    ///
    /// Supported API versions: 0
    pub fn with_leader_id(mut self, value: super::BrokerId) -> Self {
        self.leader_id = value;
        self
    }
    /// Sets `leader_epoch` to the passed value.
    ///
    /// The current epoch
    ///
    /// Supported API versions: 0
    pub fn with_leader_epoch(mut self, value: i32) -> Self {
        self.leader_epoch = value;
        self
    }
    /// Sets `preferred_successors` to the passed value.
    ///
    /// A sorted list of preferred successors to start the election
    ///
    /// Supported API versions: 0
    pub fn with_preferred_successors(mut self, value: Vec<i32>) -> Self {
        self.preferred_successors = value;
        self
    }
}

impl Encodable for PartitionData {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int32.encode(buf, &self.leader_id)?;
        types::Int32.encode(buf, &self.leader_epoch)?;
        types::Array(types::Int32).encode(buf, &self.preferred_successors)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int32.compute_size(&self.leader_id)?;
        total_size += types::Int32.compute_size(&self.leader_epoch)?;
        total_size += types::Array(types::Int32).compute_size(&self.preferred_successors)?;

        Ok(total_size)
    }
}

impl Decodable for PartitionData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let partition_index = types::Int32.decode(buf)?;
        let leader_id = types::Int32.decode(buf)?;
        let leader_epoch = types::Int32.decode(buf)?;
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
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
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

impl TopicData {
    /// Sets `topic_name` to the passed value.
    ///
    /// The topic name.
    ///
    /// Supported API versions: 0
    pub fn with_topic_name(mut self, value: super::TopicName) -> Self {
        self.topic_name = value;
        self
    }
    /// Sets `partitions` to the passed value.
    ///
    ///
    ///
    /// Supported API versions: 0
    pub fn with_partitions(mut self, value: Vec<PartitionData>) -> Self {
        self.partitions = value;
        self
    }
}

impl Encodable for TopicData {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::String.encode(buf, &self.topic_name)?;
        types::Array(types::Struct { version }).encode(buf, &self.partitions)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.topic_name)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.partitions)?;

        Ok(total_size)
    }
}

impl Decodable for TopicData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
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
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for EndQuorumEpochRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}
