//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;

use protocol_base::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-5
#[derive(Debug, Clone)]
pub struct ListOffsetPartition {
    /// The partition index.
    /// 
    /// Supported API versions: 0-5
    pub partition_index: i32,

    /// The current leader epoch.
    /// 
    /// Supported API versions: 4-5
    pub current_leader_epoch: i32,

    /// The current timestamp.
    /// 
    /// Supported API versions: 0-5
    pub timestamp: i64,

    /// The maximum number of offsets to report.
    /// 
    /// Supported API versions: 0
    pub max_num_offsets: i32,

}

impl Encodable for ListOffsetPartition {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        if version >= 4 {
            types::Int32.encode(buf, &self.current_leader_epoch)?;
        } else {
            if self.current_leader_epoch != 0 {
                return Err(EncodeError)
            }
        }
        types::Int64.encode(buf, &self.timestamp)?;
        if version == 0 {
            types::Int32.encode(buf, &self.max_num_offsets)?;
        } else {
            if self.max_num_offsets != 0 {
                return Err(EncodeError)
            }
        }

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        if version >= 4 {
            total_size += types::Int32.compute_size(&self.current_leader_epoch)?;
        } else {
            if self.current_leader_epoch != 0 {
                return Err(EncodeError)
            }
        }
        total_size += types::Int64.compute_size(&self.timestamp)?;
        if version == 0 {
            total_size += types::Int32.compute_size(&self.max_num_offsets)?;
        } else {
            if self.max_num_offsets != 0 {
                return Err(EncodeError)
            }
        }

        Ok(total_size)
    }
}

impl Decodable for ListOffsetPartition {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        let current_leader_epoch = if version >= 4 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let timestamp = types::Int64.decode(buf)?;
        let max_num_offsets = if version == 0 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        Ok(Self {
            partition_index,
            current_leader_epoch,
            timestamp,
            max_num_offsets,
        })
    }
}

impl Default for ListOffsetPartition {
    fn default() -> Self {
        Self {
            partition_index: 0,
            current_leader_epoch: 0,
            timestamp: 0,
            max_num_offsets: 0,
        }
    }
}

impl Message for ListOffsetPartition {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

/// Valid versions: 0-5
#[derive(Debug, Clone)]
pub struct ListOffsetTopic {
    /// The topic name.
    /// 
    /// Supported API versions: 0-5
    pub name: super::TopicName,

    /// Each partition in the request.
    /// 
    /// Supported API versions: 0-5
    pub partitions: Vec<ListOffsetPartition>,

}

impl Encodable for ListOffsetTopic {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, &self.name)?;
        types::Array(types::Struct { version }).encode(buf, &self.partitions)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.name)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.partitions)?;

        Ok(total_size)
    }
}

impl Decodable for ListOffsetTopic {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::String.decode(buf)?;
        let partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            name,
            partitions,
        })
    }
}

impl Default for ListOffsetTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl Message for ListOffsetTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

/// Valid versions: 0-5
#[derive(Debug, Clone)]
pub struct ListOffsetRequest {
    /// The broker ID of the requestor, or -1 if this request is being made by a normal consumer.
    /// 
    /// Supported API versions: 0-5
    pub replica_id: super::BrokerId,

    /// This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records
    /// 
    /// Supported API versions: 2-5
    pub isolation_level: i8,

    /// Each topic in the request.
    /// 
    /// Supported API versions: 0-5
    pub topics: Vec<ListOffsetTopic>,

}

impl Encodable for ListOffsetRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.replica_id)?;
        if version >= 2 {
            types::Int8.encode(buf, &self.isolation_level)?;
        } else {
            if self.isolation_level != 0 {
                return Err(EncodeError)
            }
        }
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.replica_id)?;
        if version >= 2 {
            total_size += types::Int8.compute_size(&self.isolation_level)?;
        } else {
            if self.isolation_level != 0 {
                return Err(EncodeError)
            }
        }
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for ListOffsetRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let replica_id = types::Int32.decode(buf)?;
        let isolation_level = if version >= 2 {
            types::Int8.decode(buf)?
        } else {
            0
        };
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            replica_id,
            isolation_level,
            topics,
        })
    }
}

impl Default for ListOffsetRequest {
    fn default() -> Self {
        Self {
            replica_id: (0).into(),
            isolation_level: 0,
            topics: Default::default(),
        }
    }
}

impl Message for ListOffsetRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

impl HeaderVersion for ListOffsetRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}

