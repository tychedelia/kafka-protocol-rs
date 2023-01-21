//! BrokerRegistrationRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/BrokerRegistrationRequest.json).
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


/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
pub struct Listener {
    /// The hostname.
    /// 
    /// Supported API versions: 0-1
    pub host: StrBytes,

    /// The port.
    /// 
    /// Supported API versions: 0-1
    pub port: u16,

    /// The security protocol.
    /// 
    /// Supported API versions: 0-1
    pub security_protocol: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for Listener {
    type Builder = ListenerBuilder;

    fn builder() -> Self::Builder{
        ListenerBuilder::default()
    }
}

impl MapEncodable for Listener {
    type Key = StrBytes;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::CompactString.encode(buf, key)?;
        types::CompactString.encode(buf, &self.host)?;
        types::UInt16.encode(buf, &self.port)?;
        types::Int16.encode(buf, &self.security_protocol)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::CompactString.compute_size(key)?;
        total_size += types::CompactString.compute_size(&self.host)?;
        total_size += types::UInt16.compute_size(&self.port)?;
        total_size += types::Int16.compute_size(&self.security_protocol)?;
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

impl MapDecodable for Listener {
    type Key = StrBytes;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::CompactString.decode(buf)?;
        let host = types::CompactString.decode(buf)?;
        let port = types::UInt16.decode(buf)?;
        let security_protocol = types::Int16.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let mut unknown_value = vec![0; size as usize];
            buf.try_copy_to_slice(&mut unknown_value)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok((key_field, Self {
            host,
            port,
            security_protocol,
            unknown_tagged_fields,
        }))
    }
}

impl Default for Listener {
    fn default() -> Self {
        Self {
            host: Default::default(),
            port: 0,
            security_protocol: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for Listener {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
pub struct Feature {
    /// The minimum supported feature level.
    /// 
    /// Supported API versions: 0-1
    pub min_supported_version: i16,

    /// The maximum supported feature level.
    /// 
    /// Supported API versions: 0-1
    pub max_supported_version: i16,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for Feature {
    type Builder = FeatureBuilder;

    fn builder() -> Self::Builder{
        FeatureBuilder::default()
    }
}

impl MapEncodable for Feature {
    type Key = StrBytes;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::CompactString.encode(buf, key)?;
        types::Int16.encode(buf, &self.min_supported_version)?;
        types::Int16.encode(buf, &self.max_supported_version)?;
        let num_tagged_fields = self.unknown_tagged_fields.len();
        if num_tagged_fields > std::u32::MAX as usize {
            error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            return Err(EncodeError);
        }
        types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

        write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::CompactString.compute_size(key)?;
        total_size += types::Int16.compute_size(&self.min_supported_version)?;
        total_size += types::Int16.compute_size(&self.max_supported_version)?;
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

impl MapDecodable for Feature {
    type Key = StrBytes;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::CompactString.decode(buf)?;
        let min_supported_version = types::Int16.decode(buf)?;
        let max_supported_version = types::Int16.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
        for _ in 0..num_tagged_fields {
            let tag: u32 = types::UnsignedVarInt.decode(buf)?;
            let size: u32 = types::UnsignedVarInt.decode(buf)?;
            let mut unknown_value = vec![0; size as usize];
            buf.try_copy_to_slice(&mut unknown_value)?;
            unknown_tagged_fields.insert(tag as i32, unknown_value);
        }
        Ok((key_field, Self {
            min_supported_version,
            max_supported_version,
            unknown_tagged_fields,
        }))
    }
}

impl Default for Feature {
    fn default() -> Self {
        Self {
            min_supported_version: 0,
            max_supported_version: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for Feature {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
pub struct BrokerRegistrationRequest {
    /// The broker ID.
    /// 
    /// Supported API versions: 0-1
    pub broker_id: super::BrokerId,

    /// The cluster id of the broker process.
    /// 
    /// Supported API versions: 0-1
    pub cluster_id: StrBytes,

    /// The incarnation id of the broker process.
    /// 
    /// Supported API versions: 0-1
    pub incarnation_id: Uuid,

    /// The listeners of this broker
    /// 
    /// Supported API versions: 0-1
    pub listeners: indexmap::IndexMap<StrBytes, Listener>,

    /// The features on this broker
    /// 
    /// Supported API versions: 0-1
    pub features: indexmap::IndexMap<StrBytes, Feature>,

    /// The rack which this broker is in.
    /// 
    /// Supported API versions: 0-1
    pub rack: Option<StrBytes>,

    /// If the required configurations for ZK migration are present, this value is set to true
    /// 
    /// Supported API versions: 1
    pub is_migrating_zk_broker: bool,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for BrokerRegistrationRequest {
    type Builder = BrokerRegistrationRequestBuilder;

    fn builder() -> Self::Builder{
        BrokerRegistrationRequestBuilder::default()
    }
}

impl Encodable for BrokerRegistrationRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.broker_id)?;
        types::CompactString.encode(buf, &self.cluster_id)?;
        types::Uuid.encode(buf, &self.incarnation_id)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.listeners)?;
        types::CompactArray(types::Struct { version }).encode(buf, &self.features)?;
        types::CompactString.encode(buf, &self.rack)?;
        if version >= 1 {
            types::Boolean.encode(buf, &self.is_migrating_zk_broker)?;
        } else {
            if self.is_migrating_zk_broker {
                return Err(EncodeError)
            }
        }
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
        total_size += types::CompactString.compute_size(&self.cluster_id)?;
        total_size += types::Uuid.compute_size(&self.incarnation_id)?;
        total_size += types::CompactArray(types::Struct { version }).compute_size(&self.listeners)?;
        total_size += types::CompactArray(types::Struct { version }).compute_size(&self.features)?;
        total_size += types::CompactString.compute_size(&self.rack)?;
        if version >= 1 {
            total_size += types::Boolean.compute_size(&self.is_migrating_zk_broker)?;
        } else {
            if self.is_migrating_zk_broker {
                return Err(EncodeError)
            }
        }
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

impl Decodable for BrokerRegistrationRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let broker_id = types::Int32.decode(buf)?;
        let cluster_id = types::CompactString.decode(buf)?;
        let incarnation_id = types::Uuid.decode(buf)?;
        let listeners = types::CompactArray(types::Struct { version }).decode(buf)?;
        let features = types::CompactArray(types::Struct { version }).decode(buf)?;
        let rack = types::CompactString.decode(buf)?;
        let is_migrating_zk_broker = if version >= 1 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
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
            cluster_id,
            incarnation_id,
            listeners,
            features,
            rack,
            is_migrating_zk_broker,
            unknown_tagged_fields,
        })
    }
}

impl Default for BrokerRegistrationRequest {
    fn default() -> Self {
        Self {
            broker_id: (0).into(),
            cluster_id: Default::default(),
            incarnation_id: Uuid::nil(),
            listeners: Default::default(),
            features: Default::default(),
            rack: Some(Default::default()),
            is_migrating_zk_broker: false,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for BrokerRegistrationRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for BrokerRegistrationRequest {
    fn header_version(version: i16) -> i16 {
        2
    }
}

