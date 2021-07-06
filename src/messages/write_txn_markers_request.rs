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
pub struct WritableTxnMarkerTopic {
    /// The topic name.
    /// 
    /// Supported API versions: 0
    pub name: super::TopicName,

    /// The indexes of the partitions to write transaction markers for.
    /// 
    /// Supported API versions: 0
    pub partition_indexes: Vec<i32>,

}

impl Encodable for WritableTxnMarkerTopic {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, &self.name)?;
        types::Array(types::Int32).encode(buf, &self.partition_indexes)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.name)?;
        total_size += types::Array(types::Int32).compute_size(&self.partition_indexes)?;

        Ok(total_size)
    }
}

impl Decodable for WritableTxnMarkerTopic {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::String.decode(buf)?;
        let partition_indexes = types::Array(types::Int32).decode(buf)?;
        Ok(Self {
            name,
            partition_indexes,
        })
    }
}

impl Default for WritableTxnMarkerTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            partition_indexes: Default::default(),
        }
    }
}

impl Message for WritableTxnMarkerTopic {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[derive(Debug, Clone)]
pub struct WritableTxnMarker {
    /// The current producer ID.
    /// 
    /// Supported API versions: 0
    pub producer_id: super::ProducerId,

    /// The current epoch associated with the producer ID.
    /// 
    /// Supported API versions: 0
    pub producer_epoch: i16,

    /// The result of the transaction to write to the partitions (false = ABORT, true = COMMIT).
    /// 
    /// Supported API versions: 0
    pub transaction_result: bool,

    /// Each topic that we want to write transaction marker(s) for.
    /// 
    /// Supported API versions: 0
    pub topics: Vec<WritableTxnMarkerTopic>,

    /// Epoch associated with the transaction state partition hosted by this transaction coordinator
    /// 
    /// Supported API versions: 0
    pub coordinator_epoch: i32,

}

impl Encodable for WritableTxnMarker {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int64.encode(buf, &self.producer_id)?;
        types::Int16.encode(buf, &self.producer_epoch)?;
        types::Boolean.encode(buf, &self.transaction_result)?;
        types::Array(types::Struct { version }).encode(buf, &self.topics)?;
        types::Int32.encode(buf, &self.coordinator_epoch)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int64.compute_size(&self.producer_id)?;
        total_size += types::Int16.compute_size(&self.producer_epoch)?;
        total_size += types::Boolean.compute_size(&self.transaction_result)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.topics)?;
        total_size += types::Int32.compute_size(&self.coordinator_epoch)?;

        Ok(total_size)
    }
}

impl Decodable for WritableTxnMarker {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let producer_id = types::Int64.decode(buf)?;
        let producer_epoch = types::Int16.decode(buf)?;
        let transaction_result = types::Boolean.decode(buf)?;
        let topics = types::Array(types::Struct { version }).decode(buf)?;
        let coordinator_epoch = types::Int32.decode(buf)?;
        Ok(Self {
            producer_id,
            producer_epoch,
            transaction_result,
            topics,
            coordinator_epoch,
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
        }
    }
}

impl Message for WritableTxnMarker {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[derive(Debug, Clone)]
pub struct WriteTxnMarkersRequest {
    /// The transaction markers to be written.
    /// 
    /// Supported API versions: 0
    pub markers: Vec<WritableTxnMarker>,

}

impl Encodable for WriteTxnMarkersRequest {
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

impl Decodable for WriteTxnMarkersRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let markers = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            markers,
        })
    }
}

impl Default for WriteTxnMarkersRequest {
    fn default() -> Self {
        Self {
            markers: Default::default(),
        }
    }
}

impl Message for WriteTxnMarkersRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

impl HeaderVersion for WriteTxnMarkersRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}

