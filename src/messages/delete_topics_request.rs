//! DeleteTopicsRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/DeleteTopicsRequest.json).
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


/// Valid versions: 0-6
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
pub struct DeleteTopicState {
    /// The topic name
    /// 
    /// Supported API versions: 6
    pub name: Option<super::TopicName>,

    /// The unique topic ID
    /// 
    /// Supported API versions: 6
    pub topic_id: Uuid,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for DeleteTopicState {
    type Builder = DeleteTopicStateBuilder;

    fn builder() -> Self::Builder{
        DeleteTopicStateBuilder::default()
    }
}

impl Encodable for DeleteTopicState {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 6 {
            types::CompactString.encode(buf, &self.name)?;
        } else {
            if !self.name.is_none() {
                return Err(EncodeError)
            }
        }
        if version >= 6 {
            types::Uuid.encode(buf, &self.topic_id)?;
        } else {
            if &self.topic_id != &Uuid::nil() {
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
        if version >= 6 {
            total_size += types::CompactString.compute_size(&self.name)?;
        } else {
            if !self.name.is_none() {
                return Err(EncodeError)
            }
        }
        if version >= 6 {
            total_size += types::Uuid.compute_size(&self.topic_id)?;
        } else {
            if &self.topic_id != &Uuid::nil() {
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

impl Decodable for DeleteTopicState {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = if version >= 6 {
            types::CompactString.decode(buf)?
        } else {
            None
        };
        let topic_id = if version >= 6 {
            types::Uuid.decode(buf)?
        } else {
            Uuid::nil()
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
            name,
            topic_id,
            unknown_tagged_fields,
        })
    }
}

impl Default for DeleteTopicState {
    fn default() -> Self {
        Self {
            name: None,
            topic_id: Uuid::nil(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DeleteTopicState {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 6 };
}

/// Valid versions: 0-6
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
pub struct DeleteTopicsRequest {
    /// The name or topic ID of the topic
    /// 
    /// Supported API versions: 6
    pub topics: Vec<DeleteTopicState>,

    /// The names of the topics to delete
    /// 
    /// Supported API versions: 0-5
    pub topic_names: Vec<super::TopicName>,

    /// The length of time in milliseconds to wait for the deletions to complete.
    /// 
    /// Supported API versions: 0-6
    pub timeout_ms: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for DeleteTopicsRequest {
    type Builder = DeleteTopicsRequestBuilder;

    fn builder() -> Self::Builder{
        DeleteTopicsRequestBuilder::default()
    }
}

impl Encodable for DeleteTopicsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 6 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            if !self.topics.is_empty() {
                return Err(EncodeError)
            }
        }
        if version <= 5 {
            if version >= 4 {
                types::CompactArray(types::CompactString).encode(buf, &self.topic_names)?;
            } else {
                types::Array(types::String).encode(buf, &self.topic_names)?;
            }
        }
        types::Int32.encode(buf, &self.timeout_ms)?;
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
        if version >= 6 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            if !self.topics.is_empty() {
                return Err(EncodeError)
            }
        }
        if version <= 5 {
            if version >= 4 {
                total_size += types::CompactArray(types::CompactString).compute_size(&self.topic_names)?;
            } else {
                total_size += types::Array(types::String).compute_size(&self.topic_names)?;
            }
        }
        total_size += types::Int32.compute_size(&self.timeout_ms)?;
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

impl Decodable for DeleteTopicsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topics = if version >= 6 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let topic_names = if version <= 5 {
            if version >= 4 {
                types::CompactArray(types::CompactString).decode(buf)?
            } else {
                types::Array(types::String).decode(buf)?
            }
        } else {
            Default::default()
        };
        let timeout_ms = types::Int32.decode(buf)?;
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
            topics,
            topic_names,
            timeout_ms,
            unknown_tagged_fields,
        })
    }
}

impl Default for DeleteTopicsRequest {
    fn default() -> Self {
        Self {
            topics: Default::default(),
            topic_names: Default::default(),
            timeout_ms: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DeleteTopicsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 6 };
}

impl HeaderVersion for DeleteTopicsRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 4 {
            2
        } else {
            1
        }
    }
}

