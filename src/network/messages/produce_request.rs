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


/// Valid versions: 0-8
#[derive(Debug, Clone)]
pub struct PartitionProduceData {
    /// The partition index.
    /// 
    /// Supported API versions: 0-8
    pub partition_index: i32,

    /// The record data to be produced.
    /// 
    /// Supported API versions: 0-8
    pub records: Option<Bytes>,

}

impl Encodable for PartitionProduceData {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Bytes.encode(buf, &self.records)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Bytes.compute_size(&self.records)?;

        Ok(total_size)
    }
}

impl Decodable for PartitionProduceData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        let records = types::Bytes.decode(buf)?;
        Ok(Self {
            partition_index,
            records,
        })
    }
}

impl Default for PartitionProduceData {
    fn default() -> Self {
        Self {
            partition_index: 0,
            records: Some(Default::default()),
        }
    }
}

impl Message for PartitionProduceData {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
}

/// Valid versions: 0-8
#[derive(Debug, Clone)]
pub struct TopicProduceData {
    /// The topic name.
    /// 
    /// Supported API versions: 0-8
    pub name: super::TopicName,

    /// Each partition to produce to.
    /// 
    /// Supported API versions: 0-8
    pub partitions: Vec<PartitionProduceData>,

}

impl Encodable for TopicProduceData {
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

impl Decodable for TopicProduceData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::String.decode(buf)?;
        let partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            name,
            partitions,
        })
    }
}

impl Default for TopicProduceData {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl Message for TopicProduceData {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
}

/// Valid versions: 0-8
#[derive(Debug, Clone)]
pub struct ProduceRequest {
    /// The transactional ID, or null if the producer is not transactional.
    /// 
    /// Supported API versions: 3-8
    pub transactional_id: Option<super::TransactionalId>,

    /// The number of acknowledgments the producer requires the leader to have received before considering a request complete. Allowed values: 0 for no acknowledgments, 1 for only the leader and -1 for the full ISR.
    /// 
    /// Supported API versions: 0-8
    pub acks: i16,

    /// The timeout to await a response in miliseconds.
    /// 
    /// Supported API versions: 0-8
    pub timeout_ms: i32,

    /// Each topic to produce to.
    /// 
    /// Supported API versions: 0-8
    pub topics: Vec<TopicProduceData>,

}

impl Encodable for ProduceRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 3 {
            types::String.encode(buf, &self.transactional_id)?;
        } else {
            if !self.transactional_id.as_ref().map(|x| x.is_empty()).unwrap_or_default() {
                return Err(EncodeError)
            }
        }
        types::Int16.encode(buf, &self.acks)?;
        types::Int32.encode(buf, &self.timeout_ms)?;
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 3 {
            total_size += types::String.compute_size(&self.transactional_id)?;
        } else {
            if !self.transactional_id.as_ref().map(|x| x.is_empty()).unwrap_or_default() {
                return Err(EncodeError)
            }
        }
        total_size += types::Int16.compute_size(&self.acks)?;
        total_size += types::Int32.compute_size(&self.timeout_ms)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for ProduceRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let transactional_id = if version >= 3 {
            types::String.decode(buf)?
        } else {
            Some(Default::default())
        };
        let acks = types::Int16.decode(buf)?;
        let timeout_ms = types::Int32.decode(buf)?;
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            transactional_id,
            acks,
            timeout_ms,
            topics,
        })
    }
}

impl Default for ProduceRequest {
    fn default() -> Self {
        Self {
            transactional_id: Some(Default::default()),
            acks: 0,
            timeout_ms: 0,
            topics: Default::default(),
        }
    }
}

impl Message for ProduceRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 8 };
}

impl HeaderVersion for ProduceRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}

