//! VoteRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/VoteRequest.json).
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
pub struct PartitionData {
    /// The partition index.
    ///
    /// Supported API versions: 0
    pub partition_index: i32,

    /// The bumped epoch of the candidate sending the request
    ///
    /// Supported API versions: 0
    pub candidate_epoch: i32,

    /// The ID of the voter sending the request
    ///
    /// Supported API versions: 0
    pub candidate_id: super::BrokerId,

    /// The epoch of the last record written to the metadata log
    ///
    /// Supported API versions: 0
    pub last_offset_epoch: i32,

    /// The offset of the last record written to the metadata log
    ///
    /// Supported API versions: 0
    pub last_offset: i64,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
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
    /// Sets `candidate_epoch` to the passed value.
    ///
    /// The bumped epoch of the candidate sending the request
    ///
    /// Supported API versions: 0
    pub fn with_candidate_epoch(mut self, value: i32) -> Self {
        self.candidate_epoch = value;
        self
    }
    /// Sets `candidate_id` to the passed value.
    ///
    /// The ID of the voter sending the request
    ///
    /// Supported API versions: 0
    pub fn with_candidate_id(mut self, value: super::BrokerId) -> Self {
        self.candidate_id = value;
        self
    }
    /// Sets `last_offset_epoch` to the passed value.
    ///
    /// The epoch of the last record written to the metadata log
    ///
    /// Supported API versions: 0
    pub fn with_last_offset_epoch(mut self, value: i32) -> Self {
        self.last_offset_epoch = value;
        self
    }
    /// Sets `last_offset` to the passed value.
    ///
    /// The offset of the last record written to the metadata log
    ///
    /// Supported API versions: 0
    pub fn with_last_offset(mut self, value: i64) -> Self {
        self.last_offset = value;
        self
    }
    /// Sets unknown_tagged_fields to the passed value.
    pub fn with_unknown_tagged_fields(mut self, value: BTreeMap<i32, Bytes>) -> Self {
        self.unknown_tagged_fields = value;
        self
    }
    /// Inserts an entry into unknown_tagged_fields.
    pub fn with_unknown_tagged_field(mut self, key: i32, value: Bytes) -> Self {
        self.unknown_tagged_fields.insert(key, value);
        self
    }
}

impl Encodable for PartitionData {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int32.encode(buf, &self.candidate_epoch)?;
        types::Int32.encode(buf, &self.candidate_id)?;
        types::Int32.encode(buf, &self.last_offset_epoch)?;
        types::Int64.encode(buf, &self.last_offset)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int32.compute_size(&self.candidate_epoch)?;
        total_size += types::Int32.compute_size(&self.candidate_id)?;
        total_size += types::Int32.compute_size(&self.last_offset_epoch)?;
        total_size += types::Int64.compute_size(&self.last_offset)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for PartitionData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let partition_index = types::Int32.decode(buf)?;
        let candidate_epoch = types::Int32.decode(buf)?;
        let candidate_id = types::Int32.decode(buf)?;
        let last_offset_epoch = types::Int32.decode(buf)?;
        let last_offset = types::Int64.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            partition_index,
            candidate_epoch,
            candidate_id,
            last_offset_epoch,
            last_offset,
            unknown_tagged_fields,
        })
    }
}

impl Default for PartitionData {
    fn default() -> Self {
        Self {
            partition_index: 0,
            candidate_epoch: 0,
            candidate_id: (0).into(),
            last_offset_epoch: 0,
            last_offset: 0,
            unknown_tagged_fields: BTreeMap::new(),
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

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
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
    /// Sets unknown_tagged_fields to the passed value.
    pub fn with_unknown_tagged_fields(mut self, value: BTreeMap<i32, Bytes>) -> Self {
        self.unknown_tagged_fields = value;
        self
    }
    /// Inserts an entry into unknown_tagged_fields.
    pub fn with_unknown_tagged_field(mut self, key: i32, value: Bytes) -> Self {
        self.unknown_tagged_fields.insert(key, value);
        self
    }
}

impl Encodable for TopicData {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::CompactString.encode(buf, &self.topic_name)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.partitions)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::CompactString.compute_size(&self.topic_name)?;
        total_size +=
            types::CompactArray(types::Struct { version }).compute_size(&self.partitions)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for TopicData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let topic_name = types::CompactString.decode(buf)?;
        let partitions = types::CompactArray(types::Struct { version }).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            topic_name,
            partitions,
            unknown_tagged_fields,
        })
    }
}

impl Default for TopicData {
    fn default() -> Self {
        Self {
            topic_name: Default::default(),
            partitions: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for TopicData {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct VoteRequest {
    ///
    ///
    /// Supported API versions: 0
    pub cluster_id: Option<StrBytes>,

    ///
    ///
    /// Supported API versions: 0
    pub topics: Vec<TopicData>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl VoteRequest {
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
    /// Sets unknown_tagged_fields to the passed value.
    pub fn with_unknown_tagged_fields(mut self, value: BTreeMap<i32, Bytes>) -> Self {
        self.unknown_tagged_fields = value;
        self
    }
    /// Inserts an entry into unknown_tagged_fields.
    pub fn with_unknown_tagged_field(mut self, key: i32, value: Bytes) -> Self {
        self.unknown_tagged_fields.insert(key, value);
        self
    }
}

impl Encodable for VoteRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::CompactString.encode(buf, &self.cluster_id)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::CompactString.compute_size(&self.cluster_id)?;
        total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for VoteRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let cluster_id = types::CompactString.decode(buf)?;
        let topics = types::CompactArray(types::Struct { version }).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            cluster_id,
            topics,
            unknown_tagged_fields,
        })
    }
}

impl Default for VoteRequest {
    fn default() -> Self {
        Self {
            cluster_id: None,
            topics: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for VoteRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for VoteRequest {
    fn header_version(version: i16) -> i16 {
        2
    }
}
