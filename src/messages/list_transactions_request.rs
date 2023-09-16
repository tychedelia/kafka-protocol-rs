//! ListTransactionsRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ListTransactionsRequest.json).
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
pub struct ListTransactionsRequest {
    /// The transaction states to filter by: if empty, all transactions are returned; if non-empty, then only transactions matching one of the filtered states will be returned
    /// 
    /// Supported API versions: 0
    pub state_filters: Vec<StrBytes>,

    /// The producerIds to filter by: if empty, all transactions will be returned; if non-empty, only transactions which match one of the filtered producerIds will be returned
    /// 
    /// Supported API versions: 0
    pub producer_id_filters: Vec<super::ProducerId>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for ListTransactionsRequest {
    type Builder = ListTransactionsRequestBuilder;

    fn builder() -> Self::Builder{
        ListTransactionsRequestBuilder::default()
    }
}

impl Encodable for ListTransactionsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::CompactArray(types::CompactString).encode(buf, &self.state_filters)?;
        types::CompactArray(types::Int64).encode(buf, &self.producer_id_filters)?;
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
        total_size += types::CompactArray(types::CompactString).compute_size(&self.state_filters)?;
        total_size += types::CompactArray(types::Int64).compute_size(&self.producer_id_filters)?;
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

impl Decodable for ListTransactionsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let state_filters = types::CompactArray(types::CompactString).decode(buf)?;
        let producer_id_filters = types::CompactArray(types::Int64).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let mut unknown_value = vec![0; size as usize];
            buf.try_copy_to_slice(&mut unknown_value)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            state_filters,
            producer_id_filters,
            unknown_tagged_fields,
        })
    }
}

impl Default for ListTransactionsRequest {
    fn default() -> Self {
        Self {
            state_filters: Default::default(),
            producer_id_filters: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ListTransactionsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

impl HeaderVersion for ListTransactionsRequest {
    fn header_version(version: i16) -> i16 {
        2
    }
}

