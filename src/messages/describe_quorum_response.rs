//! DescribeQuorumResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/DescribeQuorumResponse.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0
#[derive(Debug, Clone, PartialEq)]
pub struct ReplicaState {
    /// 
    /// 
    /// Supported API versions: 0
    pub replica_id: super::BrokerId,

    /// The last known log end offset of the follower or -1 if it is unknown
    /// 
    /// Supported API versions: 0
    pub log_end_offset: i64,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for ReplicaState {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.replica_id)?;
        types::Int64.encode(buf, &self.log_end_offset)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.replica_id)?;
        total_size += types::Int64.compute_size(&self.log_end_offset)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for ReplicaState {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let replica_id = types::Int32.decode(buf)?;
        let log_end_offset = types::Int64.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let mut unknown_value = vec![0; size as usize];
            buf.try_copy_to_slice(&mut unknown_value)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            replica_id,
            log_end_offset,
            unknown_tagged_fields,
        })
    }
}

impl Default for ReplicaState {
    fn default() -> Self {
        Self {
            replica_id: (0).into(),
            log_end_offset: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ReplicaState {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[derive(Debug, Clone, PartialEq)]
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

    /// 
    /// 
    /// Supported API versions: 0
    pub high_watermark: i64,

    /// 
    /// 
    /// Supported API versions: 0
    pub current_voters: Vec<ReplicaState>,

    /// 
    /// 
    /// Supported API versions: 0
    pub observers: Vec<ReplicaState>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for PartitionData {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int16.encode(buf, &self.error_code)?;
        types::Int32.encode(buf, &self.leader_id)?;
        types::Int32.encode(buf, &self.leader_epoch)?;
        types::Int64.encode(buf, &self.high_watermark)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.current_voters)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.observers)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::Int32.compute_size(&self.leader_id)?;
        total_size += types::Int32.compute_size(&self.leader_epoch)?;
        total_size += types::Int64.compute_size(&self.high_watermark)?;
        total_size += types::CompactArray(types::Struct { version }).compute_size(&self.current_voters)?;
        total_size += types::CompactArray(types::Struct { version }).compute_size(&self.observers)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for PartitionData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
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
            let mut unknown_value = vec![0; size as usize];
            buf.try_copy_to_slice(&mut unknown_value)?;
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
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
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
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for TopicData {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::CompactString.encode(buf, &self.topic_name)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.partitions)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::CompactString.compute_size(&self.topic_name)?;
        total_size += types::CompactArray(types::Struct { version }).compute_size(&self.partitions)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for TopicData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topic_name = types::CompactString.decode(buf)?;
        let partitions = types::CompactArray(types::Struct { version }).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let mut unknown_value = vec![0; size as usize];
            buf.try_copy_to_slice(&mut unknown_value)?;
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
}

/// Valid versions: 0
#[derive(Debug, Clone, PartialEq)]
pub struct DescribeQuorumResponse {
    /// The top level error code.
    /// 
    /// Supported API versions: 0
    pub error_code: i16,

    /// 
    /// 
    /// Supported API versions: 0
    pub topics: Vec<TopicData>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for DescribeQuorumResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for DescribeQuorumResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let topics = types::CompactArray(types::Struct { version }).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let mut unknown_value = vec![0; size as usize];
            buf.try_copy_to_slice(&mut unknown_value)?;
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
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

impl HeaderVersion for DescribeQuorumResponse {
    fn header_version(version: i16) -> i16 {
        1
    }
}

