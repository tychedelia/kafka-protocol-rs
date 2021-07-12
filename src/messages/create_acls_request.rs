//! CreateAclsRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/CreateAclsRequest.json).
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
pub struct AclCreation {
    /// The type of the resource.
    /// 
    /// Supported API versions: 0-2
    pub resource_type: i8,

    /// The resource name for the ACL.
    /// 
    /// Supported API versions: 0-2
    pub resource_name: StrBytes,

    /// The pattern type for the ACL.
    /// 
    /// Supported API versions: 1-2
    pub resource_pattern_type: i8,

    /// The principal for the ACL.
    /// 
    /// Supported API versions: 0-2
    pub principal: StrBytes,

    /// The host for the ACL.
    /// 
    /// Supported API versions: 0-2
    pub host: StrBytes,

    /// The operation type for the ACL (read, write, etc.).
    /// 
    /// Supported API versions: 0-2
    pub operation: i8,

    /// The permission type for the ACL (allow, deny, etc.).
    /// 
    /// Supported API versions: 0-2
    pub permission_type: i8,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for AclCreation {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int8.encode(buf, &self.resource_type)?;
        if version >= 2 {
            types::CompactString.encode(buf, &self.resource_name)?;
        } else {
            types::String.encode(buf, &self.resource_name)?;
        }
        if version >= 1 {
            types::Int8.encode(buf, &self.resource_pattern_type)?;
        } else {
            if self.resource_pattern_type != 3 {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
            types::CompactString.encode(buf, &self.principal)?;
        } else {
            types::String.encode(buf, &self.principal)?;
        }
        if version >= 2 {
            types::CompactString.encode(buf, &self.host)?;
        } else {
            types::String.encode(buf, &self.host)?;
        }
        types::Int8.encode(buf, &self.operation)?;
        types::Int8.encode(buf, &self.permission_type)?;
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
        if version >= 1 {
            total_size += types::Int8.compute_size(&self.resource_pattern_type)?;
        } else {
            if self.resource_pattern_type != 3 {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.principal)?;
        } else {
            total_size += types::String.compute_size(&self.principal)?;
        }
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.host)?;
        } else {
            total_size += types::String.compute_size(&self.host)?;
        }
        total_size += types::Int8.compute_size(&self.operation)?;
        total_size += types::Int8.compute_size(&self.permission_type)?;
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

impl Decodable for AclCreation {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let resource_type = types::Int8.decode(buf)?;
        let resource_name = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let resource_pattern_type = if version >= 1 {
            types::Int8.decode(buf)?
        } else {
            3
        };
        let principal = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let host = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let operation = types::Int8.decode(buf)?;
        let permission_type = types::Int8.decode(buf)?;
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
            resource_pattern_type,
            principal,
            host,
            operation,
            permission_type,
            unknown_tagged_fields,
        })
    }
}

impl Default for AclCreation {
    fn default() -> Self {
        Self {
            resource_type: 0,
            resource_name: Default::default(),
            resource_pattern_type: 3,
            principal: Default::default(),
            host: Default::default(),
            operation: 0,
            permission_type: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AclCreation {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

/// Valid versions: 0-2
#[derive(Debug, Clone, PartialEq)]
pub struct CreateAclsRequest {
    /// The ACLs that we want to create.
    /// 
    /// Supported API versions: 0-2
    pub creations: Vec<AclCreation>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for CreateAclsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 2 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.creations)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.creations)?;
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
        if version >= 2 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.creations)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.creations)?;
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

impl Decodable for CreateAclsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let creations = if version >= 2 {
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
            creations,
            unknown_tagged_fields,
        })
    }
}

impl Default for CreateAclsRequest {
    fn default() -> Self {
        Self {
            creations: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for CreateAclsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

impl HeaderVersion for CreateAclsRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 2 {
            2
        } else {
            1
        }
    }
}

