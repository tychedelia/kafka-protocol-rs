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


/// Valid versions: 0-7
#[derive(Debug, Clone, PartialEq)]
pub struct CreatableReplicaAssignment {
    /// The brokers to place the partition on.
    /// 
    /// Supported API versions: 0-7
    pub broker_ids: Vec<super::BrokerId>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl MapEncodable for CreatableReplicaAssignment {
    type Key = i32;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, key)?;
        if version >= 5 {
            types::CompactArray(types::Int32).encode(buf, &self.broker_ids)?;
        } else {
            types::Array(types::Int32).encode(buf, &self.broker_ids)?;
        }
        if version >= 5 {
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
        total_size += types::Int32.compute_size(key)?;
        if version >= 5 {
            total_size += types::CompactArray(types::Int32).compute_size(&self.broker_ids)?;
        } else {
            total_size += types::Array(types::Int32).compute_size(&self.broker_ids)?;
        }
        if version >= 5 {
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

impl MapDecodable for CreatableReplicaAssignment {
    type Key = i32;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::Int32.decode(buf)?;
        let broker_ids = if version >= 5 {
            types::CompactArray(types::Int32).decode(buf)?
        } else {
            types::Array(types::Int32).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 5 {
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
            broker_ids,
            unknown_tagged_fields,
        }))
    }
}

impl Default for CreatableReplicaAssignment {
    fn default() -> Self {
        Self {
            broker_ids: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreatableReplicaAssignment {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
}

/// Valid versions: 0-7
#[derive(Debug, Clone, PartialEq)]
pub struct CreateableTopicConfig {
    /// The configuration value.
    /// 
    /// Supported API versions: 0-7
    pub value: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl MapEncodable for CreateableTopicConfig {
    type Key = StrBytes;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 5 {
            types::CompactString.encode(buf, key)?;
        } else {
            types::String.encode(buf, key)?;
        }
        if version >= 5 {
            types::CompactString.encode(buf, &self.value)?;
        } else {
            types::String.encode(buf, &self.value)?;
        }
        if version >= 5 {
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
            total_size += types::CompactString.compute_size(key)?;
        } else {
            total_size += types::String.compute_size(key)?;
        }
        if version >= 5 {
            total_size += types::CompactString.compute_size(&self.value)?;
        } else {
            total_size += types::String.compute_size(&self.value)?;
        }
        if version >= 5 {
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

impl MapDecodable for CreateableTopicConfig {
    type Key = StrBytes;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = if version >= 5 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let value = if version >= 5 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 5 {
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
            value,
            unknown_tagged_fields,
        }))
    }
}

impl Default for CreateableTopicConfig {
    fn default() -> Self {
        Self {
            value: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreateableTopicConfig {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
}

/// Valid versions: 0-7
#[derive(Debug, Clone, PartialEq)]
pub struct CreatableTopic {
    /// The number of partitions to create in the topic, or -1 if we are either specifying a manual partition assignment or using the default partitions.
    /// 
    /// Supported API versions: 0-7
    pub num_partitions: i32,

    /// The number of replicas to create for each partition in the topic, or -1 if we are either specifying a manual partition assignment or using the default replication factor.
    /// 
    /// Supported API versions: 0-7
    pub replication_factor: i16,

    /// The manual partition assignment, or the empty array if we are using automatic assignment.
    /// 
    /// Supported API versions: 0-7
    pub assignments: indexmap::IndexMap<i32, CreatableReplicaAssignment>,

    /// The custom topic configurations to set.
    /// 
    /// Supported API versions: 0-7
    pub configs: indexmap::IndexMap<StrBytes, CreateableTopicConfig>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl MapEncodable for CreatableTopic {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 5 {
            types::CompactString.encode(buf, key)?;
        } else {
            types::String.encode(buf, key)?;
        }
        types::Int32.encode(buf, &self.num_partitions)?;
        types::Int16.encode(buf, &self.replication_factor)?;
        if version >= 5 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.assignments)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.assignments)?;
        }
        if version >= 5 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.configs)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.configs)?;
        }
        if version >= 5 {
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
            total_size += types::CompactString.compute_size(key)?;
        } else {
            total_size += types::String.compute_size(key)?;
        }
        total_size += types::Int32.compute_size(&self.num_partitions)?;
        total_size += types::Int16.compute_size(&self.replication_factor)?;
        if version >= 5 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.assignments)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.assignments)?;
        }
        if version >= 5 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.configs)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.configs)?;
        }
        if version >= 5 {
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

impl MapDecodable for CreatableTopic {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = if version >= 5 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let num_partitions = types::Int32.decode(buf)?;
        let replication_factor = types::Int16.decode(buf)?;
        let assignments = if version >= 5 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let configs = if version >= 5 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 5 {
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
            num_partitions,
            replication_factor,
            assignments,
            configs,
            unknown_tagged_fields,
        }))
    }
}

impl Default for CreatableTopic {
    fn default() -> Self {
        Self {
            num_partitions: 0,
            replication_factor: 0,
            assignments: Default::default(),
            configs: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreatableTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
}

/// Valid versions: 0-7
#[derive(Debug, Clone, PartialEq)]
pub struct CreateTopicsRequest {
    /// The topics to create.
    /// 
    /// Supported API versions: 0-7
    pub topics: indexmap::IndexMap<super::TopicName, CreatableTopic>,

    /// How long to wait in milliseconds before timing out the request.
    /// 
    /// Supported API versions: 0-7
    pub timeout_ms: i32,

    /// If true, check that the topics can be created as specified, but don't create anything.
    /// 
    /// Supported API versions: 1-7
    pub validate_only: bool,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for CreateTopicsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 5 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.topics)?;
        }
        types::Int32.encode(buf, &self.timeout_ms)?;
        if version >= 1 {
            types::Boolean.encode(buf, &self.validate_only)?;
        } else {
            if self.validate_only {
                return Err(EncodeError)
            }
        }
        if version >= 5 {
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
        if version >= 5 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
        }
        total_size += types::Int32.compute_size(&self.timeout_ms)?;
        if version >= 1 {
            total_size += types::Boolean.compute_size(&self.validate_only)?;
        } else {
            if self.validate_only {
                return Err(EncodeError)
            }
        }
        if version >= 5 {
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

impl Decodable for CreateTopicsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topics = if version >= 5 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let timeout_ms = types::Int32.decode(buf)?;
        let validate_only = if version >= 1 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 5 {
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

impl Default for CreateTopicsRequest {
    fn default() -> Self {
        Self {
            topics: Default::default(),
            timeout_ms: 60000,
            validate_only: false,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreateTopicsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
}

impl HeaderVersion for CreateTopicsRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 5 {
            2
        } else {
            1
        }
    }
}

