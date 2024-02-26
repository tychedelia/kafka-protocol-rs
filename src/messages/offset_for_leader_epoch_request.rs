//! OffsetForLeaderEpochRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/OffsetForLeaderEpochRequest.json).
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


/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetForLeaderPartition {
    /// The partition index.
    /// 
    /// Supported API versions: 0-4
    pub partition: i32,

    /// An epoch used to fence consumers/replicas with old metadata. If the epoch provided by the client is larger than the current epoch known to the broker, then the UNKNOWN_LEADER_EPOCH error code will be returned. If the provided epoch is smaller, then the FENCED_LEADER_EPOCH error code will be returned.
    /// 
    /// Supported API versions: 2-4
    pub current_leader_epoch: i32,

    /// The epoch to look up an offset for.
    /// 
    /// Supported API versions: 0-4
    pub leader_epoch: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for OffsetForLeaderPartition {
    type Builder = OffsetForLeaderPartitionBuilder;

    fn builder() -> Self::Builder{
        OffsetForLeaderPartitionBuilder::default()
    }
}

impl Encodable for OffsetForLeaderPartition {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition)?;
        if version >= 2 {
            types::Int32.encode(buf, &self.current_leader_epoch)?;
        }
        types::Int32.encode(buf, &self.leader_epoch)?;
        if version >= 4 {
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
        total_size += types::Int32.compute_size(&self.partition)?;
        if version >= 2 {
            total_size += types::Int32.compute_size(&self.current_leader_epoch)?;
        }
        total_size += types::Int32.compute_size(&self.leader_epoch)?;
        if version >= 4 {
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

impl Decodable for OffsetForLeaderPartition {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition = types::Int32.decode(buf)?;
        let current_leader_epoch = if version >= 2 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let leader_epoch = types::Int32.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            partition,
            current_leader_epoch,
            leader_epoch,
            unknown_tagged_fields,
        })
    }
}

impl Default for OffsetForLeaderPartition {
    fn default() -> Self {
        Self {
            partition: 0,
            current_leader_epoch: -1,
            leader_epoch: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetForLeaderPartition {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetForLeaderTopic {
    /// Each partition to get offsets for.
    /// 
    /// Supported API versions: 0-4
    pub partitions: Vec<OffsetForLeaderPartition>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for OffsetForLeaderTopic {
    type Builder = OffsetForLeaderTopicBuilder;

    fn builder() -> Self::Builder{
        OffsetForLeaderTopicBuilder::default()
    }
}

impl MapEncodable for OffsetForLeaderTopic {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 4 {
            types::CompactString.encode(buf, key)?;
        } else {
            types::String.encode(buf, key)?;
        }
        if version >= 4 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.partitions)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.partitions)?;
        }
        if version >= 4 {
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
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 4 {
            total_size += types::CompactString.compute_size(key)?;
        } else {
            total_size += types::String.compute_size(key)?;
        }
        if version >= 4 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.partitions)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.partitions)?;
        }
        if version >= 4 {
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

impl MapDecodable for OffsetForLeaderTopic {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = if version >= 4 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let partitions = if version >= 4 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok((key_field, Self {
            partitions,
            unknown_tagged_fields,
        }))
    }
}

impl Default for OffsetForLeaderTopic {
    fn default() -> Self {
        Self {
            partitions: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetForLeaderTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetForLeaderEpochRequest {
    /// The broker ID of the follower, of -1 if this request is from a consumer.
    /// 
    /// Supported API versions: 3-4
    pub replica_id: super::BrokerId,

    /// Each topic to get offsets for.
    /// 
    /// Supported API versions: 0-4
    pub topics: indexmap::IndexMap<super::TopicName, OffsetForLeaderTopic>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for OffsetForLeaderEpochRequest {
    type Builder = OffsetForLeaderEpochRequestBuilder;

    fn builder() -> Self::Builder{
        OffsetForLeaderEpochRequestBuilder::default()
    }
}

impl Encodable for OffsetForLeaderEpochRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 3 {
            types::Int32.encode(buf, &self.replica_id)?;
        }
        if version >= 4 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.topics)?;
        }
        if version >= 4 {
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
        if version >= 3 {
            total_size += types::Int32.compute_size(&self.replica_id)?;
        }
        if version >= 4 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
        }
        if version >= 4 {
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

impl Decodable for OffsetForLeaderEpochRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let replica_id = if version >= 3 {
            types::Int32.decode(buf)?
        } else {
            (-2).into()
        };
        let topics = if version >= 4 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            replica_id,
            topics,
            unknown_tagged_fields,
        })
    }
}

impl Default for OffsetForLeaderEpochRequest {
    fn default() -> Self {
        Self {
            replica_id: (-2).into(),
            topics: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetForLeaderEpochRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

impl HeaderVersion for OffsetForLeaderEpochRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 4 {
            2
        } else {
            1
        }
    }
}

