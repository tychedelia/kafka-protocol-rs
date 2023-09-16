//! ListGroupsResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/ListGroupsResponse.json).
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
pub struct ListedGroup {
    /// The group ID.
    /// 
    /// Supported API versions: 0-4
    pub group_id: super::GroupId,

    /// The group protocol type.
    /// 
    /// Supported API versions: 0-4
    pub protocol_type: StrBytes,

    /// The group state name.
    /// 
    /// Supported API versions: 4
    pub group_state: StrBytes,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for ListedGroup {
    type Builder = ListedGroupBuilder;

    fn builder() -> Self::Builder{
        ListedGroupBuilder::default()
    }
}

impl Encodable for ListedGroup {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 3 {
            types::CompactString.encode(buf, &self.group_id)?;
        } else {
            types::String.encode(buf, &self.group_id)?;
        }
        if version >= 3 {
            types::CompactString.encode(buf, &self.protocol_type)?;
        } else {
            types::String.encode(buf, &self.protocol_type)?;
        }
        if version >= 4 {
            types::CompactString.encode(buf, &self.group_state)?;
        }
        if version >= 3 {
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
        if version >= 3 {
            total_size += types::CompactString.compute_size(&self.group_id)?;
        } else {
            total_size += types::String.compute_size(&self.group_id)?;
        }
        if version >= 3 {
            total_size += types::CompactString.compute_size(&self.protocol_type)?;
        } else {
            total_size += types::String.compute_size(&self.protocol_type)?;
        }
        if version >= 4 {
            total_size += types::CompactString.compute_size(&self.group_state)?;
        }
        if version >= 3 {
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

impl Decodable for ListedGroup {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let group_id = if version >= 3 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let protocol_type = if version >= 3 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let group_state = if version >= 4 {
            types::CompactString.decode(buf)?
        } else {
            Default::default()
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 3 {
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
            group_id,
            protocol_type,
            group_state,
            unknown_tagged_fields,
        })
    }
}

impl Default for ListedGroup {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            protocol_type: Default::default(),
            group_state: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ListedGroup {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

/// Valid versions: 0-4
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct ListGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 1-4
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-4
    pub error_code: i16,

    /// Each group in the response.
    /// 
    /// Supported API versions: 0-4
    pub groups: Vec<ListedGroup>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for ListGroupsResponse {
    type Builder = ListGroupsResponseBuilder;

    fn builder() -> Self::Builder{
        ListGroupsResponseBuilder::default()
    }
}

impl Encodable for ListGroupsResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            types::Int32.encode(buf, &self.throttle_time_ms)?;
        }
        types::Int16.encode(buf, &self.error_code)?;
        if version >= 3 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.groups)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.groups)?;
        }
        if version >= 3 {
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
        if version >= 1 {
            total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        }
        total_size += types::Int16.compute_size(&self.error_code)?;
        if version >= 3 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.groups)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.groups)?;
        }
        if version >= 3 {
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

impl Decodable for ListGroupsResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = if version >= 1 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let error_code = types::Int16.decode(buf)?;
        let groups = if version >= 3 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 3 {
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
            groups,
            unknown_tagged_fields,
        })
    }
}

impl Default for ListGroupsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            groups: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ListGroupsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 4 };
}

impl HeaderVersion for ListGroupsResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 3 {
            1
        } else {
            0
        }
    }
}

