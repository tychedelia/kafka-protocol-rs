//! AllocateProducerIdsRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/AllocateProducerIdsRequest.json).
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


/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct AllocateProducerIdsRequest {
    /// The ID of the requesting broker
    /// 
    /// Supported API versions: 0
    pub broker_id: super::BrokerId,

    /// The epoch of the requesting broker
    /// 
    /// Supported API versions: 0
    pub broker_epoch: i64,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for AllocateProducerIdsRequest {
    type Builder = AllocateProducerIdsRequestBuilder;

    fn builder() -> Self::Builder{
        AllocateProducerIdsRequestBuilder::default()
    }
}

impl Encodable for AllocateProducerIdsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.broker_id)?;
        types::Int64.encode(buf, &self.broker_epoch)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.broker_id)?;
        total_size += types::Int64.compute_size(&self.broker_epoch)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for AllocateProducerIdsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let broker_id = types::Int32.decode(buf)?;
        let broker_epoch = types::Int64.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            broker_id,
            broker_epoch,
            unknown_tagged_fields,
        })
    }
}

impl Default for AllocateProducerIdsRequest {
    fn default() -> Self {
        Self {
            broker_id: (0).into(),
            broker_epoch: -1,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AllocateProducerIdsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

impl HeaderVersion for AllocateProducerIdsRequest {
    fn header_version(version: i16) -> i16 {
        2
    }
}

