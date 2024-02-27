//! DescribeConfigsResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/DescribeConfigsResponse.json).
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


/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct DescribeConfigsSynonym {
    /// The synonym name.
    /// 
    /// Supported API versions: 1-4
    pub name: StrBytes,

    /// The synonym value.
    /// 
    /// Supported API versions: 1-4
    pub value: Option<StrBytes>,

    /// The synonym source.
    /// 
    /// Supported API versions: 1-4
    pub source: i8,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for DescribeConfigsSynonym {
    type Builder = DescribeConfigsSynonymBuilder;

    fn builder() -> Self::Builder{
        DescribeConfigsSynonymBuilder::default()
    }
}

impl Encodable for DescribeConfigsSynonym {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            if version >= 4 {
                types::CompactString.encode(buf, &self.name)?;
            } else {
                types::String.encode(buf, &self.name)?;
            }
        } else {
            if !self.name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            if version >= 4 {
                types::CompactString.encode(buf, &self.value)?;
            } else {
                types::String.encode(buf, &self.value)?;
            }
        } else {
            if !self.value.as_ref().map(|x| x.is_empty()).unwrap_or_default() {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            buf.put_i8(self.source);
        } else {
            if self.source != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt::put_u32(buf, num_tagged_fields as u32);

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 1 {
            if version >= 4 {
                total_size += types::CompactString.compute_size(&self.name)?;
            } else {
                total_size += types::String.compute_size(&self.name)?;
            }
        } else {
            if !self.name.is_empty() {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            if version >= 4 {
                total_size += types::CompactString.compute_size(&self.value)?;
            } else {
                total_size += types::String.compute_size(&self.value)?;
            }
        } else {
            if !self.value.as_ref().map(|x| x.is_empty()).unwrap_or_default() {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            total_size += 1;
        } else {
            if self.source != 0 {
                return Err(EncodeError)
            }
        }
        if version >= 4 {
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

impl Decodable for DescribeConfigsSynonym {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = if version >= 1 {
            if version >= 4 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Default::default()
        };
        let value = if version >= 1 {
            if version >= 4 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Some(Default::default())
        };
        let source = if version >= 1 {
            buf.try_get_i8()?
        } else {
            0
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            name,
            value,
            source,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribeConfigsSynonym {
    fn default() -> Self {
        Self {
            name: Default::default(),
            value: Some(Default::default()),
            source: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribeConfigsSynonym {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct DescribeConfigsResourceResult {
    /// The configuration name.
    /// 
    /// Supported API versions: 0-4
    pub name: StrBytes,

    /// The configuration value.
    /// 
    /// Supported API versions: 0-4
    pub value: Option<StrBytes>,

    /// True if the configuration is read-only.
    /// 
    /// Supported API versions: 0-4
    pub read_only: bool,

    /// True if the configuration is not set.
    /// 
    /// Supported API versions: 0
    pub is_default: bool,

    /// The configuration source.
    /// 
    /// Supported API versions: 1-4
    pub config_source: i8,

    /// True if this configuration is sensitive.
    /// 
    /// Supported API versions: 0-4
    pub is_sensitive: bool,

    /// The synonyms for this configuration key.
    /// 
    /// Supported API versions: 1-4
    pub synonyms: Vec<DescribeConfigsSynonym>,

    /// The configuration data type. Type can be one of the following values - BOOLEAN, STRING, INT, SHORT, LONG, DOUBLE, LIST, CLASS, PASSWORD
    /// 
    /// Supported API versions: 3-4
    pub config_type: i8,

    /// The configuration documentation.
    /// 
    /// Supported API versions: 3-4
    pub documentation: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for DescribeConfigsResourceResult {
    type Builder = DescribeConfigsResourceResultBuilder;

    fn builder() -> Self::Builder{
        DescribeConfigsResourceResultBuilder::default()
    }
}

impl Encodable for DescribeConfigsResourceResult {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 4 {
            types::CompactString.encode(buf, &self.name)?;
        } else {
            types::String.encode(buf, &self.name)?;
        }
        if version >= 4 {
            types::CompactString.encode(buf, &self.value)?;
        } else {
            types::String.encode(buf, &self.value)?;
        }
        types::Boolean.encode(buf, self.read_only)?;
        if version == 0 {
            types::Boolean.encode(buf, self.is_default)?;
        } else {
            if self.is_default {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            buf.put_i8(self.config_source);
        }
        types::Boolean.encode(buf, self.is_sensitive)?;
        if version >= 1 {
            if version >= 4 {
                types::CompactArray(types::Struct { version }).encode(buf, &self.synonyms)?;
            } else {
                types::Array(types::Struct { version }).encode(buf, &self.synonyms)?;
            }
        }
        if version >= 3 {
            buf.put_i8(self.config_type);
        }
        if version >= 3 {
            if version >= 4 {
                types::CompactString.encode(buf, &self.documentation)?;
            } else {
                types::String.encode(buf, &self.documentation)?;
            }
        }
        if version >= 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt::put_u32(buf, num_tagged_fields as u32);

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 4 {
            total_size += types::CompactString.compute_size(&self.name)?;
        } else {
            total_size += types::String.compute_size(&self.name)?;
        }
        if version >= 4 {
            total_size += types::CompactString.compute_size(&self.value)?;
        } else {
            total_size += types::String.compute_size(&self.value)?;
        }
        total_size += types::Boolean.compute_size(self.read_only)?;
        if version == 0 {
            total_size += types::Boolean.compute_size(self.is_default)?;
        } else {
            if self.is_default {
                return Err(EncodeError)
            }
        }
        if version >= 1 {
            total_size += 1;
        }
        total_size += types::Boolean.compute_size(self.is_sensitive)?;
        if version >= 1 {
            if version >= 4 {
                total_size += types::CompactArray(types::Struct { version }).compute_size(&self.synonyms)?;
            } else {
                total_size += types::Array(types::Struct { version }).compute_size(&self.synonyms)?;
            }
        }
        if version >= 3 {
            total_size += 1;
        }
        if version >= 3 {
            if version >= 4 {
                total_size += types::CompactString.compute_size(&self.documentation)?;
            } else {
                total_size += types::String.compute_size(&self.documentation)?;
            }
        }
        if version >= 4 {
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

impl Decodable for DescribeConfigsResourceResult {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = if version >= 4 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let value = if version >= 4 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let read_only = types::Boolean.decode(buf)?;
        let is_default = if version == 0 {
            types::Boolean.decode(buf)?
        } else {
            false
        };
        let config_source = if version >= 1 {
            buf.try_get_i8()?
        } else {
            -1
        };
        let is_sensitive = types::Boolean.decode(buf)?;
        let synonyms = if version >= 1 {
            if version >= 4 {
                types::CompactArray(types::Struct { version }).decode(buf)?
            } else {
                types::Array(types::Struct { version }).decode(buf)?
            }
        } else {
            Default::default()
        };
        let config_type = if version >= 3 {
            buf.try_get_i8()?
        } else {
            0
        };
        let documentation = if version >= 3 {
            if version >= 4 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            Some(Default::default())
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            name,
            value,
            read_only,
            is_default,
            config_source,
            is_sensitive,
            synonyms,
            config_type,
            documentation,
            unknown_tagged_fields,
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
            config_type: 0,
            documentation: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribeConfigsResourceResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct DescribeConfigsResult {
    /// The error code, or 0 if we were able to successfully describe the configurations.
    /// 
    /// Supported API versions: 0-4
    pub error_code: i16,

    /// The error message, or null if we were able to successfully describe the configurations.
    /// 
    /// Supported API versions: 0-4
    pub error_message: Option<StrBytes>,

    /// The resource type.
    /// 
    /// Supported API versions: 0-4
    pub resource_type: i8,

    /// The resource name.
    /// 
    /// Supported API versions: 0-4
    pub resource_name: StrBytes,

    /// Each listed configuration.
    /// 
    /// Supported API versions: 0-4
    pub configs: Vec<DescribeConfigsResourceResult>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for DescribeConfigsResult {
    type Builder = DescribeConfigsResultBuilder;

    fn builder() -> Self::Builder{
        DescribeConfigsResultBuilder::default()
    }
}

impl Encodable for DescribeConfigsResult {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        buf.put_i16(self.error_code);
        if version >= 4 {
            types::CompactString.encode(buf, &self.error_message)?;
        } else {
            types::String.encode(buf, &self.error_message)?;
        }
        buf.put_i8(self.resource_type);
        if version >= 4 {
            types::CompactString.encode(buf, &self.resource_name)?;
        } else {
            types::String.encode(buf, &self.resource_name)?;
        }
        if version >= 4 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.configs)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.configs)?;
        }
        if version >= 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt::put_u32(buf, num_tagged_fields as u32);

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += 2;
        if version >= 4 {
            total_size += types::CompactString.compute_size(&self.error_message)?;
        } else {
            total_size += types::String.compute_size(&self.error_message)?;
        }
        total_size += 1;
        if version >= 4 {
            total_size += types::CompactString.compute_size(&self.resource_name)?;
        } else {
            total_size += types::String.compute_size(&self.resource_name)?;
        }
        if version >= 4 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.configs)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.configs)?;
        }
        if version >= 4 {
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

impl Decodable for DescribeConfigsResult {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = buf.try_get_i16()?;
        let error_message = if version >= 4 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let resource_type = buf.try_get_i8()?;
        let resource_name = if version >= 4 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let configs = if version >= 4 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            error_code,
            error_message,
            resource_type,
            resource_name,
            configs,
            unknown_tagged_fields,
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
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribeConfigsResult {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct DescribeConfigsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 0-4
    pub throttle_time_ms: i32,

    /// The results for each resource.
    /// 
    /// Supported API versions: 0-4
    pub results: Vec<DescribeConfigsResult>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for DescribeConfigsResponse {
    type Builder = DescribeConfigsResponseBuilder;

    fn builder() -> Self::Builder{
        DescribeConfigsResponseBuilder::default()
    }
}

impl Encodable for DescribeConfigsResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        buf.put_i32(self.throttle_time_ms);
        if version >= 4 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.results)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.results)?;
        }
        if version >= 4 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt::put_u32(buf, num_tagged_fields as u32);

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += 4;
        if version >= 4 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.results)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.results)?;
        }
        if version >= 4 {
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

impl Decodable for DescribeConfigsResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = buf.try_get_i32()?;
        let results = if version >= 4 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            throttle_time_ms,
            results,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribeConfigsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            results: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribeConfigsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

impl HeaderVersion for DescribeConfigsResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 4 {
            1
        } else {
            0
        }
    }
}

