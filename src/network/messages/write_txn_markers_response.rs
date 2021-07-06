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


/// Valid versions: 0
#[derive(Debug, Clone)]
pub struct WritableTxnMarkerPartitionResult {
    /// The partition index.
    /// 
    /// Supported API versions: 0
    pub partition_index: i32,

    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0
    pub error_code: i16,

}

impl Encodable for WritableTxnMarkerPartitionResult {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.partition_index)?;
        types::Int16.encode(buf, &self.error_code)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.partition_index)?;
        total_size += types::Int16.compute_size(&self.error_code)?;

        Ok(total_size)
    }
}

impl Decodable for WritableTxnMarkerPartitionResult {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let partition_index = types::Int32.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        Ok(Self {
            partition_index,
            error_code,
        })
    }
}

impl Default for WritableTxnMarkerPartitionResult {
    fn default() -> Self {
        Self {
            partition_index: 0,
            error_code: 0,
        }
    }
}

impl Message for WritableTxnMarkerPartitionResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[derive(Debug, Clone)]
pub struct WritableTxnMarkerTopicResult {
    /// The topic name.
    /// 
    /// Supported API versions: 0
    pub name: super::TopicName,

    /// The results by partition.
    /// 
    /// Supported API versions: 0
    pub partitions: Vec<WritableTxnMarkerPartitionResult>,

}

impl Encodable for WritableTxnMarkerTopicResult {
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

impl Decodable for WritableTxnMarkerTopicResult {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::String.decode(buf)?;
        let partitions = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            name,
            partitions,
        })
    }
}

impl Default for WritableTxnMarkerTopicResult {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partitions: Default::default(),
        }
    }
}

impl Message for WritableTxnMarkerTopicResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[derive(Debug, Clone)]
pub struct WritableTxnMarkerResult {
    /// The current producer ID in use by the transactional ID.
    /// 
    /// Supported API versions: 0
    pub producer_id: super::ProducerId,

    /// The results by topic.
    /// 
    /// Supported API versions: 0
    pub topics: Vec<WritableTxnMarkerTopicResult>,

}

impl Encodable for WritableTxnMarkerResult {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int64.encode(buf, &self.producer_id)?;
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int64.compute_size(&self.producer_id)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;

        Ok(total_size)
    }
}

impl Decodable for WritableTxnMarkerResult {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let producer_id = types::Int64.decode(buf)?;
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            producer_id,
            topics,
        })
    }
}

impl Default for WritableTxnMarkerResult {
    fn default() -> Self {
        Self {
            producer_id: (0).into(),
            topics: Default::default(),
        }
    }
}

impl Message for WritableTxnMarkerResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[derive(Debug, Clone)]
pub struct WriteTxnMarkersResponse {
    /// The results for writing makers.
    /// 
    /// Supported API versions: 0
    pub markers: Vec<WritableTxnMarkerResult>,

}

impl Encodable for WriteTxnMarkersResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Array(types::Struct { version }).encode(buf, &self.markers)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Array(types::Struct { version }).compute_size(&self.markers)?;

        Ok(total_size)
    }
}

impl Decodable for WriteTxnMarkersResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let markers = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            markers,
        })
    }
}

impl Default for WriteTxnMarkersResponse {
    fn default() -> Self {
        Self {
            markers: Default::default(),
        }
    }
}

impl Message for WriteTxnMarkersResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

impl HeaderVersion for WriteTxnMarkersResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}

