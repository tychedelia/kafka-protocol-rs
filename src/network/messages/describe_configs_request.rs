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


/// Valid versions: 0-2
#[derive(Debug, Clone)]
pub struct DescribeConfigsResource {
    /// The resource type.
    /// 
    /// Supported API versions: 0-2
    pub resource_type: i8,

    /// The resource name.
    /// 
    /// Supported API versions: 0-2
    pub resource_name: StrBytes,

    /// The configuration keys to list, or null to list all configuration keys.
    /// 
    /// Supported API versions: 0-2
    pub configuration_keys: Option<Vec<StrBytes>>,

}

impl Encodable for DescribeConfigsResource {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int8.encode(buf, &self.resource_type)?;
        types::String.encode(buf, &self.resource_name)?;
        types::Array(types::String).encode(buf, &self.configuration_keys)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int8.compute_size(&self.resource_type)?;
        total_size += types::String.compute_size(&self.resource_name)?;
        total_size += types::Array(types::String).compute_size(&self.configuration_keys)?;

        Ok(total_size)
    }
}

impl Decodable for DescribeConfigsResource {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let resource_type = types::Int8.decode(buf)?;
        let resource_name = types::String.decode(buf)?;
        let configuration_keys = types::Array(types::String).decode(buf)?;
        Ok(Self {
            resource_type,
            resource_name,
            configuration_keys,
        })
    }
}

impl Default for DescribeConfigsResource {
    fn default() -> Self {
        Self {
            resource_type: 0,
            resource_name: Default::default(),
            configuration_keys: Some(Default::default()),
        }
    }
}

impl Message for DescribeConfigsResource {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

/// Valid versions: 0-2
#[derive(Debug, Clone)]
pub struct DescribeConfigsRequest {
    /// The resources whose configurations we want to describe.
    /// 
    /// Supported API versions: 0-2
    pub resources: Vec<DescribeConfigsResource>,

    /// True if we should include all synonyms.
    /// 
    /// Supported API versions: 1-2
    pub include_synoyms: bool,

}

impl Encodable for DescribeConfigsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Array(types::Struct { version }).encode(buf, &self.resources)?;
        if version >= 1 {
            types::Boolean.encode(buf, &self.include_synoyms)?;
        } else {
            if self.include_synoyms {
                return Err(EncodeError)
            }
        }

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Array(types::Struct { version }).compute_size(&self.resources)?;
        if version >= 1 {
            total_size += types::Boolean.compute_size(&self.include_synoyms)?;
        } else {
            if self.include_synoyms {
                return Err(EncodeError)
            }
        }

        Ok(total_size)
    }
}

impl Decodable for DescribeConfigsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let resources = types::Array(types::Struct { version }).decode(buf)?;
        let include_synoyms = if version >= 1 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        Ok(Self {
            resources,
            include_synoyms,
        })
    }
}

impl Default for DescribeConfigsRequest {
    fn default() -> Self {
        Self {
            resources: Default::default(),
            include_synoyms: false,
        }
    }
}

impl Message for DescribeConfigsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

impl HeaderVersion for DescribeConfigsRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}

