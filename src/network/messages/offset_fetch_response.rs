//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::{Buf, BufMut};
use log::error;

use franz_protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size,
};


/// Valid versions: 0-7
#[derive(Debug, Clone)]
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
    pub metadata: Option<String>,

    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-7
    pub error_code: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for OffsetFetchResponsePartition {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int64.encode(buf, &self.committed_offset)?;
        if version >= 5 {
            types::Int32.encode(buf, &self.committed_leader_epoch)?;
        }
        if version >= 6 {
            types::CompactString.encode(buf, &self.metadata)?;
        } else {
            types::String.encode(buf, &self.metadata)?;
        }
        types::Int16.encode(buf, &self.error_code)?;
        if version >= 6 {
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
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int64.compute_size(&self.committed_offset)?;
        if version >= 5 {
            total_size += types::Int32.compute_size(&self.committed_leader_epoch)?;
        }
        if version >= 6 {
            total_size += types::CompactString.compute_size(&self.metadata)?;
        } else {
            total_size += types::String.compute_size(&self.metadata)?;
        }
        total_size += types::Int16.compute_size(&self.error_code)?;
        if version >= 6 {
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

impl Decodable for OffsetFetchResponsePartition {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        let committed_offset = types::Int64.decode(buf)?;
        let committed_leader_epoch = if version >= 5 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let metadata = if version >= 6 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let error_code = types::Int16.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
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
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
}

/// Valid versions: 0-7
#[derive(Debug, Clone)]
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
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for OffsetFetchResponseTopic {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 6 {
            types::CompactString.encode(buf, &self.name)?;
        } else {
            types::String.encode(buf, &self.name)?;
        }
        if version >= 6 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.partitions)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.partitions)?;
        }
        if version >= 6 {
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
            total_size += types::String.compute_size(&self.name)?;
        }
        if version >= 6 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.partitions)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.partitions)?;
        }
        if version >= 6 {
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

impl Decodable for OffsetFetchResponseTopic {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = if version >= 6 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let partitions = if version >= 6 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
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
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
}

/// Valid versions: 0-7
#[derive(Debug, Clone)]
pub struct OffsetFetchResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 3-7
    pub throttle_time_ms: i32,

    /// The responses per topic.
    /// 
    /// Supported API versions: 0-7
    pub topics: Vec<OffsetFetchResponseTopic>,

    /// The top-level error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 2-7
    pub error_code: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for OffsetFetchResponse {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 3 {
            types::Int32.encode(buf, &self.throttle_time_ms)?;
        }
        if version >= 6 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.topics)?;
        }
        if version >= 2 {
            types::Int16.encode(buf, &self.error_code)?;
        }
        if version >= 6 {
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
        if version >= 3 {
            total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        }
        if version >= 6 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
        }
        if version >= 2 {
            total_size += types::Int16.compute_size(&self.error_code)?;
        }
        if version >= 6 {
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

impl Decodable for OffsetFetchResponse {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = if version >= 3 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let topics = if version >= 6 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let error_code = if version >= 2 {
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
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            throttle_time_ms,
            topics,
            error_code,
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
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for OffsetFetchResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
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

