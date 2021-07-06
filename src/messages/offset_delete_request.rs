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


/// Valid versions: 0
#[derive(Debug, Clone, PartialEq)]
pub struct OffsetDeleteRequestPartition {
    /// The partition index.
    /// 
    /// Supported API versions: 0
    pub partition_index: i32,

}

impl Encodable for OffsetDeleteRequestPartition {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;

        Ok(total_size)
    }
}

impl Decodable for OffsetDeleteRequestPartition {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        Ok(Self {
            partition_index,
        })
    }
}

impl Default for OffsetDeleteRequestPartition {
    fn default() -> Self {
        Self {
            partition_index: 0,
        }
    }
}

impl Message for OffsetDeleteRequestPartition {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[derive(Debug, Clone, PartialEq)]
pub struct OffsetDeleteRequestTopic {
    /// Each partition to delete offsets for.
    /// 
    /// Supported API versions: 0
    pub partitions: Vec<OffsetDeleteRequestPartition>,

}

impl MapEncodable for OffsetDeleteRequestTopic {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, key)?;
        types::Array(types::Struct { version }).encode(buf, &self.partitions)?;

        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(key)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.partitions)?;

        Ok(total_size)
    }
}

impl MapDecodable for OffsetDeleteRequestTopic {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::String.decode(buf)?;
        let partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok((key_field, Self {
            partitions,
        }))
    }
}

impl Default for OffsetDeleteRequestTopic {
    fn default() -> Self {
        Self {
            partitions: Default::default(),
        }
    }
}

impl Message for OffsetDeleteRequestTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[derive(Debug, Clone, PartialEq)]
pub struct OffsetDeleteRequest {
    /// The unique group identifier.
    /// 
    /// Supported API versions: 0
    pub group_id: super::GroupId,

    /// The topics to delete offsets for
    /// 
    /// Supported API versions: 0
    pub topics: indexmap::IndexMap<super::TopicName, OffsetDeleteRequestTopic>,

}

impl Encodable for OffsetDeleteRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, &self.group_id)?;
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.group_id)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for OffsetDeleteRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let group_id = types::String.decode(buf)?;
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            group_id,
            topics,
        })
    }
}

impl Default for OffsetDeleteRequest {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            topics: Default::default(),
        }
    }
}

impl Message for OffsetDeleteRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

impl HeaderVersion for OffsetDeleteRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}

