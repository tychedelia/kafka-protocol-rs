//! DescribeClusterResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/DescribeClusterResponse.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use uuid::Uuid;
use anyhow::bail;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}, Builder
};


/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct DescribeClusterBroker {
    /// The broker hostname.
    /// 
    /// Supported API versions: 0
    pub host: StrBytes,

    /// The broker port.
    /// 
    /// Supported API versions: 0
    pub port: i32,

    /// The rack of the broker, or null if it has not been assigned to a rack.
    /// 
    /// Supported API versions: 0
    pub rack: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for DescribeClusterBroker {
    type Builder = DescribeClusterBrokerBuilder;

    fn builder() -> Self::Builder{
        DescribeClusterBrokerBuilder::default()
    }
}

impl MapEncodable for DescribeClusterBroker {
    type Key = super::BrokerId;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, key)?;
        types::CompactString.encode(buf, &self.host)?;
        types::Int32.encode(buf, &self.port)?;
        types::CompactString.encode(buf, &self.rack)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(key)?;
        total_size += types::CompactString.compute_size(&self.host)?;
        total_size += types::Int32.compute_size(&self.port)?;
        total_size += types::CompactString.compute_size(&self.rack)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl MapDecodable for DescribeClusterBroker {
    type Key = super::BrokerId;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::Int32.decode(buf)?;
        let host = types::CompactString.decode(buf)?;
        let port = types::Int32.decode(buf)?;
        let rack = types::CompactString.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let unknown_value = buf.try_get_bytes(size as usize)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok((key_field, Self {
            host,
            port,
            rack,
            unknown_tagged_fields,
        }))
    }
}

impl Default for DescribeClusterBroker {
    fn default() -> Self {
        Self {
            host: Default::default(),
            port: 0,
            rack: None,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribeClusterBroker {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

/// Valid versions: 0
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct DescribeClusterResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 0
    pub throttle_time_ms: i32,

    /// The top-level error code, or 0 if there was no error
    /// 
    /// Supported API versions: 0
    pub error_code: i16,

    /// The top-level error message, or null if there was no error.
    /// 
    /// Supported API versions: 0
    pub error_message: Option<StrBytes>,

    /// The cluster ID that responding broker belongs to.
    /// 
    /// Supported API versions: 0
    pub cluster_id: StrBytes,

    /// The ID of the controller broker.
    /// 
    /// Supported API versions: 0
    pub controller_id: super::BrokerId,

    /// Each broker in the response.
    /// 
    /// Supported API versions: 0
    pub brokers: indexmap::IndexMap<super::BrokerId, DescribeClusterBroker>,

    /// 32-bit bitfield to represent authorized operations for this cluster.
    /// 
    /// Supported API versions: 0
    pub cluster_authorized_operations: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for DescribeClusterResponse {
    type Builder = DescribeClusterResponseBuilder;

    fn builder() -> Self::Builder{
        DescribeClusterResponseBuilder::default()
    }
}

impl Encodable for DescribeClusterResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.throttle_time_ms)?;
        types::Int16.encode(buf, &self.error_code)?;
        types::CompactString.encode(buf, &self.error_message)?;
        types::CompactString.encode(buf, &self.cluster_id)?;
        types::Int32.encode(buf, &self.controller_id)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.brokers)?;
        types::Int32.encode(buf, &self.cluster_authorized_operations)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::CompactString.compute_size(&self.error_message)?;
        total_size += types::CompactString.compute_size(&self.cluster_id)?;
        total_size += types::Int32.compute_size(&self.controller_id)?;
        total_size += types::CompactArray(types::Struct { version }).compute_size(&self.brokers)?;
        total_size += types::Int32.compute_size(&self.cluster_authorized_operations)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
        }
        total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

        total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        Ok(total_size)
    }
}

impl Decodable for DescribeClusterResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = types::Int32.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        let error_message = types::CompactString.decode(buf)?;
        let cluster_id = types::CompactString.decode(buf)?;
        let controller_id = types::Int32.decode(buf)?;
        let brokers = types::CompactArray(types::Struct { version }).decode(buf)?;
        let cluster_authorized_operations = types::Int32.decode(buf)?;
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
            error_message,
            cluster_id,
            controller_id,
            brokers,
            cluster_authorized_operations,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribeClusterResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            error_message: None,
            cluster_id: Default::default(),
            controller_id: (-1).into(),
            brokers: Default::default(),
            cluster_authorized_operations: -2147483648,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribeClusterResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 0 };
}

impl HeaderVersion for DescribeClusterResponse {
    fn header_version(version: i16) -> i16 {
        1
    }
}

