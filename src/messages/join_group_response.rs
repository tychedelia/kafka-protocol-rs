//! THIS CODE IS AUTOMATICALLY GENERATED. DO NOT EDIT.
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use protocol_base::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}
};


/// Valid versions: 0-7
#[derive(Debug, Clone, PartialEq)]
pub struct JoinGroupResponseMember {
    /// The group member ID.
    /// 
    /// Supported API versions: 0-7
    pub member_id: StrBytes,

    /// The unique identifier of the consumer instance provided by end user.
    /// 
    /// Supported API versions: 5-7
    pub group_instance_id: Option<StrBytes>,

    /// The group member metadata.
    /// 
    /// Supported API versions: 0-7
    pub metadata: Bytes,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for JoinGroupResponseMember {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 6 {
            types::CompactString.encode(buf, &self.member_id)?;
        } else {
            types::String.encode(buf, &self.member_id)?;
        }
        if version >= 5 {
            if version >= 6 {
                types::CompactString.encode(buf, &self.group_instance_id)?;
            } else {
                types::String.encode(buf, &self.group_instance_id)?;
            }
        } else {
            if !self.group_instance_id.is_none() {
                return Err(EncodeError)
            }
        }
        if version >= 6 {
            types::CompactBytes.encode(buf, &self.metadata)?;
        } else {
            types::Bytes.encode(buf, &self.metadata)?;
        }
        if version >= 6 {
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
        if version >= 6 {
            total_size += types::CompactString.compute_size(&self.member_id)?;
        } else {
            total_size += types::String.compute_size(&self.member_id)?;
        }
        if version >= 5 {
            if version >= 6 {
                total_size += types::CompactString.compute_size(&self.group_instance_id)?;
            } else {
                total_size += types::String.compute_size(&self.group_instance_id)?;
            }
        } else {
            if !self.group_instance_id.is_none() {
                return Err(EncodeError)
            }
        }
        if version >= 6 {
            total_size += types::CompactBytes.compute_size(&self.metadata)?;
        } else {
            total_size += types::Bytes.compute_size(&self.metadata)?;
        }
        if version >= 6 {
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

impl Decodable for JoinGroupResponseMember {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let member_id = if version >= 6 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let group_instance_id = if version >= 5 {
            if version >= 6 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            None
        };
        let metadata = if version >= 6 {
            types::CompactBytes.decode(buf)?
        } else {
            types::Bytes.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
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
            member_id,
            group_instance_id,
            metadata,
            unknown_tagged_fields,
        })
    }
}

impl Default for JoinGroupResponseMember {
    fn default() -> Self {
        Self {
            member_id: Default::default(),
            group_instance_id: None,
            metadata: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for JoinGroupResponseMember {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
}

/// Valid versions: 0-7
#[derive(Debug, Clone, PartialEq)]
pub struct JoinGroupResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 2-7
    pub throttle_time_ms: i32,

    /// The error code, or 0 if there was no error.
    /// 
    /// Supported API versions: 0-7
    pub error_code: i16,

    /// The generation ID of the group.
    /// 
    /// Supported API versions: 0-7
    pub generation_id: i32,

    /// The group protocol name.
    /// 
    /// Supported API versions: 7-7
    pub protocol_type: Option<StrBytes>,

    /// The group protocol selected by the coordinator.
    /// 
    /// Supported API versions: 0-7
    pub protocol_name: Option<StrBytes>,

    /// The leader of the group.
    /// 
    /// Supported API versions: 0-7
    pub leader: StrBytes,

    /// The member ID assigned by the group coordinator.
    /// 
    /// Supported API versions: 0-7
    pub member_id: StrBytes,

    /// 
    /// 
    /// Supported API versions: 0-7
    pub members: Vec<JoinGroupResponseMember>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for JoinGroupResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 2 {
            types::Int32.encode(buf, &self.throttle_time_ms)?;
        }
        types::Int16.encode(buf, &self.error_code)?;
        types::Int32.encode(buf, &self.generation_id)?;
        if version == 7 {
            types::CompactString.encode(buf, &self.protocol_type)?;
        }
        if version >= 6 {
            types::CompactString.encode(buf, &self.protocol_name)?;
        } else {
            types::String.encode(buf, &self.protocol_name)?;
        }
        if version >= 6 {
            types::CompactString.encode(buf, &self.leader)?;
        } else {
            types::String.encode(buf, &self.leader)?;
        }
        if version >= 6 {
            types::CompactString.encode(buf, &self.member_id)?;
        } else {
            types::String.encode(buf, &self.member_id)?;
        }
        if version >= 6 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.members)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.members)?;
        }
        if version >= 6 {
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
            total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        }
        total_size += types::Int16.compute_size(&self.error_code)?;
        total_size += types::Int32.compute_size(&self.generation_id)?;
        if version == 7 {
            total_size += types::CompactString.compute_size(&self.protocol_type)?;
        }
        if version >= 6 {
            total_size += types::CompactString.compute_size(&self.protocol_name)?;
        } else {
            total_size += types::String.compute_size(&self.protocol_name)?;
        }
        if version >= 6 {
            total_size += types::CompactString.compute_size(&self.leader)?;
        } else {
            total_size += types::String.compute_size(&self.leader)?;
        }
        if version >= 6 {
            total_size += types::CompactString.compute_size(&self.member_id)?;
        } else {
            total_size += types::String.compute_size(&self.member_id)?;
        }
        if version >= 6 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.members)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.members)?;
        }
        if version >= 6 {
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

impl Decodable for JoinGroupResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = if version >= 2 {
            types::Int32.decode(buf)?
        } else {
            0
        };
        let error_code = types::Int16.decode(buf)?;
        let generation_id = types::Int32.decode(buf)?;
        let protocol_type = if version == 7 {
            types::CompactString.decode(buf)?
        } else {
            None
        };
        let protocol_name = if version >= 6 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let leader = if version >= 6 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let member_id = if version >= 6 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let members = if version >= 6 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 6 {
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
            generation_id,
            protocol_type,
            protocol_name,
            leader,
            member_id,
            members,
            unknown_tagged_fields,
        })
    }
}

impl Default for JoinGroupResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            generation_id: -1,
            protocol_type: None,
            protocol_name: Some(Default::default()),
            leader: Default::default(),
            member_id: Default::default(),
            members: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for JoinGroupResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 7 };
}

impl HeaderVersion for JoinGroupResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 6 {
            1
        } else {
            0
        }
    }
}

