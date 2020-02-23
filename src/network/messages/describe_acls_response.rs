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


/// Valid versions: 0-2
#[derive(Debug, Clone)]
pub struct AclDescription {
    /// The ACL principal.
    /// 
    /// Supported API versions: 0-2
    pub principal: String,

    /// The ACL host.
    /// 
    /// Supported API versions: 0-2
    pub host: String,

    /// The ACL operation.
    /// 
    /// Supported API versions: 0-2
    pub operation: i8,

    /// The ACL permission type.
    /// 
    /// Supported API versions: 0-2
    pub permission_type: i8,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for AclDescription {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version == 2 {
            types::CompactString.encode(buf, &self.principal)?;
        } else {
            types::String.encode(buf, &self.principal)?;
        }
        if version == 2 {
            types::CompactString.encode(buf, &self.host)?;
        } else {
            types::String.encode(buf, &self.host)?;
        }
        types::Int8.encode(buf, &self.operation)?;
        types::Int8.encode(buf, &self.permission_type)?;
        if version == 2 {
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
        if version == 2 {
            total_size += types::CompactString.compute_size(&self.principal)?;
        } else {
            total_size += types::String.compute_size(&self.principal)?;
        }
        if version == 2 {
            total_size += types::CompactString.compute_size(&self.host)?;
        } else {
            total_size += types::String.compute_size(&self.host)?;
        }
        total_size += types::Int8.compute_size(&self.operation)?;
        total_size += types::Int8.compute_size(&self.permission_type)?;
        if version == 2 {
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

impl Decodable for AclDescription {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let principal = if version == 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let host = if version == 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let operation = types::Int8.decode(buf)?;
        let permission_type = types::Int8.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 2 {
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
            principal,
            host,
            operation,
            permission_type,
            unknown_tagged_fields,
        })
    }
}

impl Default for AclDescription {
    fn default() -> Self {
        Self {
            principal: Default::default(),
            host: Default::default(),
            operation: 0,
            permission_type: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for AclDescription {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

/// Valid versions: 0-2
#[derive(Debug, Clone)]
pub struct DescribeAclsResource {
    /// The resource type.
    /// 
    /// Supported API versions: 0-2
    pub resource_type: i8,

    /// The resource name.
    /// 
    /// Supported API versions: 0-2
    pub resource_name: String,

    /// The resource pattern type.
    /// 
    /// Supported API versions: 1-2
    pub pattern_type: i8,

    /// The ACLs.
    /// 
    /// Supported API versions: 0-2
    pub acls: Vec<AclDescription>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for DescribeAclsResource {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int8.encode(buf, &self.resource_type)?;
        if version == 2 {
            types::CompactString.encode(buf, &self.resource_name)?;
        } else {
            types::String.encode(buf, &self.resource_name)?;
        }
        if version >= 1 {
            types::Int8.encode(buf, &self.pattern_type)?;
        } else {
            if self.pattern_type != 3 {
                return Err(EncodeError)
            }
        }
        if version == 2 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.acls)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.acls)?;
        }
        if version == 2 {
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
        if version == 2 {
            total_size += types::CompactString.compute_size(&self.resource_name)?;
        } else {
            total_size += types::String.compute_size(&self.resource_name)?;
        }
        if version >= 1 {
            total_size += types::Int8.compute_size(&self.pattern_type)?;
        } else {
            if self.pattern_type != 3 {
                return Err(EncodeError)
            }
        }
        if version == 2 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.acls)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.acls)?;
        }
        if version == 2 {
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

impl Decodable for DescribeAclsResource {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let resource_type = types::Int8.decode(buf)?;
        let resource_name = if version == 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let pattern_type = if version >= 1 {
            types::Int8.decode(buf)?
        } else {
            3
        };
        let acls = if version == 2 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 2 {
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
            pattern_type,
            acls,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribeAclsResource {
    fn default() -> Self {
        Self {
            resource_type: 0,
            resource_name: Default::default(),
            pattern_type: 3,
            acls: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribeAclsResource {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

/// Valid versions: 0-2
#[derive(Debug, Clone)]
pub struct DescribeAclsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 0-2
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-2
    pub error_code: i16,

    /// The error message, or null if there was no error.
    /// 
    /// Supported API versions: 0-2
    pub error_message: Option<String>,

    /// Each Resource that is referenced in an ACL.
    /// 
    /// Supported API versions: 0-2
    pub resources: Vec<DescribeAclsResource>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for DescribeAclsResponse {
    fn encode<B: BufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.throttle_time_ms)?;
        types::Int16.encode(buf, &self.error_code)?;
        if version == 2 {
            types::CompactString.encode(buf, &self.error_message)?;
        } else {
            types::String.encode(buf, &self.error_message)?;
        }
        if version == 2 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.resources)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.resources)?;
        }
        if version == 2 {
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
        total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        total_size += types::Int16.compute_size(&self.error_code)?;
        if version == 2 {
            total_size += types::CompactString.compute_size(&self.error_message)?;
        } else {
            total_size += types::String.compute_size(&self.error_message)?;
        }
        if version == 2 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.resources)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.resources)?;
        }
        if version == 2 {
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

impl Decodable for DescribeAclsResponse {
    fn decode<B: Buf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = types::Int32.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        let error_message = if version == 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let resources = if version == 2 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version == 2 {
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
            throttle_time_ms,
            error_code,
            error_message,
            resources,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribeAclsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            error_message: Some(Default::default()),
            resources: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribeAclsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 2 };
}

impl HeaderVersion for DescribeAclsResponse {
    fn header_version(version: i16) -> i16 {
        if version == 2 {
            1
        } else {
            0
        }
    }
}

