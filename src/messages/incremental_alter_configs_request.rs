//! IncrementalAlterConfigsRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/IncrementalAlterConfigsRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use anyhow::bail;
use bytes::Bytes;
use uuid::Uuid;

use crate::protocol::{
    buf::{ByteBuf, ByteBufMut},
    compute_unknown_tagged_fields_size, types, write_unknown_tagged_fields, Builder, Decodable,
    DecodeError, Decoder, Encodable, EncodeError, Encoder, HeaderVersion, MapDecodable,
    MapEncodable, Message, StrBytes, VersionRange,
};

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct AlterConfigsResource {
    /// The resource type.
    ///
    /// Supported API versions: 0-1
    pub resource_type: i8,

    /// The resource name.
    ///
    /// Supported API versions: 0-1
    pub resource_name: StrBytes,

    /// The configurations.
    ///
    /// Supported API versions: 0-1
    pub configs: Vec<AlterableConfig>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for AlterConfigsResource {
    type Builder = AlterConfigsResourceBuilder;

    fn builder() -> Self::Builder {
        AlterConfigsResourceBuilder::default()
    }
}

impl Encodable for AlterConfigsResource {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int8.encode(buf, &self.resource_type)?;
        if version >= 1 {
            types::CompactString.encode(buf, &self.resource_name)?;
        } else {
            types::String.encode(buf, &self.resource_name)?;
        }
        if version >= 1 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.configs)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.configs)?;
        }
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int8.compute_size(&self.resource_type)?;
        if version >= 1 {
            total_size += types::CompactString.compute_size(&self.resource_name)?;
        } else {
            total_size += types::String.compute_size(&self.resource_name)?;
        }
        if version >= 1 {
            total_size +=
                types::CompactArray(types::Struct { version }).compute_size(&self.configs)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.configs)?;
        }
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for AlterConfigsResource {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let resource_type = types::Int8.decode(buf)?;
        let resource_name = if version >= 1 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let configs = if version >= 1 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 1 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            resource_type,
            resource_name,
            configs,
            unknown_tagged_fields,
        })
    }
}

impl Default for AlterConfigsResource {
    fn default() -> Self {
        Self {
            resource_type: 0,
            resource_name: Default::default(),
            configs: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AlterConfigsResource {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct AlterableConfig {
    /// The configuration key name.
    ///
    /// Supported API versions: 0-1
    pub name: StrBytes,

    /// The type (Set, Delete, Append, Subtract) of operation.
    ///
    /// Supported API versions: 0-1
    pub config_operation: i8,

    /// The value to set for the configuration key.
    ///
    /// Supported API versions: 0-1
    pub value: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for AlterableConfig {
    type Builder = AlterableConfigBuilder;

    fn builder() -> Self::Builder {
        AlterableConfigBuilder::default()
    }
}

impl Encodable for AlterableConfig {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            types::CompactString.encode(buf, &self.name)?;
        } else {
            types::String.encode(buf, &self.name)?;
        }
        types::Int8.encode(buf, &self.config_operation)?;
        if version >= 1 {
            types::CompactString.encode(buf, &self.value)?;
        } else {
            types::String.encode(buf, &self.value)?;
        }
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 1 {
            total_size += types::CompactString.compute_size(&self.name)?;
        } else {
            total_size += types::String.compute_size(&self.name)?;
        }
        total_size += types::Int8.compute_size(&self.config_operation)?;
        if version >= 1 {
            total_size += types::CompactString.compute_size(&self.value)?;
        } else {
            total_size += types::String.compute_size(&self.value)?;
        }
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for AlterableConfig {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let name = if version >= 1 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let config_operation = types::Int8.decode(buf)?;
        let value = if version >= 1 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 1 {
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
            config_operation,
            value,
            unknown_tagged_fields,
        })
    }
}

impl Default for AlterableConfig {
    fn default() -> Self {
        Self {
            name: Default::default(),
            config_operation: 0,
            value: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AlterableConfig {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct IncrementalAlterConfigsRequest {
    /// The incremental updates for each resource.
    ///
    /// Supported API versions: 0-1
    pub resources: Vec<AlterConfigsResource>,

    /// True if we should validate the request, but not change the configurations.
    ///
    /// Supported API versions: 0-1
    pub validate_only: bool,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for IncrementalAlterConfigsRequest {
    type Builder = IncrementalAlterConfigsRequestBuilder;

    fn builder() -> Self::Builder {
        IncrementalAlterConfigsRequestBuilder::default()
    }
}

impl Encodable for IncrementalAlterConfigsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.resources)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.resources)?;
        }
        types::Boolean.encode(buf, &self.validate_only)?;
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 1 {
            total_size +=
                types::CompactArray(types::Struct { version }).compute_size(&self.resources)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.resources)?;
        }
        total_size += types::Boolean.compute_size(&self.validate_only)?;
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!(
                    "Too many tagged fields to encode ({} fields)",
                    num_tagged_fields
                );
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for IncrementalAlterConfigsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let resources = if version >= 1 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let validate_only = types::Boolean.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 1 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            resources,
            validate_only,
            unknown_tagged_fields,
        })
    }
}

impl Default for IncrementalAlterConfigsRequest {
    fn default() -> Self {
        Self {
            resources: Default::default(),
            validate_only: false,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for IncrementalAlterConfigsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
    const DEPRECATED_VERSIONS: Option<VersionRange> = None;
}

impl HeaderVersion for IncrementalAlterConfigsRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 1 {
            2
        } else {
            1
        }
    }
}
