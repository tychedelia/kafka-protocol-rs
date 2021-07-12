//! CreatePartitionsRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/CreatePartitionsRequest.json).
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


/// Valid versions: 0-3
#[derive(Debug, Clone, PartialEq)]
pub struct CreatePartitionsAssignment {
    /// The assigned broker IDs.
    /// 
    /// Supported API versions: 0-3
    pub broker_ids: Vec<super::BrokerId>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for CreatePartitionsAssignment {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 2 {
            types::CompactArray(types::Int32).encode(buf, &self.broker_ids)?;
        } else {
            types::Array(types::Int32).encode(buf, &self.broker_ids)?;
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
        if version >= 2 {
            total_size += types::CompactArray(types::Int32).compute_size(&self.broker_ids)?;
        } else {
            total_size += types::Array(types::Int32).compute_size(&self.broker_ids)?;
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

impl Decodable for CreatePartitionsAssignment {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let broker_ids = if version >= 2 {
            types::CompactArray(types::Int32).decode(buf)?
        } else {
            types::Array(types::Int32).decode(buf)?
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
            broker_ids,
            unknown_tagged_fields,
        })
    }
}

impl Default for CreatePartitionsAssignment {
    fn default() -> Self {
        Self {
            broker_ids: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreatePartitionsAssignment {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[derive(Debug, Clone, PartialEq)]
pub struct CreatePartitionsTopic {
    /// The new partition count.
    /// 
    /// Supported API versions: 0-3
    pub count: i32,

    /// The new partition assignments.
    /// 
    /// Supported API versions: 0-3
    pub assignments: Option<Vec<CreatePartitionsAssignment>>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl MapEncodable for CreatePartitionsTopic {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 2 {
            types::CompactString.encode(buf, key)?;
        } else {
            types::String.encode(buf, key)?;
        }
        types::Int32.encode(buf, &self.count)?;
        if version >= 2 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.assignments)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.assignments)?;
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
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 2 {
            total_size += types::CompactString.compute_size(key)?;
        } else {
            total_size += types::String.compute_size(key)?;
        }
        total_size += types::Int32.compute_size(&self.count)?;
        if version >= 2 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.assignments)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.assignments)?;
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

impl MapDecodable for CreatePartitionsTopic {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let count = types::Int32.decode(buf)?;
        let assignments = if version >= 2 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
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
        Ok((key_field, Self {
            count,
            assignments,
            unknown_tagged_fields,
        }))
    }
}

impl Default for CreatePartitionsTopic {
    fn default() -> Self {
        Self {
            count: 0,
            assignments: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreatePartitionsTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[derive(Debug, Clone, PartialEq)]
pub struct CreatePartitionsRequest {
    /// Each topic that we want to create new partitions inside.
    /// 
    /// Supported API versions: 0-3
    pub topics: indexmap::IndexMap<super::TopicName, CreatePartitionsTopic>,

    /// The time in ms to wait for the partitions to be created.
    /// 
    /// Supported API versions: 0-3
    pub timeout_ms: i32,

    /// If true, then validate the request, but don't actually increase the number of partitions.
    /// 
    /// Supported API versions: 0-3
    pub validate_only: bool,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for CreatePartitionsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 2 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.topics)?;
        }
        types::Int32.encode(buf, &self.timeout_ms)?;
        types::Boolean.encode(buf, &self.validate_only)?;
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
        if version >= 2 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
        }
        total_size += types::Int32.compute_size(&self.timeout_ms)?;
        total_size += types::Boolean.compute_size(&self.validate_only)?;
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

impl Decodable for CreatePartitionsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topics = if version >= 2 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let timeout_ms = types::Int32.decode(buf)?;
        let validate_only = types::Boolean.decode(buf)?;
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
            topics,
            timeout_ms,
            validate_only,
            unknown_tagged_fields,
        })
    }
}

impl Default for CreatePartitionsRequest {
    fn default() -> Self {
        Self {
            topics: Default::default(),
            timeout_ms: 0,
            validate_only: false,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreatePartitionsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

impl HeaderVersion for CreatePartitionsRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 2 {
            2
        } else {
            1
        }
    }
}

