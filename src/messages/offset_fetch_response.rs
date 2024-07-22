//! OffsetFetchResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/OffsetFetchResponse.json).
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

/// Valid versions: 0-9
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetFetchResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    ///
    /// Supported API versions: 3-9
    pub throttle_time_ms: i32,

    /// The responses per topic.
    ///
    /// Supported API versions: 0-7
    pub topics: Vec<OffsetFetchResponseTopic>,

    /// The top-level error code, or 0 if there was no error.
    ///
    /// Supported API versions: 2-7
    pub error_code: i16,

    /// The responses per group id.
    ///
    /// Supported API versions: 8-9
    pub groups: Vec<OffsetFetchResponseGroup>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for OffsetFetchResponse {
    type Builder = OffsetFetchResponseBuilder;

    fn builder() -> Self::Builder {
        OffsetFetchResponseBuilder::default()
    }
}

impl Encodable for OffsetFetchResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        if version >= 3 {
            types::Int32.encode(buf, &self.throttle_time_ms)?;
        }
        if version <= 7 {
            if version >= 6 {
                types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
            } else {
                types::Array(types::Struct { version }).encode(buf, &self.topics)?;
            }
        } else {
            if !self.topics.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 2 && version <= 7 {
            types::Int16.encode(buf, &self.error_code)?;
        }
        if version >= 8 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.groups)?;
        } else {
            if !self.groups.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        if version >= 3 {
            total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        }
        if version <= 7 {
            if version >= 6 {
                total_size +=
                    types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
            } else {
                total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
            }
        } else {
            if !self.topics.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 2 && version <= 7 {
            total_size += types::Int16.compute_size(&self.error_code)?;
        }
        if version >= 8 {
            total_size +=
                types::CompactArray(types::Struct { version }).compute_size(&self.groups)?;
        } else {
            if !self.groups.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for OffsetFetchResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let throttle_time_ms = if version >= 3 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let topics = if version <= 7 {
            if version >= 6 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Default::default()
        };
        let error_code = if version >= 2 && version <= 7 {
            types::Int16.decode(buf)?
        } else {
            0
        };
        let groups = if version >= 8 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            throttle_time_ms,
            topics,
            error_code,
            groups,
            unknown_tagged_fields,
        })
    }
}

