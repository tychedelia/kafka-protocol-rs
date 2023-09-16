//! FetchResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/FetchResponse.json).
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


/// Valid versions: 0-13
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct EpochEndOffset {
    /// 
    /// 
    /// Supported API versions: 12-13
    pub epoch: i32,

    /// 
    /// 
    /// Supported API versions: 12-13
    pub end_offset: i64,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for EpochEndOffset {
    type Builder = EpochEndOffsetBuilder;

    fn builder() -> Self::Builder{
        EpochEndOffsetBuilder::default()
    }
}

impl Encodable for EpochEndOffset {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 12 {
            types::Int32.encode(buf, &self.epoch)?;
        } else {
            if self.epoch != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 12 {
            types::Int64.encode(buf, &self.end_offset)?;
        } else {
            if self.end_offset != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 12 {
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
        if version >= 12 {
            total_size += types::Int32.compute_size(&self.epoch)?;
        } else {
            if self.epoch != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 12 {
            total_size += types::Int64.compute_size(&self.end_offset)?;
        } else {
            if self.end_offset != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 12 {
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

impl Decodable for EpochEndOffset {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let epoch = if version >= 12 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let end_offset = if version >= 12 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 12 {
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
            epoch,
            end_offset,
            unknown_tagged_fields,
        })
    }
}

impl Default for EpochEndOffset {
    fn default() -> Self {
        Self {
            epoch: -1,
            end_offset: -1,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for EpochEndOffset {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 13 };
}

/// Valid versions: 0-13
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct LeaderIdAndEpoch {
    /// The ID of the current leader or -1 if the leader is unknown.
    /// 
    /// Supported API versions: 12-13
    pub leader_id: super::BrokerId,

    /// The latest known leader epoch
    /// 
    /// Supported API versions: 12-13
    pub leader_epoch: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for LeaderIdAndEpoch {
    type Builder = LeaderIdAndEpochBuilder;

    fn builder() -> Self::Builder{
        LeaderIdAndEpochBuilder::default()
    }
}

impl Encodable for LeaderIdAndEpoch {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 12 {
            types::Int32.encode(buf, &self.leader_id)?;
        } else {
            if self.leader_id != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 12 {
            types::Int32.encode(buf, &self.leader_epoch)?;
        } else {
            if self.leader_epoch != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 12 {
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
        if version >= 12 {
            total_size += types::Int32.compute_size(&self.leader_id)?;
        } else {
            if self.leader_id != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 12 {
            total_size += types::Int32.compute_size(&self.leader_epoch)?;
        } else {
            if self.leader_epoch != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 12 {
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

impl Decodable for LeaderIdAndEpoch {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let leader_id = if version >= 12 {
            types::Int32.decode(buf)?
        } else {
            (-1).into()
        };
        let leader_epoch = if version >= 12 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 12 {
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
            leader_id,
            leader_epoch,
            unknown_tagged_fields,
        })
    }
}

impl Default for LeaderIdAndEpoch {
    fn default() -> Self {
        Self {
            leader_id: (-1).into(),
            leader_epoch: -1,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for LeaderIdAndEpoch {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 13 };
}

/// Valid versions: 0-13
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct SnapshotId {
    /// 
    /// 
    /// Supported API versions: 0-13
    pub end_offset: i64,

    /// 
    /// 
    /// Supported API versions: 0-13
    pub epoch: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for SnapshotId {
    type Builder = SnapshotIdBuilder;

    fn builder() -> Self::Builder{
        SnapshotIdBuilder::default()
    }
}

impl Encodable for SnapshotId {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int64.encode(buf, &self.end_offset)?;
        types::Int32.encode(buf, &self.epoch)?;
        if version >= 12 {
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
        total_size += types::Int64.compute_size(&self.end_offset)?;
        total_size += types::Int32.compute_size(&self.epoch)?;
        if version >= 12 {
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

impl Decodable for SnapshotId {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let end_offset = types::Int64.decode(buf)?;
        let epoch = types::Int32.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 12 {
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
            end_offset,
            epoch,
            unknown_tagged_fields,
        })
    }
}

impl Default for SnapshotId {
    fn default() -> Self {
        Self {
            end_offset: -1,
            epoch: -1,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for SnapshotId {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 13 };
}

/// Valid versions: 0-13
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct AbortedTransaction {
    /// The producer id associated with the aborted transaction.
    /// 
    /// Supported API versions: 4-13
    pub producer_id: super::ProducerId,

    /// The first offset in the aborted transaction.
    /// 
    /// Supported API versions: 4-13
    pub first_offset: i64,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for AbortedTransaction {
    type Builder = AbortedTransactionBuilder;

    fn builder() -> Self::Builder{
        AbortedTransactionBuilder::default()
    }
}

impl Encodable for AbortedTransaction {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 4 {
            types::Int64.encode(buf, &self.producer_id)?;
        } else {
            if self.producer_id != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 4 {
            types::Int64.encode(buf, &self.first_offset)?;
        } else {
            if self.first_offset != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 12 {
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
        if version >= 4 {
            total_size += types::Int64.compute_size(&self.producer_id)?;
        } else {
            if self.producer_id != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 4 {
            total_size += types::Int64.compute_size(&self.first_offset)?;
        } else {
            if self.first_offset != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 12 {
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

impl Decodable for AbortedTransaction {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let producer_id = if version >= 4 {
            types::Int64.decode(buf)?
        } else {
            (0).into()
        };
        let first_offset = if version >= 4 {
            types::Int64.decode(buf)?
        } else {
            0
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 12 {
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
            producer_id,
            first_offset,
            unknown_tagged_fields,
        })
    }
}

impl Default for AbortedTransaction {
    fn default() -> Self {
        Self {
            producer_id: (0).into(),
            first_offset: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AbortedTransaction {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 13 };
}

/// Valid versions: 0-13
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct PartitionData {
    /// The partition index.
    /// 
    /// Supported API versions: 0-13
    pub partition_index: i32,

    /// The error code, or 0 if there was no fetch error.
    /// 
    /// Supported API versions: 0-13
    pub error_code: i16,

    /// The current high water mark.
    /// 
    /// Supported API versions: 0-13
    pub high_watermark: i64,

    /// The last stable offset (or LSO) of the partition. This is the last offset such that the state of all transactional records prior to this offset have been decided (ABORTED or COMMITTED)
    /// 
    /// Supported API versions: 4-13
    pub last_stable_offset: i64,

    /// The current log start offset.
    /// 
    /// Supported API versions: 5-13
    pub log_start_offset: i64,

    /// In case divergence is detected based on the `LastFetchedEpoch` and `FetchOffset` in the request, this field indicates the largest epoch and its end offset such that subsequent records are known to diverge
    /// 
    /// Supported API versions: 12-13
    pub diverging_epoch: EpochEndOffset,

    /// 
    /// 
    /// Supported API versions: 12-13
    pub current_leader: LeaderIdAndEpoch,

    /// In the case of fetching an offset less than the LogStartOffset, this is the end offset and epoch that should be used in the FetchSnapshot request.
    /// 
    /// Supported API versions: 12-13
    pub snapshot_id: SnapshotId,

    /// The aborted transactions.
    /// 
    /// Supported API versions: 4-13
    pub aborted_transactions: Option<Vec<AbortedTransaction>>,

    /// The preferred read replica for the consumer to use on its next fetch request
    /// 
    /// Supported API versions: 11-13
    pub preferred_read_replica: super::BrokerId,

    /// The record data.
    /// 
    /// Supported API versions: 0-13
    pub records: Option<Bytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for PartitionData {
    type Builder = PartitionDataBuilder;

    fn builder() -> Self::Builder{
        PartitionDataBuilder::default()
    }
}

impl Encodable for PartitionData {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int16.encode(buf, &self.error_code)?;
        types::Int64.encode(buf, &self.high_watermark)?;
        if version >= 4 {
            types::Int64.encode(buf, &self.last_stable_offset)?;
        }
        if version >= 5 {
            types::Int64.encode(buf, &self.log_start_offset)?;
        }
        if version >= 4 {
            if version >= 12 {
                types::CompactArray(types::Struct { version }).encode(buf, &self.aborted_transactions)?;
            } else {
                types::Array(types::Struct { version }).encode(buf, &self.aborted_transactions)?;
            }
        }
        if version >= 11 {
            types::Int32.encode(buf, &self.preferred_read_replica)?;
        } else {
            if self.preferred_read_replica != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 12 {
            types::CompactBytes.encode(buf, &self.records)?;
        } else {
            types::Bytes.encode(buf, &self.records)?;
        }
        if version >= 12 {
            let mut num_tagged_fields = self.unknown_tagged_fields.len();
            if &self.diverging_epoch != &Default::default() {
                num_tagged_fields += 1;
            }
            if &self.current_leader != &Default::default() {
                num_tagged_fields += 1;
            }
            if &self.snapshot_id != &Default::default() {
                num_tagged_fields += 1;
            }
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;
            if &self.diverging_epoch != &Default::default() {
                let computed_size = types::Struct { version }.compute_size(&self.diverging_epoch)?;
                if computed_size > std::u32::MAX as usize {
                    error!("Tagged field is too large to encode ({} bytes)", computed_size);
                    return Err(EncodeError);
                }
                types::UnsignedVarInt.encode(buf, 0)?;
                types::UnsignedVarInt.encode(buf, computed_size as u32)?;
                types::Struct { version }.encode(buf, &self.diverging_epoch)?;
            }
            if &self.current_leader != &Default::default() {
                let computed_size = types::Struct { version }.compute_size(&self.current_leader)?;
                if computed_size > std::u32::MAX as usize {
                    error!("Tagged field is too large to encode ({} bytes)", computed_size);
                    return Err(EncodeError);
                }
                types::UnsignedVarInt.encode(buf, 1)?;
                types::UnsignedVarInt.encode(buf, computed_size as u32)?;
                types::Struct { version }.encode(buf, &self.current_leader)?;
            }
            if &self.snapshot_id != &Default::default() {
                let computed_size = types::Struct { version }.compute_size(&self.snapshot_id)?;
                if computed_size > std::u32::MAX as usize {
                    error!("Tagged field is too large to encode ({} bytes)", computed_size);
                    return Err(EncodeError);
                }
                types::UnsignedVarInt.encode(buf, 2)?;
                types::UnsignedVarInt.encode(buf, computed_size as u32)?;
                types::Struct { version }.encode(buf, &self.snapshot_id)?;
            }

            write_unknown_tagged_fields(buf, 3.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::Int64.compute_size(&self.high_watermark)?;
        if version >= 4 {
            total_size += types::Int64.compute_size(&self.last_stable_offset)?;
        }
        if version >= 5 {
            total_size += types::Int64.compute_size(&self.log_start_offset)?;
        }
        if version >= 4 {
            if version >= 12 {
                total_size += types::CompactArray(types::Struct { version }).compute_size(&self.aborted_transactions)?;
            } else {
                total_size += types::Array(types::Struct { version }).compute_size(&self.aborted_transactions)?;
            }
        }
        if version >= 11 {
            total_size += types::Int32.compute_size(&self.preferred_read_replica)?;
        } else {
            if self.preferred_read_replica != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 12 {
            total_size += types::CompactBytes.compute_size(&self.records)?;
        } else {
            total_size += types::Bytes.compute_size(&self.records)?;
        }
        if version >= 12 {
            let mut num_tagged_fields = self.unknown_tagged_fields.len();
            if &self.diverging_epoch != &Default::default() {
                num_tagged_fields += 1;
            }
            if &self.current_leader != &Default::default() {
                num_tagged_fields += 1;
            }
            if &self.snapshot_id != &Default::default() {
                num_tagged_fields += 1;
            }
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;
            if &self.diverging_epoch != &Default::default() {
                let computed_size = types::Struct { version }.compute_size(&self.diverging_epoch)?;
                if computed_size > std::u32::MAX as usize {
                    error!("Tagged field is too large to encode ({} bytes)", computed_size);
                    return Err(EncodeError);
                }
                total_size += types::UnsignedVarInt.compute_size(0)?;
                total_size += types::UnsignedVarInt.compute_size(computed_size as u32)?;
                total_size += computed_size;
            }
            if &self.current_leader != &Default::default() {
                let computed_size = types::Struct { version }.compute_size(&self.current_leader)?;
                if computed_size > std::u32::MAX as usize {
                    error!("Tagged field is too large to encode ({} bytes)", computed_size);
                    return Err(EncodeError);
                }
                total_size += types::UnsignedVarInt.compute_size(1)?;
                total_size += types::UnsignedVarInt.compute_size(computed_size as u32)?;
                total_size += computed_size;
            }
            if &self.snapshot_id != &Default::default() {
                let computed_size = types::Struct { version }.compute_size(&self.snapshot_id)?;
                if computed_size > std::u32::MAX as usize {
                    error!("Tagged field is too large to encode ({} bytes)", computed_size);
                    return Err(EncodeError);
                }
                total_size += types::UnsignedVarInt.compute_size(2)?;
                total_size += types::UnsignedVarInt.compute_size(computed_size as u32)?;
                total_size += computed_size;
            }

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for PartitionData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        let high_watermark = types::Int64.decode(buf)?;
        let last_stable_offset = if version >= 4 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let log_start_offset = if version >= 5 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let mut diverging_epoch = Default::default();
        let mut current_leader = Default::default();
        let mut snapshot_id = Default::default();
        let aborted_transactions = if version >= 4 {
            if version >= 12 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Some(Default::default())
        };
        let preferred_read_replica = if version >= 11 {
            types::Int32.decode(buf)?
        } else {
            (-1).into()
        };
        let records = if version >= 12 {
            types::CompactBytes.decode(buf)?
        } else {
            types::Bytes.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 12 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                match tag {
                    0 => {
                        diverging_epoch = types::Struct { version }.decode(buf)?;
                    },
                    1 => {
                        current_leader = types::Struct { version }.decode(buf)?;
                    },
                    2 => {
                        snapshot_id = types::Struct { version }.decode(buf)?;
                    },
                    _ => {
                        let mut unknown_value = vec![0; size as usize];
                        buf.try_copy_to_slice(&mut unknown_value)?;
                        unknown_tagged_fields.insert(tag as i32, unknown_value);
                    }
                }
            }
        }
        Ok(Self {
            partition_index,
            error_code,
            high_watermark,
            last_stable_offset,
            log_start_offset,
            diverging_epoch,
            current_leader,
            snapshot_id,
            aborted_transactions,
            preferred_read_replica,
            records,
            unknown_tagged_fields,
        })
    }
}

impl Default for PartitionData {
    fn default() -> Self {
        Self {
            partition_index: 0,
            error_code: 0,
            high_watermark: 0,
            last_stable_offset: -1,
            log_start_offset: -1,
            diverging_epoch: Default::default(),
            current_leader: Default::default(),
            snapshot_id: Default::default(),
            aborted_transactions: Some(Default::default()),
            preferred_read_replica: (-1).into(),
            records: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for PartitionData {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 13 };
}

/// Valid versions: 0-13
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct FetchableTopicResponse {
    /// The topic name.
    /// 
    /// Supported API versions: 0-12
    pub topic: super::TopicName,

    /// The unique topic ID
    /// 
    /// Supported API versions: 13
    pub topic_id: Uuid,

    /// The topic partitions.
    /// 
    /// Supported API versions: 0-13
    pub partitions: Vec<PartitionData>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for FetchableTopicResponse {
    type Builder = FetchableTopicResponseBuilder;

    fn builder() -> Self::Builder{
        FetchableTopicResponseBuilder::default()
    }
}

impl Encodable for FetchableTopicResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version <= 12 {
            if version >= 12 {
                types::CompactString.encode(buf, &self.topic)?;
            } else {
                types::String.encode(buf, &self.topic)?;
            }
        }
        if version >= 13 {
            types::Uuid.encode(buf, &self.topic_id)?;
        }
        if version >= 12 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.partitions)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.partitions)?;
        }
        if version >= 12 {
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
        if version <= 12 {
            if version >= 12 {
                total_size += types::CompactString.compute_size(&self.topic)?;
            } else {
                total_size += types::String.compute_size(&self.topic)?;
            }
        }
        if version >= 13 {
            total_size += types::Uuid.compute_size(&self.topic_id)?;
        }
        if version >= 12 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.partitions)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.partitions)?;
        }
        if version >= 12 {
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

impl Decodable for FetchableTopicResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topic = if version <= 12 {
            if version >= 12 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let topic_id = if version >= 13 {
            types::Uuid.decode(buf)?
        } else {
            Uuid::nil()
        };
        let partitions = if version >= 12 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 12 {
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
            topic_id,
            partitions,
            unknown_tagged_fields,
        })
    }
}

impl Default for FetchableTopicResponse {
    fn default() -> Self {
        Self {
            topic: Default::default(),
            topic_id: Uuid::nil(),
            partitions: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for FetchableTopicResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 13 };
}

/// Valid versions: 0-13
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct FetchResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 1-13
    pub throttle_time_ms: i32,

    /// The top level response error code.
    /// 
    /// Supported API versions: 7-13
    pub error_code: i16,

    /// The fetch session ID, or 0 if this is not part of a fetch session.
    /// 
    /// Supported API versions: 7-13
    pub session_id: i32,

    /// The response topics.
    /// 
    /// Supported API versions: 0-13
    pub responses: Vec<FetchableTopicResponse>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for FetchResponse {
    type Builder = FetchResponseBuilder;

    fn builder() -> Self::Builder{
        FetchResponseBuilder::default()
    }
}

impl Encodable for FetchResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            types::Int32.encode(buf, &self.throttle_time_ms)?;
        }
        if version >= 7 {
            types::Int16.encode(buf, &self.error_code)?;
        }
        if version >= 7 {
            types::Int32.encode(buf, &self.session_id)?;
        } else {
            if self.session_id != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 12 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.responses)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.responses)?;
        }
        if version >= 12 {
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
            total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        }
        if version >= 7 {
            total_size += types::Int16.compute_size(&self.error_code)?;
        }
        if version >= 7 {
            total_size += types::Int32.compute_size(&self.session_id)?;
        } else {
            if self.session_id != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 12 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.responses)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.responses)?;
        }
        if version >= 12 {
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

impl Decodable for FetchResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = if version >= 1 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let error_code = if version >= 7 {
            types::Int16.decode(buf)?
        } else {
            0
        };
        let session_id = if version >= 7 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let responses = if version >= 12 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 12 {
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
            throttle_time_ms,
            error_code,
            session_id,
            responses,
            unknown_tagged_fields,
        })
    }
}

impl Default for FetchResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            session_id: 0,
            responses: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for FetchResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 13 };
}

impl HeaderVersion for FetchResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 12 {
            1
        } else {
            0
        }
    }
}

