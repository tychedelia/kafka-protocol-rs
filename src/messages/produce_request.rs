//! ProduceRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ProduceRequest.json).
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


/// Valid versions: 0-9
#[derive(Debug, Clone, PartialEq)]
pub struct PartitionProduceData {
    /// The partition index.
    /// 
    /// Supported API versions: 0-9
    pub index: i32,

    /// The record data to be produced.
    /// 
    /// Supported API versions: 0-9
    pub records: Option<Bytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for PartitionProduceData {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.index)?;
        if version >= 9 {
            types::CompactBytes.encode(buf, &self.records)?;
        } else {
            types::Bytes.encode(buf, &self.records)?;
        }
        if version >= 9 {
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
        total_size += types::Int32.compute_size(&self.index)?;
        if version >= 9 {
            total_size += types::CompactBytes.compute_size(&self.records)?;
        } else {
            total_size += types::Bytes.compute_size(&self.records)?;
        }
        if version >= 9 {
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

impl Decodable for PartitionProduceData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let index = types::Int32.decode(buf)?;
        let records = if version >= 9 {
            types::CompactBytes.decode(buf)?
        } else {
            types::Bytes.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 9 {
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
            index,
            records,
            unknown_tagged_fields,
        })
    }
}

impl Default for PartitionProduceData {
    fn default() -> Self {
        Self {
            index: 0,
            records: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for PartitionProduceData {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
}

/// Valid versions: 0-9
#[derive(Debug, Clone, PartialEq)]
pub struct TopicProduceData {
    /// Each partition to produce to.
    /// 
    /// Supported API versions: 0-9
    pub partition_data: Vec<PartitionProduceData>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl MapEncodable for TopicProduceData {
    type Key = super::TopicName;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 9 {
            types::CompactString.encode(buf, key)?;
        } else {
            types::String.encode(buf, key)?;
        }
        if version >= 9 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.partition_data)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.partition_data)?;
        }
        if version >= 9 {
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
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 9 {
            total_size += types::CompactString.compute_size(key)?;
        } else {
            total_size += types::String.compute_size(key)?;
        }
        if version >= 9 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.partition_data)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.partition_data)?;
        }
        if version >= 9 {
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

impl MapDecodable for TopicProduceData {
    type Key = super::TopicName;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = if version >= 9 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let partition_data = if version >= 9 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 9 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok((key_field, Self {
            partition_data,
            unknown_tagged_fields,
        }))
    }
}

impl Default for TopicProduceData {
    fn default() -> Self {
        Self {
            partition_data: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for TopicProduceData {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
}

/// Valid versions: 0-9
#[derive(Debug, Clone, PartialEq)]
pub struct ProduceRequest {
    /// The transactional ID, or null if the producer is not transactional.
    /// 
    /// Supported API versions: 3-9
    pub transactional_id: Option<super::TransactionalId>,

    /// The number of acknowledgments the producer requires the leader to have received before considering a request complete. Allowed values: 0 for no acknowledgments, 1 for only the leader and -1 for the full ISR.
    /// 
    /// Supported API versions: 0-9
    pub acks: i16,

    /// The timeout to await a response in milliseconds.
    /// 
    /// Supported API versions: 0-9
    pub timeout_ms: i32,

    /// Each topic to produce to.
    /// 
    /// Supported API versions: 0-9
    pub topic_data: indexmap::IndexMap<super::TopicName, TopicProduceData>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for ProduceRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 3 {
            if version >= 9 {
                types::CompactString.encode(buf, &self.transactional_id)?;
            } else {
                types::String.encode(buf, &self.transactional_id)?;
            }
        } else {
            if !self.transactional_id.is_none() {
                return Err(EncodeError)
            }
        }
        types::Int16.encode(buf, &self.acks)?;
        types::Int32.encode(buf, &self.timeout_ms)?;
        if version >= 9 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topic_data)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.topic_data)?;
        }
        if version >= 9 {
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
            if version >= 9 {
                total_size += types::CompactString.compute_size(&self.transactional_id)?;
            } else {
                total_size += types::String.compute_size(&self.transactional_id)?;
            }
        } else {
            if !self.transactional_id.is_none() {
                return Err(EncodeError)
            }
        }
        total_size += types::Int16.compute_size(&self.acks)?;
        total_size += types::Int32.compute_size(&self.timeout_ms)?;
        if version >= 9 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topic_data)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.topic_data)?;
        }
        if version >= 9 {
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

impl Decodable for ProduceRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let transactional_id = if version >= 3 {
            if version >= 9 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            None
        };
        let acks = types::Int16.decode(buf)?;
        let timeout_ms = types::Int32.decode(buf)?;
        let topic_data = if version >= 9 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 9 {
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
            transactional_id,
            acks,
            timeout_ms,
            topic_data,
            unknown_tagged_fields,
        })
    }
}

impl Default for ProduceRequest {
    fn default() -> Self {
        Self {
            transactional_id: None,
            acks: 0,
            timeout_ms: 0,
            topic_data: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ProduceRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 9 };
}

impl HeaderVersion for ProduceRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 9 {
            2
        } else {
            1
        }
    }
}

