//! LeaderAndIsrResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/LeaderAndIsrResponse.json).
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


/// Valid versions: 0-5
#[derive(Debug, Clone, PartialEq)]
pub struct LeaderAndIsrPartitionError {
    /// The topic name.
    /// 
    /// Supported API versions: 0-4
    pub topic_name: super::TopicName,

    /// The partition index.
    /// 
    /// Supported API versions: 0-5
    pub partition_index: i32,

    /// The partition error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-5
    pub error_code: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for LeaderAndIsrPartitionError {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version <= 4 {
            if version >= 4 {
                types::CompactString.encode(buf, &self.topic_name)?;
            } else {
                types::String.encode(buf, &self.topic_name)?;
            }
        }
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int16.encode(buf, &self.error_code)?;
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
        if version <= 4 {
            if version >= 4 {
                total_size += types::CompactString.compute_size(&self.topic_name)?;
            } else {
                total_size += types::String.compute_size(&self.topic_name)?;
            }
        }
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int16.compute_size(&self.error_code)?;
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

impl Decodable for LeaderAndIsrPartitionError {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topic_name = if version <= 4 {
            if version >= 4 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let partition_index = types::Int32.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
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
            error_code,
            unknown_tagged_fields,
        })
    }
}

impl Default for LeaderAndIsrPartitionError {
    fn default() -> Self {
        Self {
            topic_name: Default::default(),
            partition_index: 0,
            error_code: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for LeaderAndIsrPartitionError {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

/// Valid versions: 0-5
#[derive(Debug, Clone, PartialEq)]
pub struct LeaderAndIsrTopicError {
    /// Each partition.
    /// 
    /// Supported API versions: 5
    pub partition_errors: Vec<LeaderAndIsrPartitionError>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl MapEncodable for LeaderAndIsrTopicError {
    type Key = Uuid;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 5 {
            types::Uuid.encode(buf, key)?;
        } else {
            if key != &Uuid::nil() {
                return Err(EncodeError)
            }
        }
        if version >= 5 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.partition_errors)?;
        } else {
            if !self.partition_errors.is_empty() {
                return Err(EncodeError)
            }
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
        if version >= 5 {
            total_size += types::Uuid.compute_size(key)?;
        } else {
            if key != &Uuid::nil() {
                return Err(EncodeError)
            }
        }
        if version >= 5 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.partition_errors)?;
        } else {
            if !self.partition_errors.is_empty() {
                return Err(EncodeError)
            }
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

impl MapDecodable for LeaderAndIsrTopicError {
    type Key = Uuid;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = if version >= 5 {
            types::Uuid.decode(buf)?
        } else {
            Uuid::nil()
        };
        let partition_errors = if version >= 5 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
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
            partition_errors,
            unknown_tagged_fields,
        }))
    }
}

impl Default for LeaderAndIsrTopicError {
    fn default() -> Self {
        Self {
            partition_errors: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for LeaderAndIsrTopicError {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

/// Valid versions: 0-5
#[derive(Debug, Clone, PartialEq)]
pub struct LeaderAndIsrResponse {
    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-5
    pub error_code: i16,

    /// Each partition in v0 to v4 message.
    /// 
    /// Supported API versions: 0-4
    pub partition_errors: Vec<LeaderAndIsrPartitionError>,

    /// Each topic
    /// 
    /// Supported API versions: 5
    pub topics: indexmap::IndexMap<Uuid, LeaderAndIsrTopicError>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for LeaderAndIsrResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        if version <= 4 {
            if version >= 4 {
                types::CompactArray(types::Struct { version }).encode(buf, &self.partition_errors)?;
            } else {
                types::Array(types::Struct { version }).encode(buf, &self.partition_errors)?;
            }
        } else {
            if !self.partition_errors.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 5 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            if !self.topics.is_empty() {
                return Err(EncodeError)
            }
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
        total_size += types::Int16.compute_size(&self.error_code)?;
        if version <= 4 {
            if version >= 4 {
                total_size += types::CompactArray(types::Struct { version }).compute_size(&self.partition_errors)?;
            } else {
                total_size += types::Array(types::Struct { version }).compute_size(&self.partition_errors)?;
            }
        } else {
            if !self.partition_errors.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 5 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            if !self.topics.is_empty() {
                return Err(EncodeError)
            }
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

impl Decodable for LeaderAndIsrResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let partition_errors = if version <= 4 {
            if version >= 4 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Default::default()
        };
        let topics = if version >= 5 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
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
            error_code,
            partition_errors,
            topics,
            unknown_tagged_fields,
        })
    }
}

impl Default for LeaderAndIsrResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            partition_errors: Default::default(),
            topics: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for LeaderAndIsrResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

impl HeaderVersion for LeaderAndIsrResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 4 {
            1
        } else {
            0
        }
    }
}

