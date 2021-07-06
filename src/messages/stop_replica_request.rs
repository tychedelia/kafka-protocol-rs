//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use protocol_base::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-3
#[derive(Debug, Clone, PartialEq)]
pub struct StopReplicaPartitionV0 {
    /// The topic name.
    /// 
    /// Supported API versions: 0
    pub topic_name: super::TopicName,

    /// The partition index.
    /// 
    /// Supported API versions: 0
    pub partition_index: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for StopReplicaPartitionV0 {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 0 {
            types::String.encode(buf, &self.topic_name)?;
        } else {
            if !self.topic_name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version == 0 {
            types::Int32.encode(buf, &self.partition_index)?;
        } else {
            if self.partition_index != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
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
        if version == 0 {
            total_size += types::String.compute_size(&self.topic_name)?;
        } else {
            if !self.topic_name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version == 0 {
            total_size += types::Int32.compute_size(&self.partition_index)?;
        } else {
            if self.partition_index != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
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

impl Decodable for StopReplicaPartitionV0 {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topic_name = if version == 0 {
            types::String.decode(buf)?
        } else {
            Default::default()
        };
        let partition_index = if version == 0 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
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
            topic_name,
            partition_index,
            unknown_tagged_fields,
        })
    }
}

impl Default for StopReplicaPartitionV0 {
    fn default() -> Self {
        Self {
            topic_name: Default::default(),
            partition_index: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for StopReplicaPartitionV0 {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[derive(Debug, Clone, PartialEq)]
pub struct StopReplicaTopicV1 {
    /// The topic name.
    /// 
    /// Supported API versions: 1-2
    pub name: super::TopicName,

    /// The partition indexes.
    /// 
    /// Supported API versions: 1-2
    pub partition_indexes: Vec<i32>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for StopReplicaTopicV1 {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 && version <= 2 {
            if version == 2 {
                types::CompactString.encode(buf, &self.name)?;
            } else {
                types::String.encode(buf, &self.name)?;
            }
        } else {
            if !self.name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 1 && version <= 2 {
            if version == 2 {
                types::CompactArray(types::Int32).encode(buf, &self.partition_indexes)?;
            } else {
                types::Array(types::Int32).encode(buf, &self.partition_indexes)?;
            }
        } else {
            if !self.partition_indexes.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
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
        if version >= 1 && version <= 2 {
            if version == 2 {
                total_size += types::CompactString.compute_size(&self.name)?;
            } else {
                total_size += types::String.compute_size(&self.name)?;
            }
        } else {
            if !self.name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 1 && version <= 2 {
            if version == 2 {
                total_size += types::CompactArray(types::Int32).compute_size(&self.partition_indexes)?;
            } else {
                total_size += types::Array(types::Int32).compute_size(&self.partition_indexes)?;
            }
        } else {
            if !self.partition_indexes.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
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

impl Decodable for StopReplicaTopicV1 {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = if version >= 1 && version <= 2 {
            if version == 2 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let partition_indexes = if version >= 1 && version <= 2 {
            if version == 2 {
                types::CompactArray(types::Int32).decode(buf)?
            } else {
                types::Array(types::Int32).decode(buf)?
            }
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
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

impl Default for StopReplicaTopicV1 {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partition_indexes: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for StopReplicaTopicV1 {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[derive(Debug, Clone, PartialEq)]
pub struct StopReplicaPartitionState {
    /// The partition index.
    /// 
    /// Supported API versions: 3
    pub partition_index: i32,

    /// The leader epoch.
    /// 
    /// Supported API versions: 3
    pub leader_epoch: i32,

    /// Whether this partition should be deleted.
    /// 
    /// Supported API versions: 3
    pub delete_partition: bool,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for StopReplicaPartitionState {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 3 {
            types::Int32.encode(buf, &self.partition_index)?;
        } else {
            if self.partition_index != 0 {
                return Err(EncodeError)
            }
        }
        if version == 3 {
            types::Int32.encode(buf, &self.leader_epoch)?;
        } else {
            if self.leader_epoch != -1 {
                return Err(EncodeError)
            }
        }
        if version == 3 {
            types::Boolean.encode(buf, &self.delete_partition)?;
        } else {
            if self.delete_partition {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
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
        if version == 3 {
            total_size += types::Int32.compute_size(&self.partition_index)?;
        } else {
            if self.partition_index != 0 {
                return Err(EncodeError)
            }
        }
        if version == 3 {
            total_size += types::Int32.compute_size(&self.leader_epoch)?;
        } else {
            if self.leader_epoch != -1 {
                return Err(EncodeError)
            }
        }
        if version == 3 {
            total_size += types::Boolean.compute_size(&self.delete_partition)?;
        } else {
            if self.delete_partition {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
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

impl Decodable for StopReplicaPartitionState {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = if version == 3 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let leader_epoch = if version == 3 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let delete_partition = if version == 3 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
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
            partition_index,
            leader_epoch,
            delete_partition,
            unknown_tagged_fields,
        })
    }
}

impl Default for StopReplicaPartitionState {
    fn default() -> Self {
        Self {
            partition_index: 0,
            leader_epoch: -1,
            delete_partition: false,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for StopReplicaPartitionState {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[derive(Debug, Clone, PartialEq)]
pub struct StopReplicaTopicState {
    /// The topic name.
    /// 
    /// Supported API versions: 3
    pub topic_name: super::TopicName,

    /// The state of each partition
    /// 
    /// Supported API versions: 3
    pub partition_states: Vec<StopReplicaPartitionState>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for StopReplicaTopicState {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 3 {
            types::CompactString.encode(buf, &self.topic_name)?;
        } else {
            if !self.topic_name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version == 3 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.partition_states)?;
        } else {
            if !self.partition_states.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
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
        if version == 3 {
            total_size += types::CompactString.compute_size(&self.topic_name)?;
        } else {
            if !self.topic_name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version == 3 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.partition_states)?;
        } else {
            if !self.partition_states.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
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

impl Decodable for StopReplicaTopicState {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topic_name = if version == 3 {
            types::CompactString.decode(buf)?
        } else {
            Default::default()
        };
        let partition_states = if version == 3 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
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
            topic_name,
            partition_states,
            unknown_tagged_fields,
        })
    }
}

impl Default for StopReplicaTopicState {
    fn default() -> Self {
        Self {
            topic_name: Default::default(),
            partition_states: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for StopReplicaTopicState {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[derive(Debug, Clone, PartialEq)]
pub struct StopReplicaRequest {
    /// The controller id.
    /// 
    /// Supported API versions: 0-3
    pub controller_id: super::BrokerId,

    /// The controller epoch.
    /// 
    /// Supported API versions: 0-3
    pub controller_epoch: i32,

    /// The broker epoch.
    /// 
    /// Supported API versions: 1-3
    pub broker_epoch: i64,

    /// Whether these partitions should be deleted.
    /// 
    /// Supported API versions: 0-2
    pub delete_partitions: bool,

    /// The partitions to stop.
    /// 
    /// Supported API versions: 0
    pub ungrouped_partitions: Vec<StopReplicaPartitionV0>,

    /// The topics to stop.
    /// 
    /// Supported API versions: 1-2
    pub topics: Vec<StopReplicaTopicV1>,

    /// Each topic.
    /// 
    /// Supported API versions: 3
    pub topic_states: Vec<StopReplicaTopicState>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for StopReplicaRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.controller_id)?;
        types::Int32.encode(buf, &self.controller_epoch)?;
        if version >= 1 {
            types::Int64.encode(buf, &self.broker_epoch)?;
        }
        if version <= 2 {
            types::Boolean.encode(buf, &self.delete_partitions)?;
        } else {
            if self.delete_partitions {
                return Err(EncodeError)
            }
        }
        if version == 0 {
            types::Array(types::Struct { version }).encode(buf, &self.ungrouped_partitions)?;
        } else {
            if !self.ungrouped_partitions.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 1 && version <= 2 {
            if version == 2 {
                types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
            } else {
                types::Array(types::Struct { version }).encode(buf, &self.topics)?;
            }
        } else {
            if !self.topics.is_empty() {
                return Err(EncodeError)
            }
        }
        if version == 3 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topic_states)?;
        } else {
            if !self.topic_states.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
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
        total_size += types::Int32.compute_size(&self.controller_id)?;
        total_size += types::Int32.compute_size(&self.controller_epoch)?;
        if version >= 1 {
            total_size += types::Int64.compute_size(&self.broker_epoch)?;
        }
        if version <= 2 {
            total_size += types::Boolean.compute_size(&self.delete_partitions)?;
        } else {
            if self.delete_partitions {
                return Err(EncodeError)
            }
        }
        if version == 0 {
            total_size += types::Array(types::Struct { version }).compute_size(&self.ungrouped_partitions)?;
        } else {
            if !self.ungrouped_partitions.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 1 && version <= 2 {
            if version == 2 {
                total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
            } else {
                total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
            }
        } else {
            if !self.topics.is_empty() {
                return Err(EncodeError)
            }
        }
        if version == 3 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topic_states)?;
        } else {
            if !self.topic_states.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
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

impl Decodable for StopReplicaRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let controller_id = types::Int32.decode(buf)?;
        let controller_epoch = types::Int32.decode(buf)?;
        let broker_epoch = if version >= 1 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let delete_partitions = if version <= 2 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let ungrouped_partitions = if version == 0 {
            types::Array(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let topics = if version >= 1 && version <= 2 {
            if version == 2 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Default::default()
        };
        let topic_states = if version == 3 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
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
            controller_id,
            controller_epoch,
            broker_epoch,
            delete_partitions,
            ungrouped_partitions,
            topics,
            topic_states,
            unknown_tagged_fields,
        })
    }
}

impl Default for StopReplicaRequest {
    fn default() -> Self {
        Self {
            controller_id: (0).into(),
            controller_epoch: 0,
            broker_epoch: -1,
            delete_partitions: false,
            ungrouped_partitions: Default::default(),
            topics: Default::default(),
            topic_states: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for StopReplicaRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

impl HeaderVersion for StopReplicaRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 2 {
            2
        } else {
            1
        }
    }
}

