//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;

use protocol_base::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-2
#[derive(Debug, Clone)]
pub struct TopicPartitions {
    /// The name of a topic.
    /// 
    /// Supported API versions: 0-2
    pub topic: super::TopicName,

    /// The partitions of this topic whose leader should be elected.
    /// 
    /// Supported API versions: 0-2
    pub partition_id: Vec<i32>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for TopicPartitions {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 2 {
            types::CompactString.encode(buf, &self.topic)?;
        } else {
            types::String.encode(buf, &self.topic)?;
        }
        if version == 2 {
            types::CompactArray(types::Int32).encode(buf, &self.partition_id)?;
        } else {
            types::Array(types::Int32).encode(buf, &self.partition_id)?;
        }
        if version == 2 {
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
        if version == 2 {
            total_size += types::CompactString.compute_size(&self.topic)?;
        } else {
            total_size += types::String.compute_size(&self.topic)?;
        }
        if version == 2 {
            total_size += types::CompactArray(types::Int32).compute_size(&self.partition_id)?;
        } else {
            total_size += types::Array(types::Int32).compute_size(&self.partition_id)?;
        }
        if version == 2 {
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

impl Decodable for TopicPartitions {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topic = if version == 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let partition_id = if version == 2 {
            types::CompactArray(types::Int32).decode(buf)?
        } else {
            types::Array(types::Int32).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 2 {
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
            topic,
            partition_id,
            unknown_tagged_fields,
        })
    }
}

impl Default for TopicPartitions {
    fn default() -> Self {
        Self {
            topic: Default::default(),
            partition_id: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for TopicPartitions {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

/// Valid versions: 0-2
#[derive(Debug, Clone)]
pub struct ElectLeadersRequest {
    /// Type of elections to conduct for the partition. A value of '0' elects the preferred replica. A value of '1' elects the first live replica if there are no in-sync replica.
    /// 
    /// Supported API versions: 1-2
    pub election_type: i8,

    /// The topic partitions to elect leaders.
    /// 
    /// Supported API versions: 0-2
    pub topic_partitions: Option<Vec<TopicPartitions>>,

    /// The time in ms to wait for the election to complete.
    /// 
    /// Supported API versions: 0-2
    pub timeout_ms: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for ElectLeadersRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            types::Int8.encode(buf, &self.election_type)?;
        } else {
            if self.election_type != 0 {
                return Err(EncodeError)
            }
        }
        if version == 2 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topic_partitions)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.topic_partitions)?;
        }
        types::Int32.encode(buf, &self.timeout_ms)?;
        if version == 2 {
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
        if version >= 1 {
            total_size += types::Int8.compute_size(&self.election_type)?;
        } else {
            if self.election_type != 0 {
                return Err(EncodeError)
            }
        }
        if version == 2 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topic_partitions)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.topic_partitions)?;
        }
        total_size += types::Int32.compute_size(&self.timeout_ms)?;
        if version == 2 {
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

impl Decodable for ElectLeadersRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let election_type = if version >= 1 {
            types::Int8.decode(buf)?
        } else {
            0
        };
        let topic_partitions = if version == 2 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let timeout_ms = types::Int32.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 2 {
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
            election_type,
            topic_partitions,
            timeout_ms,
            unknown_tagged_fields,
        })
    }
}

impl Default for ElectLeadersRequest {
    fn default() -> Self {
        Self {
            election_type: 0,
            topic_partitions: Some(Default::default()),
            timeout_ms: 60000,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ElectLeadersRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

impl HeaderVersion for ElectLeadersRequest {
    fn header_version(version: i16) -> i16 {
        if version == 2 {
            2
        } else {
            1
        }
    }
}

