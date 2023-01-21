//! BrokerHeartbeatRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/BrokerHeartbeatRequest.json).
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
pub struct BrokerHeartbeatRequest {
    /// The broker ID.
    /// 
    /// Supported API versions: 0
    pub broker_id: super::BrokerId,

    /// The broker epoch.
    /// 
    /// Supported API versions: 0
    pub broker_epoch: i64,

    /// The highest metadata offset which the broker has reached.
    /// 
    /// Supported API versions: 0
    pub current_metadata_offset: i64,

    /// True if the broker wants to be fenced, false otherwise.
    /// 
    /// Supported API versions: 0
    pub want_fence: bool,

    /// True if the broker wants to be shut down, false otherwise.
    /// 
    /// Supported API versions: 0
    pub want_shut_down: bool,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for BrokerHeartbeatRequest {
    type Builder = BrokerHeartbeatRequestBuilder;

    fn builder() -> Self::Builder{
        BrokerHeartbeatRequestBuilder::default()
    }
}

impl Encodable for BrokerHeartbeatRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.broker_id)?;
        types::Int64.encode(buf, &self.broker_epoch)?;
        types::Int64.encode(buf, &self.current_metadata_offset)?;
        types::Boolean.encode(buf, &self.want_fence)?;
        types::Boolean.encode(buf, &self.want_shut_down)?;
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
        total_size += types::Int64.compute_size(&self.current_metadata_offset)?;
        total_size += types::Boolean.compute_size(&self.want_fence)?;
        total_size += types::Boolean.compute_size(&self.want_shut_down)?;
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

impl Decodable for BrokerHeartbeatRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let broker_id = types::Int32.decode(buf)?;
        let broker_epoch = types::Int64.decode(buf)?;
        let current_metadata_offset = types::Int64.decode(buf)?;
        let want_fence = types::Boolean.decode(buf)?;
        let want_shut_down = types::Boolean.decode(buf)?;
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
            broker_id,
            broker_epoch,
            current_metadata_offset,
            want_fence,
            want_shut_down,
            unknown_tagged_fields,
        })
    }
}

impl Default for BrokerHeartbeatRequest {
    fn default() -> Self {
        Self {
            broker_id: (0).into(),
            broker_epoch: -1,
            current_metadata_offset: 0,
            want_fence: false,
            want_shut_down: false,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for BrokerHeartbeatRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

impl HeaderVersion for BrokerHeartbeatRequest {
    fn header_version(version: i16) -> i16 {
        2
    }
}

