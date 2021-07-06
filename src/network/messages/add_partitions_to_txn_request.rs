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


/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct AddPartitionsToTxnTopic {
    /// The partition indexes to add to the transaction
    /// 
    /// Supported API versions: 0-1
    pub partitions: Vec<i32>,

}

impl MapEncodable for AddPartitionsToTxnTopic {
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

impl MapDecodable for AddPartitionsToTxnTopic {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::String.decode(buf)?;
        let partitions = types::Array(types::Int32).decode(buf)?;
        Ok((key_field, Self {
            partitions,
        }))
    }
}

impl Default for AddPartitionsToTxnTopic {
    fn default() -> Self {
        Self {
            partitions: Default::default(),
        }
    }
}

impl Message for AddPartitionsToTxnTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct AddPartitionsToTxnRequest {
    /// The transactional id corresponding to the transaction.
    /// 
    /// Supported API versions: 0-1
    pub transactional_id: super::TransactionalId,

    /// Current producer id in use by the transactional id.
    /// 
    /// Supported API versions: 0-1
    pub producer_id: super::ProducerId,

    /// Current epoch associated with the producer id.
    /// 
    /// Supported API versions: 0-1
    pub producer_epoch: i16,

    /// The partitions to add to the transation.
    /// 
    /// Supported API versions: 0-1
    pub topics: indexmap::IndexMap<super::TopicName, AddPartitionsToTxnTopic>,

}

impl Encodable for AddPartitionsToTxnRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, &self.transactional_id)?;
        types::Int64.encode(buf, &self.producer_id)?;
        types::Int16.encode(buf, &self.producer_epoch)?;
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.transactional_id)?;
        total_size += types::Int64.compute_size(&self.producer_id)?;
        total_size += types::Int16.compute_size(&self.producer_epoch)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for AddPartitionsToTxnRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let transactional_id = types::String.decode(buf)?;
        let producer_id = types::Int64.decode(buf)?;
        let producer_epoch = types::Int16.decode(buf)?;
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            transactional_id,
            producer_id,
            producer_epoch,
            topics,
        })
    }
}

impl Default for AddPartitionsToTxnRequest {
    fn default() -> Self {
        Self {
            transactional_id: Default::default(),
            producer_id: (0).into(),
            producer_epoch: 0,
            topics: Default::default(),
        }
    }
}

impl Message for AddPartitionsToTxnRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for AddPartitionsToTxnRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}

