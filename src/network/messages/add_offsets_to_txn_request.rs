//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::{Buf, BufMut};
use log::error;

use franz_protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size,
};


/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct AddOffsetsToTxnRequest {
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

    /// The unique group identifier.
    /// 
    /// Supported API versions: 0-1
    pub group_id: super::GroupId,

}

impl Encodable for AddOffsetsToTxnRequest {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, &self.transactional_id)?;
        types::Int64.encode(buf, &self.producer_id)?;
        types::Int16.encode(buf, &self.producer_epoch)?;
        types::String.encode(buf, &self.group_id)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.transactional_id)?;
        total_size += types::Int64.compute_size(&self.producer_id)?;
        total_size += types::Int16.compute_size(&self.producer_epoch)?;
        total_size += types::String.compute_size(&self.group_id)?;

        Ok(total_size)
    }
}

impl Decodable for AddOffsetsToTxnRequest {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let transactional_id = types::String.decode(buf)?;
        let producer_id = types::Int64.decode(buf)?;
        let producer_epoch = types::Int16.decode(buf)?;
        let group_id = types::String.decode(buf)?;
        Ok(Self {
            transactional_id,
            producer_id,
            producer_epoch,
            group_id,
        })
    }
}

impl Default for AddOffsetsToTxnRequest {
    fn default() -> Self {
        Self {
            transactional_id: Default::default(),
            producer_id: (0).into(),
            producer_epoch: 0,
            group_id: Default::default(),
        }
    }
}

impl Message for AddOffsetsToTxnRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for AddOffsetsToTxnRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}

