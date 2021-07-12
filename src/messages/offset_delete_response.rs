//! OffsetDeleteResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/OffsetDeleteResponse.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0
#[derive(Debug, Clone, PartialEq)]
pub struct OffsetDeleteResponsePartition {
    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0
    pub error_code: i16,

}

impl MapEncodable for OffsetDeleteResponsePartition {
    type Key = i32;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, key)?;
        types::Int16.encode(buf, &self.error_code)?;

        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(key)?;
        total_size += types::Int16.compute_size(&self.error_code)?;

        Ok(total_size)
    }
}

impl MapDecodable for OffsetDeleteResponsePartition {
    type Key = i32;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::Int32.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        Ok((key_field, Self {
            error_code,
        }))
    }
}

impl Default for OffsetDeleteResponsePartition {
    fn default() -> Self {
        Self {
            error_code: 0,
        }
    }
}

impl Message for OffsetDeleteResponsePartition {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[derive(Debug, Clone, PartialEq)]
pub struct OffsetDeleteResponseTopic {
    /// The responses for each partition in the topic.
    /// 
    /// Supported API versions: 0
    pub partitions: indexmap::IndexMap<i32, OffsetDeleteResponsePartition>,

}

impl MapEncodable for OffsetDeleteResponseTopic {
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

impl MapDecodable for OffsetDeleteResponseTopic {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::String.decode(buf)?;
        let partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok((key_field, Self {
            partitions,
        }))
    }
}

impl Default for OffsetDeleteResponseTopic {
    fn default() -> Self {
        Self {
            partitions: Default::default(),
        }
    }
}

impl Message for OffsetDeleteResponseTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[derive(Debug, Clone, PartialEq)]
pub struct OffsetDeleteResponse {
    /// The top-level error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0
    pub error_code: i16,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 0
    pub throttle_time_ms: i32,

    /// The responses for each topic.
    /// 
    /// Supported API versions: 0
    pub topics: indexmap::IndexMap<super::TopicName, OffsetDeleteResponseTopic>,

}

impl Encodable for OffsetDeleteResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        types::Int32.encode(buf, &self.throttle_time_ms)?;
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for OffsetDeleteResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let throttle_time_ms = types::Int32.decode(buf)?;
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            error_code,
            throttle_time_ms,
            topics,
        })
    }
}

impl Default for OffsetDeleteResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            throttle_time_ms: 0,
            topics: Default::default(),
        }
    }
}

impl Message for OffsetDeleteResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

impl HeaderVersion for OffsetDeleteResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}

