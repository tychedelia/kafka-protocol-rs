//! AlterConfigsRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/AlterConfigsRequest.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-2
#[derive(Debug, Clone, PartialEq)]
pub struct AlterableConfig {
    /// The value to set for the configuration key.
    /// 
    /// Supported API versions: 0-2
    pub value: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl MapEncodable for AlterableConfig {
    type Key = StrBytes;
    fn encode<B: ByteBufMut>(&self, key: &Self::Key, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 2 {
            types::CompactString.encode(buf, key)?;
        } else {
            types::String.encode(buf, key)?;
        }
        if version >= 2 {
            types::CompactString.encode(buf, &self.value)?;
        } else {
            types::String.encode(buf, &self.value)?;
        }
        if version >= 2 {
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
    fn compute_size(&self, key: &Self::Key, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 2 {
            total_size += types::CompactString.compute_size(key)?;
        } else {
            total_size += types::String.compute_size(key)?;
        }
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.value)?;
        } else {
            total_size += types::String.compute_size(&self.value)?;
        }
        if version >= 2 {
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

impl MapDecodable for AlterableConfig {
    type Key = StrBytes;
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<(Self::Key, Self), DecodeError> {
        let key_field = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let value = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok((key_field, Self {
            value,
            unknown_tagged_fields,
        }))
    }
}

impl Default for AlterableConfig {
    fn default() -> Self {
        Self {
            value: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AlterableConfig {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

/// Valid versions: 0-2
#[derive(Debug, Clone, PartialEq)]
pub struct AlterConfigsResource {
    /// The resource type.
    /// 
    /// Supported API versions: 0-2
    pub resource_type: i8,

    /// The resource name.
    /// 
    /// Supported API versions: 0-2
    pub resource_name: StrBytes,

    /// The configurations.
    /// 
    /// Supported API versions: 0-2
    pub configs: indexmap::IndexMap<StrBytes, AlterableConfig>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for AlterConfigsResource {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int8.encode(buf, &self.resource_type)?;
        if version >= 2 {
            types::CompactString.encode(buf, &self.resource_name)?;
        } else {
            types::String.encode(buf, &self.resource_name)?;
        }
        if version >= 2 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.configs)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.configs)?;
        }
        if version >= 2 {
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
        total_size += types::Int8.compute_size(&self.resource_type)?;
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.resource_name)?;
        } else {
            total_size += types::String.compute_size(&self.resource_name)?;
        }
        if version >= 2 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.configs)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.configs)?;
        }
        if version >= 2 {
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

impl Decodable for AlterConfigsResource {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let resource_type = types::Int8.decode(buf)?;
        let resource_name = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let configs = if version >= 2 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
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
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

/// Valid versions: 0-2
#[derive(Debug, Clone, PartialEq)]
pub struct AlterConfigsRequest {
    /// The updates for each resource.
    /// 
    /// Supported API versions: 0-2
    pub resources: Vec<AlterConfigsResource>,

    /// True if we should validate the request, but not change the configurations.
    /// 
    /// Supported API versions: 0-2
    pub validate_only: bool,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for AlterConfigsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 2 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.resources)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.resources)?;
        }
        types::Boolean.encode(buf, &self.validate_only)?;
        if version >= 2 {
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
        if version >= 2 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.resources)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.resources)?;
        }
        total_size += types::Boolean.compute_size(&self.validate_only)?;
        if version >= 2 {
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

impl Decodable for AlterConfigsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let resources = if version >= 2 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let validate_only = types::Boolean.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 2 {
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
            resources,
            validate_only,
            unknown_tagged_fields,
        })
    }
}

impl Default for AlterConfigsRequest {
    fn default() -> Self {
        Self {
            resources: Default::default(),
            validate_only: false,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AlterConfigsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

impl HeaderVersion for AlterConfigsRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 2 {
            2
        } else {
            1
        }
    }
}

