//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;

use franz_protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-5
#[derive(Debug, Clone)]
pub struct ListOffsetPartitionResponse {
    /// The partition index.
    /// 
    /// Supported API versions: 0-5
    pub partition_index: i32,

    /// The partition error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-5
    pub error_code: i16,

    /// The result offsets.
    /// 
    /// Supported API versions: 0
    pub old_style_offsets: Vec<i64>,

    /// The timestamp associated with the returned offset.
    /// 
    /// Supported API versions: 1-5
    pub timestamp: i64,

    /// The returned offset.
    /// 
    /// Supported API versions: 1-5
    pub offset: i64,

    /// 
    /// 
    /// Supported API versions: 4-5
    pub leader_epoch: i32,

}

impl Encodable for ListOffsetPartitionResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int16.encode(buf, &self.error_code)?;
        if version == 0 {
            types::Array(types::Int64).encode(buf, &self.old_style_offsets)?;
        } else {
            if !self.old_style_offsets.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            types::Int64.encode(buf, &self.timestamp)?;
        } else {
            if self.timestamp != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            types::Int64.encode(buf, &self.offset)?;
        } else {
            if self.offset != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 4 {
            types::Int32.encode(buf, &self.leader_epoch)?;
        } else {
            if self.leader_epoch != 0 {
                return Err(EncodeError)
            }
        }

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int16.compute_size(&self.error_code)?;
        if version == 0 {
            total_size += types::Array(types::Int64).compute_size(&self.old_style_offsets)?;
        } else {
            if !self.old_style_offsets.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            total_size += types::Int64.compute_size(&self.timestamp)?;
        } else {
            if self.timestamp != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            total_size += types::Int64.compute_size(&self.offset)?;
        } else {
            if self.offset != -1 {
                return Err(EncodeError)
            }
        }
        if version >= 4 {
            total_size += types::Int32.compute_size(&self.leader_epoch)?;
        } else {
            if self.leader_epoch != 0 {
                return Err(EncodeError)
            }
        }

        Ok(total_size)
    }
}

impl Decodable for ListOffsetPartitionResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        let old_style_offsets = if version == 0 {
            types::Array(types::Int64).decode(buf)?
        } else {
            Default::default()
        };
        let timestamp = if version >= 1 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let offset = if version >= 1 {
            types::Int64.decode(buf)?
        } else {
            -1
        };
        let leader_epoch = if version >= 4 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        Ok(Self {
            partition_index,
            error_code,
            old_style_offsets,
            timestamp,
            offset,
            leader_epoch,
        })
    }
}

impl Default for ListOffsetPartitionResponse {
    fn default() -> Self {
        Self {
            partition_index: 0,
            error_code: 0,
            old_style_offsets: Default::default(),
            timestamp: -1,
            offset: -1,
            leader_epoch: 0,
        }
    }
}

impl Message for ListOffsetPartitionResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

/// Valid versions: 0-5
#[derive(Debug, Clone)]
pub struct ListOffsetTopicResponse {
    /// The topic name
    /// 
    /// Supported API versions: 0-5
    pub name: super::TopicName,

    /// Each partition in the response.
    /// 
    /// Supported API versions: 0-5
    pub partitions: Vec<ListOffsetPartitionResponse>,

}

impl Encodable for ListOffsetTopicResponse {
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

impl Decodable for ListOffsetTopicResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::String.decode(buf)?;
        let partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            name,
            partitions,
        })
    }
}

impl Default for ListOffsetTopicResponse {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl Message for ListOffsetTopicResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

/// Valid versions: 0-5
#[derive(Debug, Clone)]
pub struct ListOffsetResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 2-5
    pub throttle_time_ms: i32,

    /// Each topic in the response.
    /// 
    /// Supported API versions: 0-5
    pub topics: Vec<ListOffsetTopicResponse>,

}

impl Encodable for ListOffsetResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 2 {
            types::Int32.encode(buf, &self.throttle_time_ms)?;
        }
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 2 {
            total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        }
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for ListOffsetResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = if version >= 2 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            throttle_time_ms,
            topics,
        })
    }
}

impl Default for ListOffsetResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            topics: Default::default(),
        }
    }
}

impl Message for ListOffsetResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

impl HeaderVersion for ListOffsetResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}

