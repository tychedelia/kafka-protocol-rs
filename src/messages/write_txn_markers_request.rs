//! WriteTxnMarkersRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/WriteTxnMarkersRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}, Builder
};


/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
pub struct WritableTxnMarkerTopic {
    /// The topic name.
    /// 
    /// Supported API versions: 0-1
    pub name: super::TopicName,

    /// The indexes of the partitions to write transaction markers for.
    /// 
    /// Supported API versions: 0-1
    pub partition_indexes: Vec<i32>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for WritableTxnMarkerTopic {
    type Builder = WritableTxnMarkerTopicBuilder;

    fn builder() -> Self::Builder{
        WritableTxnMarkerTopicBuilder::default()
    }
}

impl Encodable for WritableTxnMarkerTopic {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            types::CompactString.encode(buf, &self.name)?;
        } else {
            types::String.encode(buf, &self.name)?;
        }
        if version >= 1 {
            types::CompactArray(types::Int32).encode(buf, &self.partition_indexes)?;
        } else {
            types::Array(types::Int32).encode(buf, &self.partition_indexes)?;
        }
        if version >= 1 {
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
        if version >= 1 {
            total_size += types::CompactString.compute_size(&self.name)?;
        } else {
            total_size += types::String.compute_size(&self.name)?;
        }
        if version >= 1 {
            total_size += types::CompactArray(types::Int32).compute_size(&self.partition_indexes)?;
        } else {
            total_size += types::Array(types::Int32).compute_size(&self.partition_indexes)?;
        }
        if version >= 1 {
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

impl Decodable for WritableTxnMarkerTopic {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = if version >= 1 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let partition_indexes = if version >= 1 {
            types::CompactArray(types::Int32).decode(buf)?
        } else {
            types::Array(types::Int32).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 1 {
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
            partition_indexes,
            unknown_tagged_fields,
        })
    }
}

impl Default for WritableTxnMarkerTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partition_indexes: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for WritableTxnMarkerTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
pub struct WritableTxnMarker {
    /// The current producer ID.
    /// 
    /// Supported API versions: 0-1
    pub producer_id: super::ProducerId,

    /// The current epoch associated with the producer ID.
    /// 
    /// Supported API versions: 0-1
    pub producer_epoch: i16,

    /// The result of the transaction to write to the partitions (false = ABORT, true = COMMIT).
    /// 
    /// Supported API versions: 0-1
    pub transaction_result: bool,

    /// Each topic that we want to write transaction marker(s) for.
    /// 
    /// Supported API versions: 0-1
    pub topics: Vec<WritableTxnMarkerTopic>,

    /// Epoch associated with the transaction state partition hosted by this transaction coordinator
    /// 
    /// Supported API versions: 0-1
    pub coordinator_epoch: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for WritableTxnMarker {
    type Builder = WritableTxnMarkerBuilder;

    fn builder() -> Self::Builder{
        WritableTxnMarkerBuilder::default()
    }
}

impl Encodable for WritableTxnMarker {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int64.encode(buf, &self.producer_id)?;
        types::Int16.encode(buf, &self.producer_epoch)?;
        types::Boolean.encode(buf, &self.transaction_result)?;
        if version >= 1 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.topics)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.topics)?;
        }
        types::Int32.encode(buf, &self.coordinator_epoch)?;
        if version >= 1 {
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
        total_size += types::Int64.compute_size(&self.producer_id)?;
        total_size += types::Int16.compute_size(&self.producer_epoch)?;
        total_size += types::Boolean.compute_size(&self.transaction_result)?;
        if version >= 1 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.topics)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
        }
        total_size += types::Int32.compute_size(&self.coordinator_epoch)?;
        if version >= 1 {
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

impl Decodable for WritableTxnMarker {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let producer_id = types::Int64.decode(buf)?;
        let producer_epoch = types::Int16.decode(buf)?;
        let transaction_result = types::Boolean.decode(buf)?;
        let topics = if version >= 1 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let coordinator_epoch = types::Int32.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 1 {
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
            producer_id,
            producer_epoch,
            transaction_result,
            topics,
            coordinator_epoch,
            unknown_tagged_fields,
        })
    }
}

impl Default for WritableTxnMarker {
    fn default() -> Self {
        Self {
            producer_id: (0).into(),
            producer_epoch: 0,
            transaction_result: false,
            topics: Default::default(),
            coordinator_epoch: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for WritableTxnMarker {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
pub struct WriteTxnMarkersRequest {
    /// The transaction markers to be written.
    /// 
    /// Supported API versions: 0-1
    pub markers: Vec<WritableTxnMarker>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for WriteTxnMarkersRequest {
    type Builder = WriteTxnMarkersRequestBuilder;

    fn builder() -> Self::Builder{
        WriteTxnMarkersRequestBuilder::default()
    }
}

impl Encodable for WriteTxnMarkersRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.markers)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.markers)?;
        }
        if version >= 1 {
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
        if version >= 1 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.markers)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.markers)?;
        }
        if version >= 1 {
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

impl Decodable for WriteTxnMarkersRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let markers = if version >= 1 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 1 {
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
            markers,
            unknown_tagged_fields,
        })
    }
}

impl Default for WriteTxnMarkersRequest {
    fn default() -> Self {
        Self {
            markers: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for WriteTxnMarkersRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for WriteTxnMarkersRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 1 {
            2
        } else {
            1
        }
    }
}

