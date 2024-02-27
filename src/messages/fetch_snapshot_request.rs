//! FetchSnapshotRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/FetchSnapshotRequest.json).
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


/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct SnapshotId {
    /// 
    /// 
    /// Supported API versions: 0
    pub end_offset: i64,

    /// 
    /// 
    /// Supported API versions: 0
    pub epoch: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for SnapshotId {
    type Builder = SnapshotIdBuilder;

    fn builder() -> Self::Builder{
        SnapshotIdBuilder::default()
    }
}

impl Encodable for SnapshotId {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        buf.put_i64(self.end_offset);
        buf.put_i32(self.epoch);
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt::put_u32(buf, num_tagged_fields as u32);

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += 8;
        total_size += 4;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for SnapshotId {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let end_offset = buf.try_get_i64()?;
        let epoch = buf.try_get_i32()?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            end_offset,
            epoch,
            unknown_tagged_fields,
        })
    }
}

impl Default for SnapshotId {
    fn default() -> Self {
        Self {
            end_offset: 0,
            epoch: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for SnapshotId {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct PartitionSnapshot {
    /// The partition index
    /// 
    /// Supported API versions: 0
    pub partition: i32,

    /// The current leader epoch of the partition, -1 for unknown leader epoch
    /// 
    /// Supported API versions: 0
    pub current_leader_epoch: i32,

    /// The snapshot endOffset and epoch to fetch
    /// 
    /// Supported API versions: 0
    pub snapshot_id: SnapshotId,

    /// The byte position within the snapshot to start fetching from
    /// 
    /// Supported API versions: 0
    pub position: i64,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for PartitionSnapshot {
    type Builder = PartitionSnapshotBuilder;

    fn builder() -> Self::Builder{
        PartitionSnapshotBuilder::default()
    }
}

impl Encodable for PartitionSnapshot {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        buf.put_i32(self.partition);
        buf.put_i32(self.current_leader_epoch);
        types::Struct { version }.encode(buf, &self.snapshot_id)?;
        buf.put_i64(self.position);
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt::put_u32(buf, num_tagged_fields as u32);

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += 4;
        total_size += 4;
        total_size += types::Struct { version }.compute_size(&self.snapshot_id)?;
        total_size += 8;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for PartitionSnapshot {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition = buf.try_get_i32()?;
        let current_leader_epoch = buf.try_get_i32()?;
        let snapshot_id = types::Struct { version }.decode(buf)?;
        let position = buf.try_get_i64()?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            partition,
            current_leader_epoch,
            snapshot_id,
            position,
            unknown_tagged_fields,
        })
    }
}

impl Default for PartitionSnapshot {
    fn default() -> Self {
        Self {
            partition: 0,
            current_leader_epoch: 0,
            snapshot_id: Default::default(),
            position: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for PartitionSnapshot {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct TopicSnapshot {
    /// The name of the topic to fetch
    /// 
    /// Supported API versions: 0
    pub name: super::TopicName,

    /// The partitions to fetch
    /// 
    /// Supported API versions: 0
    pub partitions: Vec<PartitionSnapshot>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for TopicSnapshot {
    type Builder = TopicSnapshotBuilder;

    fn builder() -> Self::Builder{
        TopicSnapshotBuilder::default()
    }
}

impl Encodable for TopicSnapshot {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::CompactString.encode(buf, &self.name)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.partitions)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt::put_u32(buf, num_tagged_fields as u32);

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::CompactString.compute_size(&self.name)?;
        total_size += types::CompactArray(types::Struct { version }).compute_size(&self.partitions)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for TopicSnapshot {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::CompactString.decode(buf)?;
        let partitions = types::CompactArray(types::Struct { version }).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            name,
            partitions,
            unknown_tagged_fields,
        })
    }
}

impl Default for TopicSnapshot {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for TopicSnapshot {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct FetchSnapshotRequest {
    /// The clusterId if known, this is used to validate metadata fetches prior to broker registration
    /// 
    /// Supported API versions: 0
    pub cluster_id: Option<StrBytes>,

    /// The broker ID of the follower
    /// 
    /// Supported API versions: 0
    pub replica_id: super::BrokerId,

    /// The maximum bytes to fetch from all of the snapshots
    /// 
    /// Supported API versions: 0
    pub max_bytes: i32,

    /// The topics to fetch
    /// 
    /// Supported API versions: 0
    pub topics: Vec<TopicSnapshot>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for FetchSnapshotRequest {
    type Builder = FetchSnapshotRequestBuilder;

    fn builder() -> Self::Builder{
        FetchSnapshotRequestBuilder::default()
    }
}

impl Encodable for FetchSnapshotRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.replica_id)?;
        buf.put_i32(self.max_bytes);
        types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        let mut num_tagged_fields = self.unknown_tagged_fields.len();
        if !self.cluster_id.is_none() {
            num_tagged_fields += 1;
        }
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt::put_u32(buf, num_tagged_fields as u32);
        if !self.cluster_id.is_none() {
            let computed_size = types::CompactString.compute_size(&self.cluster_id)?;
            if computed_size > std::u32::MAX as usize {
                error!("Tagged field is too large to encode ({} bytes)", computed_size);
                return Err(EncodeError);
            }
            buf.put_i32(0);
            buf.put_i32(computed_size as i32);
            types::CompactString.encode(buf, &self.cluster_id)?;
        }

        write_unknown_tagged_fields(buf, 1.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.replica_id)?;
        total_size += 4;
        total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        let mut num_tagged_fields = self.unknown_tagged_fields.len();
        if !self.cluster_id.is_none() {
            num_tagged_fields += 1;
        }
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;
        if !self.cluster_id.is_none() {
            let computed_size = types::CompactString.compute_size(&self.cluster_id)?;
            if computed_size > std::u32::MAX as usize {
                error!("Tagged field is too large to encode ({} bytes)", computed_size);
                return Err(EncodeError);
            }
            total_size += 4;
            total_size += 4;
            total_size += computed_size;
        }

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for FetchSnapshotRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let mut cluster_id = None;
        let replica_id = types::Int32.decode(buf)?;
        let max_bytes = buf.try_get_i32()?;
        let topics = types::CompactArray(types::Struct { version }).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            match tag {
                0 => {
                    cluster_id = types::CompactString.decode(buf)?;
                },
                _ => {
                    let unknown_value = buf.try_get_bytes(size as usize)?;
                    unknown_tagged_fields.insert(tag as i32, unknown_value);
                }
            }
        }
        Ok(Self {
            cluster_id,
            replica_id,
            max_bytes,
            topics,
            unknown_tagged_fields,
        })
    }
}

impl Default for FetchSnapshotRequest {
    fn default() -> Self {
        Self {
            cluster_id: None,
            replica_id: (-1).into(),
            max_bytes: 0x7fffffff,
            topics: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for FetchSnapshotRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

impl HeaderVersion for FetchSnapshotRequest {
    fn header_version(version: i16) -> i16 {
        2
    }
}

