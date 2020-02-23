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


/// Valid versions: 0-3
#[derive(Debug, Clone)]
pub struct OffsetForLeaderPartition {
    /// The partition index.
    /// 
    /// Supported API versions: 0-3
    pub partition_index: i32,

    /// An epoch used to fence consumers/replicas with old metadata.  If the epoch provided by the client is larger than the current epoch known to the broker, then the UNKNOWN_LEADER_EPOCH error code will be returned. If the provided epoch is smaller, then the FENCED_LEADER_EPOCH error code will be returned.
    /// 
    /// Supported API versions: 2-3
    pub current_leader_epoch: i32,

    /// The epoch to look up an offset for.
    /// 
    /// Supported API versions: 0-3
    pub leader_epoch: i32,

}

impl Encodable for OffsetForLeaderPartition {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        if version >= 2 {
            types::Int32.encode(buf, &self.current_leader_epoch)?;
        }
        types::Int32.encode(buf, &self.leader_epoch)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        if version >= 2 {
            total_size += types::Int32.compute_size(&self.current_leader_epoch)?;
        }
        total_size += types::Int32.compute_size(&self.leader_epoch)?;

        Ok(total_size)
    }
}

impl Decodable for OffsetForLeaderPartition {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        let current_leader_epoch = if version >= 2 {
            types::Int32.decode(buf)?
        } else {
            -1
        };
        let leader_epoch = types::Int32.decode(buf)?;
        Ok(Self {
            partition_index,
            current_leader_epoch,
            leader_epoch,
        })
    }
}

impl Default for OffsetForLeaderPartition {
    fn default() -> Self {
        Self {
            partition_index: 0,
            current_leader_epoch: -1,
            leader_epoch: 0,
        }
    }
}

impl Message for OffsetForLeaderPartition {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[derive(Debug, Clone)]
pub struct OffsetForLeaderTopic {
    /// The topic name.
    /// 
    /// Supported API versions: 0-3
    pub name: super::TopicName,

    /// Each partition to get offsets for.
    /// 
    /// Supported API versions: 0-3
    pub partitions: Vec<OffsetForLeaderPartition>,

}

impl Encodable for OffsetForLeaderTopic {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
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

impl Decodable for OffsetForLeaderTopic {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::String.decode(buf)?;
        let partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            name,
            partitions,
        })
    }
}

impl Default for OffsetForLeaderTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl Message for OffsetForLeaderTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

/// Valid versions: 0-3
#[derive(Debug, Clone)]
pub struct OffsetForLeaderEpochRequest {
    /// The broker ID of the follower, of -1 if this request is from a consumer.
    /// 
    /// Supported API versions: 3
    pub replica_id: i32,

    /// Each topic to get offsets for.
    /// 
    /// Supported API versions: 0-3
    pub topics: Vec<OffsetForLeaderTopic>,

}

impl Encodable for OffsetForLeaderEpochRequest {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 3 {
            types::Int32.encode(buf, &self.replica_id)?;
        }
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version == 3 {
            total_size += types::Int32.compute_size(&self.replica_id)?;
        }
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for OffsetForLeaderEpochRequest {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let replica_id = if version == 3 {
            types::Int32.decode(buf)?
        } else {
            -2
        };
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            replica_id,
            topics,
        })
    }
}

impl Default for OffsetForLeaderEpochRequest {
    fn default() -> Self {
        Self {
            replica_id: -2,
            topics: Default::default(),
        }
    }
}

impl Message for OffsetForLeaderEpochRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

impl HeaderVersion for OffsetForLeaderEpochRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}

