//! DescribeGroupsResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/DescribeGroupsResponse.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use uuid::Uuid;
use anyhow::bail;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}, Builder
};


/// Valid versions: 0-5
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct DescribedGroupMember {
    /// The member ID assigned by the group coordinator.
    /// 
    /// Supported API versions: 0-5
    pub member_id: StrBytes,

    /// The unique identifier of the consumer instance provided by end user.
    /// 
    /// Supported API versions: 4-5
    pub group_instance_id: Option<StrBytes>,

    /// The client ID used in the member's latest join group request.
    /// 
    /// Supported API versions: 0-5
    pub client_id: StrBytes,

    /// The client host.
    /// 
    /// Supported API versions: 0-5
    pub client_host: StrBytes,

    /// The metadata corresponding to the current group protocol in use.
    /// 
    /// Supported API versions: 0-5
    pub member_metadata: Bytes,

    /// The current assignment provided by the group leader.
    /// 
    /// Supported API versions: 0-5
    pub member_assignment: Bytes,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for DescribedGroupMember {
    type Builder = DescribedGroupMemberBuilder;

    fn builder() -> Self::Builder{
        DescribedGroupMemberBuilder::default()
    }
}

impl Encodable for DescribedGroupMember {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 5 {
            types::CompactString.encode(buf, &self.member_id)?;
        } else {
            types::String.encode(buf, &self.member_id)?;
        }
        if version >= 4 {
            if version >= 5 {
                types::CompactString.encode(buf, &self.group_instance_id)?;
            } else {
                types::String.encode(buf, &self.group_instance_id)?;
            }
        }
        if version >= 5 {
            types::CompactString.encode(buf, &self.client_id)?;
        } else {
            types::String.encode(buf, &self.client_id)?;
        }
        if version >= 5 {
            types::CompactString.encode(buf, &self.client_host)?;
        } else {
            types::String.encode(buf, &self.client_host)?;
        }
        if version >= 5 {
            types::CompactBytes.encode(buf, &self.member_metadata)?;
        } else {
            types::Bytes.encode(buf, &self.member_metadata)?;
        }
        if version >= 5 {
            types::CompactBytes.encode(buf, &self.member_assignment)?;
        } else {
            types::Bytes.encode(buf, &self.member_assignment)?;
        }
        if version >= 5 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 5 {
            total_size += types::CompactString.compute_size(&self.member_id)?;
        } else {
            total_size += types::String.compute_size(&self.member_id)?;
        }
        if version >= 4 {
            if version >= 5 {
                total_size += types::CompactString.compute_size(&self.group_instance_id)?;
            } else {
                total_size += types::String.compute_size(&self.group_instance_id)?;
            }
        }
        if version >= 5 {
            total_size += types::CompactString.compute_size(&self.client_id)?;
        } else {
            total_size += types::String.compute_size(&self.client_id)?;
        }
        if version >= 5 {
            total_size += types::CompactString.compute_size(&self.client_host)?;
        } else {
            total_size += types::String.compute_size(&self.client_host)?;
        }
        if version >= 5 {
            total_size += types::CompactBytes.compute_size(&self.member_metadata)?;
        } else {
            total_size += types::Bytes.compute_size(&self.member_metadata)?;
        }
        if version >= 5 {
            total_size += types::CompactBytes.compute_size(&self.member_assignment)?;
        } else {
            total_size += types::Bytes.compute_size(&self.member_assignment)?;
        }
        if version >= 5 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for DescribedGroupMember {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let member_id = if version >= 5 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let group_instance_id = if version >= 4 {
            if version >= 5 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            None
        };
        let client_id = if version >= 5 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let client_host = if version >= 5 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let member_metadata = if version >= 5 {
            types::CompactBytes.decode(buf)?
        } else {
            types::Bytes.decode(buf)?
        };
        let member_assignment = if version >= 5 {
            types::CompactBytes.decode(buf)?
        } else {
            types::Bytes.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 5 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let unknown_value = buf.try_get_bytes(size as usize)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            member_id,
            group_instance_id,
            client_id,
            client_host,
            member_metadata,
            member_assignment,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribedGroupMember {
    fn default() -> Self {
        Self {
            member_id: Default::default(),
            group_instance_id: None,
            client_id: Default::default(),
            client_host: Default::default(),
            member_metadata: Default::default(),
            member_assignment: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribedGroupMember {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

/// Valid versions: 0-5
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct DescribedGroup {
    /// The describe error, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-5
    pub error_code: i16,

    /// The group ID string.
    /// 
    /// Supported API versions: 0-5
    pub group_id: super::GroupId,

    /// The group state string, or the empty string.
    /// 
    /// Supported API versions: 0-5
    pub group_state: StrBytes,

    /// The group protocol type, or the empty string.
    /// 
    /// Supported API versions: 0-5
    pub protocol_type: StrBytes,

    /// The group protocol data, or the empty string.
    /// 
    /// Supported API versions: 0-5
    pub protocol_data: StrBytes,

    /// The group members.
    /// 
    /// Supported API versions: 0-5
    pub members: Vec<DescribedGroupMember>,

    /// 32-bit bitfield to represent authorized operations for this group.
    /// 
    /// Supported API versions: 3-5
    pub authorized_operations: i32,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for DescribedGroup {
    type Builder = DescribedGroupBuilder;

    fn builder() -> Self::Builder{
        DescribedGroupBuilder::default()
    }
}

impl Encodable for DescribedGroup {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int16.encode(buf, &self.error_code)?;
        if version >= 5 {
            types::CompactString.encode(buf, &self.group_id)?;
        } else {
            types::String.encode(buf, &self.group_id)?;
        }
        if version >= 5 {
            types::CompactString.encode(buf, &self.group_state)?;
        } else {
            types::String.encode(buf, &self.group_state)?;
        }
        if version >= 5 {
            types::CompactString.encode(buf, &self.protocol_type)?;
        } else {
            types::String.encode(buf, &self.protocol_type)?;
        }
        if version >= 5 {
            types::CompactString.encode(buf, &self.protocol_data)?;
        } else {
            types::String.encode(buf, &self.protocol_data)?;
        }
        if version >= 5 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.members)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.members)?;
        }
        if version >= 3 {
            types::Int32.encode(buf, &self.authorized_operations)?;
        } else {
            if self.authorized_operations != -2147483648 {
                bail!("failed to decode");
            }
        }
        if version >= 5 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int16.compute_size(&self.error_code)?;
        if version >= 5 {
            total_size += types::CompactString.compute_size(&self.group_id)?;
        } else {
            total_size += types::String.compute_size(&self.group_id)?;
        }
        if version >= 5 {
            total_size += types::CompactString.compute_size(&self.group_state)?;
        } else {
            total_size += types::String.compute_size(&self.group_state)?;
        }
        if version >= 5 {
            total_size += types::CompactString.compute_size(&self.protocol_type)?;
        } else {
            total_size += types::String.compute_size(&self.protocol_type)?;
        }
        if version >= 5 {
            total_size += types::CompactString.compute_size(&self.protocol_data)?;
        } else {
            total_size += types::String.compute_size(&self.protocol_data)?;
        }
        if version >= 5 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.members)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.members)?;
        }
        if version >= 3 {
            total_size += types::Int32.compute_size(&self.authorized_operations)?;
        } else {
            if self.authorized_operations != -2147483648 {
                bail!("failed to decode");
            }
        }
        if version >= 5 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for DescribedGroup {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let error_code = types::Int16.decode(buf)?;
        let group_id = if version >= 5 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let group_state = if version >= 5 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let protocol_type = if version >= 5 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let protocol_data = if version >= 5 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let members = if version >= 5 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let authorized_operations = if version >= 3 {
            types::Int32.decode(buf)?
        } else {
            -2147483648
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 5 {
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
            group_id,
            group_state,
            protocol_type,
            protocol_data,
            members,
            authorized_operations,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribedGroup {
    fn default() -> Self {
        Self {
            error_code: 0,
            group_id: Default::default(),
            group_state: Default::default(),
            protocol_type: Default::default(),
            protocol_data: Default::default(),
            members: Default::default(),
            authorized_operations: -2147483648,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribedGroup {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

/// Valid versions: 0-5
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
#[builder(default)]
pub struct DescribeGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 1-5
    pub throttle_time_ms: i32,

    /// Each described group.
    /// 
    /// Supported API versions: 0-5
    pub groups: Vec<DescribedGroup>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Bytes>,
}

impl Builder for DescribeGroupsResponse {
    type Builder = DescribeGroupsResponseBuilder;

    fn builder() -> Self::Builder{
        DescribeGroupsResponseBuilder::default()
    }
}

impl Encodable for DescribeGroupsResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            types::Int32.encode(buf, &self.throttle_time_ms)?;
        }
        if version >= 5 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.groups)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.groups)?;
        }
        if version >= 5 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
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
        if version >= 5 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.groups)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.groups)?;
        }
        if version >= 5 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                bail!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for DescribeGroupsResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = if version >= 1 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let groups = if version >= 5 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 5 {
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
            groups,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribeGroupsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            groups: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribeGroupsResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

impl HeaderVersion for DescribeGroupsResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 5 {
            1
        } else {
            0
        }
    }
}

