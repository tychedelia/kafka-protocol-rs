//! ListTransactionsResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ListTransactionsResponse.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use anyhow::bail;
use bytes::Bytes;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Builder, Decodable,
    DecodeError, Decoder, Encodable, EncodeError, Encoder, HeaderVersion, MapDecodable,
    MapEncodable, Message, StrBytes, VersionRange,
};

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct ListTransactionsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    ///
    /// Supported API versions: 0
    pub throttle_time_ms: i32,

    ///
    ///
    /// Supported API versions: 0
    pub error_code: i16,

    /// Set of state filters provided in the request which were unknown to the transaction coordinator
    ///
    /// Supported API versions: 0
    pub unknown_state_filters: Vec<StrBytes>,

    ///
    ///
    /// Supported API versions: 0
    pub transaction_states: Vec<TransactionState>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for ListTransactionsResponse {
    type Builder = ListTransactionsResponseBuilder;

    fn builder() -> Self::Builder {
        ListTransactionsResponseBuilder::default()
    }
}

impl Encodable for ListTransactionsResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.throttle_time_ms)?;
        types::Int16.encode(buf, &self.error_code)?;
        types::CompactArray(types::CompactString).encode(buf, &self.unknown_state_filters)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.transaction_states)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size +=
            types::CompactArray(types::CompactString).compute_size(&self.unknown_state_filters)?;
        total_size += types::CompactArray(types::Struct { version })
            .compute_size(&self.transaction_states)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for ListTransactionsResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = types::Int32.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        let unknown_state_filters = types::CompactArray(types::CompactString).decode(buf)?;
        let transaction_states = types::CompactArray(types::Struct { version }).decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            throttle_time_ms,
            error_code,
            unknown_state_filters,
            transaction_states,
            unknown_tagged_fields,
        })
    }
}

impl Default for ListTransactionsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            unknown_state_filters: Default::default(),
            transaction_states: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ListTransactionsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct TransactionState {
    ///
    ///
    /// Supported API versions: 0
    pub transactional_id: super::TransactionalId,

    ///
    ///
    /// Supported API versions: 0
    pub producer_id: super::ProducerId,

    /// The current transaction state of the producer
    ///
    /// Supported API versions: 0
    pub transaction_state: StrBytes,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for TransactionState {
    type Builder = TransactionStateBuilder;

    fn builder() -> Self::Builder {
        TransactionStateBuilder::default()
    }
}

impl Encodable for TransactionState {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::CompactString.encode(buf, &self.transactional_id)?;
        types::Int64.encode(buf, &self.producer_id)?;
        types::CompactString.encode(buf, &self.transaction_state)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::CompactString.compute_size(&self.transactional_id)?;
        total_size += types::Int64.compute_size(&self.producer_id)?;
        total_size += types::CompactString.compute_size(&self.transaction_state)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!(
                "Too many tagged fields to encode ({} fields)",
                num_tagged_fields
            );
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for TransactionState {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let transactional_id = types::CompactString.decode(buf)?;
        let producer_id = types::Int64.decode(buf)?;
        let transaction_state = types::CompactString.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok(Self {
            transactional_id,
            producer_id,
            transaction_state,
            unknown_tagged_fields,
        })
    }
}

impl Default for TransactionState {
    fn default() -> Self {
        Self {
            transactional_id: Default::default(),
            producer_id: (0).into(),
            transaction_state: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for TransactionState {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for ListTransactionsResponse {
    fn header_version(version: i16) -> i16 {
        1
    }
}
