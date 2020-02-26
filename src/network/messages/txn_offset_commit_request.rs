//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;

use franz_protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-3
#[derive(Debug, Clone)]
pub struct TxnOffsetCommitRequestPartition {
    /// The index of the partition within the topic.
    /// 
    /// Supported API versions: 0-3
    pub partition_index: i32,

    /// The message offset to be committed.
    /// 
    /// Supported API versions: 0-3
    pub committed_offset: i64,

    /// The leader epoch of the last consumed record.
    /// 
    /// Supported API versions: 2-3
    pub committed_leader_epoch: i32,

    /// Any associated metadata the client wants to keep.
    /// 
    /// Supported API versions: 0-3
    pub committed_metadata: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for TxnOffsetCommitRequestPartition {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int64.encode(buf, &self.committed_offset)?;
        if version >= 2 {
            types::Int32.encode(buf, &self.committed_leader_epoch)?;
        }
        if version == 3 {
            types::CompactString.encode(buf, &self.committed_metadata)?;
        } else {
            types::String.encode(buf, &self.committed_metadata)?;
        }
        if version == 3 {
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
        if version >= 2 {
            total_size += types::Int32.compute_size(&self.committed_leader_epoch)?;
        }
        if version == 3 {
            total_size += types::CompactString.compute_size(&self.committed_metadata)?;
        } else {
            total_size += types::String.compute_size(&self.committed_metadata)?;
        }
        if version == 3 {
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

impl Decodable for TxnOffsetCommitRequestPartition {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        let committed_offset = types::Int64.decode(buf)?;
        let committed_leader_epoch = if version >= 2 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let committed_metadata = if version == 3 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 3 {
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
            committed_metadata,
            unknown_tagged_fields,
        })
    }
}

impl Default for TxnOffsetCommitRequestPartition {
    fn default() -> Self {
        Self {
            partition_index: 0,
            committed_offset: 0,
            committed_leader_epoch: -1,
            committed_metadata: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for TxnOffsetCommitRequestPartition {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[derive(Debug, Clone)]
pub struct TxnOffsetCommitRequestTopic {
    /// The topic name.
    /// 
    /// Supported API versions: 0-3
    pub name: super::TopicName,

    /// The partitions inside the topic that we want to committ offsets for.
    /// 
    /// Supported API versions: 0-3
    pub partitions: Vec<TxnOffsetCommitRequestPartition>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for TxnOffsetCommitRequestTopic {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 3 {
            types::CompactString.encode(buf, &self.name)?;
        } else {
            types::String.encode(buf, &self.name)?;
        }
        if version == 3 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.partitions)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.partitions)?;
        }
        if version == 3 {
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
            total_size += types::CompactString.compute_size(&self.name)?;
        } else {
            total_size += types::String.compute_size(&self.name)?;
        }
        if version == 3 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.partitions)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.partitions)?;
        }
        if version == 3 {
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

impl Decodable for TxnOffsetCommitRequestTopic {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = if version == 3 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let partitions = if version == 3 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 3 {
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

impl Default for TxnOffsetCommitRequestTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for TxnOffsetCommitRequestTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[derive(Debug, Clone)]
pub struct TxnOffsetCommitRequest {
    /// The ID of the transaction.
    /// 
    /// Supported API versions: 0-3
    pub transactional_id: StrBytes,

    /// The ID of the group.
    /// 
    /// Supported API versions: 0-3
    pub group_id: super::GroupId,

    /// The current producer ID in use by the transactional ID.
    /// 
    /// Supported API versions: 0-3
    pub producer_id: super::ProducerId,

    /// The current epoch associated with the producer ID.
    /// 
    /// Supported API versions: 0-3
    pub producer_epoch: i16,

    /// The generation of the consumer.
    /// 
    /// Supported API versions: 3
    pub generation_id: i32,

    /// The member ID assigned by the group coordinator.
    /// 
    /// Supported API versions: 3
    pub member_id: StrBytes,

    /// The unique identifier of the consumer instance provided by end user.
    /// 
    /// Supported API versions: 3
    pub group_instance_id: Option<StrBytes>,

    /// Each topic that we want to commit offsets for.
    /// 
    /// Supported API versions: 0-3
    pub topics: Vec<TxnOffsetCommitRequestTopic>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for TxnOffsetCommitRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 3 {
            types::CompactString.encode(buf, &self.transactional_id)?;
        } else {
            types::String.encode(buf, &self.transactional_id)?;
        }
        if version == 3 {
            types::CompactString.encode(buf, &self.group_id)?;
        } else {
            types::String.encode(buf, &self.group_id)?;
        }
        types::Int64.encode(buf, &self.producer_id)?;
        types::Int16.encode(buf, &self.producer_epoch)?;
        if version == 3 {
            types::Int32.encode(buf, &self.generation_id)?;
        } else {
            if self.generation_id != -1 {
                return Err(EncodeError)
            }
        }
        if version == 3 {
            types::CompactString.encode(buf, &self.member_id)?;
        } else {
            if &self.member_id != "" {
                return Err(EncodeError)
            }
        }
        if version == 3 {
            types::CompactString.encode(buf, &self.group_instance_id)?;
        } else {
            if !self.group_instance_id.is_none() {
                return Err(EncodeError)
            }
        }
        if version == 3 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.topics)?;
        }
        if version == 3 {
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
            total_size += types::CompactString.compute_size(&self.transactional_id)?;
        } else {
            total_size += types::String.compute_size(&self.transactional_id)?;
        }
        if version == 3 {
            total_size += types::CompactString.compute_size(&self.group_id)?;
        } else {
            total_size += types::String.compute_size(&self.group_id)?;
        }
        total_size += types::Int64.compute_size(&self.producer_id)?;
        total_size += types::Int16.compute_size(&self.producer_epoch)?;
        if version == 3 {
            total_size += types::Int32.compute_size(&self.generation_id)?;
        } else {
            if self.generation_id != -1 {
                return Err(EncodeError)
            }
        }
        if version == 3 {
            total_size += types::CompactString.compute_size(&self.member_id)?;
        } else {
            if &self.member_id != "" {
                return Err(EncodeError)
            }
        }
        if version == 3 {
            total_size += types::CompactString.compute_size(&self.group_instance_id)?;
        } else {
            if !self.group_instance_id.is_none() {
                return Err(EncodeError)
            }
        }
        if version == 3 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
        }
        if version == 3 {
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

impl Decodable for TxnOffsetCommitRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let transactional_id = if version == 3 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let group_id = if version == 3 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let producer_id = types::Int64.decode(buf)?;
        let producer_epoch = types::Int16.decode(buf)?;
        let generation_id = if version == 3 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let member_id = if version == 3 {
            types::CompactString.decode(buf)?
        } else {
            StrBytes::from_str("")
        };
        let group_instance_id = if version == 3 {
            types::CompactString.decode(buf)?
        } else {
            None
        };
        let topics = if version == 3 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 3 {
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
            transactional_id,
            group_id,
            producer_id,
            producer_epoch,
            generation_id,
            member_id,
            group_instance_id,
            topics,
            unknown_tagged_fields,
        })
    }
}

impl Default for TxnOffsetCommitRequest {
    fn default() -> Self {
        Self {
            transactional_id: Default::default(),
            group_id: Default::default(),
            producer_id: (0).into(),
            producer_epoch: 0,
            generation_id: -1,
            member_id: StrBytes::from_str(""),
            group_instance_id: None,
            topics: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for TxnOffsetCommitRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

impl HeaderVersion for TxnOffsetCommitRequest {
    fn header_version(version: i16) -> i16 {
        if version == 3 {
            2
        } else {
            1
        }
    }
}

