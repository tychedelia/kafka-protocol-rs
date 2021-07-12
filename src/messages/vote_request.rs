//! VoteRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/VoteRequest.json).
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
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for PartitionData {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int32.encode(buf, &self.candidate_epoch)?;
        types::Int32.encode(buf, &self.candidate_id)?;
        types::Int32.encode(buf, &self.last_offset_epoch)?;
        types::Int64.encode(buf, &self.last_offset)?;
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
        total_size += types::Int32.compute_size(&self.candidate_epoch)?;
        total_size += types::Int32.compute_size(&self.candidate_id)?;
        total_size += types::Int32.compute_size(&self.last_offset_epoch)?;
        total_size += types::Int64.compute_size(&self.last_offset)?;
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
        let candidate_epoch = types::Int32.decode(buf)?;
        let candidate_id = types::Int32.decode(buf)?;
        let last_offset_epoch = types::Int32.decode(buf)?;
        let last_offset = types::Int64.decode(buf)?;
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
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for VoteRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::CompactString.encode(buf, &self.cluster_id)?;
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
        total_size += types::CompactString.compute_size(&self.cluster_id)?;
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

impl Decodable for VoteRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let cluster_id = types::CompactString.decode(buf)?;
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
}

impl HeaderVersion for VoteRequest {
    fn header_version(version: i16) -> i16 {
        2
    }
}

