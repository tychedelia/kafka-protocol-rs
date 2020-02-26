//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;

use franz_protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-2
#[derive(Debug, Clone)]
pub struct DescribeConfigsSynonym {
    /// The synonym name.
    /// 
    /// Supported API versions: 1-2
    pub name: StrBytes,

    /// The synonym value.
    /// 
    /// Supported API versions: 1-2
    pub value: Option<StrBytes>,

    /// The synonym source.
    /// 
    /// Supported API versions: 1-2
    pub source: i8,

}

impl Encodable for DescribeConfigsSynonym {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            types::String.encode(buf, &self.name)?;
        } else {
            if !self.name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            types::String.encode(buf, &self.value)?;
        } else {
            if !self.value.as_ref().map(|x| x.is_empty()).unwrap_or_default() {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            types::Int8.encode(buf, &self.source)?;
        } else {
            if self.source != 0 {
                return Err(EncodeError)
            }
        }

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 1 {
            total_size += types::String.compute_size(&self.name)?;
        } else {
            if !self.name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            total_size += types::String.compute_size(&self.value)?;
        } else {
            if !self.value.as_ref().map(|x| x.is_empty()).unwrap_or_default() {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            total_size += types::Int8.compute_size(&self.source)?;
        } else {
            if self.source != 0 {
                return Err(EncodeError)
            }
        }

        Ok(total_size)
    }
}

impl Decodable for DescribeConfigsSynonym {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = if version >= 1 {
            types::String.decode(buf)?
        } else {
            Default::default()
        };
        let value = if version >= 1 {
            types::String.decode(buf)?
        } else {
            Some(Default::default())
        };
        let source = if version >= 1 {
            types::Int8.decode(buf)?
        } else {
            0
        };
        Ok(Self {
            name,
            value,
            source,
        })
    }
}

impl Default for DescribeConfigsSynonym {
    fn default() -> Self {
        Self {
            name: Default::default(),
            value: Some(Default::default()),
            source: 0,
        }
    }
}

impl Message for DescribeConfigsSynonym {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

/// Valid versions: 0-2
#[derive(Debug, Clone)]
pub struct DescribeConfigsResourceResult {
    /// The configuration name.
    /// 
    /// Supported API versions: 0-2
    pub name: StrBytes,

    /// The configuration value.
    /// 
    /// Supported API versions: 0-2
    pub value: Option<StrBytes>,

    /// True if the configuration is read-only.
    /// 
    /// Supported API versions: 0-2
    pub read_only: bool,

    /// True if the configuration is not set.
    /// 
    /// Supported API versions: 0
    pub is_default: bool,

    /// The configuration source.
    /// 
    /// Supported API versions: 1-2
    pub config_source: i8,

    /// True if this configuration is sensitive.
    /// 
    /// Supported API versions: 0-2
    pub is_sensitive: bool,

