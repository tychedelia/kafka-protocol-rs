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
pub struct AlterableConfig {
    /// The value to set for the configuration key.
    /// 
    /// Supported API versions: 0-1
    pub value: Option<String>,

}

impl MapEncodable for AlterableConfig {
    type Key = String;
    fn encode<B: BufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, key)?;
        types::String.encode(buf, &self.value)?;

        Ok(())
    }
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(key)?;
        total_size += types::String.compute_size(&self.value)?;

        Ok(total_size)
    }
}

impl MapDecodable for AlterableConfig {
    type Key = String;
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = types::String.decode(buf)?;
        let value = types::String.decode(buf)?;
        Ok((key_field, Self {
            value,
        }))
    }
}

impl Default for AlterableConfig {
    fn default() -> Self {
        Self {
            value: Some(Default::default()),
        }
    }
}

impl Message for AlterableConfig {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct AlterConfigsResource {
    /// The resource type.
    /// 
    /// Supported API versions: 0-1
    pub resource_type: i8,

    /// The resource name.
    /// 
    /// Supported API versions: 0-1
    pub resource_name: String,

    /// The configurations.
    /// 
    /// Supported API versions: 0-1
    pub configs: indexmap::IndexMap<String, AlterableConfig>,

}

impl Encodable for AlterConfigsResource {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int8.encode(buf, &self.resource_type)?;
        types::String.encode(buf, &self.resource_name)?;
        types::Array(types::Struct { version }).encode(buf, &self.configs)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int8.compute_size(&self.resource_type)?;
        total_size += types::String.compute_size(&self.resource_name)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.configs)?;

        Ok(total_size)
    }
}

impl Decodable for AlterConfigsResource {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let resource_type = types::Int8.decode(buf)?;
        let resource_name = types::String.decode(buf)?;
        let configs = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            resource_type,
            resource_name,
            configs,
        })
    }
}

impl Default for AlterConfigsResource {
    fn default() -> Self {
        Self {
            resource_type: 0,
            resource_name: Default::default(),
            configs: Default::default(),
        }
    }
}

impl Message for AlterConfigsResource {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[derive(Debug, Clone)]
pub struct AlterConfigsRequest {
    /// The updates for each resource.
    /// 
    /// Supported API versions: 0-1
    pub resources: Vec<AlterConfigsResource>,

    /// True if we should validate the request, but not change the configurations.
    /// 
    /// Supported API versions: 0-1
    pub validate_only: bool,

}

impl Encodable for AlterConfigsRequest {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Array(types::Struct { version }).encode(buf, &self.resources)?;
        types::Boolean.encode(buf, &self.validate_only)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Array(types::Struct { version }).compute_size(&self.resources)?;
        total_size += types::Boolean.compute_size(&self.validate_only)?;

        Ok(total_size)
    }
}

impl Decodable for AlterConfigsRequest {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let resources = types::Array(types::Struct { version }).decode(buf)?;
        let validate_only = types::Boolean.decode(buf)?;
        Ok(Self {
            resources,
            validate_only,
        })
    }
}

impl Default for AlterConfigsRequest {
    fn default() -> Self {
        Self {
            resources: Default::default(),
            validate_only: false,
        }
    }
}

impl Message for AlterConfigsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for AlterConfigsRequest {
    fn header_version(version: i16) -> i16 {
        1
    }
}

