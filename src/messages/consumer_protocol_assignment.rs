//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use protocol_base::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-1
#[derive(Debug, Clone, PartialEq)]
pub struct TopicPartition {
    /// 
    /// 
    /// Supported API versions: 0-1
    pub partitions: Vec<i32>,

}

impl MapEncodable for TopicPartition {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, key)?;
        types::Array(types::Int32).encode(buf, &self.partitions)?;

        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(key)?;
        total_size += types::Array(types::Int32).compute_size(&self.partitions)?;

        Ok(total_size)
    }
}

impl MapDecodable for TopicPartition {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::String.decode(buf)?;
        let partitions = types::Array(types::Int32).decode(buf)?;
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
pub struct ConsumerProtocolAssignment {
    /// 
    /// 
    /// Supported API versions: 0-1
    pub assigned_partitions: indexmap::IndexMap<super::TopicName, TopicPartition>,

    /// 
    /// 
    /// Supported API versions: 0-1
    pub user_data: Option<Bytes>,

}

impl Encodable for ConsumerProtocolAssignment {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Array(types::Struct { version }).encode(buf, &self.assigned_partitions)?;
        types::Bytes.encode(buf, &self.user_data)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Array(types::Struct { version }).compute_size(&self.assigned_partitions)?;
        total_size += types::Bytes.compute_size(&self.user_data)?;

        Ok(total_size)
    }
}

impl Decodable for ConsumerProtocolAssignment {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let assigned_partitions = types::Array(types::Struct { version }).decode(buf)?;
        let user_data = types::Bytes.decode(buf)?;
        Ok(Self {
            assigned_partitions,
            user_data,
        })
    }
}

impl Default for ConsumerProtocolAssignment {
    fn default() -> Self {
        Self {
            assigned_partitions: Default::default(),
            user_data: None,
        }
    }
}

impl Message for ConsumerProtocolAssignment {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

