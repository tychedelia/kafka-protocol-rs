//! SyncGroupRequest
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/SyncGroupRequest.json).
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


/// Valid versions: 0-5
#[derive(Debug, Clone, PartialEq)]
pub struct SyncGroupRequestAssignment {
    /// The ID of the member to assign.
    /// 
    /// Supported API versions: 0-5
    pub member_id: StrBytes,

    /// The member assignment.
    /// 
    /// Supported API versions: 0-5
    pub assignment: Bytes,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for SyncGroupRequestAssignment {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 4 {
            types::CompactString.encode(buf, &self.member_id)?;
        } else {
            types::String.encode(buf, &self.member_id)?;
        }
        if version >= 4 {
            types::CompactBytes.encode(buf, &self.assignment)?;
        } else {
            types::Bytes.encode(buf, &self.assignment)?;
        }
        if version >= 4 {
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
        if version >= 4 {
            total_size += types::CompactString.compute_size(&self.member_id)?;
        } else {
            total_size += types::String.compute_size(&self.member_id)?;
        }
        if version >= 4 {
            total_size += types::CompactBytes.compute_size(&self.assignment)?;
        } else {
            total_size += types::Bytes.compute_size(&self.assignment)?;
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

impl Decodable for SyncGroupRequestAssignment {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let member_id = if version >= 4 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let assignment = if version >= 4 {
            types::CompactBytes.decode(buf)?
        } else {
            types::Bytes.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 4 {
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
            assignment,
            unknown_tagged_fields,
        })
    }
}

impl Default for SyncGroupRequestAssignment {
    fn default() -> Self {
        Self {
            member_id: Default::default(),
            assignment: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for SyncGroupRequestAssignment {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

/// Valid versions: 0-5
#[derive(Debug, Clone, PartialEq)]
pub struct SyncGroupRequest {
    /// The unique group identifier.
    /// 
    /// Supported API versions: 0-5
    pub group_id: super::GroupId,

    /// The generation of the group.
    /// 
    /// Supported API versions: 0-5
    pub generation_id: i32,

    /// The member ID assigned by the group.
    /// 
    /// Supported API versions: 0-5
    pub member_id: StrBytes,

    /// The unique identifier of the consumer instance provided by end user.
    /// 
    /// Supported API versions: 3-5
    pub group_instance_id: Option<StrBytes>,

    /// The group protocol type.
    /// 
    /// Supported API versions: 5
    pub protocol_type: Option<StrBytes>,

    /// The group protocol name.
    /// 
    /// Supported API versions: 5
    pub protocol_name: Option<StrBytes>,

    /// Each assignment.
    /// 
    /// Supported API versions: 0-5
    pub assignments: Vec<SyncGroupRequestAssignment>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Encodable for SyncGroupRequest {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 4 {
            types::CompactString.encode(buf, &self.group_id)?;
        } else {
            types::String.encode(buf, &self.group_id)?;
        }
        types::Int32.encode(buf, &self.generation_id)?;
        if version >= 4 {
            types::CompactString.encode(buf, &self.member_id)?;
        } else {
            types::String.encode(buf, &self.member_id)?;
        }
        if version >= 3 {
            if version >= 4 {
                types::CompactString.encode(buf, &self.group_instance_id)?;
            } else {
                types::String.encode(buf, &self.group_instance_id)?;
            }
        } else {
            if !self.group_instance_id.is_none() {
                return Err(EncodeError)
            }
        }
        if version >= 5 {
            types::CompactString.encode(buf, &self.protocol_type)?;
        }
        if version >= 5 {
            types::CompactString.encode(buf, &self.protocol_name)?;
        }
        if version >= 4 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.assignments)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.assignments)?;
        }
        if version >= 4 {
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
        if version >= 4 {
            total_size += types::CompactString.compute_size(&self.group_id)?;
        } else {
            total_size += types::String.compute_size(&self.group_id)?;
        }
        total_size += types::Int32.compute_size(&self.generation_id)?;
        if version >= 4 {
            total_size += types::CompactString.compute_size(&self.member_id)?;
        } else {
            total_size += types::String.compute_size(&self.member_id)?;
        }
        if version >= 3 {
            if version >= 4 {
                total_size += types::CompactString.compute_size(&self.group_instance_id)?;
            } else {
                total_size += types::String.compute_size(&self.group_instance_id)?;
            }
        } else {
            if !self.group_instance_id.is_none() {
                return Err(EncodeError)
            }
        }
        if version >= 5 {
            total_size += types::CompactString.compute_size(&self.protocol_type)?;
        }
        if version >= 5 {
            total_size += types::CompactString.compute_size(&self.protocol_name)?;
        }
        if version >= 4 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.assignments)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.assignments)?;
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

impl Decodable for SyncGroupRequest {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let group_id = if version >= 4 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let generation_id = types::Int32.decode(buf)?;
        let member_id = if version >= 4 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let group_instance_id = if version >= 3 {
            if version >= 4 {
                types::CompactString.decode(buf)?
            } else {
                types::String.decode(buf)?
            }
        } else {
            None
        };
        let protocol_type = if version >= 5 {
            types::CompactString.decode(buf)?
        } else {
            None
        };
        let protocol_name = if version >= 5 {
            types::CompactString.decode(buf)?
        } else {
            None
        };
        let assignments = if version >= 4 {
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
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            group_id,
            generation_id,
            member_id,
            group_instance_id,
            protocol_type,
            protocol_name,
            assignments,
            unknown_tagged_fields,
        })
    }
}

impl Default for SyncGroupRequest {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            generation_id: 0,
            member_id: Default::default(),
            group_instance_id: None,
            protocol_type: None,
            protocol_name: None,
            assignments: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for SyncGroupRequest {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 5 };
}

impl HeaderVersion for SyncGroupRequest {
    fn header_version(version: i16) -> i16 {
        if version >= 4 {
            2
        } else {
            1
        }
    }
}

