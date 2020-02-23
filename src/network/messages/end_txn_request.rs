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
pub struct EndTxnRequest {
    /// The ID of the transaction to end.
    /// 
    /// Supported API versions: 0-1
    pub transactional_id: super::TransactionalId,

    /// The producer ID.
    /// 
    /// Supported API versions: 0-1
    pub producer_id: super::ProducerId,

    /// The current epoch associated with the producer.
    /// 
    /// Supported API versions: 0-1
    pub producer_epoch: i16,

    /// True if the transaction was committed, false if it was aborted.
    /// 
    /// Supported API versions: 0-1
    pub committed: bool,

}

impl Encodable for EndTxnRequest {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, &self.transactional_id)?;
        types::Int64.encode(buf, &self.producer_id)?;
        types::Int16.encode(buf, &self.producer_epoch)?;
        types::Boolean.encode(buf, &self.committed)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.transactional_id)?;
        total_size += types::Int64.compute_size(&self.producer_id)?;
        total_size += types::Int16.compute_size(&self.producer_epoch)?;
        total_size += types::Boolean.compute_size(&self.committed)?;

        Ok(total_size)
    }
}

impl Decodable for EndTxnRequest {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let transactional_id = types::String.decode(buf)?;
        let producer_id = types::Int64.decode(buf)?;
        let producer_epoch = types::Int16.decode(buf)?;
        let committed = types::Boolean.decode(buf)?;
        Ok(Self {
            transactional_id,
            producer_id,
            producer_epoch,
            committed,
        })
    }
}

impl Default for EndTxnRequest {
    fn default() -> Self {
        Self {
            transactional_id: Default::default(),
            producer_id: (0).into(),
            producer_epoch: 0,
            committed: false,
        }
    }
}

impl Message for EndTxnRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for EndTxnRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}