impl Default for OffsetFetchResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            topics: Default::default(),
            error_code: 0,
            groups: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetFetchResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-9
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetFetchResponseGroup {
    /// The group ID.
    ///
    /// Supported API versions: 8-9
    pub group_id: super::GroupId,

    /// The responses per topic.
    ///
    /// Supported API versions: 8-9
    pub topics: Vec<OffsetFetchResponseTopics>,

    /// The group-level error code, or 0 if there was no error.
    ///
    /// Supported API versions: 8-9
    pub error_code: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for OffsetFetchResponseGroup {
    type Builder = OffsetFetchResponseGroupBuilder;

    fn builder() -> Self::Builder {
        OffsetFetchResponseGroupBuilder::default()
    }
}

impl Encodable for OffsetFetchResponseGroup {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        if version >= 8 {
            types::CompactString.encode(buf, &self.group_id)?;
        } else {
            if !self.group_id.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 8 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            if !self.topics.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 8 {
            types::Int16.encode(buf, &self.error_code)?;
        } else {
            if self.error_code != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        if version >= 8 {
            total_size += types::CompactString.compute_size(&self.group_id)?;
        } else {
            if !self.group_id.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 8 {
            total_size +=
                types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            if !self.topics.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 8 {
            total_size += types::Int16.compute_size(&self.error_code)?;
        } else {
            if self.error_code != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for OffsetFetchResponseGroup {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let group_id = if version >= 8 {
            types::CompactString.decode(buf)?
        } else {
            Default::default()
        };
        let topics = if version >= 8 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let error_code = if version >= 8 {
            types::Int16.decode(buf)?
        } else {
            0
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            group_id,
            topics,
            error_code,
            unknown_tagged_fields,
        })
    }
}

impl Default for OffsetFetchResponseGroup {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            topics: Default::default(),
            error_code: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetFetchResponseGroup {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-9
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetFetchResponsePartition {
    /// The partition index.
    ///
    /// Supported API versions: 0-7
    pub partition_index: i32,

    /// The committed message offset.
    ///
    /// Supported API versions: 0-7
    pub committed_offset: i64,

    /// The leader epoch.
    ///
    /// Supported API versions: 5-7
    pub committed_leader_epoch: i32,

    /// The partition metadata.
    ///
    /// Supported API versions: 0-7
    pub metadata: Option<StrBytes>,

    /// The error code, or 0 if there was no error.
    ///
    /// Supported API versions: 0-7
    pub error_code: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for OffsetFetchResponsePartition {
    type Builder = OffsetFetchResponsePartitionBuilder;

    fn builder() -> Self::Builder {
        OffsetFetchResponsePartitionBuilder::default()
    }
}

impl Encodable for OffsetFetchResponsePartition {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        if version <= 7 {
            types::Int32.encode(buf, &self.partition_index)?;
        } else {
            if self.partition_index != 0 {
                bail!("failed to encode");
            }
        }
        if version <= 7 {
            types::Int64.encode(buf, &self.committed_offset)?;
        } else {
            if self.committed_offset != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 5 && version <= 7 {
            types::Int32.encode(buf, &self.committed_leader_epoch)?;
        }
        if version <= 7 {
            if version >= 6 {
                types::CompactString.encode(buf, &self.metadata)?;
            } else {
                types::String.encode(buf, &self.metadata)?;
            }
        } else {
            if !self
                .metadata
                .as_ref()
                .map(|x| x.is_empty())
                .unwrap_or_default()
            {
                bail!("failed to encode");
            }
        }
        if version <= 7 {
            types::Int16.encode(buf, &self.error_code)?;
        } else {
            if self.error_code != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        if version <= 7 {
            total_size += types::Int32.compute_size(&self.partition_index)?;
        } else {
            if self.partition_index != 0 {
                bail!("failed to encode");
            }
        }
        if version <= 7 {
            total_size += types::Int64.compute_size(&self.committed_offset)?;
        } else {
            if self.committed_offset != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 5 && version <= 7 {
            total_size += types::Int32.compute_size(&self.committed_leader_epoch)?;
        }
        if version <= 7 {
            if version >= 6 {
                total_size += types::CompactString.compute_size(&self.metadata)?;
            } else {
                total_size += types::String.compute_size(&self.metadata)?;
            }
        } else {
            if !self
                .metadata
                .as_ref()
                .map(|x| x.is_empty())
                .unwrap_or_default()
            {
                bail!("failed to encode");
            }
        }
        if version <= 7 {
            total_size += types::Int16.compute_size(&self.error_code)?;
        } else {
            if self.error_code != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for OffsetFetchResponsePartition {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let partition_index = if version <= 7 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let committed_offset = if version <= 7 {
            types::Int64.decode(buf)?
        } else {
            0
        };
        let committed_leader_epoch = if version >= 5 && version <= 7 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let metadata = if version <= 7 {
            if version >= 6 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Some(Default::default())
        };
        let error_code = if version <= 7 {
            types::Int16.decode(buf)?
        } else {
            0
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            partition_index,
            committed_offset,
            committed_leader_epoch,
            metadata,
            error_code,
            unknown_tagged_fields,
        })
    }
}

impl Default for OffsetFetchResponsePartition {
    fn default() -> Self {
        Self {
            partition_index: 0,
            committed_offset: 0,
            committed_leader_epoch: -1,
            metadata: Some(Default::default()),
            error_code: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetFetchResponsePartition {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-9
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetFetchResponsePartitions {
    /// The partition index.
    ///
    /// Supported API versions: 8-9
    pub partition_index: i32,

    /// The committed message offset.
    ///
    /// Supported API versions: 8-9
    pub committed_offset: i64,

    /// The leader epoch.
    ///
    /// Supported API versions: 8-9
    pub committed_leader_epoch: i32,

    /// The partition metadata.
    ///
    /// Supported API versions: 8-9
    pub metadata: Option<StrBytes>,

    /// The partition-level error code, or 0 if there was no error.
    ///
    /// Supported API versions: 8-9
    pub error_code: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for OffsetFetchResponsePartitions {
    type Builder = OffsetFetchResponsePartitionsBuilder;

    fn builder() -> Self::Builder {
        OffsetFetchResponsePartitionsBuilder::default()
    }
}

impl Encodable for OffsetFetchResponsePartitions {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        if version >= 8 {
            types::Int32.encode(buf, &self.partition_index)?;
        } else {
            if self.partition_index != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 8 {
            types::Int64.encode(buf, &self.committed_offset)?;
        } else {
            if self.committed_offset != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 8 {
            types::Int32.encode(buf, &self.committed_leader_epoch)?;
        }
        if version >= 8 {
            types::CompactString.encode(buf, &self.metadata)?;
        } else {
            if !self
                .metadata
                .as_ref()
                .map(|x| x.is_empty())
                .unwrap_or_default()
            {
                bail!("failed to encode");
            }
        }
        if version >= 8 {
            types::Int16.encode(buf, &self.error_code)?;
        } else {
            if self.error_code != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        if version >= 8 {
            total_size += types::Int32.compute_size(&self.partition_index)?;
        } else {
            if self.partition_index != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 8 {
            total_size += types::Int64.compute_size(&self.committed_offset)?;
        } else {
            if self.committed_offset != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 8 {
            total_size += types::Int32.compute_size(&self.committed_leader_epoch)?;
        }
        if version >= 8 {
            total_size += types::CompactString.compute_size(&self.metadata)?;
        } else {
            if !self
                .metadata
                .as_ref()
                .map(|x| x.is_empty())
                .unwrap_or_default()
            {
                bail!("failed to encode");
            }
        }
        if version >= 8 {
            total_size += types::Int16.compute_size(&self.error_code)?;
        } else {
            if self.error_code != 0 {
                bail!("failed to encode");
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for OffsetFetchResponsePartitions {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let partition_index = if version >= 8 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let committed_offset = if version >= 8 {
            types::Int64.decode(buf)?
        } else {
            0
        };
        let committed_leader_epoch = if version >= 8 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let metadata = if version >= 8 {
            types::CompactString.decode(buf)?
        } else {
            Some(Default::default())
        };
        let error_code = if version >= 8 {
            types::Int16.decode(buf)?
        } else {
            0
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            partition_index,
            committed_offset,
            committed_leader_epoch,
            metadata,
            error_code,
            unknown_tagged_fields,
        })
    }
}

impl Default for OffsetFetchResponsePartitions {
    fn default() -> Self {
        Self {
            partition_index: 0,
            committed_offset: 0,
            committed_leader_epoch: -1,
            metadata: Some(Default::default()),
            error_code: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetFetchResponsePartitions {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-9
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetFetchResponseTopic {
    /// The topic name.
    ///
    /// Supported API versions: 0-7
    pub name: super::TopicName,

    /// The responses per partition
    ///
    /// Supported API versions: 0-7
    pub partitions: Vec<OffsetFetchResponsePartition>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for OffsetFetchResponseTopic {
    type Builder = OffsetFetchResponseTopicBuilder;

    fn builder() -> Self::Builder {
        OffsetFetchResponseTopicBuilder::default()
    }
}

impl Encodable for OffsetFetchResponseTopic {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        if version <= 7 {
            if version >= 6 {
                types::CompactString.encode(buf, &self.name)?;
            } else {
                types::String.encode(buf, &self.name)?;
            }
        } else {
            if !self.name.is_empty() {
                bail!("failed to encode");
            }
        }
        if version <= 7 {
            if version >= 6 {
                types::CompactArray(types::Struct { version }).encode(buf, &self.partitions)?;
            } else {
                types::Array(types::Struct { version }).encode(buf, &self.partitions)?;
            }
        } else {
            if !self.partitions.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        if version <= 7 {
            if version >= 6 {
                total_size += types::CompactString.compute_size(&self.name)?;
            } else {
                total_size += types::String.compute_size(&self.name)?;
            }
        } else {
            if !self.name.is_empty() {
                bail!("failed to encode");
            }
        }
        if version <= 7 {
            if version >= 6 {
                total_size += types::CompactArray(types::Struct { version })
                    .compute_size(&self.partitions)?;
            } else {
                total_size +=
                    types::Array(types::Struct { version }).compute_size(&self.partitions)?;
            }
        } else {
            if !self.partitions.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for OffsetFetchResponseTopic {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let name = if version <= 7 {
            if version >= 6 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let partitions = if version <= 7 {
            if version >= 6 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
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

impl Default for OffsetFetchResponseTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetFetchResponseTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-9
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct OffsetFetchResponseTopics {
    /// The topic name.
    ///
    /// Supported API versions: 8-9
    pub name: super::TopicName,

    /// The responses per partition
    ///
    /// Supported API versions: 8-9
    pub partitions: Vec<OffsetFetchResponsePartitions>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for OffsetFetchResponseTopics {
    type Builder = OffsetFetchResponseTopicsBuilder;

    fn builder() -> Self::Builder {
        OffsetFetchResponseTopicsBuilder::default()
    }
}

impl Encodable for OffsetFetchResponseTopics {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<()> {
        if version >= 8 {
            types::CompactString.encode(buf, &self.name)?;
        } else {
            if !self.name.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 8 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.partitions)?;
        } else {
            if !self.partitions.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize> {
        let mut total_size = 0;
        if version >= 8 {
            total_size += types::CompactString.compute_size(&self.name)?;
        } else {
            if !self.name.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 8 {
            total_size +=
                types::CompactArray(types::Struct { version }).compute_size(&self.partitions)?;
        } else {
            if !self.partitions.is_empty() {
                bail!("failed to encode");
            }
        }
        if version >= 6 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for OffsetFetchResponseTopics {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self> {
        let name = if version >= 8 {
            types::CompactString.decode(buf)?
        } else {
            Default::default()
        };
        let partitions = if version >= 8 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
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

impl Default for OffsetFetchResponseTopics {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetFetchResponseTopics {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for OffsetFetchResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 6 {
            1
        } else {
            0
        }
    }
}
