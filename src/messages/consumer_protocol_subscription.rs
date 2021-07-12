//! ConsumerProtocolSubscription
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ConsumerProtocolSubscription.json).
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


/// Valid versions: 0-1
#[derive(Debug, Clone, PartialEq)]
pub struct TopicPartition {
    /// 
    /// 
    /// Supported API versions: 1
    pub partitions: Vec<i32>,

}

impl MapEncodable for TopicPartition {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            types::String.encode(buf, key)?;
        } else {
            if !key.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            types::Array(types::Int32).encode(buf, &self.partitions)?;
        } else {
            if !self.partitions.is_empty() {
                return Err(EncodeError)
            }
        }

        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 1 {
            total_size += types::String.compute_size(key)?;
        } else {
            if !key.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            total_size += types::Array(types::Int32).compute_size(&self.partitions)?;
        } else {
            if !self.partitions.is_empty() {
                return Err(EncodeError)
            }
        }

        Ok(total_size)
    }
}

impl MapDecodable for TopicPartition {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = if version >= 1 {
            types::String.decode(buf)?
        } else {
            Default::default()
        };
        let partitions = if version >= 1 {
            types::Array(types::Int32).decode(buf)?
        } else {
            Default::default()
        };
        Ok((key_field, Self {
            partitions,
        }))
    }
}

impl Default for TopicPartition {
    fn default() -> Self {
        Self {
            partitions: Default::default(),
        }
    }
}

impl Message for TopicPartition {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone, PartialEq)]
pub struct ConsumerProtocolSubscription {
    /// 
    /// 
    /// Supported API versions: 0-1
    pub topics: Vec<StrBytes>,

    /// 
    /// 
    /// Supported API versions: 0-1
    pub user_data: Option<Bytes>,

    /// 
    /// 
    /// Supported API versions: 1
    pub owned_partitions: indexmap::IndexMap<super::TopicName, TopicPartition>,

}

impl Encodable for ConsumerProtocolSubscription {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Array(types::String).encode(buf, &self.topics)?;
        types::Bytes.encode(buf, &self.user_data)?;
        if version >= 1 {
            types::Array(types::Struct { version }).encode(buf, &self.owned_partitions)?;
        }

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Array(types::String).compute_size(&self.topics)?;
        total_size += types::Bytes.compute_size(&self.user_data)?;
        if version >= 1 {
            total_size += types::Array(types::Struct { version }).compute_size(&self.owned_partitions)?;
        }

        Ok(total_size)
    }
}

impl Decodable for ConsumerProtocolSubscription {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let topics = types::Array(types::String).decode(buf)?;
        let user_data = types::Bytes.decode(buf)?;
        let owned_partitions = if version >= 1 {
            types::Array(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        Ok(Self {
            topics,
            user_data,
            owned_partitions,
        })
    }
}

impl Default for ConsumerProtocolSubscription {
    fn default() -> Self {
        Self {
            topics: Default::default(),
            user_data: None,
            owned_partitions: Default::default(),
        }
    }
}

impl Message for ConsumerProtocolSubscription {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

