//! OffsetDeleteRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/OffsetDeleteRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use anyhow::{bail, Result};
use bytes::Bytes;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Builder, Decodable,
    Decoder, Encodable, Encoder, HeaderVersion, MapDecodable, MapEncodable, Message, StrBytes,
    VersionRange,
};

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
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

impl Builder for OffsetDeleteRequest {
    type Builder = OffsetDeleteRequestBuilder;

    fn builder() -> Self::Builder {
        OffsetDeleteRequestBuilder::default()
    }
}

impl Encodable for OffsetDeleteRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::String.encode(buf, &self.group_id)?;
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.group_id)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for OffsetDeleteRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let group_id = types::String.decode(buf)?;
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self { group_id, topics })
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
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetDeleteRequestPartition {
    /// The partition index.
    ///
    /// Supported API versions: 0
    pub partition_index: i32,
}

impl Builder for OffsetDeleteRequestPartition {
    type Builder = OffsetDeleteRequestPartitionBuilder;

    fn builder() -> Self::Builder {
        OffsetDeleteRequestPartitionBuilder::default()
    }
}

impl Encodable for OffsetDeleteRequestPartition {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        types::Int32.encode(buf, &self.partition_index)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;

        Ok(total_size)
    }
}

impl Decodable for OffsetDeleteRequestPartition {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let partition_index = types::Int32.decode(buf)?;
        Ok(Self { partition_index })
    }
}

impl Default for OffsetDeleteRequestPartition {
    fn default() -> Self {
        Self { partition_index: 0 }
    }
}

impl Message for OffsetDeleteRequestPartition {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetDeleteRequestTopic {
    /// Each partition to delete offsets for.
    ///
    /// Supported API versions: 0
    pub partitions: Vec<OffsetDeleteRequestPartition>,
}

impl Builder for OffsetDeleteRequestTopic {
    type Builder = OffsetDeleteRequestTopicBuilder;

    fn builder() -> Self::Builder {
        OffsetDeleteRequestTopicBuilder::default()
    }
}

impl MapEncodable for OffsetDeleteRequestTopic {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<()> {
        types::String.encode(buf, key)?;
        types::Array(types::Struct { version }).encode(buf, &self.partitions)?;

        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize> {
        let mut total_size = 0;
        total_size += types::String.compute_size(key)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.partitions)?;

        Ok(total_size)
    }
}

impl MapDecodable for OffsetDeleteRequestTopic {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self)> {
        let key_field = types::String.decode(buf)?;
        let partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok((key_field, Self { partitions }))
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
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for OffsetDeleteRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}
