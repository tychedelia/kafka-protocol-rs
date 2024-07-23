//! DescribeQuorumResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/DescribeQuorumResponse.json).
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

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct DescribeQuorumResponse {
    /// The top level error code.
    ///
    /// Supported API versions: 0-1
    pub error_code: i16,

    ///
    ///
    /// Supported API versions: 0-1
    pub topics: Vec<TopicData>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl DescribeQuorumResponse {
    /// Sets `error_code` to the passed value.
    ///
    /// The top level error code.
    ///
    /// Supported API versions: 0-1
    pub fn with_error_code(mut self, value: i16) -> Self {
        self.error_code = value;
        self
    }
    /// Sets `topics` to the passed value.
    ///
    ///
    ///
    /// Supported API versions: 0-1
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

impl Encodable for DescribeQuorumResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::Int16.encode(buf, &self.error_code)?;
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
        total_size += types::Int16.compute_size(&self.error_code)?;
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

impl Decodable for DescribeQuorumResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let error_code = types::Int16.decode(buf)?;
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
            error_code,
            topics,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribeQuorumResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            topics: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribeQuorumResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct PartitionData {
    /// The partition index.
    ///
    /// Supported API versions: 0-1
    pub partition_index: i32,

    ///
    ///
    /// Supported API versions: 0-1
    pub error_code: i16,

    /// The ID of the current leader or -1 if the leader is unknown.
    ///
    /// Supported API versions: 0-1
    pub leader_id: super::BrokerId,

    /// The latest known leader epoch
    ///
    /// Supported API versions: 0-1
    pub leader_epoch: i32,

    ///
    ///
    /// Supported API versions: 0-1
    pub high_watermark: i64,

    ///
    ///
    /// Supported API versions: 0-1
    pub current_voters: Vec<ReplicaState>,

    ///
    ///
    /// Supported API versions: 0-1
    pub observers: Vec<ReplicaState>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl PartitionData {
    /// Sets `partition_index` to the passed value.
    ///
    /// The partition index.
    ///
    /// Supported API versions: 0-1
    pub fn with_partition_index(mut self, value: i32) -> Self {
        self.partition_index = value;
        self
    }
    /// Sets `error_code` to the passed value.
    ///
    ///
    ///
    /// Supported API versions: 0-1
    pub fn with_error_code(mut self, value: i16) -> Self {
        self.error_code = value;
        self
    }
    /// Sets `leader_id` to the passed value.
    ///
    /// The ID of the current leader or -1 if the leader is unknown.
    ///
    /// Supported API versions: 0-1
    pub fn with_leader_id(mut self, value: super::BrokerId) -> Self {
        self.leader_id = value;
        self
    }
    /// Sets `leader_epoch` to the passed value.
    ///
    /// The latest known leader epoch
    ///
    /// Supported API versions: 0-1
    pub fn with_leader_epoch(mut self, value: i32) -> Self {
        self.leader_epoch = value;
        self
    }
    /// Sets `high_watermark` to the passed value.
    ///
    ///
    ///
    /// Supported API versions: 0-1
    pub fn with_high_watermark(mut self, value: i64) -> Self {
        self.high_watermark = value;
        self
    }
    /// Sets `current_voters` to the passed value.
    ///
    ///
    ///
    /// Supported API versions: 0-1
    pub fn with_current_voters(mut self, value: Vec<ReplicaState>) -> Self {
        self.current_voters = value;
        self
    }
    /// Sets `observers` to the passed value.
    ///
    ///
    ///
    /// Supported API versions: 0-1
    pub fn with_observers(mut self, value: Vec<ReplicaState>) -> Self {
        self.observers = value;
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
        types::Int16.encode(buf, &self.error_code)?;
        types::Int32.encode(buf, &self.leader_id)?;
        types::Int32.encode(buf, &self.leader_epoch)?;
        types::Int64.encode(buf, &self.high_watermark)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.current_voters)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.observers)?;
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
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::Int32.compute_size(&self.leader_id)?;
        total_size += types::Int32.compute_size(&self.leader_epoch)?;
        total_size += types::Int64.compute_size(&self.high_watermark)?;
        total_size +=
            types::CompactArray(types::Struct { version }).compute_size(&self.current_voters)?;
        total_size +=
            types::CompactArray(types::Struct { version }).compute_size(&self.observers)?;
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
        let error_code = types::Int16.decode(buf)?;
        let leader_id = types::Int32.decode(buf)?;
        let leader_epoch = types::Int32.decode(buf)?;
        let high_watermark = types::Int64.decode(buf)?;
        let current_voters = types::CompactArray(types::Struct { version }).decode(buf)?;
        let observers = types::CompactArray(types::Struct { version }).decode(buf)?;
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
            error_code,
            leader_id,
            leader_epoch,
            high_watermark,
            current_voters,
            observers,
            unknown_tagged_fields,
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
            high_watermark: 0,
            current_voters: Default::default(),
            observers: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for PartitionData {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct ReplicaState {
    ///
    ///
    /// Supported API versions: 0-1
    pub replica_id: super::BrokerId,

    /// The last known log end offset of the follower or -1 if it is unknown
    ///
    /// Supported API versions: 0-1
    pub log_end_offset: i64,

    /// The last known leader wall clock time time when a follower fetched from the leader. This is reported as -1 both for the current leader or if it is unknown for a voter
    ///
    /// Supported API versions: 1
    pub last_fetch_timestamp: i64,

    /// The leader wall clock append time of the offset for which the follower made the most recent fetch request. This is reported as the current time for the leader and -1 if unknown for a voter
    ///
    /// Supported API versions: 1
    pub last_caught_up_timestamp: i64,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl ReplicaState {
    /// Sets `replica_id` to the passed value.
    ///
    ///
    ///
    /// Supported API versions: 0-1
    pub fn with_replica_id(mut self, value: super::BrokerId) -> Self {
        self.replica_id = value;
        self
    }
    /// Sets `log_end_offset` to the passed value.
    ///
    /// The last known log end offset of the follower or -1 if it is unknown
    ///
    /// Supported API versions: 0-1
    pub fn with_log_end_offset(mut self, value: i64) -> Self {
        self.log_end_offset = value;
        self
    }
    /// Sets `last_fetch_timestamp` to the passed value.
    ///
    /// The last known leader wall clock time time when a follower fetched from the leader. This is reported as -1 both for the current leader or if it is unknown for a voter
    ///
    /// Supported API versions: 1
    pub fn with_last_fetch_timestamp(mut self, value: i64) -> Self {
        self.last_fetch_timestamp = value;
        self
    }
    /// Sets `last_caught_up_timestamp` to the passed value.
    ///
    /// The leader wall clock append time of the offset for which the follower made the most recent fetch request. This is reported as the current time for the leader and -1 if unknown for a voter
    ///
    /// Supported API versions: 1
    pub fn with_last_caught_up_timestamp(mut self, value: i64) -> Self {
        self.last_caught_up_timestamp = value;
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

impl Encodable for ReplicaState {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::Int32.encode(buf, &self.replica_id)?;
        types::Int64.encode(buf, &self.log_end_offset)?;
        if version >= 1 {
            types::Int64.encode(buf, &self.last_fetch_timestamp)?;
        }
        if version >= 1 {
            types::Int64.encode(buf, &self.last_caught_up_timestamp)?;
        }
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
        total_size += types::Int32.compute_size(&self.replica_id)?;
        total_size += types::Int64.compute_size(&self.log_end_offset)?;
        if version >= 1 {
            total_size += types::Int64.compute_size(&self.last_fetch_timestamp)?;
        }
        if version >= 1 {
            total_size += types::Int64.compute_size(&self.last_caught_up_timestamp)?;
        }
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

impl Decodable for ReplicaState {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let replica_id = types::Int32.decode(buf)?;
        let log_end_offset = types::Int64.decode(buf)?;
        let last_fetch_timestamp = if version >= 1 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let last_caught_up_timestamp = if version >= 1 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            replica_id,
            log_end_offset,
            last_fetch_timestamp,
            last_caught_up_timestamp,
            unknown_tagged_fields,
        })
    }
}

impl Default for ReplicaState {
    fn default() -> Self {
        Self {
            replica_id: (0).into(),
            log_end_offset: 0,
            last_fetch_timestamp: -1,
            last_caught_up_timestamp: -1,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ReplicaState {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct TopicData {
    /// The topic name.
    ///
    /// Supported API versions: 0-1
    pub topic_name: super::TopicName,

    ///
    ///
    /// Supported API versions: 0-1
    pub partitions: Vec<PartitionData>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl TopicData {
    /// Sets `topic_name` to the passed value.
    ///
    /// The topic name.
    ///
    /// Supported API versions: 0-1
    pub fn with_topic_name(mut self, value: super::TopicName) -> Self {
        self.topic_name = value;
        self
    }
    /// Sets `partitions` to the passed value.
    ///
    ///
    ///
    /// Supported API versions: 0-1
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
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for DescribeQuorumResponse {
    fn header_version(version: i16) -> i16 {
        1
    }
}