    /// The synonyms for this configuration key.
    /// 
    /// Supported API versions: 1-2
    pub synonyms: Vec<DescribeConfigsSynonym>,

}

impl Encodable for DescribeConfigsResourceResult {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::String.encode(buf, &self.name)?;
        types::String.encode(buf, &self.value)?;
        types::Boolean.encode(buf, &self.read_only)?;
        if version == 0 {
            types::Boolean.encode(buf, &self.is_default)?;
        } else {
            if self.is_default {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            types::Int8.encode(buf, &self.config_source)?;
        }
        types::Boolean.encode(buf, &self.is_sensitive)?;
        if version >= 1 {
            types::Array(types::Struct { version }).encode(buf, &self.synonyms)?;
        }

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::String.compute_size(&self.name)?;
        total_size += types::String.compute_size(&self.value)?;
        total_size += types::Boolean.compute_size(&self.read_only)?;
        if version == 0 {
            total_size += types::Boolean.compute_size(&self.is_default)?;
        } else {
            if self.is_default {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            total_size += types::Int8.compute_size(&self.config_source)?;
        }
        total_size += types::Boolean.compute_size(&self.is_sensitive)?;
        if version >= 1 {
            total_size += types::Array(types::Struct { version }).compute_size(&self.synonyms)?;
        }

        Ok(total_size)
    }
}

impl Decodable for DescribeConfigsResourceResult {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = types::String.decode(buf)?;
        let value = types::String.decode(buf)?;
        let read_only = types::Boolean.decode(buf)?;
        let is_default = if version == 0 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let config_source = if version >= 1 {
            types::Int8.decode(buf)?
        } else {
            -1
        };
        let is_sensitive = types::Boolean.decode(buf)?;
        let synonyms = if version >= 1 {
            types::Array(types::Struct { version }).decode(buf)?
        } else {
            Default::default()
        };
        Ok(Self {
            name,
            value,
            read_only,
            is_default,
            config_source,
            is_sensitive,
            synonyms,
        })
    }
}

impl Default for DescribeConfigsResourceResult {
    fn default() -> Self {
        Self {
            name: Default::default(),
            value: Some(Default::default()),
            read_only: false,
            is_default: false,
            config_source: -1,
            is_sensitive: false,
            synonyms: Default::default(),
        }
    }
}

impl Message for DescribeConfigsResourceResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

/// Valid versions: 0-2
#[derive(Debug, Clone)]
pub struct DescribeConfigsResult {
    /// The error code, or 0 if we were able to successfully describe the configurations.
    /// 
    /// Supported API versions: 0-2
    pub error_code: i16,

    /// The error message, or null if we were able to successfully describe the configurations.
    /// 
    /// Supported API versions: 0-2
    pub error_message: Option<StrBytes>,

    /// The resource type.
    /// 
    /// Supported API versions: 0-2
    pub resource_type: i8,

    /// The resource name.
    /// 
    /// Supported API versions: 0-2
    pub resource_name: StrBytes,

    /// Each listed configuration.
    /// 
    /// Supported API versions: 0-2
    pub configs: Vec<DescribeConfigsResourceResult>,

}

impl Encodable for DescribeConfigsResult {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        types::String.encode(buf, &self.error_message)?;
        types::Int8.encode(buf, &self.resource_type)?;
        types::String.encode(buf, &self.resource_name)?;
        types::Array(types::Struct { version }).encode(buf, &self.configs)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::String.compute_size(&self.error_message)?;
        total_size += types::Int8.compute_size(&self.resource_type)?;
        total_size += types::String.compute_size(&self.resource_name)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.configs)?;

        Ok(total_size)
    }
}

impl Decodable for DescribeConfigsResult {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let error_message = types::String.decode(buf)?;
        let resource_type = types::Int8.decode(buf)?;
        let resource_name = types::String.decode(buf)?;
        let configs = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            error_code,
            error_message,
            resource_type,
            resource_name,
            configs,
        })
    }
}

impl Default for DescribeConfigsResult {
    fn default() -> Self {
        Self {
            error_code: 0,
            error_message: Some(Default::default()),
            resource_type: 0,
            resource_name: Default::default(),
            configs: Default::default(),
        }
    }
}

impl Message for DescribeConfigsResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

/// Valid versions: 0-2
#[derive(Debug, Clone)]
pub struct DescribeConfigsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 0-2
    pub throttle_time_ms: i32,

    /// The results for each resource.
    /// 
    /// Supported API versions: 0-2
    pub results: Vec<DescribeConfigsResult>,

}

impl Encodable for DescribeConfigsResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.throttle_time_ms)?;
        types::Array(types::Struct { version }).encode(buf, &self.results)?;

        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        total_size += types::Array(types::Struct { version }).compute_size(&self.results)?;

        Ok(total_size)
    }
}

impl Decodable for DescribeConfigsResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = types::Int32.decode(buf)?;
        let results = types::Array(types::Struct { version }).decode(buf)?;
        Ok(Self {
            throttle_time_ms,
            results,
        })
    }
}

impl Default for DescribeConfigsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            results: Default::default(),
        }
    }
}

impl Message for DescribeConfigsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

impl HeaderVersion for DescribeConfigsResponse {
    fn header_version(version: i16) -> i16 {
        0
    }
}

