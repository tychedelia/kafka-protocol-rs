//! DescribeQuorumRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/DescribeQuorumRequest.json).
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


/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct PartitionData {
    /// The partition index.
    /// 
    /// Supported API versions: 0-1
    pub partition_index: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
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
            unknown_tagged_fields,
        })
    }
}

impl Default for PartitionData {
    fn default() -> Self {
        Self {
            partition_index: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for PartitionData {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
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
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for TopicData {
    type Builder = TopicDataBuilder;

    fn builder() -> Self::Builder{
        TopicDataBuilder::default()
    }
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
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct DescribeQuorumRequest {
    /// 
    /// 
    /// Supported API versions: 0-1
    pub topics: Vec<TopicData>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for DescribeQuorumRequest {
    type Builder = DescribeQuorumRequestBuilder;

    fn builder() -> Self::Builder{
        DescribeQuorumRequestBuilder::default()
    }
}

impl Encodable for DescribeQuorumRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
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

impl Decodable for DescribeQuorumRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
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
            topics,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribeQuorumRequest {
    fn default() -> Self {
        Self {
            topics: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribeQuorumRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for DescribeQuorumRequest {
    fn header_version(version: i16) -> i16 {
        2
    }
}

