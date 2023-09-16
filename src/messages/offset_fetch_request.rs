//! OffsetFetchRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/OffsetFetchRequest.json).
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


/// Valid versions: 0-8
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetFetchRequestTopic {
    /// The topic name.
    /// 
    /// Supported API versions: 0-7
    pub name: super::TopicName,

    /// The partition indexes we would like to fetch offsets for.
    /// 
    /// Supported API versions: 0-7
    pub partition_indexes: Vec<i32>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for OffsetFetchRequestTopic {
    type Builder = OffsetFetchRequestTopicBuilder;

    fn builder() -> Self::Builder{
        OffsetFetchRequestTopicBuilder::default()
    }
}

impl Encodable for OffsetFetchRequestTopic {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version <= 7 {
            if version >= 6 {
                types::CompactString.encode(buf, &self.name)?;
            } else {
                types::String.encode(buf, &self.name)?;
            }
        } else {
            if !self.name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version <= 7 {
            if version >= 6 {
                types::CompactArray(types::Int32).encode(buf, &self.partition_indexes)?;
            } else {
                types::Array(types::Int32).encode(buf, &self.partition_indexes)?;
            }
        } else {
            if !self.partition_indexes.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version <= 7 {
            if version >= 6 {
                total_size += types::CompactString.compute_size(&self.name)?;
            } else {
                total_size += types::String.compute_size(&self.name)?;
            }
        } else {
            if !self.name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version <= 7 {
            if version >= 6 {
                total_size += types::CompactArray(types::Int32).compute_size(&self.partition_indexes)?;
            } else {
                total_size += types::Array(types::Int32).compute_size(&self.partition_indexes)?;
            }
        } else {
            if !self.partition_indexes.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for OffsetFetchRequestTopic {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = if version <= 7 {
            if version >= 6 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let partition_indexes = if version <= 7 {
            if version >= 6 {
                types::CompactArray(types::Int32).decode(buf)?
            } else {
                types::Array(types::Int32).decode(buf)?
            }
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            name,
            partition_indexes,
            unknown_tagged_fields,
        })
    }
}

impl Default for OffsetFetchRequestTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partition_indexes: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetFetchRequestTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
}

/// Valid versions: 0-8
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetFetchRequestTopics {
    /// The topic name.
    /// 
    /// Supported API versions: 8
    pub name: super::TopicName,

    /// The partition indexes we would like to fetch offsets for.
    /// 
    /// Supported API versions: 8
    pub partition_indexes: Vec<i32>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for OffsetFetchRequestTopics {
    type Builder = OffsetFetchRequestTopicsBuilder;

    fn builder() -> Self::Builder{
        OffsetFetchRequestTopicsBuilder::default()
    }
}

impl Encodable for OffsetFetchRequestTopics {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 8 {
            types::CompactString.encode(buf, &self.name)?;
        } else {
            if !self.name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 8 {
            types::CompactArray(types::Int32).encode(buf, &self.partition_indexes)?;
        } else {
            if !self.partition_indexes.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 8 {
            total_size += types::CompactString.compute_size(&self.name)?;
        } else {
            if !self.name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 8 {
            total_size += types::CompactArray(types::Int32).compute_size(&self.partition_indexes)?;
        } else {
            if !self.partition_indexes.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for OffsetFetchRequestTopics {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = if version >= 8 {
            types::CompactString.decode(buf)?
        } else {
            Default::default()
        };
        let partition_indexes = if version >= 8 {
            types::CompactArray(types::Int32).decode(buf)?
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            name,
            partition_indexes,
            unknown_tagged_fields,
        })
    }
}

impl Default for OffsetFetchRequestTopics {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partition_indexes: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetFetchRequestTopics {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
}

/// Valid versions: 0-8
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetFetchRequestGroup {
    /// The group ID.
    /// 
    /// Supported API versions: 8
    pub group_id: super::GroupId,

    /// Each topic we would like to fetch offsets for, or null to fetch offsets for all topics.
    /// 
    /// Supported API versions: 8
    pub topics: Option<Vec<OffsetFetchRequestTopics>>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for OffsetFetchRequestGroup {
    type Builder = OffsetFetchRequestGroupBuilder;

    fn builder() -> Self::Builder{
        OffsetFetchRequestGroupBuilder::default()
    }
}

impl Encodable for OffsetFetchRequestGroup {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 8 {
            types::CompactString.encode(buf, &self.group_id)?;
        } else {
            if !self.group_id.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 8 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            if !self.topics.as_ref().map(|x| x.is_empty()).unwrap_or_default() {
                return Err(EncodeError)
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 8 {
            total_size += types::CompactString.compute_size(&self.group_id)?;
        } else {
            if !self.group_id.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 8 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            if !self.topics.as_ref().map(|x| x.is_empty()).unwrap_or_default() {
                return Err(EncodeError)
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for OffsetFetchRequestGroup {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let group_id = if version >= 8 {
            types::CompactString.decode(buf)?
        } else {
            Default::default()
        };
        let topics = if version >= 8 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            Some(Default::default())
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            group_id,
            topics,
            unknown_tagged_fields,
        })
    }
}

impl Default for OffsetFetchRequestGroup {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            topics: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetFetchRequestGroup {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
}

/// Valid versions: 0-8
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetFetchRequest {
    /// The group to fetch offsets for.
    /// 
    /// Supported API versions: 0-7
    pub group_id: super::GroupId,

    /// Each topic we would like to fetch offsets for, or null to fetch offsets for all topics.
    /// 
    /// Supported API versions: 0-7
    pub topics: Option<Vec<OffsetFetchRequestTopic>>,

    /// Each group we would like to fetch offsets for
    /// 
    /// Supported API versions: 8
    pub groups: Vec<OffsetFetchRequestGroup>,

    /// Whether broker should hold on returning unstable offsets but set a retriable error code for the partitions.
    /// 
    /// Supported API versions: 7-8
    pub require_stable: bool,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for OffsetFetchRequest {
    type Builder = OffsetFetchRequestBuilder;

    fn builder() -> Self::Builder{
        OffsetFetchRequestBuilder::default()
    }
}

impl Encodable for OffsetFetchRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version <= 7 {
            if version >= 6 {
                types::CompactString.encode(buf, &self.group_id)?;
            } else {
                types::String.encode(buf, &self.group_id)?;
            }
        } else {
            if !self.group_id.is_empty() {
                return Err(EncodeError)
            }
        }
        if version <= 7 {
            if version >= 6 {
                types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
            } else {
                types::Array(types::Struct { version }).encode(buf, &self.topics)?;
            }
        } else {
            if !self.topics.as_ref().map(|x| x.is_empty()).unwrap_or_default() {
                return Err(EncodeError)
            }
        }
        if version >= 8 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.groups)?;
        } else {
            if !self.groups.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 7 {
            types::Boolean.encode(buf, &self.require_stable)?;
        } else {
            if self.require_stable {
                return Err(EncodeError)
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version <= 7 {
            if version >= 6 {
                total_size += types::CompactString.compute_size(&self.group_id)?;
            } else {
                total_size += types::String.compute_size(&self.group_id)?;
            }
        } else {
            if !self.group_id.is_empty() {
                return Err(EncodeError)
            }
        }
        if version <= 7 {
            if version >= 6 {
                total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
            } else {
                total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
            }
        } else {
            if !self.topics.as_ref().map(|x| x.is_empty()).unwrap_or_default() {
                return Err(EncodeError)
            }
        }
        if version >= 8 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.groups)?;
        } else {
            if !self.groups.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 7 {
            total_size += types::Boolean.compute_size(&self.require_stable)?;
        } else {
            if self.require_stable {
                return Err(EncodeError)
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for OffsetFetchRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let group_id = if version <= 7 {
            if version >= 6 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let topics = if version <= 7 {
            if version >= 6 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Some(Default::default())
        };
        let groups = if version >= 8 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let require_stable = if version >= 7 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            group_id,
            topics,
            groups,
            require_stable,
            unknown_tagged_fields,
        })
    }
}

impl Default for OffsetFetchRequest {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            topics: Some(Default::default()),
            groups: Default::default(),
            require_stable: false,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetFetchRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
}

impl HeaderVersion for OffsetFetchRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 6 {
            2
        } else {
            1
        }
    }
}

