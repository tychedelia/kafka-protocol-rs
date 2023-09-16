//! DescribeAclsRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/DescribeAclsRequest.json).
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


/// Valid versions: 0-3
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct DescribeAclsRequest {
    /// The resource type.
    /// 
    /// Supported API versions: 0-3
    pub resource_type_filter: i8,

    /// The resource name, or null to match any resource name.
    /// 
    /// Supported API versions: 0-3
    pub resource_name_filter: Option<StrBytes>,

    /// The resource pattern to match.
    /// 
    /// Supported API versions: 1-3
    pub pattern_type_filter: i8,

    /// The principal to match, or null to match any principal.
    /// 
    /// Supported API versions: 0-3
    pub principal_filter: Option<StrBytes>,

    /// The host to match, or null to match any host.
    /// 
    /// Supported API versions: 0-3
    pub host_filter: Option<StrBytes>,

    /// The operation to match.
    /// 
    /// Supported API versions: 0-3
    pub operation: i8,

    /// The permission type to match.
    /// 
    /// Supported API versions: 0-3
    pub permission_type: i8,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for DescribeAclsRequest {
    type Builder = DescribeAclsRequestBuilder;

    fn builder() -> Self::Builder{
        DescribeAclsRequestBuilder::default()
    }
}

impl Encodable for DescribeAclsRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int8.encode(buf, &self.resource_type_filter)?;
        if version >= 2 {
            types::CompactString.encode(buf, &self.resource_name_filter)?;
        } else {
            types::String.encode(buf, &self.resource_name_filter)?;
        }
        if version >= 1 {
            types::Int8.encode(buf, &self.pattern_type_filter)?;
        } else {
            if self.pattern_type_filter != 3 {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
            types::CompactString.encode(buf, &self.principal_filter)?;
        } else {
            types::String.encode(buf, &self.principal_filter)?;
        }
        if version >= 2 {
            types::CompactString.encode(buf, &self.host_filter)?;
        } else {
            types::String.encode(buf, &self.host_filter)?;
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
        total_size += types::Int8.compute_size(&self.resource_type_filter)?;
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.resource_name_filter)?;
        } else {
            total_size += types::String.compute_size(&self.resource_name_filter)?;
        }
        if version >= 1 {
            total_size += types::Int8.compute_size(&self.pattern_type_filter)?;
        } else {
            if self.pattern_type_filter != 3 {
                return Err(EncodeError)
            }
        }
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.principal_filter)?;
        } else {
            total_size += types::String.compute_size(&self.principal_filter)?;
        }
        if version >= 2 {
            total_size += types::CompactString.compute_size(&self.host_filter)?;
        } else {
            total_size += types::String.compute_size(&self.host_filter)?;
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

impl Decodable for DescribeAclsRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let resource_type_filter = types::Int8.decode(buf)?;
        let resource_name_filter = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let pattern_type_filter = if version >= 1 {
            types::Int8.decode(buf)?
        } else {
            3
        };
        let principal_filter = if version >= 2 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let host_filter = if version >= 2 {
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
            resource_type_filter,
            resource_name_filter,
            pattern_type_filter,
            principal_filter,
            host_filter,
            operation,
            permission_type,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribeAclsRequest {
    fn default() -> Self {
        Self {
            resource_type_filter: 0,
            resource_name_filter: Some(Default::default()),
            pattern_type_filter: 3,
            principal_filter: Some(Default::default()),
            host_filter: Some(Default::default()),
            operation: 0,
            permission_type: 0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribeAclsRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 3 };
}

impl HeaderVersion for DescribeAclsRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 2 {
            2
        } else {
            1
        }
    }
}

