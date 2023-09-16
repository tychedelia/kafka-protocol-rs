//! OffsetCommitRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/OffsetCommitRequest.json).
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
pub struct OffsetCommitRequestPartition {
    /// The partition index.
    /// 
    /// Supported API versions: 0-8
    pub partition_index: i32,

    /// The message offset to be committed.
    /// 
    /// Supported API versions: 0-8
    pub committed_offset: i64,

    /// The leader epoch of this partition.
    /// 
    /// Supported API versions: 6-8
    pub committed_leader_epoch: i32,

    /// The timestamp of the commit.
    /// 
    /// Supported API versions: 1
    pub commit_timestamp: i64,

    /// Any associated metadata the client wants to keep.
    /// 
    /// Supported API versions: 0-8
    pub committed_metadata: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for OffsetCommitRequestPartition {
    type Builder = OffsetCommitRequestPartitionBuilder;

    fn builder() -> Self::Builder{
        OffsetCommitRequestPartitionBuilder::default()
    }
}

impl Encodable for OffsetCommitRequestPartition {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int64.encode(buf, &self.committed_offset)?;
        if version >= 6 {
            types::Int32.encode(buf, &self.committed_leader_epoch)?;
        }
        if version == 1 {
            types::Int64.encode(buf, &self.commit_timestamp)?;
        } else {
            if self.commit_timestamp != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 8 {
            types::CompactString.encode(buf, &self.committed_metadata)?;
        } else {
            types::String.encode(buf, &self.committed_metadata)?;
        }
        if version >= 8 {
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
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int64.compute_size(&self.committed_offset)?;
        if version >= 6 {
            total_size += types::Int32.compute_size(&self.committed_leader_epoch)?;
        }
        if version == 1 {
            total_size += types::Int64.compute_size(&self.commit_timestamp)?;
        } else {
            if self.commit_timestamp != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 8 {
            total_size += types::CompactString.compute_size(&self.committed_metadata)?;
        } else {
            total_size += types::String.compute_size(&self.committed_metadata)?;
        }
        if version >= 8 {
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

impl Decodable for OffsetCommitRequestPartition {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        let committed_offset = types::Int64.decode(buf)?;
        let committed_leader_epoch = if version >= 6 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let commit_timestamp = if version == 1 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let committed_metadata = if version >= 8 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 8 {
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
            committed_offset,
            committed_leader_epoch,
            commit_timestamp,
            committed_metadata,
            unknown_tagged_fields,
        })
    }
}

impl Default for OffsetCommitRequestPartition {
    fn default() -> Self {
        Self {
            partition_index: 0,
            committed_offset: 0,
            committed_leader_epoch: -1,
            commit_timestamp: -1,
            committed_metadata: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetCommitRequestPartition {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
}

/// Valid versions: 0-8
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetCommitRequestTopic {
    /// The topic name.
    /// 
    /// Supported API versions: 0-8
    pub name: super::TopicName,

    /// Each partition to commit offsets for.
    /// 
    /// Supported API versions: 0-8
    pub partitions: Vec<OffsetCommitRequestPartition>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for OffsetCommitRequestTopic {
    type Builder = OffsetCommitRequestTopicBuilder;

    fn builder() -> Self::Builder{
        OffsetCommitRequestTopicBuilder::default()
    }
}

impl Encodable for OffsetCommitRequestTopic {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 8 {
            types::CompactString.encode(buf, &self.name)?;
        } else {
            types::String.encode(buf, &self.name)?;
        }
        if version >= 8 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.partitions)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.partitions)?;
        }
        if version >= 8 {
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
            total_size += types::String.compute_size(&self.name)?;
        }
        if version >= 8 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.partitions)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.partitions)?;
        }
        if version >= 8 {
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

impl Decodable for OffsetCommitRequestTopic {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = if version >= 8 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let partitions = if version >= 8 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 8 {
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
            partitions,
            unknown_tagged_fields,
        })
    }
}

impl Default for OffsetCommitRequestTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetCommitRequestTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
}

/// Valid versions: 0-8
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetCommitRequest {
    /// The unique group identifier.
    /// 
    /// Supported API versions: 0-8
    pub group_id: super::GroupId,

    /// The generation of the group.
    /// 
    /// Supported API versions: 1-8
    pub generation_id: i32,

    /// The member ID assigned by the group coordinator.
    /// 
    /// Supported API versions: 1-8
    pub member_id: StrBytes,

    /// The unique identifier of the consumer instance provided by end user.
    /// 
    /// Supported API versions: 7-8
    pub group_instance_id: Option<StrBytes>,

    /// The time period in ms to retain the offset.
    /// 
    /// Supported API versions: 2-4
    pub retention_time_ms: i64,

    /// The topics to commit offsets for.
    /// 
    /// Supported API versions: 0-8
    pub topics: Vec<OffsetCommitRequestTopic>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for OffsetCommitRequest {
    type Builder = OffsetCommitRequestBuilder;

    fn builder() -> Self::Builder{
        OffsetCommitRequestBuilder::default()
    }
}

impl Encodable for OffsetCommitRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 8 {
            types::CompactString.encode(buf, &self.group_id)?;
        } else {
            types::String.encode(buf, &self.group_id)?;
        }
        if version >= 1 {
            types::Int32.encode(buf, &self.generation_id)?;
        }
        if version >= 1 {
            if version >= 8 {
                types::CompactString.encode(buf, &self.member_id)?;
            } else {
                types::String.encode(buf, &self.member_id)?;
            }
        }
        if version >= 7 {
            if version >= 8 {
                types::CompactString.encode(buf, &self.group_instance_id)?;
            } else {
                types::String.encode(buf, &self.group_instance_id)?;
            }
        } else {
            if !self.group_instance_id.is_none() {
                return Err(EncodeError)
            }
        }
        if version >= 2 && version <= 4 {
            types::Int64.encode(buf, &self.retention_time_ms)?;
        }
        if version >= 8 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.topics)?;
        }
        if version >= 8 {
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
            total_size += types::String.compute_size(&self.group_id)?;
        }
        if version >= 1 {
            total_size += types::Int32.compute_size(&self.generation_id)?;
        }
        if version >= 1 {
            if version >= 8 {
                total_size += types::CompactString.compute_size(&self.member_id)?;
            } else {
                total_size += types::String.compute_size(&self.member_id)?;
            }
        }
        if version >= 7 {
            if version >= 8 {
                total_size += types::CompactString.compute_size(&self.group_instance_id)?;
            } else {
                total_size += types::String.compute_size(&self.group_instance_id)?;
            }
        } else {
            if !self.group_instance_id.is_none() {
                return Err(EncodeError)
            }
        }
        if version >= 2 && version <= 4 {
            total_size += types::Int64.compute_size(&self.retention_time_ms)?;
        }
        if version >= 8 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
        }
        if version >= 8 {
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

impl Decodable for OffsetCommitRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let group_id = if version >= 8 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let generation_id = if version >= 1 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let member_id = if version >= 1 {
            if version >= 8 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let group_instance_id = if version >= 7 {
            if version >= 8 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            None
        };
        let retention_time_ms = if version >= 2 && version <= 4 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let topics = if version >= 8 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 8 {
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
            generation_id,
            member_id,
            group_instance_id,
            retention_time_ms,
            topics,
            unknown_tagged_fields,
        })
    }
}

impl Default for OffsetCommitRequest {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            generation_id: -1,
            member_id: Default::default(),
            group_instance_id: None,
            retention_time_ms: -1,
            topics: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetCommitRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
}

impl HeaderVersion for OffsetCommitRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 8 {
            2
        } else {
            1
        }
    }
}

