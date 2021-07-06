//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use protocol_base::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-3
#[derive(Debug, Clone, PartialEq)]
pub struct EndTxnRequest {
    /// The ID of the transaction to end.
    /// 
    /// Supported API versions: 0-3
    pub transactional_id: super::TransactionalId,

    /// The producer ID.
    /// 
    /// Supported API versions: 0-3
    pub producer_id: super::ProducerId,

    /// The current epoch associated with the producer.
    /// 
    /// Supported API versions: 0-3
    pub producer_epoch: i16,

    /// True if the transaction was committed, false if it was aborted.
    /// 
    /// Supported API versions: 0-3
    pub committed: bool,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for EndTxnRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 3 {
            types::CompactString.encode(buf, &self.transactional_id)?;
        } else {
            types::String.encode(buf, &self.transactional_id)?;
        }
        types::Int64.encode(buf, &self.producer_id)?;
        types::Int16.encode(buf, &self.producer_epoch)?;
        types::Boolean.encode(buf, &self.committed)?;
        if version == 3 {
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
        if version == 3 {
            total_size += types::CompactString.compute_size(&self.transactional_id)?;
        } else {
            total_size += types::String.compute_size(&self.transactional_id)?;
        }
        total_size += types::Int64.compute_size(&self.producer_id)?;
        total_size += types::Int16.compute_size(&self.producer_epoch)?;
        total_size += types::Boolean.compute_size(&self.committed)?;
        if version == 3 {
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

impl Decodable for EndTxnRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let transactional_id = if version == 3 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let producer_id = types::Int64.decode(buf)?;
        let producer_epoch = types::Int16.decode(buf)?;
        let committed = types::Boolean.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 3 {
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
            producer_id,
            producer_epoch,
            committed,
            unknown_tagged_fields,
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
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for EndTxnRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

impl HeaderVersion for EndTxnRequest {
    fn header_version(version: i16) -> i16 {
        if version == 3 {
            2
        } else {
            1
        }
    }
}

